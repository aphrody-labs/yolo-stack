// SPDX-License-Identifier: Apache-2.0
//! CDN and static asset plan (`assetPrefix`, `basePath`, `deploymentId`, `output`).
//!
//! STUB: signatures are the contract; implementation lands in the `cdn` work slice.

use crate::types::{CdnAssetManifestEntry, CdnPlan, NextConfig};

/// Builds the CDN plan from a parsed `next.config`.
pub fn cdn_plan(_cfg: &NextConfig) -> CdnPlan {
    CdnPlan::default()
}

/// Returns the public URL of an output file (asset prefix + static route + skew query).
pub fn public_url(_plan: &CdnPlan, path: &str) -> String {
    path.to_owned()
}

/// Appends the skew protection query (`?dpl=<deploymentId>`) when configured.
pub fn with_skew(_plan: &CdnPlan, url: &str) -> String {
    url.to_owned()
}

/// `Cache-Control` value for a served path.
pub fn cache_control_for(plan: &CdnPlan, _path: &str) -> String {
    plan.public_cache_control.clone()
}

/// MIME type guessed from the extension.
pub fn content_type_for(_path: &str) -> &'static str {
    "application/octet-stream"
}

/// True when the file name carries a content hash (`name-<hash>.ext`).
pub fn is_hashed_asset(_path: &str) -> bool {
    false
}

/// Builds the upload manifest for `(path, size)` pairs.
pub fn asset_manifest(_plan: &CdnPlan, _files: &[(String, u64)]) -> Vec<CdnAssetManifestEntry> {
    Vec::new()
}

/// Emits a Bun script uploading hashed assets with `Bun.S3Client`.
pub fn upload_script_ts(_plan: &CdnPlan) -> String {
    String::new()
}
