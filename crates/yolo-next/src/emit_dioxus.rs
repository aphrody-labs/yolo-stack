// SPDX-License-Identifier: Apache-2.0
//! Dioxus 0.8 lane emitter: `Cargo.toml`, `Dioxus.toml`, `src/main.rs` (`Routable`), server fns, assets.
//!
//! STUB: signature is the contract; implementation lands in the `emit` work slice.

use crate::types::{EmittedFile, ProjectAnalysis};

/// Emits the Dioxus fullstack project.
pub fn emit_dioxus_project(_analysis: &ProjectAnalysis) -> Vec<EmittedFile> {
    Vec::new()
}
