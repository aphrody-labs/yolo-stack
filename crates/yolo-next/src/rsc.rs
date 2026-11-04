// SPDX-License-Identifier: Apache-2.0
//! React Server Components module graph: `"use client"` / `"use server"` / `"use cache"`
//! classification, Flight client/server reference manifests, boundary violations.
//!
//! STUB: signatures are the contract; implementation lands in the `rsc` work slice.

use std::path::Path;

use serde_json::Value;

use crate::types::{Diagnostic, ModuleGraph, ModuleInfo};

/// Analyzes one module (directives, exports, imports, inline actions, cache functions, hooks).
pub fn analyze_module(_source: &str, id: &str) -> ModuleInfo {
    ModuleInfo {
        id: id.to_owned(),
        ..ModuleInfo::default()
    }
}

/// Scans `dirs` (relative to `root`) for `.ts/.tsx/.js/.jsx/.mjs` modules and builds the graph.
pub fn scan_module_graph(root: &Path, _dirs: &[&str]) -> ModuleGraph {
    ModuleGraph {
        root: root.to_string_lossy().replace('\\', "/"),
        ..ModuleGraph::default()
    }
}

/// 40-hex server reference id: `hex(0x00 ++ sha256(salt + module + ':' + export)[..19])`.
pub fn server_reference_id(_module: &str, _export_name: &str, _salt: &str) -> String {
    String::new()
}

/// `react-client-manifest.json`-shaped JSON (`clientModules`).
pub fn client_manifest_json(_graph: &ModuleGraph) -> Value {
    Value::Null
}

/// `server-reference-manifest.json`-shaped JSON.
pub fn server_manifest_json(_graph: &ModuleGraph) -> Value {
    Value::Null
}

/// TypeScript module map shim (`__webpack_require__`-compatible) for `react-server-dom-webpack`.
pub fn flight_bootstrap_ts(_graph: &ModuleGraph) -> String {
    String::new()
}

/// Server-only imports in client modules and other boundary violations.
pub fn boundary_violations(_module: &ModuleInfo) -> Vec<Diagnostic> {
    Vec::new()
}
