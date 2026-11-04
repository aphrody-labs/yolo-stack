# yolo-stack

The yolo web framework monorepo: one `Bun.serve` process hosting a TanStack app shell, a Next.js
migration path, and Rust islands from Dioxus 0.8 (with Leptos as an optional size lever). Decision
and evidence: `docs/DECISION.md`, `docs/BENCHMARKS.md` (both dated 2026-11-04).

```text
yolo-stack/
  package.json           Bun 1.4.4 workspaces (packages/*, apps/showcase-tanstack, bench)
  Cargo.toml             Cargo workspace; path deps on ../yolo-ui crates and Dioxus 0.8 packages
  AGENTS.md              contributor rules
  crates/                Rust: shared routes (compiles), dioxus and leptos adapters (planned)
  packages/              Bun/TS: shared routes (compiles), tanstack adapter and next-compat (planned)
  apps/                  one showcase per stack on the same five routes (planned)
  bench/                 harness: build time, bundle size, rps/latency per route (baseline run)
  docs/                  BENCHMARKS.md, DECISION.md, receipts/
```

## Commands

```sh
bun install
bun check && bun lint && bun test        # TS
cargo test -p yolo-stack-routes          # Rust
bun run bench/bench.ts --target baseline --receipt
```

## Status (2026-11-04)

| Piece                                        | State                                                            |
| -------------------------------------------- | ---------------------------------------------------------------- |
| Shared route table (TS + Rust)               | compiles, tests pass                                             |
| Benchmark harness                            | runs; only the framework-free `Bun.serve` baseline has a receipt |
| TanStack adapter, next-compat, showcase apps | planned, README per directory                                    |
| Dioxus and Leptos adapter crates             | planned, excluded from the Cargo workspace until code lands      |

Upstream inputs are read-only: `C:/src/yolo-ui` (`@yolo-rs/yolo-ui`, Dioxus 0.8.0-alpha.3 vendored,
`@yolo-rs/fullstack` in progress), `C:/src/bun-ui/packages/bun-next` (origin of `@yolo-rs/fullstack`).
