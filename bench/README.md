# bench/

Reproducible local benchmark harness for the three showcase apps. Bun only; no npm, npx or Node.

| File                 | Role                                                                                                                                                                      |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bench.ts`           | Builds, serves and load-tests each target from `targets.json`; writes `bench/out/<date>-<target>.json` and, with `--receipt`, `docs/receipts/bench-<target>-<date>.json`. |
| `targets.json`       | Per-stack build/serve commands, port and artifact globs. `status` is `runnable` or `planned`.                                                                             |
| `baseline-server.ts` | Framework-free `Bun.serve` that renders the shared route set as static strings: the ceiling every stack is compared against.                                              |

Metrics per target: build wall time, artifact sizes (raw, gzip, brotli), and per shared route (`packages/yolo-stack-routes`) the requests/second, mean/p50/p99 latency and error count from an in-process `fetch` load generator.

## Status (2026-11-04)

| Target     | Status  | Note                                                                                                                             |
| ---------- | ------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `baseline` | run     | `docs/receipts/bench-baseline-2026-11-04.json`.                                                                                  |
| `tanstack` | planned | Waits for `apps/showcase-tanstack` to build with `Bun.build` and start under `Bun.serve`.                                        |
| `dioxus`   | planned | Waits for `apps/showcase-dioxus` (`dx build --platform web --fullstack`, served through the `@yolo-rs/fullstack` Dioxus bridge). |
| `leptos`   | planned | Waits for `apps/showcase-leptos` (`cargo leptos build --release`).                                                               |

The load generator shares the host with the server, so numbers are only comparable between targets measured in the same session on the same machine. Do not quote them as absolute throughput. Any cross-stack claim in `docs/BENCHMARKS.md` must cite a receipt in `docs/receipts/`.

```sh
bun run bench/bench.ts --target baseline --duration 5 --connections 32 --receipt
bun run bench/bench.ts --all --force     # once the showcase apps exist
```
