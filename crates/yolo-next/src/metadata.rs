// SPDX-License-Identifier: Apache-2.0
//! Metadata API: `metadata` / `viewport` objects, merge semantics and head renderers.
//!
//! STUB: signatures are the contract; implementation lands in the `metadata` work slice.

use serde_json::Value;

use crate::types::{Metadata, Viewport};

/// Converts a statically evaluated `metadata` object into [`Metadata`].
pub fn metadata_from_value(_value: &Value) -> Metadata {
    Metadata::default()
}

/// Converts a statically evaluated `viewport` object into [`Viewport`].
pub fn viewport_from_value(_value: &Value) -> Viewport {
    Viewport::default()
}

/// Default viewport (`width=device-width, initial-scale=1`).
pub fn default_viewport() -> Viewport {
    Viewport::default()
}

/// Shallow merge root -> leaf (child wins, `title.template` applies to child titles).
pub fn merge_metadata(_parent: &Metadata, child: &Metadata) -> Metadata {
    child.clone()
}

/// Resolves the final `<title>` text.
pub fn resolve_title(_parent: Option<&Metadata>, _child: &Metadata) -> Option<String> {
    None
}

/// Renders `<head>` tags as HTML.
pub fn render_head_html(_metadata: &Metadata, _viewport: Option<&Viewport>) -> String {
    String::new()
}

/// Renders Dioxus `document::{Title, Meta, Link}` RSX.
pub fn render_dioxus_head_rsx(_metadata: &Metadata, _viewport: Option<&Viewport>) -> String {
    String::new()
}

/// Renders Leptos `leptos_meta` (`<Title/>`, `<Meta/>`, `<Link/>`) view code.
pub fn render_leptos_head(_metadata: &Metadata, _viewport: Option<&Viewport>) -> String {
    String::new()
}
