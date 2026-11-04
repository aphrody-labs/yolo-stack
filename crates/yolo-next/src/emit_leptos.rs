// SPDX-License-Identifier: Apache-2.0
//! Leptos 0.8 lane emitter: `Cargo.toml`, `src/app.rs` (`leptos_router`), `src/main.rs` (axum).
//!
//! STUB: signature is the contract; implementation lands in the `emit` work slice.

use crate::types::{EmittedFile, ProjectAnalysis};

/// Emits the Leptos project.
pub fn emit_leptos_project(_analysis: &ProjectAnalysis) -> Vec<EmittedFile> {
    Vec::new()
}
