# crates/

Rust side of the yolo framework. Every crate here is a thin adapter or a shared contract; the heavy
UI crates stay in `C:/src/yolo-ui` (`@yolo-rs/yolo-ui`) and are consumed by path dependency
(`../yolo-ui/...` in the root `Cargo.toml`), switching to git dependencies once yolo-ui publishes.
Nothing is copied from yolo-ui; reference by path.

| Crate               | Status                                | Owner / merge path                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `yolo-stack-routes` | compiles, tested                      | Shared showcase route table. Twin of `packages/yolo-stack-routes`. Must stay dependency-free.                                                                                                                                                                                                                                                                                                                                                                            |
| `yolo-stack-dioxus` | planned (excluded from the workspace) | Dioxus 0.8 islands + native layer. Depends on `yolo-ui/packages/{dioxus,fullstack,router,ssr}` (0.8.0-alpha.3) and `yolo-ui/crates/{yolo-tokens,yolo-layout}`. Exposes `#[island]`-style components rendered to static HTML by `dioxus-ssr` and hydrated by `dioxus-web`, plus the `/api` server-function router the `@yolo-rs/fullstack` Dioxus bridge proxies to. Desktop/native builds reuse the same components through `yolo-ui/packages/{desktop,native}` (Blitz). |
| `yolo-stack-leptos` | planned (excluded from the workspace) | Optional minimal-wasm islands layer. crates.io pins `leptos`/`leptos_router`/`leptos_axum` `=0.8.23` (stable 2026-10-31). Enters the workspace only when Leptos 0.9 is stable and a receipt shows a smaller gzip wasm than the Dioxus island for the same `island-counter` route.                                                                                                                                                                                        |

## Merge path into the final framework

1. `yolo-stack-routes` becomes the single route manifest consumed by all three stacks (and by
   `bench/`). TanStack file routes, the Dioxus `Routable` enum and Leptos `<Route/>` definitions are
   generated from it, not hand-written twice.
2. `yolo-stack-dioxus` ships the Rust island runtime that `@yolo-rs/fullstack` already bridges
   (serving `dx` web builds and proxying `/api`). Islands are addressed by the same `data-component`
   contract on every stack so the TanStack shell can mount a Dioxus island, and a Dioxus desktop
   build can embed the TanStack shell through `yolo-ui/packages/yolo-tauri`.
3. `yolo-stack-leptos` is held behind a feature flag; it is a bundle-size lever, not a second app
   model. If it never beats the Dioxus island on size, it is removed with its removal condition met.

Rules: Rust 2024 edition, `unsafe_code = "forbid"` unless the crate wraps FFI, upstream release
profile (LTO, codegen-units = 1) unchanged; measure any flag change separately.
