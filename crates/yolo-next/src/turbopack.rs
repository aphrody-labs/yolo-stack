// SPDX-License-Identifier: Apache-2.0
//! Turbopack configuration (`rules`, `resolveAlias`, `resolveExtensions`, `root`) -> Bun bundler plan.
//!
//! STUB: signatures are the contract; implementation lands in the `turbopack` work slice.

use crate::types::{BunBundlerPlan, BunLoader, BunPluginKind, NextConfig, Support, YoloWebConfig};

/// Translates the Turbopack section (plus `compiler.define`, `env`, `serverExternalPackages`) into a plan.
pub fn translate_turbopack(_cfg: &NextConfig) -> BunBundlerPlan {
    BunBundlerPlan::default()
}

/// Emits `bunfig.toml` (`[serve.static]` plugins/env/sourcemap, `[install]`, `[test]`).
pub fn bunfig_toml(_plan: &BunBundlerPlan, _cfg: &YoloWebConfig) -> String {
    String::new()
}

/// Emits `yolo.bundler.ts` exporting the `BunPlugin[]` array built from the plan.
pub fn bundler_plugins_ts(_plan: &BunBundlerPlan) -> String {
    String::new()
}

/// Default Bun loader for a file extension (`svg` -> `file`, `md` -> `text`, ...).
pub fn loader_for_extension(_ext: &str) -> Option<BunLoader> {
    None
}

/// Maps a webpack/Turbopack loader package to a Bun plugin kind.
pub fn map_turbopack_loader(_loader: &str) -> (BunPluginKind, Option<BunLoader>, Support) {
    (BunPluginKind::Unsupported, None, Support::Unsupported)
}
