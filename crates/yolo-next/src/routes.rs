// SPDX-License-Identifier: Apache-2.0
//! App Router (`app/`) and Pages Router (`pages/`) file-system scanner and route matcher.
//!
//! STUB: signatures are the contract; implementation lands in the `routes` work slice.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;

use crate::types::{NextConfig, RouteEntry, RouteManifest, SegmentKind};

/// A matched route with captured params.
#[derive(Debug, Clone, PartialEq)]
pub struct RouteMatch<'a> {
    /// Matched entry.
    pub entry: &'a RouteEntry,
    /// Captured params (`slug` -> value; catch-all values joined with `/`).
    pub params: BTreeMap<String, String>,
}

/// Scans a project root (detects `app/`, `src/app/`, `pages/`, `src/pages/`, proxy, instrumentation).
pub fn scan_project(root: &Path) -> RouteManifest {
    RouteManifest {
        root: root.to_string_lossy().replace('\\', "/"),
        ..RouteManifest::default()
    }
}

/// Scans an `app/` directory (relative to `root`).
pub fn scan_app_dir(root: &Path, _app_dir: &str) -> RouteManifest {
    scan_project(root)
}

/// Adds legacy `pages/` entries to a manifest.
pub fn scan_pages_dir(_root: &Path, _pages_dir: &str, _manifest: &mut RouteManifest) {}

/// Classifies a directory name: kind, param, intercept level, intercept target.
pub fn classify_segment(_name: &str) -> (SegmentKind, Option<String>, Option<u8>, Option<String>) {
    (SegmentKind::Static, None, None, None)
}

/// `/blog/[slug]` -> `/blog/:slug`; `[...slug]`/`[[...slug]]` -> `/*`.
pub fn to_bun_pattern(url_pattern: &str) -> String {
    url_pattern.to_owned()
}

/// `/blog/[slug]` -> `/blog/$slug`; catch-all -> `/docs/$`.
pub fn to_tanstack_path(url_pattern: &str) -> String {
    url_pattern.to_owned()
}

/// `/blog/[slug]` -> `/blog/:slug`; catch-all -> `/docs/:..slug`.
pub fn to_dioxus_route(url_pattern: &str) -> String {
    url_pattern.to_owned()
}

/// `/blog/[slug]` -> `BlogSlug`.
pub fn to_dioxus_variant(url_pattern: &str) -> String {
    url_pattern.to_owned()
}

/// `/blog/[slug]` -> `/blog/:slug`; catch-all -> `/docs/*slug`.
pub fn to_leptos_path(url_pattern: &str) -> String {
    url_pattern.to_owned()
}

/// Sorts entries by matching precedence (static > dynamic > catch-all > optional catch-all).
pub fn sort_entries(_entries: &mut [RouteEntry]) {}

/// Matches a pathname against the manifest.
pub fn match_path<'a>(_manifest: &'a RouteManifest, _path: &str) -> Option<RouteMatch<'a>> {
    None
}

/// `routes-manifest.json`-shaped JSON (static/dynamic routes with regexes).
pub fn routes_manifest_json(_manifest: &RouteManifest, _cfg: &NextConfig) -> Value {
    Value::Null
}

/// `app-paths-manifest.json`-shaped JSON (`/page` -> module path).
pub fn app_paths_manifest_json(_manifest: &RouteManifest) -> Value {
    Value::Null
}
