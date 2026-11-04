//! Modele d'apparence coloree CAM16 (Li et al., 2017), formulation Material Color Utilities.

use crate::srgb::{WHITE_POINT_D65, mul_matrix, y_from_lstar};

const XYZ_TO_CAM16RGB: [[f64; 3]; 3] = [
    [0.401_288, 0.650_173, -0.051_461],
    [-0.250_268, 1.204_414, 0.045_854],
    [-0.002_079, 0.048_952, 0.953_127],
];

const CAM16RGB_TO_XYZ: [[f64; 3]; 3] = [
    [1.862_067_86, -1.011_254_63, 0.149_186_77],
    [0.387_526_54, 0.621_447_44, -0.008_973_98],
    [-0.015_841_50, -0.034_122_94, 1.049_964_44],
];

/// Conditions de vision CAM16 (luminance adaptante, fond, surround, adaptation chromatique).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewingConditions {
    /// Facteur de fond normalise `n`.
    pub n: f64,
    /// Reponse achromatique du blanc.
    pub aw: f64,
    /// Facteur d'induction de fond.
    pub nbb: f64,
    /// Facteur d'induction chromatique.
    pub ncb: f64,
    /// Impact du surround.
    pub c: f64,
    /// Facteur d'induction chromatique du surround.
    pub nc: f64,
    /// Degre d'adaptation applique a chaque canal.
    pub rgb_d: [f64; 3],
    /// Facteur de niveau de luminance.
    pub fl: f64,
    /// `fl` a la puissance 0.25.
    pub fl_root: f64,
    /// Exposant de base `z`.
    pub z: f64,
}

impl ViewingConditions {
    /// Conditions par defaut : luminance adaptante 200/pi * Y(L* = 50), fond L* = 50,
    /// surround moyen (2.0), illuminant non actualise.
    #[must_use]
    pub fn standard() -> Self {
        Self::new(
            WHITE_POINT_D65,
            200.0 / std::f64::consts::PI * y_from_lstar(50.0) / 100.0,
            50.0,
            2.0,
            false,
        )
    }

