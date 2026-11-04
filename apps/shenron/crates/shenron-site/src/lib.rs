//! Shenron site: dragonballfr.com rebuilt in pure Rust.
//!
//! Leptos 0.8 SSR (static route tree, `SsrMode::Static`) served by Axum, with
//! WASM hydration on the client. Locale-prefixed routes (`/fr`, `/en`, `/ja`),
//! build-time i18n catalogs, `leptos_meta` SEO tags, a generated stylesheet and
//! a `ReaderIsland` placeholder for the wgpu manga painter.

#![forbid(unsafe_code)]

pub mod app;
pub mod content;
pub mod i18n;
pub mod pages;
pub mod reader;
pub mod seo;
pub mod tokens;

#[cfg(feature = "ssr")]
pub mod server;

/// Client entry point: hydrates the server-rendered `<body>`.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
