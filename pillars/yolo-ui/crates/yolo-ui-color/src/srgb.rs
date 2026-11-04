//! Conversions sRGB <-> sRGB lineaire <-> CIE XYZ (illuminant D65).

/// Couleur sRGB 8 bits empaquetee en `0xAARRGGBB`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Argb(pub u32);

/// Point blanc D65 (Y = 100).
pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.412_338_95, 0.357_620_64, 0.180_510_42],
    [0.212_6, 0.715_2, 0.072_2],
    [0.019_321_41, 0.119_163_82, 0.950_344_78],
];

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.241_377_479_238_868_5,
        -1.537_665_240_285_185_1,
        -0.498_853_668_462_680_53,
    ],
    [
        -0.969_145_251_300_532_1,
        1.875_885_345_106_787_2,
        0.041_565_856_169_120_61,
    ],
    [
        0.055_620_936_896_913_05,
        -0.203_955_245_647_421_23,
        1.057_179_911_122_033_5,
    ],
];

impl Argb {
    /// Noir opaque.
    pub const BLACK: Self = Self(0xFF00_0000);
    /// Blanc opaque.
    pub const WHITE: Self = Self(0xFFFF_FFFF);

    /// Construit une couleur opaque a partir de ses composantes 8 bits.
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(0xFF00_0000 | (r as u32) << 16 | (g as u32) << 8 | b as u32)
    }

    /// Canal alpha.
    #[must_use]
    pub const fn alpha(self) -> u8 {
        (self.0 >> 24) as u8
    }

    /// Canal rouge.
    #[must_use]
    pub const fn red(self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// Canal vert.
    #[must_use]
    pub const fn green(self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Canal bleu.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.0 as u8
    }

    /// Representation hexadecimale `#rrggbb` (alpha omis).
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red(), self.green(), self.blue())
    }

    /// Convertit vers CIE XYZ (echelle 0..100).
    #[must_use]
    pub fn to_xyz(self) -> [f64; 3] {
        let r = linearize(self.red());
        let g = linearize(self.green());
        let b = linearize(self.blue());
        mul_matrix(&SRGB_TO_XYZ, [r, g, b])
    }

    /// Convertit depuis CIE XYZ (echelle 0..100) en ecretant dans le gamut sRGB.
    #[must_use]
    pub fn from_xyz(xyz: [f64; 3]) -> Self {
        let [r, g, b] = xyz_to_linear_rgb(xyz);
        Self::from_rgb(delinearize(r), delinearize(g), delinearize(b))
    }

    /// Luminance relative Y (0..100).
    #[must_use]
    pub fn luminance_y(self) -> f64 {
        self.to_xyz()[1]
    }

    /// Clarte CIE L* (0..100).
    #[must_use]
    pub fn lstar(self) -> f64 {
        lstar_from_y(self.luminance_y())
    }
}

impl From<u32> for Argb {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Argb> for u32 {
    fn from(value: Argb) -> Self {
        value.0
    }
}

/// Produit matrice 3x3 * vecteur.
#[must_use]
pub(crate) fn mul_matrix(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// XYZ (0..100) -> RGB lineaire (0..1, non ecrete).
#[must_use]
pub fn xyz_to_linear_rgb(xyz: [f64; 3]) -> [f64; 3] {
    mul_matrix(&XYZ_TO_SRGB, xyz)
}

/// Composante sRGB 8 bits -> valeur lineaire sur l'echelle 0..100.
#[must_use]
pub fn linearize(component: u8) -> f64 {
    let normalized = f64::from(component) / 255.0;
    if normalized <= 0.040_449_936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// Valeur lineaire (echelle 0..100) -> composante sRGB 8 bits ecretee.
#[must_use]
pub fn delinearize(linear: f64) -> u8 {
    let normalized = linear / 100.0;
    let delinearized = if normalized <= 0.003_130_8 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    (delinearized * 255.0).round().clamp(0.0, 255.0) as u8
}

const LAB_E: f64 = 216.0 / 24389.0;
const LAB_KAPPA: f64 = 24389.0 / 27.0;

fn lab_f(t: f64) -> f64 {
    if t > LAB_E {
        t.cbrt()
    } else {
        (LAB_KAPPA * t + 16.0) / 116.0
    }
}

fn lab_inv_f(ft: f64) -> f64 {
    let ft3 = ft * ft * ft;
    if ft3 > LAB_E {
        ft3
    } else {
        (116.0 * ft - 16.0) / LAB_KAPPA
    }
}

/// Luminance Y (0..100) -> clarte L* (0..100).
#[must_use]
pub fn lstar_from_y(y: f64) -> f64 {
    116.0 * lab_f(y / 100.0) - 16.0
}

/// Clarte L* (0..100) -> luminance Y (0..100).
#[must_use]
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_inv_f((lstar + 16.0) / 116.0)
}

/// Couleur grise (achromatique) de clarte L* donnee.
#[must_use]
pub fn argb_from_lstar(lstar: f64) -> Argb {
    let component = delinearize(y_from_lstar(lstar));
    Argb::from_rgb(component, component, component)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_round_trip() {
        let c = Argb::from_rgb(0x12, 0x34, 0x56);
        assert_eq!(c.0, 0xFF12_3456);
        assert_eq!(
            (c.red(), c.green(), c.blue(), c.alpha()),
            (0x12, 0x34, 0x56, 0xFF)
        );
        assert_eq!(c.to_hex(), "#123456");
    }

    #[test]
    fn white_maps_to_d65() {
        let xyz = Argb::WHITE.to_xyz();
        assert!((xyz[0] - WHITE_POINT_D65[0]).abs() < 0.05);
        assert!((xyz[1] - 100.0).abs() < 0.01);
        assert!((xyz[2] - WHITE_POINT_D65[2]).abs() < 0.05);
        assert!((Argb::WHITE.lstar() - 100.0).abs() < 0.01);
        assert!(Argb::BLACK.lstar().abs() < 1e-9);
    }

    #[test]
    fn xyz_round_trip_is_exact_on_8_bits() {
        for v in [0u8, 1, 7, 64, 127, 128, 200, 254, 255] {
            let c = Argb::from_rgb(v, 255 - v, v / 2);
            assert_eq!(Argb::from_xyz(c.to_xyz()), c);
        }
    }

    #[test]
    fn lstar_round_trip() {
        for l in [0.0, 10.0, 50.0, 87.3, 100.0] {
            assert!((lstar_from_y(y_from_lstar(l)) - l).abs() < 1e-9);
        }
        assert_eq!(argb_from_lstar(50.0), Argb::from_rgb(119, 119, 119));
    }
}
