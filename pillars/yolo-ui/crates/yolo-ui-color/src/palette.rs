//! Palettes tonales : une teinte et une chroma fixees, declinees sur les tonalites 0..=100.

use crate::hct::Hct;
use crate::srgb::Argb;

/// Palette tonale Material : `tone(t)` renvoie la couleur de tonalite `t` pour la paire
/// `(hue, chroma)` de la palette, ecretee dans le gamut.
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    hue: f64,
    chroma: f64,
    tones: Box<[Argb; 101]>,
}

/// Tonalites canoniques du systeme Material 3.
pub const M3_TONES: [u8; 13] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

impl TonalPalette {
    /// Construit une palette depuis une teinte et une chroma ; les 101 tonalites sont
    /// resolues immediatement.
    #[must_use]
    pub fn from_hue_chroma(hue: f64, chroma: f64) -> Self {
        let mut tones = Box::new([Argb::BLACK; 101]);
        for (t, slot) in tones.iter_mut().enumerate() {
            *slot = Hct::new(hue, chroma, t as f64).to_argb();
        }
        Self { hue, chroma, tones }
    }

    /// Construit la palette dont la teinte et la chroma sont celles de la couleur source.
    #[must_use]
    pub fn from_argb(argb: Argb) -> Self {
        let hct = Hct::from_argb(argb);
        Self::from_hue_chroma(hct.hue(), hct.chroma())
    }

    /// Teinte de la palette.
    #[must_use]
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma demandee de la palette.
    #[must_use]
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Couleur de tonalite `tone` (0..=100, ecrete).
    #[must_use]
    pub fn tone(&self, tone: u8) -> Argb {
        self.tones[usize::from(tone.min(100))]
    }

    /// Toutes les tonalites, index = tonalite.
    #[must_use]
    pub fn tones(&self) -> &[Argb; 101] {
        &self.tones
    }

    /// Les 13 tonalites canoniques Material 3, dans l'ordre de [`M3_TONES`].
    #[must_use]
    pub fn m3_tones(&self) -> [Argb; 13] {
        M3_TONES.map(|t| self.tone(t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extremes_are_black_and_white() {
        let palette = TonalPalette::from_hue_chroma(258.0, 48.0);
        assert_eq!(palette.tone(0), Argb::BLACK);
        assert_eq!(palette.tone(100), Argb::WHITE);
        assert_eq!(palette.tones().len(), 101);
        assert_eq!(palette.tone(200), Argb::WHITE);
    }

    #[test]
    fn luminance_is_non_decreasing() {
        let palette = TonalPalette::from_argb(Argb(0xFF67_50A4));
        let mut previous = -1.0;
        for argb in palette.tones() {
            let y = argb.luminance_y();
            assert!(y >= previous - 1e-9);
            previous = y;
        }
    }

    #[test]
    fn tone_matches_requested_lstar() {
        let palette = TonalPalette::from_hue_chroma(27.0, 40.0);
        for t in M3_TONES {
            let l = palette.tone(t).lstar();
            assert!((l - f64::from(t)).abs() < 0.6, "tone {t}: L* {l}");
        }
    }

    #[test]
    fn m3_tones_follow_order() {
        let palette = TonalPalette::from_hue_chroma(142.0, 30.0);
        let m3 = palette.m3_tones();
        assert_eq!(m3[0], Argb::BLACK);
        assert_eq!(m3[12], Argb::WHITE);
        assert_eq!(m3[4], palette.tone(40));
    }
}