    /// Construit des conditions de vision personnalisees.
    #[must_use]
    pub fn new(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> Self {
        let rgb_w = mul_matrix(&XYZ_TO_CAM16RGB, white_point);
        let f = 0.8 + surround / 10.0;
        let c = if f >= 0.9 {
            lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };
        let d = if discounting_illuminant {
            1.0
        } else {
            (f * (1.0 - (1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())).clamp(0.0, 1.0)
        };
        let nc = f;
        let rgb_d = [
            d * (100.0 / rgb_w[0]) + 1.0 - d,
            d * (100.0 / rgb_w[1]) + 1.0 - d,
            d * (100.0 / rgb_w[2]) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = k4 * adapting_luminance + 0.1 * k4f * k4f * (5.0 * adapting_luminance).cbrt();
        let n = y_from_lstar(background_lstar) / white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;
        let rgb_a: [f64; 3] = std::array::from_fn(|i| {
            let af = (fl * rgb_d[i] * rgb_w[i] / 100.0).powf(0.42);
            400.0 * af / (af + 27.13)
        });
        let aw = (2.0 * rgb_a[0] + rgb_a[1] + 0.05 * rgb_a[2]) * nbb;
        Self { n, aw, nbb, ncb, c, nc, rgb_d, fl, fl_root: fl.powf(0.25), z }
    }
}

impl Default for ViewingConditions {
    fn default() -> Self {
        Self::standard()
    }
}

fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

/// Couleur exprimee dans les dimensions CAM16.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cam16 {
    /// Teinte en degres `[0, 360)`.
    pub hue: f64,
    /// Chroma `C`.
    pub chroma: f64,
    /// Clarte `J`.
    pub j: f64,
    /// Luminosite `Q`.
    pub q: f64,
    /// Coloration `M`.
    pub m: f64,
    /// Saturation `s`.
    pub s: f64,
}

impl Cam16 {
    /// Calcule l'apparence CAM16 d'une couleur XYZ (0..100) dans les conditions donnees.
    #[must_use]
    pub fn from_xyz(xyz: [f64; 3], vc: &ViewingConditions) -> Self {
        let rgb_c = mul_matrix(&XYZ_TO_CAM16RGB, xyz);
        let rgb_a: [f64; 3] = std::array::from_fn(|i| {
            let d = rgb_c[i] * vc.rgb_d[i];
            let af = (vc.fl * d.abs() / 100.0).powf(0.42);
            d.signum() * 400.0 * af / (af + 27.13)
        });
        let [ra, ga, ba] = rgb_a;

        let a = (11.0 * ra + -12.0 * ga + ba) / 11.0;
        let b = (ra + ga - 2.0 * ba) / 9.0;
        let u = (20.0 * ra + 20.0 * ga + 21.0 * ba) / 20.0;
        let p2 = (40.0 * ra + 20.0 * ga + ba) / 20.0;

        let hue = sanitize_degrees(b.atan2(a).to_degrees());

        let ac = p2 * vc.nbb;
        let j = 100.0 * (ac / vc.aw).powf(vc.c * vc.z);
        let q = 4.0 / vc.c * (j / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;

        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * vc.nc * vc.ncb;
        let t = p1 * a.hypot(b) / (u + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29_f64.powf(vc.n)).powf(0.73);
        let chroma = alpha * (j / 100.0).sqrt();
        let m = chroma * vc.fl_root;
        let s = 50.0 * (alpha * vc.c / (vc.aw + 4.0)).sqrt();
        Self { hue, chroma, j, q, m, s }
    }

    /// Construit une couleur CAM16 depuis `(J, C, h)` ; les attributs derives sont recalcules.
    #[must_use]
    pub fn from_jch(j: f64, chroma: f64, hue: f64, vc: &ViewingConditions) -> Self {
        let q = 4.0 / vc.c * (j / 100.0).sqrt() * (vc.aw + 4.0) * vc.fl_root;
        let m = chroma * vc.fl_root;
        let alpha = if j <= 0.0 {
            0.0
        } else {
            chroma / (j / 100.0).sqrt()
        };
        let s = 50.0 * (alpha * vc.c / (vc.aw + 4.0)).sqrt();
        Self { hue: sanitize_degrees(hue), chroma, j, q, m, s }
    }

    /// Inverse CAM16 -> XYZ (0..100) dans les conditions donnees.
    #[must_use]
    pub fn to_xyz(&self, vc: &ViewingConditions) -> [f64; 3] {
        let alpha = if self.chroma <= 0.0 || self.j <= 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };
        let t = (alpha / (1.64 - 0.29_f64.powf(vc.n)).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = self.hue.to_radians();
        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = vc.aw * (self.j / 100.0).powf(1.0 / vc.c / vc.z);
        let p1 = e_hue * (50000.0 / 13.0) * vc.nc * vc.ncb;
        let p2 = ac / vc.nbb;
        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let rgb_a = [
            (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0,
            (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0,
            (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0,
        ];
        let rgb_f: [f64; 3] = std::array::from_fn(|i| {
            let v = rgb_a[i];
            let base = (27.13 * v.abs() / (400.0 - v.abs())).max(0.0);
            let c = v.signum() * (100.0 / vc.fl) * base.powf(1.0 / 0.42);
            c / vc.rgb_d[i]
        });
        mul_matrix(&CAM16RGB_TO_XYZ, rgb_f)
    }
}

/// Ramene un angle dans `[0, 360)`.
#[must_use]
pub fn sanitize_degrees(degrees: f64) -> f64 {
    let d = degrees % 360.0;
    if d < 0.0 { d + 360.0 } else { d }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::srgb::Argb;

    fn close(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn standard_viewing_conditions_match_reference() {
        let vc = ViewingConditions::standard();
        assert!(close(vc.n, 0.184_186_518_512_444_16, 1e-9));
        assert!(close(vc.aw, 29.980_997_194_447_333, 1e-6));
        assert!(close(vc.nbb, 1.016_919_180_445_875_5, 1e-9));
        assert!(close(vc.c, 0.69, 1e-12));
        assert!(close(vc.nc, 1.0, 1e-12));
        assert!(close(vc.fl, 0.388_481_453_780_035_3, 1e-9));
        assert!(close(vc.z, 1.909_169_568_483_652, 1e-9));
        assert!(close(vc.rgb_d[0], 1.021_177_702_757_52, 1e-9));
    }

    #[test]
    fn primaries_match_material_reference_values() {
        let vc = ViewingConditions::standard();
        let red = Cam16::from_xyz(Argb::from_rgb(255, 0, 0).to_xyz(), &vc);
        assert!(close(red.hue, 27.408, 0.01), "{}", red.hue);
        assert!(close(red.chroma, 113.357, 0.01), "{}", red.chroma);
        let green = Cam16::from_xyz(Argb::from_rgb(0, 255, 0).to_xyz(), &vc);
        assert!(close(green.hue, 142.139, 0.01), "{}", green.hue);
        assert!(close(green.chroma, 108.410, 0.01), "{}", green.chroma);
        let blue = Cam16::from_xyz(Argb::from_rgb(0, 0, 255).to_xyz(), &vc);
        assert!(close(blue.hue, 282.788, 0.01), "{}", blue.hue);
        assert!(close(blue.chroma, 87.230, 0.01), "{}", blue.chroma);
    }

    #[test]
    fn xyz_round_trip_through_cam16() {
        let vc = ViewingConditions::standard();
        for argb in [
            0xFF00_0000u32,
            0xFFFF_FFFF,
            0xFF12_3456,
            0xFF80_8080,
            0xFFE8_431F,
            0xFF67_50A4,
        ] {
            let xyz = Argb(argb).to_xyz();
            let cam = Cam16::from_xyz(xyz, &vc);
            let back = cam.to_xyz(&vc);
            for i in 0..3 {
                assert!(
                    close(xyz[i], back[i], 1e-6),
                    "{argb:#x}: {xyz:?} vs {back:?}"
                );
            }
        }
    }

    #[test]
    fn sanitize() {
        assert!(close(sanitize_degrees(-10.0), 350.0, 1e-12));
        assert!(close(sanitize_degrees(370.0), 10.0, 1e-12));
        assert!(close(sanitize_degrees(0.0), 0.0, 1e-12));
    }
}
