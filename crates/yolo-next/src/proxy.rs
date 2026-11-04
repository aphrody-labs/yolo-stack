// SPDX-License-Identifier: Apache-2.0
//! `proxy.ts` (and deprecated `middleware.ts`) analysis and skeleton emitters.
//!
//! STUB: signatures are the contract; implementation lands in the `proxy` work slice.

use std::path::{Path, PathBuf};

use crate::types::ProxySpec;

/// Finds `proxy.(ts|js)` / `middleware.(ts|js)` at the root or under `src/`.
pub fn find_proxy(_root: &Path) -> Option<PathBuf> {
    None
}

/// Analyzes the proxy module (matchers, NextResponse usage).
pub fn analyze_proxy(_source: &str, file: &str) -> ProxySpec {
    ProxySpec {
        file: file.to_owned(),
        ..ProxySpec::default()
    }
}

/// Emits a Bun `proxy` wrapper (`src/proxy.ts`) honouring the matchers.
pub fn emit_bun_proxy(_spec: &ProxySpec) -> String {
    String::new()
}

/// Emits an axum/tower middleware skeleton (`src/proxy.rs`).
pub fn emit_axum_proxy(_spec: &ProxySpec) -> String {
    String::new()
}
