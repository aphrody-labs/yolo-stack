//! Espace HCT : teinte et chroma CAM16, tonalite L* CIE.

use crate::cam16::{Cam16, ViewingConditions, sanitize_degrees};
use crate::srgb::{Argb, argb_from_lstar, lstar_from_y, xyz_to_linear_rgb};

/// Couleur HCT. `hue` en degres `[0, 360)`, `chroma >= 0`, `tone` dans `[0, 100]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hct {
    hue: f64,
    chroma: f64,
    tone: f64,
    argb: Argb,
}

/// Tolerance de gamut sur les composantes lineaires (echelle 0..100).
const GAMUT_EPSILON: f64 = 1e-3;
/// Tolerance d'ecart de tonalite acceptee par le solveur.
const TONE_EPSILON: f64 = 0.01;
/// Pas d'arret de la bissection sur la chroma.
const CHROMA_EPSILON: f64 = 0.05;

impl Hct {
    /// Convertit une couleur sRGB en HCT (exact, sans resolution).
    #[must_use]
    pub fn from_argb(argb: Argb) -> Self {
        let vc = ViewingConditions::standard();
        let xyz = argb.to_xyz();
        let cam = Cam16::from_xyz(xyz, &vc);
        Self { hue: cam.hue, chroma: cam.chroma, tone: lstar_from_y(xyz[1]), argb }
    }

    /// Construit la couleur sRGB la plus proche de `(hue, chroma, tone)` dans le gamut.
    /// La chroma reelle peut etre inferieure a la demande si celle-ci est hors gamut ;
    /// la teinte et la tonalite sont conservees.
    #[must_use]
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        let argb = solve_to_argb(
            sanitize_degrees(hue),
            chroma.max(0.0),
            tone.clamp(0.0, 100.0),
        );
        Self::from_argb(argb)
    }

    /// Teinte CAM16 en degres.
    #[must_use]
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma CAM16.
    #[must_use]
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Tonalite (L* CIE).
    #[must_use]
    pub fn tone(&self) -> f64 {
        self.tone
    }

    /// Couleur sRGB correspondante.
    #[must_use]
    pub fn to_argb(&self) -> Argb {
        self.argb
    }

    /// Copie avec une nouvelle teinte (re-resolue dans le gamut).
    #[must_use]
    pub fn with_hue(&self, hue: f64) -> Self {
        Self::new(hue, self.chroma, self.tone)
    }

    /// Copie avec une nouvelle chroma (re-resolue dans le gamut).
    #[must_use]
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::new(self.hue, chroma, self.tone)
    }

    /// Copie avec une nouvelle tonalite (re-resolue dans le gamut).
    #[must_use]
    pub fn with_tone(&self, tone: f64) -> Self {
        Self::new(self.hue, self.chroma, tone)
    }
}

impl From<Argb> for Hct {
    fn from(argb: Argb) -> Self {
        Self::from_argb(argb)
    }
}

impl From<Hct> for Argb {
    fn from(hct: Hct) -> Self {
        hct.to_argb()
    }
}

fn in_gamut(linear_rgb: [f64; 3]) -> bool {
    linear_rgb
        .iter()
        .all(|&c| (-GAMUT_EPSILON..=100.0 + GAMUT_EPSILON).contains(&c))
}

/// Pour une teinte et une chroma fixees, cherche par bissection le `J` dont la couleur
/// sRGB a la tonalite demandee. Renvoie `None` si aucune couleur du gamut ne convient.
fn find_argb_by_tone(hue: f64, chroma: f64, tone: f64, vc: &ViewingConditions) -> Option<Argb> {
    let mut low = 0.0_f64;
    let mut high = 100.0_f64;
    let mut best: Option<(f64, Argb)> = None;
    for _ in 0..64 {
        let mid = low + (high - low) / 2.0;
        let xyz = Cam16::from_jch(mid, chroma, hue, vc).to_xyz(vc);
        let linear = xyz_to_linear_rgb(xyz);
        let lstar = lstar_from_y(xyz[1]);
        let delta = (lstar - tone).abs();
        if in_gamut(linear) && delta < TONE_EPSILON * 20.0 {
            let candidate = Argb::from_xyz(xyz);
            let candidate_delta = (candidate.lstar() - tone).abs();
            if best.is_none_or(|(d, _)| candidate_delta < d) {
                best = Some((candidate_delta, candidate));
            }
        }
        if delta < TONE_EPSILON {
            break;
        }
        if lstar < tone {
            low = mid;
        } else {
            high = mid;
        }
        if high - low < 1e-9 {
            break;
        }
    }
    best.map(|(_, argb)| argb)
}

