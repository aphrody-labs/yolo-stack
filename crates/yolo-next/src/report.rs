// SPDX-License-Identifier: Apache-2.0
//! Feature parity matrix and migration report (JSON + Markdown).
//!
//! STUB: signatures are the contract; implementation lands in the `report` work slice.

use std::collections::BTreeMap;

use crate::types::{EmittedFile, FeatureStatus, MigrationReport, ProjectAnalysis, Target};

/// Computes the parity score (mapped = 1, partial = 0.5, unsupported = 0) as 0-100.
pub fn parity_score(_features: &[FeatureStatus]) -> u8 {
    0
}

/// Builds the feature matrix from an analysis.
pub fn feature_matrix(_analysis: &ProjectAnalysis) -> Vec<FeatureStatus> {
    Vec::new()
}

/// Builds the final report.
pub fn build_report(
    analysis: &ProjectAnalysis,
    targets: &[Target],
    files: &[EmittedFile],
) -> MigrationReport {
    MigrationReport {
        project: analysis.name.clone(),
        generated_at: String::new(),
        targets: targets.to_vec(),
        features: Vec::new(),
        parity_score: 0,
        diagnostics: analysis.diagnostics.clone(),
        files_emitted: files.iter().map(|f| f.path.clone()).collect(),
        counts: BTreeMap::new(),
    }
}

/// Renders the report as Markdown.
pub fn report_markdown(_report: &MigrationReport) -> String {
    String::new()
}
