# apps/

One showcase per stack. All three render the same route set from `packages/yolo-stack-routes` /
`crates/yolo-stack-routes` so `bench/` compares like with like:

| Route id         | Path          | What it renders                                                             |
| ---------------- | ------------- | --------------------------------------------------------------------------- |
| `home`           | `/`           | Static page, no data.                                                       |
| `links-100`      | `/links`      | 100 router links (mirrors the TanStack `links-100` workload).               |
| `table-1k`       | `/table`      | 1 000 server-rendered rows with a client-side select/swap (krausest-style). |
| `island-counter` | `/island`     | One interactive island on an otherwise static page.                         |
| `api-json`       | `/api/health` | JSON endpoint through the stack's server-function layer.                    |

| App                 | Status  | Build / serve                                                                                                                                                                                          |
| ------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `showcase-tanstack` | planned | `bun run build` (`Bun.build`) / `bun run start` (`Bun.serve`). Primary shell.                                                                                                                          |
| `showcase-dioxus`   | planned | `dx build --release --platform web --fullstack` / `bun run start` (served through the `@yolo-rs/fullstack` Dioxus bridge). Also the desktop (`dx serve --platform desktop`) and Blitz native showcase. |
| `showcase-leptos`   | planned | `cargo leptos build --release` / `bun run start` (Axum binary spawned by a Bun wrapper so the harness has one entry point).                                                                            |

Each app directory holds a README with its stack-specific layout. No app code is written yet; the
harness marks them `planned` and skips them without `--force`.

Theme: every showcase uses the `yolo-tokens` presets from `C:/src/yolo-ui` (Neutral by default), no
Inter/Roboto, Phosphor vector icons only.
