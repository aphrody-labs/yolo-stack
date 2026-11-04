//! Espace colorimetrique HCT (Hue, Chroma, Tone) pour `yolo-ui`.
//!
//! Pipeline : sRGB 8 bits -> sRGB lineaire -> CIE XYZ (D65) -> CAM16 -> HCT.
//! Le retour HCT -> sRGB utilise une resolution iterative (bissection sur J puis sur la chroma)
//! qui garantit un resultat dans le gamut sRGB. Les constantes et matrices proviennent du
//! modele CAM16 (Li et al., 2017) et des conditions de vision par defaut de Material Color
//! Utilities (luminance adaptante 200/pi * Y(L*=50), fond L* = 50, surround moyen).
//!
//! Aucune dependance externe : la crate est portable (wasm32, desktop, serveur).

#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod cam16;
pub mod hct;
pub mod palette;
pub mod presets;
pub mod srgb;

pub use cam16::{Cam16, ViewingConditions};
pub use hct::Hct;
pub use palette::TonalPalette;
pub use presets::BrandPreset;
pub use srgb::Argb;
