// SPDX-License-Identifier: Apache-2.0
//! Route segment config exports, `generate*` detection, directives and module facts.
//!
//! STUB: signatures are the contract; implementation lands in the `segment` work slice.

use crate::types::SegmentConfig;

/// Analyzes a page/layout/route module.
pub fn analyze_segment_module(_source: &str, _file: &str) -> SegmentConfig {
    SegmentConfig::default()
}

/// HTTP methods exported by a route handler (`GET`, `HEAD`, `POST`, `PUT`, `PATCH`, `DELETE`, `OPTIONS`).
pub fn route_handler_methods(_source: &str) -> Vec<String> {
    Vec::new()
}

/// Legacy Pages Router data functions exported by a module.
pub fn legacy_data_functions(_source: &str) -> Vec<String> {
    Vec::new()
}
