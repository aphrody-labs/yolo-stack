// SPDX-License-Identifier: Apache-2.0
//! `redirects()`, `rewrites()`, `headers()` and proxy matchers: path-to-regexp sources ->
//! matchers, `Bun.serve` patterns, axum patterns.
//!
//! STUB: signatures are the contract; implementation lands in the `rules` work slice.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{Diagnostic, NextConfig, RouteCondition, RouteRule};

/// Modifier of a named parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParamModifier {
    /// `:slug`
    One,
    /// `:path*`
    ZeroOrMore,
    /// `:path+`
    OneOrMore,
    /// `:slug?`
    Optional,
}

/// One compiled segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PatternSegment {
    /// Literal text.
    Static {
        /// Text.
        text: String,
    },
    /// Named parameter.
    Param {
        /// Name.
        name: String,
        /// Modifier.
        modifier: ParamModifier,
        /// Inline regex (`:id(\d+)`).
        regex: Option<String>,
    },
    /// Unnamed `(.*)` group.
    Wildcard,
}

/// Compiled path-to-regexp source.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompiledPattern {
    /// Original source.
    pub source: String,
    /// Segments.
    pub segments: Vec<PatternSegment>,
}

impl CompiledPattern {
    /// Matches a pathname, returning captured params.
    pub fn matches(&self, _path: &str) -> Option<BTreeMap<String, String>> {
        None
    }

    /// `Bun.serve` routes key (`/blog/:slug`, `/docs/*`).
    pub fn bun_pattern(&self) -> String {
        self.source.clone()
    }

    /// axum 0.8 route (`/blog/{slug}`, `/docs/{*rest}`).
    pub fn axum_pattern(&self) -> String {
        self.source.clone()
    }

    /// Equivalent JavaScript regex source with named groups.
    pub fn to_regex_source(&self) -> String {
        String::new()
    }
}

/// Compiles a Next.js `source` (`/old/:slug*`, `/:path(\d+)`, `/(.*)`).
pub fn compile_source(source: &str) -> Result<CompiledPattern, Diagnostic> {
    Ok(CompiledPattern {
        source: source.to_owned(),
        segments: Vec::new(),
    })
}

/// Substitutes `:param` placeholders in a destination.
pub fn apply_destination(destination: &str, _params: &BTreeMap<String, String>) -> String {
    destination.to_owned()
}

/// Status code of a redirect rule (`statusCode` > `permanent` 308 > 307).
pub fn redirect_status(_rule: &RouteRule) -> u16 {
    307
}

/// Evaluates one `has`/`missing` condition against request facts.
pub fn condition_matches(
    _cond: &RouteCondition,
    _headers: &BTreeMap<String, String>,
    _cookies: &BTreeMap<String, String>,
    _query: &BTreeMap<String, String>,
    _host: &str,
) -> bool {
    false
}

/// `routes-manifest.json`-shaped JSON for redirects, rewrites and headers.
pub fn rules_manifest_json(_cfg: &NextConfig) -> Value {
    Value::Null
}
