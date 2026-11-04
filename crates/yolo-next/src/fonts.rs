// SPDX-License-Identifier: Apache-2.0
//! `next/font/google` and `next/font/local`: request extraction, Google CSS2 URLs, self-hosting
//! plan, SFNT metrics and size-adjusted fallbacks.
//!
//! STUB: signatures are the contract; implementation lands in the `fonts` work slice.

use serde_json::Value;
use yolo_shared::YoloThemePreset;

use crate::types::{
    FallbackOverrides, FontFaceAsset, FontMetrics, FontPlan, FontRequest,
};

/// User agent that makes Google Fonts return `woff2` + `unicode-range` subsets (next/font).
pub const GOOGLE_FONTS_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36";

/// Extracts `Inter({...})` / `localFont({...})` calls from a module.
pub fn extract_font_requests(_source: &str, _file: &str) -> Vec<FontRequest> {
    Vec::new()
}

/// Builds the `https://fonts.googleapis.com/css2?family=...` URL.
pub fn google_css2_url(_request: &FontRequest) -> Option<String> {
    None
}

/// Builds the plan (class names, family CSS, fallback) for one request, without network access.
pub fn plan_font(request: &FontRequest, _fonts_route: &str) -> FontPlan {
    FontPlan {
        request: request.clone(),
        css2_url: None,
        family_css: String::new(),
        class_name: String::new(),
        variable_class: None,
        fallback: None,
        assets: Vec::new(),
        css: String::new(),
        preset_role: None,
    }
}

/// Rewrites a Google CSS response: remote `url()` -> `<fonts_route>/<hash>.<ext>`, collecting assets.
pub fn rewrite_font_css(
    css: &str,
    _request: &FontRequest,
    _fonts_route: &str,
) -> (String, Vec<FontFaceAsset>) {
    (css.to_owned(), Vec::new())
}

/// Parses `head`, `hhea`, `OS/2` tables of a TrueType/OpenType (SFNT) font.
pub fn parse_sfnt_metrics(_bytes: &[u8], _family: &str) -> Option<FontMetrics> {
    None
}

/// Arial metrics (`xAvgCharWidth` 904, `unitsPerEm` 2048).
pub fn arial_metrics() -> FontMetrics {
    FontMetrics {
        family: String::from("Arial"),
        units_per_em: 2048,
        ascent: 1854,
        descent: -434,
        line_gap: 67,
        cap_height: Some(1467),
        x_avg_char_width: 904,
        az_avg_width: Some(934.511_627_906_976_7),
    }
}

/// Times New Roman metrics (`xAvgCharWidth` 821, `unitsPerEm` 2048).
pub fn times_new_roman_metrics() -> FontMetrics {
    FontMetrics {
        family: String::from("Times New Roman"),
        units_per_em: 2048,
        ascent: 1825,
        descent: -443,
        line_gap: 87,
        cap_height: Some(1341),
        x_avg_char_width: 821,
        az_avg_width: Some(854.395_348_837_209_3),
    }
}

/// Computes `ascent-override`, `descent-override`, `line-gap-override`, `size-adjust` (next/font formula).
pub fn fallback_overrides(_main: &FontMetrics, fallback: &FontMetrics) -> FallbackOverrides {
    FallbackOverrides {
        fallback_family: fallback.family.clone(),
        ascent_override: String::new(),
        descent_override: String::new(),
        line_gap_override: String::new(),
        size_adjust: String::new(),
    }
}

/// Chooses Arial or Times New Roman for a request (explicit option, else serif heuristic).
pub fn fallback_font_for(_request: &FontRequest, _metrics: Option<&FontMetrics>) -> FontMetrics {
    arial_metrics()
}

/// `.class { font-family }` + `.variable { --var }` CSS for a plan.
pub fn font_class_css(_plan: &FontPlan) -> String {
    String::new()
}

/// `(sans, mono)` font stacks of a yolo-ui preset.
pub fn preset_font_stack(preset: YoloThemePreset) -> (String, String) {
    let style = preset.style();
    (style.font_family_ui.to_owned(), style.font_family_mono.to_owned())
}

/// `.yolo/fonts.json` manifest.
pub fn fonts_manifest_json(_plans: &[FontPlan]) -> Value {
    Value::Null
}