/// Resolution HCT -> sRGB : bissection sur la chroma jusqu'a trouver la plus grande chroma
/// (au plus la demande) representable a la tonalite voulue.
fn solve_to_argb(hue: f64, chroma: f64, tone: f64) -> Argb {
    if chroma < CHROMA_EPSILON || tone < 1e-4 || tone > 100.0 - 1e-4 {
        return argb_from_lstar(tone);
    }
    let vc = ViewingConditions::standard();
    if let Some(argb) = find_argb_by_tone(hue, chroma, tone, &vc) {
        return argb;
    }
    let mut low = 0.0_f64;
    let mut high = chroma;
    let mut best = argb_from_lstar(tone);
    while high - low >= CHROMA_EPSILON {
        let mid = low + (high - low) / 2.0;
        match find_argb_by_tone(hue, mid, tone, &vc) {
            Some(argb) => {
                best = argb;
                low = mid;
            }
            None => high = mid,
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn primaries_hct_reference() {
        let blue = Hct::from_argb(Argb::from_rgb(0, 0, 255));
        assert!(close(blue.hue(), 282.788, 0.01));
        assert!(close(blue.chroma(), 87.230, 0.01));
        assert!(close(blue.tone(), 32.302, 0.01), "{}", blue.tone());
        let red = Hct::from_argb(Argb::from_rgb(255, 0, 0));
        assert!(close(red.tone(), 53.237, 0.01), "{}", red.tone());
        let green = Hct::from_argb(Argb::from_rgb(0, 255, 0));
        assert!(close(green.tone(), 87.737, 0.01), "{}", green.tone());
    }

    #[test]
    fn achromatic_extremes() {
        assert_eq!(Hct::new(0.0, 0.0, 0.0).to_argb(), Argb::BLACK);
        assert_eq!(Hct::new(120.0, 50.0, 100.0).to_argb(), Argb::WHITE);
        assert_eq!(
            Hct::new(200.0, 0.0, 50.0).to_argb(),
            Argb::from_rgb(119, 119, 119)
        );
        let white = Hct::from_argb(Argb::WHITE);
        assert!(white.chroma() < 3.0, "{}", white.chroma());
        assert!(close(white.tone(), 100.0, 0.01));
    }

    #[test]
    fn round_trip_in_gamut_colors_is_tight() {
        let samples = [
            0xFF67_50A4u32,
            0xFF00_78D4,
            0xFF00_7AFF,
            0xFFE8_431F,
            0xFF12_3456,
            0xFF80_8080,
            0xFFFF_7A1A,
            0xFF3B_8F5A,
            0xFFC0_392B,
            0xFF9B_59B6,
        ];
        for &raw in &samples {
            let source = Argb(raw);
            let hct = Hct::from_argb(source);
            let back = Hct::new(hct.hue(), hct.chroma(), hct.tone()).to_argb();
            let dr = source.red().abs_diff(back.red());
            let dg = source.green().abs_diff(back.green());
            let db = source.blue().abs_diff(back.blue());
            assert!(
                dr <= 2 && dg <= 2 && db <= 2,
                "{} -> {} (hct {:.2}/{:.2}/{:.2})",
                source.to_hex(),
                back.to_hex(),
                hct.hue(),
                hct.chroma(),
                hct.tone()
            );
        }
    }

    #[test]
    fn out_of_gamut_request_keeps_hue_and_tone() {
        let requested = Hct::new(282.0, 200.0, 70.0);
        assert!(close(requested.tone(), 70.0, 0.5), "{}", requested.tone());
        assert!(close(requested.hue(), 282.0, 1.5), "{}", requested.hue());
        assert!(requested.chroma() < 200.0);
        assert!(requested.chroma() > 5.0);
    }

    #[test]
    fn tone_sweep_is_monotonic_in_luminance() {
        let mut previous = -1.0;
        for tone in (0..=100).step_by(5) {
            let argb = Hct::new(27.0, 60.0, f64::from(tone)).to_argb();
            let y = argb.luminance_y();
            assert!(y >= previous, "tone {tone}: {y} < {previous}");
            previous = y;
        }
    }
}
