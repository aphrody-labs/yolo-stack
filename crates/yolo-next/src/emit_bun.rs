// SPDX-License-Identifier: Apache-2.0
//! Bun lane emitter: `yolo.config.ts`, `bunfig.toml`, `src/server.ts`, TanStack route tree, `.yolo/*.json`.
//!
//! STUB: signature is the contract; implementation lands in the `emit` work slice.

use crate::types::{EmittedFile, ProjectAnalysis};

/// Emits the Bun + TanStack project.
pub fn emit_bun_project(_analysis: &ProjectAnalysis) -> Vec<EmittedFile> {
    Vec::new()
}
