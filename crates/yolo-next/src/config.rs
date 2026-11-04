// SPDX-License-Identifier: Apache-2.0
//! Static evaluation of `next.config.(js|mjs|ts)` with OXC and conversion to [`YoloWebConfig`].
//!
//! STUB: signatures are the contract; implementation lands in the `config` work slice.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oxc_ast::ast::Expression;
use serde_json::Value;

use crate::types::{
    CacheLifeProfile, ConfigMigrationReport, Diagnostic, NextConfig, YoloWebConfig,
};

/// Finds `next.config.{ts,mts,mjs,js}` under `root`.
pub fn find_next_config(_root: &Path) -> Option<PathBuf> {
    None
}

/// Parses a `next.config` source file. Returns the config plus warnings, or parse errors.
pub fn parse_next_config(
    _source: &str,
    _file_name: &str,
) -> Result<(NextConfig, Vec<Diagnostic>), Vec<Diagnostic>> {
    Ok((NextConfig::default(), Vec::new()))
}

/// Converts an already evaluated JSON object into a [`NextConfig`] (renames, removed keys).
pub fn next_config_from_value(_value: &Value) -> (NextConfig, Vec<Diagnostic>) {
    (NextConfig::default(), Vec::new())
}

/// Statically converts an OXC expression (object/array/literal/template/`process.env.X`) to JSON.
pub fn expression_to_json(_expr: &Expression<'_>) -> Option<Value> {
    None
}

/// Parses a standalone JavaScript object literal source (`{ a: 1 }`) into JSON.
pub fn object_literal_to_json(_source: &str) -> Result<Value, Vec<Diagnostic>> {
    Ok(Value::Null)
}

/// Converts a [`NextConfig`] into the normalized [`YoloWebConfig`] with a migration report.
pub fn next_config_to_yolo(
    _cfg: &NextConfig,
    _app_dir: &str,
    _pages_dir: Option<&str>,
    _proxy_file: Option<&str>,
) -> (YoloWebConfig, ConfigMigrationReport) {
    (YoloWebConfig::default(), ConfigMigrationReport::default())
}

/// Serializes a [`YoloWebConfig`] as `yolo.toml`.
pub fn yolo_config_toml(_cfg: &YoloWebConfig) -> String {
    String::new()
}

/// Serializes a [`YoloWebConfig`] as `yolo.config.ts` (`defineYoloConfig({...})`).
pub fn yolo_config_ts(_cfg: &YoloWebConfig) -> String {
    String::new()
}

/// Built-in `cacheLife` profiles of Next.js 16.
pub fn builtin_cache_life() -> BTreeMap<String, CacheLifeProfile> {
    BTreeMap::new()
}
