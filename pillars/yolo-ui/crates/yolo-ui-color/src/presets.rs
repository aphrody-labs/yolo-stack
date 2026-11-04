//! Couleurs de graine des 7 presets de marque de `yolo-ui`.
//!
//! Chaque preset est defini par une graine sRGB ; la palette tonale et le schema complet
//! sont derives via HCT. L'autorite sur ces valeurs est le pilier yolo-ui
//! (voir `pillars/yolo-ui/README.md`) ; les constantes ci-dessous sont les graines de depart.

use crate::hct::Hct;
use crate::palette::TonalPalette;
use crate::srgb::Argb;

/// Neutral : gris chaud quasi achromatique, base par defaut.
pub const NEUTRAL_SEED: Argb = Argb(0xFF77_7267);
/// M3 : violet de reference Material 3 (`#6750A4`).
pub const M3_SEED: Argb = Argb(0xFF67_50A4);
/// Fluent : bleu Microsoft Fluent (`#0078D4`).
pub const FLUENT_SEED: Argb = Argb(0xFF00_78D4);
/// Apple : bleu systeme Human Interface (`#007AFF`).
pub const APPLE_SEED: Argb = Argb(0xFF00_7AFF);
/// Omarchy : bleu-lavande Tokyo Night (`#7AA2F7`).
pub const OMARCHY_SEED: Argb = Argb(0xFF7A_A2F7);
/// Hyprland : cyan neon (`#33CCFF`).
pub const HYPRLAND_SEED: Argb = Argb(0xFF33_CCFF);
/// Aphrody : orange Shenron (`#E8431F`).
pub const APHRODY_SEED: Argb = Argb(0xFFE8_431F);

/// Les sept presets de marque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BrandPreset {
    /// Base neutre.
    Neutral,
    /// Material 3.
    M3,
    /// Microsoft Fluent.
    Fluent,
    /// Apple Human Interface.
    Apple,
    /// Omarchy (Tokyo Night).
    Omarchy,
    /// Hyprland.
    Hyprland,
    /// Aphrody.
    Aphrody,
}

impl BrandPreset {
    /// Tous les presets, dans l'ordre canonique.
    pub const ALL: [Self; 7] = [
        Self::Neutral,
        Self::M3,
        Self::Fluent,
        Self::Apple,
        Self::Omarchy,
        Self::Hyprland,
        Self::Aphrody,
    ];

    /// Identifiant stable (minuscules) utilise dans les tokens.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::M3 => "m3",
            Self::Fluent => "fluent",
            Self::Apple => "apple",
            Self::Omarchy => "omarchy",
            Self::Hyprland => "hyprland",
            Self::Aphrody => "aphrody",
        }
    }

    /// Couleur de graine du preset.
    #[must_use]
    pub const fn seed(self) -> Argb {
        match self {
            Self::Neutral => NEUTRAL_SEED,
            Self::M3 => M3_SEED,
            Self::Fluent => FLUENT_SEED,
            Self::Apple => APPLE_SEED,
            Self::Omarchy => OMARCHY_SEED,
            Self::Hyprland => HYPRLAND_SEED,
            Self::Aphrody => APHRODY_SEED,
        }
    }

    /// Graine exprimee en HCT.
    #[must_use]
    pub fn seed_hct(self) -> Hct {
        Hct::from_argb(self.seed())
    }

    /// Palette primaire : teinte et chroma de la graine, chroma minimale 48 (regle M3)
    /// sauf pour `Neutral` qui conserve sa chroma faible.
    #[must_use]
    pub fn primary_palette(self) -> TonalPalette {
        let hct = self.seed_hct();
        let chroma = if matches!(self, Self::Neutral) {
            hct.chroma()
        } else {
            hct.chroma().max(48.0)
        };
        TonalPalette::from_hue_chroma(hct.hue(), chroma)
    }

    /// Palette neutre derivee : meme teinte, chroma 4.
    #[must_use]
    pub fn neutral_palette(self) -> TonalPalette {
        TonalPalette::from_hue_chroma(self.seed_hct().hue(), 4.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_are_distinct_and_opaque() {
        let mut seeds: Vec<u32> = BrandPreset::ALL.iter().map(|p| p.seed().0).collect();
        seeds.sort_unstable();
        seeds.dedup();
        assert_eq!(seeds.len(), 7);
        for p in BrandPreset::ALL {
            assert_eq!(p.seed().alpha(), 0xFF, "{}", p.id());
        }
    }

    #[test]
    fn ids_are_unique_lowercase() {
        let mut ids: Vec<&str> = BrandPreset::ALL.iter().map(|p| p.id()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 7);
        assert!(ids.iter().all(|id| {
            id.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        }));
    }

    #[test]
    fn neutral_is_low_chroma_and_others_are_chromatic() {
        assert!(BrandPreset::Neutral.seed_hct().chroma() < 12.0);
        for p in BrandPreset::ALL
            .iter()
            .filter(|p| **p != BrandPreset::Neutral)
        {
            assert!(
                p.seed_hct().chroma() > 30.0,
                "{}: {}",
                p.id(),
                p.seed_hct().chroma()
            );
        }
    }

    #[test]
    fn primary_palettes_span_black_to_white() {
        for p in BrandPreset::ALL {
            let palette = p.primary_palette();
            assert_eq!(palette.tone(0), Argb::BLACK, "{}", p.id());
            assert_eq!(palette.tone(100), Argb::WHITE, "{}", p.id());
            let l = palette.tone(40).lstar();
            assert!((l - 40.0).abs() < 0.6, "{}: L* {l}", p.id());
        }
    }
}
