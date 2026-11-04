# Decision record: the yolo web stack

Date: 2026-11-04. Status: accepted. Inputs: `docs/BENCHMARKS.md`, `C:/src/yolo-ui/AGENTS.md`,
`C:/src/bun-ui/packages/bun-next/{README.md,docs/PLAN.md}`, YOLO policy (100 percent Bun, Rust-first,
Dioxus 0.8 RSX or typed TSX, no Node).

## Decision

Primary stack: TanStack Router + Query + Start on Bun's native fullstack server (`Bun.build` +
`Bun.serve`, React 19), with `@yolo-rs/fullstack` as the RSC and Next.js migration engine.

Second layer, kept and required: Dioxus 0.8 for Rust islands, server functions on Axum, and every
native target (desktop, mobile, Blitz), consumed from `C:/src/yolo-ui`.

Third layer, kept and optional: Leptos 0.8 as a minimal-wasm islands adapter behind a feature flag,
with explicit entry and removal conditions (`crates/yolo-stack-leptos/README.md`).

## Scoring matrix

Scores 1 to 5 per criterion; weight in points. Evidence tags refer to `docs/BENCHMARKS.md` sections.

| Criterion                         | Weight | TanStack on Bun | Dioxus 0.8 | Leptos 0.8 | Evidence                                                                                                                                     |
| --------------------------------- | ------ | --------------- | ---------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Client runtime performance        | 10     | 3               | 5          | 5          | krausest: Rust rows 1.28x vanilla, React 1.82x (section 1)                                                                                   |
| Client bundle size                | 10     | 3               | 3          | 5          | 51 KB React baseline plus unmeasured TanStack; Dioxus 115 KB; Leptos 49 KB, 24 KB islands (sections 1, 3)                                    |
| SSR, streaming, RSC               | 10     | 5               | 3          | 4          | TanStack 2 357 req/s measured by its authors, streaming and RSC available; Leptos islands + streaming; Dioxus streaming only (sections 2, 5) |
| Server functions                  | 5      | 5               | 5          | 5          | all three typed, all on fetch/Axum (section 5)                                                                                               |
| Hot reload                        | 10     | 5               | 5          | 2          | Bun HMR; Subsecond under 500 ms claim; cargo-leptos full rebuild (section 4)                                                                 |
| Build time                        | 5      | 5               | 2          | 2          | Bun.build ms figures; Rust unquantified (section 4)                                                                                          |
| Ecosystem                         | 10     | 5               | 3          | 2          | React ecosystem; Dioxus Primitives + yolo-ui engines; Leptos small (section 5)                                                               |
| Typed routing                     | 5      | 5               | 5          | 3          | section 5                                                                                                                                    |
| Next.js migration cost            | 10     | 5               | 2          | 1          | `@yolo-rs/fullstack` runs Next 16 `app/` on Bun today (section 5)                                                                            |
| Native, desktop, Tauri reuse      | 10     | 3               | 5          | 2          | Blitz, dioxus-desktop, mobile, yolo-ui engines (section 5)                                                                                   |
| Windows toolchain friction        | 5      | 5               | 3          | 3          | Bun only vs dx/alpha vs cargo-leptos (section 5)                                                                                             |
| Fit with existing YOLO investment | 10     | 4               | 5          | 1          | yolo-fullstack and yolo-react in TS; all Dioxus 0.8 crates vendored in yolo-ui; no Leptos anywhere                                           |
| Total (max 500)                   | 100    | 430             | 385        | 285        |                                                                                                                                              |

## Top three reasons

1. Migration and ecosystem dominate the weighted score: the Next.js App Router replacement
   (`@yolo-rs/fullstack`, RSC through react-server-dom-parcel, server actions, streaming, prerender)
   already runs on `Bun.build` + `Bun.serve`, and TanStack Router/Query adapters are part of it. Every
   existing Next or React app moves without a rewrite; neither Rust stack offers that.
2. SSR is where the public evidence is strongest for TanStack (2 357 req/s links-100 after the
   2026-03 fix; 25 percent over React Router at 1 000 req/s on EKS), while no Leptos or Dioxus SSR
   throughput number exists publicly. Bun 1.4 adds the fastest build loop of the three
   (React Compiler pass in 465 ms on 860 components) and zero Windows friction.
3. Client-side speed and size, the only criteria where the Rust stacks win clearly, are recovered by
   keeping them as islands: Leptos and Dioxus both run update benchmarks at 1.28x vanilla against
   React's 1.82x, and Leptos islands ship 24 KB for a static page. Islands put that performance where
   it matters (tables, editors, canvases) without making the whole app shell pay the Rust compile
   and alpha-toolchain cost.

## Why not Dioxus as primary

Dioxus scores second (385) and is the Rust-first choice already vendored in `yolo-ui`. It loses the
shell role on three facts: 0.8 is still alpha (alpha.3, 2026-10-16); it has no islands mode and the
largest compressed client bundle of the three (114.9 KB vs 48.8 and 51.4 KB); and it cannot host an
existing Next or React application. It keeps everything native: desktop, mobile, Blitz, GPUI/Iced
bridges and the Rust island runtime are Dioxus and stay Dioxus.

## Why not Leptos as primary

Leptos has the best client numbers (smallest wasm, islands, equal CPU) but the weakest ecosystem,
no native story beyond a webview, a full-rebuild dev loop, a 0.9 release in flight, and no presence in
any YOLO repository. It is kept as a size lever for islands, gated on receipts.

## How the three merge (not discarded)

```text
Bun.serve (one process, one port)
  TanStack shell        apps/* routes, loaders, createServerFn, Query cache, HMR      [primary]
  @yolo-rs/fullstack    Next app/ tree, RSC, server actions, prerender, next/image    [migration path]
  Dioxus bridge         /api/* proxied to the Axum server-function router             [islands + native]
                        <Island name> renders dx web build output, hydrates on client
  Leptos adapter        same <Island> contract, feature-gated, size-benchmarked        [optional]
Shared contracts        packages/yolo-stack-routes == crates/yolo-stack-routes
Theme                   yolo-tokens HCT presets (yolo-ui), emitted once for TSX and RSX
Native                  showcase-dioxus desktop/Blitz, yolo-tauri wraps the TanStack shell
```

## Consequences

- `C:/src/yolo-stack` owns adapters, showcase apps and the benchmark harness only. UI crates stay in
  `yolo-ui`; the Bun runtime stays in `yolo-bun`; nothing is copied across repositories.
- The decision is revisited when any of these receipts changes the matrix: Dioxus 0.8 stable with an
  islands mode; Leptos 0.9 stable with a measured island smaller than Dioxus by more than 20 percent;
  TanStack Start RSC leaving experimental status (which would retire `@yolo-rs/stack-next-compat`'s
  RSC role); a measured SSR run in `bench/` where a Rust shell beats the TanStack shell by 2x on
  `table-1k` at equal p99.
- Every performance claim in this repository must point to a `docs/receipts/*.json` file or to a
  dated public URL. The only measured number today is the framework-free `Bun.serve` baseline.
