//! Generates `style/main.css` from the Rust design tokens in `src/tokens.rs`.
//!
//! The crate contains zero hand-written CSS: colors become CSS custom
//! properties, the spacing scale becomes `--space-N` variables and a small set
//! of structural rules, all derived from the token constants. The output file
//! is gitignored and regenerated on every build (cargo-leptos picks it up via
//! `style-file = "style/main.css"`).

use std::{env, fs, path::Path};

#[allow(dead_code)]
mod tokens {
    include!("src/tokens.rs");
}

fn main() {
    println!("cargo:rerun-if-changed=src/tokens.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out_dir = Path::new(&manifest_dir).join("style");
    fs::create_dir_all(&out_dir).expect("create style/");
    let css = tokens::render_css();
    let out = out_dir.join("main.css");
    // Only rewrite when the content changed, so cargo-leptos hot-reload is not
    // triggered spuriously.
    if fs::read_to_string(&out).ok().as_deref() != Some(css.as_str()) {
        fs::write(&out, css).expect("write style/main.css");
    }
}
