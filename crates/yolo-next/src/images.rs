// SPDX-License-Identifier: Apache-2.0
//! `next/image`: usage extraction, `remotePatterns` matching, loaders, `srcset` algorithm,
//! optimizer policy and Manganis `asset!()` equivalents.
//!
//! STUB: signatures are the contract; implementation lands in the `images` work slice.

use serde_json::Value;

use crate::types::{
    ImageLoaderKind, ImageOptimizerPolicy, ImageUsage, ImageUsageReport, ImagesConfig,
    LocalPattern, RemotePattern, SrcSetCandidate,
};

/// Extracts `<Image ... />` usages from a module.
pub fn extract_image_usages(_source: &str, _file: &str) -> Vec<ImageUsage> {
    Vec::new()
}

/// Matches an absolute URL against a remote pattern (`*` one label, `**` any labels).
pub fn remote_pattern_matches(_pattern: &RemotePattern, _url: &str) -> bool {
    false
}

/// Matches a local path + query against a local pattern.
pub fn local_pattern_matches(_pattern: &LocalPattern, _path_and_query: &str) -> bool {
    false
}

/// Loader kind from config (`loader` + `loaderFile`).
pub fn loader_kind(_cfg: &ImagesConfig) -> ImageLoaderKind {
    ImageLoaderKind::Default
}

/// Builds the optimizer policy (`/_yolo/image` + `/_next/image` compat).
pub fn optimizer_policy(cfg: &ImagesConfig, _base_path: &str) -> ImageOptimizerPolicy {
    ImageOptimizerPolicy {
        route: String::from("/_yolo/image"),
        compat_routes: vec![String::from("/_next/image")],
        loader: ImageLoaderKind::Default,
        loader_file: None,
        formats: cfg.formats.clone(),
        device_sizes: cfg.device_sizes.clone(),
        image_sizes: cfg.image_sizes.clone(),
        qualities: cfg.qualities.clone(),
        minimum_cache_ttl: cfg.minimum_cache_ttl,
        cache_control: String::from("public, max-age={max_age}, must-revalidate"),
        allow_svg: cfg.dangerously_allow_svg,
        allow_local_ip: cfg.dangerously_allow_local_ip,
        content_security_policy: cfg.content_security_policy.clone(),
        content_disposition_type: cfg.content_disposition_type.clone(),
        remote_patterns: cfg.remote_patterns.clone(),
        local_patterns: cfg.local_patterns.clone(),
        maximum_redirects: cfg.maximum_redirects,
        maximum_response_body: cfg.maximum_response_body,
        unoptimized: cfg.unoptimized,
    }
}

/// Builds the URL of one candidate for a loader kind.
pub fn loader_url(
    _kind: ImageLoaderKind,
    _policy: &ImageOptimizerPolicy,
    src: &str,
    _width: u32,
    _quality: u8,
) -> String {
    src.to_owned()
}

/// Coerces a requested quality to the nearest allowed value (default 75).
pub fn coerce_quality(_policy: &ImageOptimizerPolicy, quality: Option<u8>) -> u8 {
    quality.unwrap_or(75)
}

/// next/image `srcset` algorithm (`w` descriptors with `sizes`/`fill`, else `1x`/`2x`).
pub fn srcset(
    _usage: &ImageUsage,
    _policy: &ImageOptimizerPolicy,
    _kind: ImageLoaderKind,
) -> Vec<SrcSetCandidate> {
    Vec::new()
}

/// Negotiates the output format from an `Accept` header.
pub fn negotiate_format<'a>(_accept: &str, _formats: &'a [String]) -> Option<&'a str> {
    None
}

/// Manganis `asset!()` expression equivalent of a usage.
pub fn manganis_asset_expr(_usage: &ImageUsage, _policy: &ImageOptimizerPolicy) -> String {
    String::new()
}

/// Aggregates usages into a report (remote hosts, unmatched patterns).
pub fn image_usage_report(usages: Vec<ImageUsage>, _cfg: &ImagesConfig) -> ImageUsageReport {
    ImageUsageReport {
        usages,
        ..ImageUsageReport::default()
    }
}

/// `.yolo/images.json` manifest.
pub fn images_manifest_json(_policy: &ImageOptimizerPolicy) -> Value {
    Value::Null
}
