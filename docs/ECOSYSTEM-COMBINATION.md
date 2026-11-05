# Ecosystem combination: Vercel + Vercel Labs, OXC, and Bun native in yolo-stack

Status: research record, verified 2026-11-04 (registry and GitHub lookups ran between 2026-11-04 23:50 and 2026-11-05 00:40 local time, host `Windows 11 Pro 10.0.28120`, x64).
Scope: what from (a) `vercel` / `vercel-labs`, (b) `oxc-project`, and (c) Bun native (runtime, crates, plugins) enters `yolo-stack`, in which mode, and where.
Companion manifest: `C:\src\yolo-stack\ecosystem.toml` (machine-readable, one `[[source]]` per item below).

Integration modes used in this document:

| Mode | Meaning in yolo-stack |
| --- | --- |
| `vendor-pin` | Git submodule or `fusion.toml` mount under `repos/<name>` or `pillars/<pillar>/repos/<name>`; source of truth for rules, specs, fixtures. Never compiled into the product unless stated. |
| `npm-dep` | Consumed from registry.npmjs.org under Bun 1.4.4 (`bun add`), pinned exact (`bunfig.toml` has `exact = true`). |
| `cargo-dep` | crates.io dependency in the Cargo workspace. |
| `port-to-rust` | Re-implemented in `crates/yolo-next` (or sibling crates); upstream is a behavioural reference only. |
| `shim` | Compatibility module in `packages/yolo-next/src/shims/*.ts`, re-exported as `@yolo-rs/next/shims/<name>`. |
| `builtin` | Provided by Bun 1.4.4 itself (`Bun.*`, `bun:*`), nothing to install. |
| `reject` | Not integrated; reason stated. |

Number tags follow `AGENTS.md`: `quoted` = URL + retrieval date; `measured` = run in this session (commands in section 0.3, single samples, no receipt file was written because this task produces exactly two files; re-run the commands to create `docs/receipts/ecosystem-2026-11-04.json`).

## 0. Verification method

### 0.1 Sources

| Data | Source | Retrieved |
| --- | --- | --- |
| Repository stars, license, last push, archived flag | GitHub REST `GET /search/repositories?q=org:<org>+fork:false&sort=stars&order=desc` and `GET /repos/<owner>/<name>` (authenticated `gh`) | 2026-11-04 |
| Release dates | GitHub REST `GET /repos/<owner>/<name>/releases` | 2026-11-04 |
| crates.io versions, licenses, `rust-version` | `https://crates.io/api/v1/crates/<name>` | 2026-11-04 |
| npm versions, publish dates, `optionalDependencies` (prebuilt binaries), `engines`, dist-tags | `https://registry.npmjs.org/<pkg>/latest`, `https://registry.npmjs.org/<pkg>`, `https://registry.npmjs.org/-/package/<pkg>/dist-tags` | 2026-11-04 |
| Bun API surface | `Object.getOwnPropertyNames(Bun)` on Bun 1.4.4 (official `bun-windows-x64.zip`, revision `38afee6c84fcf48c0edb2466aa4b27d287a59c30`, release 2026-10-22) and on the local `1.4.4-canary.1` (`cdfa19bba`); the two key sets are identical | 2026-11-04 |
| Bun typings and docs | `bun-types@1.4.4` tarball: `bun.d.ts`, `bundle.d.ts`, `serve.d.ts`, `docs/**/*.mdx` (the same pages as `https://bun.com/docs/...`) | 2026-11-04 |
| OXC Rust API | crates.io sources of `oxc_*` 0.157.0 in `~/.cargo/registry`, plus a compiled probe (section B.4) | 2026-11-04 |
| Bun native plugin API | crates.io sources of `bun-native-plugin` 0.2.0 and `bun-macro` 0.1.0, `oven-sh/bun/packages/bun-native-plugin-rs/README.md`, `bun-types@1.4.4/docs/bundler/plugins.mdx`, plus a compiled and loaded plugin (section C.2) | 2026-11-04 |
| React 19.3 status | `https://react.dev/blog/2026/09/09/react-19-3` and `Object.keys(require("react"))` on `react@19.3.1` under Bun | 2026-11-04 |

### 0.2 Toolchain present on the host

| Tool | Version | Note |
| --- | --- | --- |
| Bun (default on PATH) | 1.4.4-canary.1 (`cdfa19bba`), reports `Bun.version = "1.4.4"` | `C:\Users\aphro\.local\opt\bun-upstream-canary\bin\bun` |
| Bun (official stable, downloaded for cross-checks) | 1.4.4 (`38afee6c84fc...`), tag `bun-v1.4.4`, published 2026-10-22 | `gh release download bun-v1.4.4 -R oven-sh/bun -p bun-windows-x64.zip` |
| rustc / cargo | 1.99.1 stable (per `README.md`); `CARGO_TARGET_DIR` is set globally to `%LOCALAPPDATA%\Aphrody\cache\cargo-target` | Pass `--target-dir` when a local artifact path is needed |
| gh | authenticated as `aphrody-dev` | REST core quota 5000/h |
| Node.js, npm, jq | absent by policy | all JSON handled with `bun -e` |

### 0.3 Reproduction commands (measured items)

```bash
# A.4 satori + resvg on Bun (temp dir outside the repo)
bun add satori@0.63.2 @resvg/resvg-js@2.6.2 @resvg/resvg-wasm@2.6.2 && bun run satori-test.ts && bun run wasm-test.ts
# B.3 oxc napi packages on Bun
bun add oxc-parser@0.157.0 oxc-transform@0.157.0 oxc-minify@0.157.0 oxc-resolver@11.24.2 && bun run oxc3.ts
bunx oxlint@1.91.0 --version && bunx oxfmt@0.76.0 --check lintme.ts
# B.4 oxc Rust probe (Cargo project with oxc_* = "0.157")
cargo build --target-dir target && ./target/debug/oxc-probe.exe
# C.2 native Bun plugin (cdylib: bun-native-plugin 0.2.0 + napi 3 + napi-derive 3)
cargo build --target-dir target && cp target/debug/yolo_bun_plugin_probe.dll plugin.node && bun run test.ts
```

## A. Vercel Labs and Vercel

### A.1 `vercel-labs`: top 30 repositories by stars

Query: `https://api.github.com/search/repositories?q=org:vercel-labs+fork:false&sort=stars&order=desc&per_page=30` (370 non-fork repositories in the org, quoted 2026-11-04). "License" is GitHub's SPDX detection; `none` means no detectable LICENSE file.

| # | Repository | Stars | License | Last push | Lang | Purpose | Role in yolo-stack | Mode |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `vercel-labs/agent-browser` | 45,529 | Apache-2.0 | 2026-11-04 | Rust | Browser automation CLI for AI agents (snapshot refs, CDP daemon, `--engine obscura` supported) | Agent-driven QA of `apps/showcase-*`; `bench/` smoke runs; pairs with yolo-web Obscura | `npm-dep` (dev tool via `bunx agent-browser`; crate `agent-browser` also on crates.io since 2026-03-14) |
| 2 | `vercel-labs/skills` | 38,119 | MIT | 2026-11-03 | TS | The open agent skills installer (`npx skills add`, `find`, `check`) | Installs and updates review skills into `.claude/skills` (`bunx skills add vercel-labs/agent-skills -a claude-code`) | `npm-dep` (dev tool) |
| 3 | `vercel-labs/agent-skills` | 33,545 | none (README states MIT) | 2026-10-28 | JS | Vercel's official agent skills | Quality gates: see A.3.1 | `vendor-pin` (`repos/agent-skills`) |
| 4 | `vercel-labs/json-render` | 18,913 | Apache-2.0 | 2026-11-04 | TS | Generative UI framework (catalog, spec, renderer, SpecStream) | Generative UI lane bound to `yolo-tokens`; see A.3.4 | `npm-dep` (`@json-render/core`, `@json-render/react` 0.22.0) |
| 5 | `vercel-labs/agent-desktop` | 14,950 | Apache-2.0 | 2026-11-04 | Rust | Computer-use CLI for AI agents | Outside the web framework; yolo-re / yolo-os may evaluate separately | `reject` |
| 6 | `vercel-labs/portless` | 12,797 | Apache-2.0 | 2026-11-04 | TS | Stable named local URLs instead of ports | Not needed: `Bun.serve` fixed ports in `justfile` | `reject` |
| 7 | `vercel-labs/deepsec` | 8,337 | Apache-2.0 | 2026-11-04 | TS | Security harness driven by coding agents | Optional CI job only; needs agent credentials | `reject` (optional CI add-on) |
| 8 | `vercel-labs/native` | 7,823 | Apache-2.0 | 2026-11-03 | Zig | Toolkit for native desktop apps | Desktop is owned by yolo-ui (Dioxus 0.8 / GPUI / WGPU) | `reject` |
| 9 | `vercel-labs/scriptc` | 6,497 | Apache-2.0 | 2026-11-04 | TS | TypeScript-to-native compiler | Bun `--compile`/`--bytecode` and Rust lanes cover this | `reject` |
| 10 | `vercel-labs/open-agents` | 5,880 | MIT | 2026-08-29 | TS | Cloud agents template | Template, Vercel-hosted | `reject` |
| 11 | `vercel-labs/zerolang` | 5,400 | Apache-2.0 | 2026-09-16 | C | Programming language for agents | Out of scope | `reject` |
| 12 | `vercel-labs/just-bash` | 4,461 | none | 2026-10-27 | TS | Bash for agents (TS implementation) | Bun has `$` shell | `reject` |
| 13 | `vercel-labs/fx` | 3,752 | Apache-2.0 | 2026-11-04 | Zig | Unix-like coding agent | Out of scope | `reject` |
| 14 | `vercel-labs/wterm` | 3,649 | Apache-2.0 | 2026-11-04 | TS | Terminal emulator for the web | `Bun.Terminal` + yolo-ui components | `reject` |
| 15 | `vercel-labs/opensrc` | 3,028 | Apache-2.0 | 2026-09-16 | Rust | Fetch npm package sources for agent context | Optional dev tool for agents, not framework code | `reject` (optional) |
| 16 | `vercel-labs/vgpu` | 2,622 | MIT | 2026-10-16 | TS | Cross-runtime WebGPU library | yolo-ui `yolo-gpu` (WGPU 30, Rust) owns GPU; API coverage reference only | `reject` |
| 17 | `vercel-labs/lat.md` | 2,049 | MIT | 2026-11-04 | TS | Knowledge graph for a codebase in markdown | Optional docs tooling | `reject` |
| 18 | `vercel-labs/emulate` | 1,967 | Apache-2.0 | 2026-10-29 | TS | Local API emulation for CI sandboxes | Optional CI tooling | `reject` |
| 19 | `vercel-labs/coding-agent-template` | 1,798 | NOASSERTION | 2026-08-25 | TS | Multi-agent coding platform (Sandbox + AI Gateway) | Template | `reject` |
| 20 | `vercel-labs/openreview` | 1,701 | none | 2026-03-06 | TS | Self-hosted AI code review bot | Template | `reject` |
| 21 | `vercel-labs/dev3000` | 1,597 | MIT | 2026-10-30 | TS | Dev timeline capture (logs, browser events, network) | Bun dev server already streams browser console (`development.console`) | `reject` |
| 22 | `vercel-labs/book-inventory` | 1,419 | none | 2025-12-12 | TS | Demo (archived) | Archived | `reject` |
| 23 | `vercel-labs/gemini-chatbot` | 1,358 | NOASSERTION | 2026-10-23 | TS | Generative UI chatbot template | Template | `reject` |
| 24 | `vercel-labs/workflow-builder-template` | 1,234 | NOASSERTION | 2026-11-03 | TS | Visual AI workflow platform | Template | `reject` |
| 25 | `vercel-labs/eve-software-factory-template` | 1,196 | MIT | 2026-11-02 | TS | eve Software Factory template | Template | `reject` |
| 26 | `vercel-labs/web-interface-guidelines` | 1,107 | MIT | 2026-10-05 | Shell | Guidelines for building web interfaces (source of the `web-design-guidelines` skill) | Rule source for UI reviews of RSX/TSX output | `vendor-pin` (`repos/web-interface-guidelines`) |
| 27 | `vercel-labs/knowledge-agent-template` | 1,078 | MIT | 2026-09-15 | TS | Knowledge-base agent template | Template | `reject` |
| 28 | `vercel-labs/tersa` | 1,046 | MIT | 2026-05-01 | TS | Canvas for AI workflows | Product, not library | `reject` |
| 29 | `vercel-labs/next-skills` | 988 | none | 2026-09-16 | - | No description, no license | Superseded by `agent-skills` | `reject` |
| 30 | `vercel-labs/webreel` | 961 | Apache-2.0 | 2026-09-16 | TS | Record scripted browser demos as video | `Bun.WebView.screenshot()` covers stills; optional | `reject` |

Not in the top 30 but mandatory for this study: `vercel-labs/mcp-handler` (694 stars, no LICENSE file detected, npm license Apache-2.0, last push 2026-10-28) and `vercel-labs/next-view-transitions` (HTTP 404 under `vercel-labs` on 2026-11-04; the project lives at `shuding/next-view-transitions`, 2,381 stars, last push 2026-03-06, not archived).

### A.2 `vercel`: top 25 repositories by stars

Query: `https://api.github.com/search/repositories?q=org:vercel+fork:false&sort=stars&order=desc&per_page=25` (203 non-fork repositories, quoted 2026-11-04). npm versions quoted from registry.npmjs.org on the same date.

| # | Repository | Stars | License | Last push | npm (latest, date) | Purpose | Role in yolo-stack | Mode |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `vercel/next.js` | 143,324 | MIT | 2026-11-04 (default branch `canary`) | `next` 16.4.5 (2026-11-03; `v16.5.0-canary.40` same day) | The React framework | Behavioural reference for `crates/yolo-next` (App Router conventions, Server Components, Server Actions, `next/font`, `next/image`, `next.config`/Turbopack) and for the `next/*` import surface; never a dependency | `port-to-rust` + `shim` (reference mount via `fusion.toml` link only) |
| 2 | `vercel/hyper` | 44,762 | MIT | 2026-11-04 | - | Terminal on web technologies (Electron); repository archived | Electron, archived | `reject` |
| 3 | `vercel/swr` | 32,491 | MIT | 2026-10-16 | `swr` 2.5.1 | React hooks for data fetching | Compatibility lane next to TanStack Query (`@yolo-rs/next/shims/swr` re-export) | `npm-dep` (compat) |
| 4 | `vercel/turborepo` | 31,305 | MIT | 2026-11-04 | `turbo` 2.11.10 | Build system for JS/TS monorepos (Rust) | Replaced by `just` recipes + Bun workspaces (`bun run --filter`, `bun install --filter`) + `cargo`; see A.3.7 | `reject` |
| 5 | `vercel/ai` | 27,740 | NOASSERTION (npm: Apache-2.0) | 2026-11-04 | `ai` 7.0.162; `@ai-sdk/react` 4.0.166 | AI SDK for TypeScript | Adapter lane only, inside `pillars/yolo-ai` (provider-neutral core stays Rust) | `npm-dep` (adapter) |
| 6 | `vercel/pkg` | 24,306 | MIT | 2024-01-03 | `pkg` 5.8.1 (2023-03-08, deprecated notice) | Package Node.js project into executable; archived | `bun build --compile` | `reject` |
| 7 | `vercel/chatbot` | 21,034 | NOASSERTION | 2026-11-03 | - | Next.js AI chatbot template | Reference for RSC + AI SDK patterns | `reject` |
| 8 | `vercel/vercel` | 16,521 | Apache-2.0 | 2026-11-03 | `vercel` 65.11.0 | Platform CLI and builders | Deployment lane is optional (`deploy-to-vercel` skill); not part of the framework | `reject` |
| 9 | `vercel/commerce` | 14,307 | MIT | 2026-10-19 | - | Next.js Commerce; archived | Archived | `reject` |
| 10 | `vercel/satori` | 14,163 | MPL-2.0 | 2026-10-23 | `satori` 0.63.2 (2026-10-20) | HTML/CSS (JSX) to SVG | `@yolo-rs/next/og` `ImageResponse`; see A.4 | `npm-dep` |
| 11 | `vercel/micro` | 10,630 | MIT | 2026-11-04 | `micro` 10.0.1 (2022-11-26) | Async HTTP microservices for Node | `Bun.serve` routes | `reject` |
| 12 | `vercel/serve` | 9,910 | MIT | 2026-06-30 | `serve` 14.2.6 (2026-03-03) | Static file serving | `Bun.serve` static and HTML routes, `bun ./index.html` | `reject` |
| 13 | `vercel/ncc` | 9,841 | MIT | 2026-08-13 | `@vercel/ncc` 0.45.0 (2026-08-13) | Compile Node project into a single file | `Bun.build` (single file, `--compile`) | `reject` |
| 14 | `vercel/styled-jsx` | 7,771 | MIT | 2026-06-09 | `styled-jsx` 5.1.7 (2025-04-30) | Scoped CSS in JSX | Needs a Babel/SWC compiler plugin; conflicts with the zero-raw-CSS policy; use `yolo-tokens` + CSS modules through Bun's CSS bundler | `reject` |
| 15 | `vercel/next-forge` | 7,735 | MIT | 2026-05-28 | - | Turborepo template for Next.js | Template | `reject` |
| 16 | `vercel/nextjs-subscription-payments` | 7,708 | MIT | 2025-01-23 | - | SaaS template; archived | Archived | `reject` |
| 17 | `vercel/platforms` | 6,724 | none | 2026-07-08 | - | Multi-tenant Next.js app | Reference for tenant routing (`Bun.ModuleGraph` per tenant) | `reject` |
| 18 | `vercel/eve` | 5,795 | Apache-2.0 | 2026-11-04 | - | Open framework for building agents | `pillars/yolo-ai` decides separately | `reject` |
| 19 | `vercel/streamdown` | 5,746 | NOASSERTION (npm: Apache-2.0) | 2026-11-04 | `streamdown` 2.7.1 | react-markdown replacement for streaming AI output | `Bun.markdown.react()` is built in (unstable); revisit only for streaming-specific needs | `reject` |
| 20 | `vercel/ms` | 5,561 | MIT | 2026-05-20 | `ms` 2.1.3 (2020-12-08) | Millisecond conversion | 30-line local utility | `reject` |
| 21 | `vercel/examples` | 5,162 | MIT | 2026-11-04 | - | Curated examples | Migration fixture corpus for `apps/next-fixture` and `crates/yolo-next` tests | `reject` (reference only) |
| 22 | `vercel/next-learn` | 4,796 | MIT | 2026-07-29 | - | Learn Next.js starter | Migration fixture (dashboard app) | `reject` (reference only) |
| 23 | `vercel/og-image` | 4,035 | MIT | 2023-01-20 | - | OG image service; archived, README says "only works with Node.js 14" | Superseded by satori | `reject` |
| 24 | `vercel/geist-font` | 3,734 | OFL-1.1 | 2026-11-04 | `geist` 1.9.1 (2026-11-04) | Geist Sans, Mono, Pixel, Serif | Optional preset font (Neutral preset only; not Inter/Roboto) | `npm-dep` (optional) |
| 25 | `vercel/release` | 3,585 | MIT | 2026-05-21 | - | Changelog generator | `just release` + `git` | `reject` |

Additional `vercel` repositories assessed (outside the top 25):

| Repository | Stars | License | Last push | npm (latest, date) | Role | Mode |
| --- | --- | --- | --- | --- | --- | --- |
| `vercel/edge-runtime` | 915 | MIT | 2026-11-02 | `edge-runtime` 4.0.1 (2024-12-03); `@edge-runtime/primitives` 6.0.0; `@edge-runtime/vm` 5.0.0 | Web-standard runtime semantics; Bun is the runtime, see A.3.12 | `reject` |
| `vercel/nft` | 1,683 | MIT | 2026-08-18 | `@vercel/nft` 1.11.0 (2026-08-18) | Node file tracing; replaced by `Bun.build` / `bun build --compile` | `reject` |
| `vercel/analytics` | 520 | MIT | 2026-10-22 | `@vercel/analytics` 2.0.1 (2026-03-12) | `@yolo-rs/next/shims/analytics` | `shim` |
| `vercel/speed-insights` | 117 | MIT | 2026-10-22 | `@vercel/speed-insights` 2.0.0 (2026-03-10) | `@yolo-rs/next/shims/speed-insights` | `shim` |
| `vercel/flags` | 628 | MIT | 2026-11-04 | `flags` 4.4.0 (2026-10-26); legacy `@vercel/flags` 3.1.1 (2025-02-14) | `@yolo-rs/next/shims/flags` over the `flags` core | `shim` (core package as `npm-dep`) |
| `vercel/async-sema` | 661 | MIT | 2024-04-18 | `async-sema` 3.1.1 (2021-08-17) | Semaphore; 20-line local utility or `tokio::sync::Semaphore` in Rust | `reject` |
| `vercel/workflow` | 2,491 | Apache-2.0 | 2026-11-04 | `workflow` 5.4.0 | Durable workflows; out of scope (`Bun.cron` + `bun:sqlite` queues) | `reject` |
| `vercel/react-tweet` | 1,886 | MIT | 2026-09-24 | `react-tweet` 3.3.1 (2026-06-03) | Not needed | `reject` |
| `vercel/sandbox` | 219 | Apache-2.0 | 2026-11-04 | `@vercel/sandbox` 3.9.1 | Vercel-hosted compute | `reject` |

### A.3 Mandatory item assessments

#### A.3.1 `vercel-labs/agent-skills` (the Claude skills)

- Repository root (quoted 2026-11-04): `AGENTS.md`, `CLAUDE.md`, `README.md`, `package.json`, `packages/`, `scripts/`, `skills/`, `skills.sh.json`.
- `skills/` directories: `composition-patterns`, `deploy-to-vercel`, `react-best-practices`, `react-native-skills`, `react-view-transitions`, `vercel-cli-with-tokens`, `vercel-optimize`, `web-design-guidelines`, `writing-guidelines` (plus `.zip` release artifacts per skill). Each skill is `SKILL.md` + optional `scripts/` and `references/`; every push to `main` publishes a GitHub release with a discovery index.
- Installation (README): `npx skills add vercel-labs/agent-skills`; under this repository's policy: `bunx skills add vercel-labs/agent-skills -a claude-code`.
- License: README says MIT; GitHub detects no LICENSE file. Treat as MIT with an attribution note until a LICENSE file appears.
- Mapping to the requested names: `vercel-react-best-practices` = `skills/react-best-practices` (40+ rules, 8 categories); `web-interface-guidelines` = `skills/web-design-guidelines` (100+ rules; rule source is `vercel-labs/web-interface-guidelines`); `writing-guidelines` = `skills/writing-guidelines` (80+ rules); composition patterns = `skills/composition-patterns`; `react-native-skills` = `skills/react-native-skills` (16 rules, 7 sections); view transitions = `skills/react-view-transitions` (React 19.3 `<ViewTransition>`, `addTransitionType`, Next.js `transitionTypes`).
- Role: quality gates, not code. `react-best-practices` and `web-design-guidelines` review `packages/yolo-next`, `packages/yolo-stack-tanstack` and the TSX showcase; `writing-guidelines` reviews `docs/`; `react-view-transitions` is the reference for the `@yolo-rs/next/navigation` transition hooks; `react-native-skills` applies to `yolo-ui/crates/yolo-react` (RN/Expo lane); `composition-patterns` governs the TSX component API of `@yolo-rs/next` and the `json-render` catalog.
- Mode: `vendor-pin` at `repos/agent-skills` (declared in `fusion.toml`) plus installation into `.claude/skills` with the `skills` CLI so updates are diffable. `deploy-to-vercel`, `vercel-cli-with-tokens`, `vercel-optimize` stay installed but are not gates (deployment target is yolo-vps, not Vercel).

#### A.3.2 `vercel-labs/mcp-handler` -> `@yolo-rs/next/mcp` on `Bun.serve`

- npm `mcp-handler` 2.3.0 (2026-10-07), Apache-2.0, `engines.node >= 20`, peer deps `next` (optional in practice) and `@modelcontextprotocol/server ^2` (`@modelcontextprotocol/server` 2.4.0 exists on npm; `@modelcontextprotocol/sdk` 1.32.2 is the 1.x line and is not used by mcp-handler 2.x), zod ^4.2.0. Bins: `mcp-adapter`, `mcp-handler`, `create-mcp-route`.
- API (README, quoted 2026-11-04): `createMcpHandler(initialize, options)` returns "a Web-standard request handler"; options = SDK server options + `serverInfo`, `verboseLogs`, `onEvent`. Removed in 2.x: `basePath`, `maxDuration`, `redisUrl`, `disableSse`, `streamableHttpEndpoint`, `sseEndpoint`, `sseMessageEndpoint`, `sessionIdGenerator`. Transport: Streamable HTTP only (2026-07-28 spec, stateless fallback for 2025-era clients); HTTP+SSE removed; GET/DELETE session operations answer 405. `withMcpAuth` verifies bearer tokens and answers 401/403 with `WWW-Authenticate`; `protectedResourceHandler` serves RFC 9728 metadata. README lists Next.js, Nuxt/Nitro (`fromWebHandler`), SvelteKit, Hono and "other frameworks that expose Fetch-compatible Request and Response"; Bun is not named but needs no adapter because `Bun.serve` route handlers are `(req: BunRequest) => Response | Promise<Response>`.
- Integration: `packages/yolo-next/src/mcp.ts` exports `createMcpRoutes(initialize, options)` returning `{ "/api/mcp": { GET: handler, POST: handler, DELETE: handler } }` for `Bun.serve({ routes })`, and re-exports `withMcpAuth` and `protectedResourceHandler`. Add `"./mcp": "./src/mcp.ts"` to `packages/yolo-next/package.json` `exports`.
- Mode: `npm-dep` (`mcp-handler` 2.3.0, `@modelcontextprotocol/server` 2.4.0) with a 40-line Bun mount; no fork.

#### A.3.3 `next-view-transitions` vs React 19.3 built-in `<ViewTransition>`

- `react@19.3.1` (2026-10-14; 19.3.0 released 2026-09-09) exports `ViewTransition` (symbol), `addTransitionType` (function), `Activity`, `useEffectEvent`, `cacheSignal`, `captureOwnerStack` (measured: `Object.keys(require("react"))` under Bun 1.4.4). The React 19.3 post states: "we shared View Transitions and Fragment Refs as new experimental APIs ... both of these are now stable in React 19.3" and "Currently, `<ViewTransition>` only works in the DOM".
- `next-view-transitions` 0.3.5 (npm, 2025-12-05; peer `next`, `react`, `react-dom`) wraps `document.startViewTransition` around the Next App Router (`<ViewTransitions>`, `Link`, `useTransitionRouter`). The repository is no longer under `vercel-labs` (404) and lives at `shuding/next-view-transitions` (last push 2026-03-06).
- Decision: `reject`. `@yolo-rs/next/navigation` builds on React 19.3 `<ViewTransition>` + `addTransitionType` with TanStack Router's navigation; the `react-view-transitions` skill is the review reference. `packages/yolo-next/package.json` already requires `react ^19.3.0`.

#### A.3.4 `vercel-labs/json-render`

- Packages (README and npm, 2026-11-04): `@json-render/core` 0.22.0 (schemas, catalogs, prompts, SpecStream; dep `zod`), `@json-render/react` 0.22.0 (renderer, contexts, hooks; peer `react`), `@json-render/shadcn` (36 shadcn components; not used here), `@json-render/vue|svelte|solid|react-native|remotion|react-pdf|react-email|ink|next`, `@json-render/image` 0.22.0 (deps `satori`, `@json-render/core`; peer `@resvg/resvg-js`, `react`, `zod`), devtools, state adapters, `mcp`, `yaml`. API names: `defineCatalog`, `defineRegistry`, `Renderer`, `createSpecStreamCompiler`, `catalog.prompt()`, `renderToHtml`, `renderToPng`, `renderToSvg`, `renderToBuffer`, `createNextApp`, `JsonRenderDevtools`. Depends on React only; Next.js support is the optional `@json-render/next`.
- Role: generative-UI lane. A `yolo-tokens`-bound catalog (`packages/yolo-next/src/json-render/catalog.ts`) exposes only Phosphor icons and the 7 presets; the React registry maps to `@yolo-rs/next` components; a later `port-to-rust` renders the same spec to Dioxus RSX (`crates/yolo-stack-dioxus`). `@json-render/image` reuses the same satori + resvg pair as `@yolo-rs/next/og` (A.4).
- Mode: `npm-dep` (`@json-render/core`, `@json-render/react`; `@json-render/image` optional).

#### A.3.5 `vercel-labs/agent-browser`

- Native Rust CLI (45,529 stars, Apache-2.0, npm `agent-browser` 0.41.2 with `engines.node >= 24` which Bun does not enforce; crates.io `agent-browser` created 2026-03-14). Daemon per `--session`, Chrome for Testing by default, `--cdp` / `--auto-connect`, `--engine lightpanda|obscura`, `agent-browser mcp` (stdio MCP server), `agent-browser skills get core`.
- Role: agent QA for the showcases and for `apps/shenron`; the Obscura engine option connects it to `pillars/yolo-web`. Not a runtime dependency.
- Mode: `npm-dep` (dev, `bunx agent-browser`); evaluate `vendor-pin` under `pillars/yolo-web/repos/agent-browser` when yolo-web starts contributing Obscura fixes upstream.

#### A.3.6 `vercel/next.js` (reference only)

- 16.4.5 stable (2026-11-03), 143,324 stars, default branch `canary`, `@next/swc-*` native bindings, `styled-jsx` dependency, peer `babel-plugin-react-compiler`.
- Role: `crates/yolo-next` already parses App Router trees with `oxc_parser` 0.155 (`routes.rs`, `segment.rs`, `rsc.rs`, `turbopack.rs`, `fonts.rs`, `images.rs`, `cdn.rs`, `proxy.rs`, `emit_bun.rs`, `emit_dioxus.rs`, `emit_leptos.rs`). The `next/*` module surface (`next/font/google`, `next/image`, `next/navigation`, `next/link`, `next/headers`, `next/cache`, `next/og`) is re-targeted to `@yolo-rs/next/shims/*` at bundle time (B.4, B.5).
- Mode: `port-to-rust` (conventions) + `shim` (imports). A `fusion.toml` `link` mount of a shallow checkout is acceptable for conformance tests; a 1.5 GB submodule is not.

#### A.3.7 `vercel/turborepo`

- `turbo` 2.11.10 (Rust binary, `@turbo/*` optional deps). Features: task graph with input hashing, local and remote cache, `--filter`, `turbo watch`, boundaries.
- yolo-stack equivalents: `justfile` recipes (task graph), Bun workspaces with `bun run --filter '<glob>' <script>` and `bun install --filter`, `cargo` incremental builds with the shared `CARGO_TARGET_DIR` and sccache for Rust, `bun --watch` / `bun --hot` for watch mode. The remaining gap is a content-addressed remote cache for TS tasks, which this monorepo does not need (TS tasks are seconds; Rust uses sccache).
- Decision: `reject`.

#### A.3.8 `vercel/swr`

- `swr` 2.5.1 (deps `dequal`, `use-sync-external-store`; peer `react`). Decision: `npm-dep` as a compatibility export (`@yolo-rs/next/shims/swr` re-exports `swr` unchanged so migrated pages keep working); TanStack Query 5.106.0 is the primary data layer (`packages/yolo-stack-tanstack`).

#### A.3.9 `vercel/satori` + resvg for `next/og` `ImageResponse` on Bun

See A.4 for measurements. Decision: `npm-dep` for `satori` 0.63.2 and `@resvg/resvg-wasm` 2.6.2 (default renderer, portable inside `bun build --compile` outputs) with `@resvg/resvg-js` 2.6.2 as an opt-in native renderer; `@vercel/og` 1.0.3 is `reject` (broken package, A.4). A Rust lane remains open: crates `resvg` 0.49.0 / `usvg` 0.49.0 / `tiny-skia` 0.12.0 / `fontdb` 0.24.0 (crates.io 2026-10-24 and earlier) can rasterize satori's SVG in `crates/yolo-next` when an OG route runs inside the Rust server.

#### A.3.10 `vercel/geist-font`

- npm `geist` 1.9.1 (2026-11-04), SIL OFL-1.1, families Geist Sans, Geist Mono, Geist Pixel (five variants), Geist Serif (`ELSH` axis). Files under `geist/dist/fonts/geist-sans/Geist-Variable.ttf`, `geist/dist/fonts/geist-mono/GeistMono-*.ttf`; the `geist/font/*` entry points wrap `next/font/local` and are Next-specific.
- Role: optional preset font for the Neutral preset only (the policy bans generic Inter/Roboto, not Geist); `@yolo-rs/next/shims/font-local` emits self-hosted `@font-face` from the TTFs; `yolo-tokens` typography presets decide per brand.
- Mode: `npm-dep` (optional).

#### A.3.11 `vercel/ai`

- `ai` 7.0.162 (Apache-2.0 on npm; GitHub detects NOASSERTION), `engines.node >= 22`, deps `@ai-sdk/gateway`, `@ai-sdk/provider`, `@ai-sdk/provider-utils`, peer `zod`; `@ai-sdk/react` 4.0.166 depends on `swr`. Dist-tags also carry `ai-v6` 6.0.322 and `ai-v5` 5.0.285.
- Mode: `npm-dep` restricted to the adapter lane in `pillars/yolo-ai` (provider-neutral Rust core; AI SDK is one UI adapter). Not imported by `@yolo-rs/next`.

#### A.3.12 `vercel/edge-runtime`

- `edge-runtime` 4.0.1 (2024-12-03), `@edge-runtime/primitives` 6.0.0, `@edge-runtime/vm` 5.0.0; repository still receives pushes (2026-11-02) for `@edge-runtime/cookies` (used by `flags`).
- Bun 1.4.4 implements the same web-standard surface natively (`fetch`, `Request`, `Response`, `Headers`, `URLPattern`, WebCrypto, streams, `HTMLRewriter`), so the primitives package is redundant and the VM sandbox is replaced by `Bun.ModuleGraph` (experimental) or `Worker`.
- Mode: `reject` (conformance reference only; `@edge-runtime/cookies` arrives transitively with `flags`).

#### A.3.13 `vercel/nft`, `vercel/ncc`, `vercel/pkg`

- `@vercel/nft` 1.11.0 traces Node dependency graphs for serverless packaging; `@vercel/ncc` 0.45.0 bundles a Node project into one file; `pkg` 5.8.1 is archived (2024-01-03) with a deprecation notice pointing to Node SEA and forks (`@yao-pkg/pkg` 6.25.0).
- Bun equivalents: `Bun.build({ compile: { target, outfile, windows: { hideConsole, icon }, autoloadTsconfig }, bytecode: true })` and `bun build --compile --target=bun-{linux,windows,darwin}-{x64,arm64}[-musl] --bytecode --minify --sourcemap` (`bun-types@1.4.4/docs/bundler/executables.mdx`); `Bun.ModuleGraph` and `Bun.embeddedFiles` for graph introspection.
- Mode: all three `reject`.

#### A.3.14 `@vercel/analytics`, `@vercel/speed-insights`, `flags`

- `@vercel/analytics` 2.0.1 and `@vercel/speed-insights` 2.0.0 inject Vercel-hosted scripts (`/_vercel/insights`, `/_vercel/speed-insights`); off-Vercel they are inert. `flags` 4.4.0 exports `flag()` definitions with `flags/next`, `flags/sveltekit`, `flags/react`, `flags/analytics` adapters; deps `jose`, `@edge-runtime/cookies`; peers `next`, `react`, `react-dom`, `@opentelemetry/api`.
- Shims: `packages/yolo-next/src/shims/analytics.ts` (`<Analytics/>` renders nothing unless `YOLO_ANALYTICS_ENDPOINT` is set; `track()` posts to a `Bun.serve` collector backed by `bun:sqlite`), `shims/speed-insights.ts` (same with web-vitals payloads), `shims/flags.ts` (re-implements the `flags/next` adapter on `Bun.CookieMap` for overrides and `Bun.CSRF` for the toolbar endpoint; the `flags` core package is consumed as-is).
- Modes: `shim`, `shim`, `shim` (+ `npm-dep` for the `flags` core).

#### A.3.15 `vercel/examples`, `vercel/next-learn`

Reference corpora: copy selected `app/` trees into `apps/next-fixture/` as migration tests for `crates/yolo-next` (`yolo-next migrate --report`). No dependency. Mode: `reject` (reference only).

#### A.3.16 `vercel/styled-jsx`, `vercel/og-image`, `vercel/hyper`, `vercel/micro`, `vercel/serve`, `vercel/ms`, `vercel/async-sema`

All `reject`: `styled-jsx` needs a Babel/SWC transform and violates the zero-raw-CSS rule; `og-image` is archived (README: Node 14 only); `hyper` is an archived Electron terminal; `micro` and `serve` are Node HTTP utilities replaced by `Bun.serve`; `ms` (2020) and `async-sema` (2021) are trivial utilities re-implemented locally (`packages/yolo-next/src/utils/{ms,semaphore}.ts`, Rust side `tokio::sync::Semaphore`).

### A.4 satori + resvg on Bun 1.4.4 (measured 2026-11-04)

Setup: fresh temp project, `bun add satori@0.63.2 @resvg/resvg-js@2.6.2 @resvg/resvg-wasm@2.6.2`, font `C:\Windows\Fonts\arial.ttf` (test input only; product fonts come from `yolo-tokens`). Bun defaulted to isolated installs (`node_modules/.bun/*`).

| Lane | Bun 1.4.4 stable (`38afee6`) | Bun 1.4.4-canary.1 (`cdfa19b`) | Output |
| --- | --- | --- | --- |
| `satori()` 1200x630 JSX object -> SVG | 33.5 ms | 44.4 ms | 15,663-byte SVG |
| `@resvg/resvg-js` (native `@resvg/resvg-js-win32-x64-msvc`) `new Resvg(svg).render().asPng()` | 227.0 ms (first call, includes font load) | 655.7 ms (first call) | 15,922-byte PNG, magic bytes OK |
| `@resvg/resvg-wasm` (`initWasm(readFileSync(index_bg.wasm))`) 400x200 | OK | OK | 4,107-byte PNG, magic bytes OK |
| `@vercel/og` 1.0.3 `new ImageResponse(...)` (`dist/index.node.js`) | fails | fails | `ENOENT .../@vercel/og/dist/hb.wasm`; the published tarball ships `resvg.wasm`, `yoga.wasm`, `Geist-Regular.ttf` but no `hb.wasm`, so the node build aborts in `instantiateArrayBuffer`. Runtime-independent packaging defect. |

Prebuilt binaries (npm `optionalDependencies`, quoted 2026-11-04): `@resvg/resvg-js` 2.6.2 lists 12 targets including `@resvg/resvg-js-win32-x64-msvc` 2.6.2 (`os: win32`, `cpu: x64`) and `@resvg/resvg-js-linux-x64-gnu` 2.6.2 (`os: linux`, `cpu: x64`), plus `darwin-arm64`, `linux-x64-musl`, `linux-arm64-gnu`, `android`, `wasm32-wasi`. `@resvg/resvg-wasm` 2.6.2 has no native dependency. Both were published 2024-03-26; `next` dist-tag is 2.7.0-alpha.2. The upstream repository is now `thx/resvg-js` (2,008 stars, last push 2026-06-30). Satori 0.63.2 runtime notes (README): layout by Taffy, text shaping by `harfbuzzjs`, fonts passed as `ArrayBuffer`, no React hooks, experimental JSX runtime (`@jsxImportSource satori`), `OffscreenCanvas` in browsers and `gl` on Node 20.16+ for `<canvas>`; none of this blocked Bun.

Implementation target: `packages/yolo-next/src/og.ts` exports `ImageResponse(element, { width = 1200, height = 630, fonts, emoji, headers, status })` built on `satori` + `@resvg/resvg-wasm`, with `YOLO_OG_RENDERER=native` switching to `@resvg/resvg-js`; fonts resolved from `yolo-tokens` typography presets; result cached with `Bun.CryptoHasher("sha256")` keys in `bun:sqlite`.

### A.5 Summary of Vercel decisions

| Mode | Items |
| --- | --- |
| `vendor-pin` | `agent-skills`, `web-interface-guidelines` |
| `npm-dep` | `skills` (dev), `agent-browser` (dev), `@json-render/core` + `@json-render/react`, `mcp-handler` + `@modelcontextprotocol/server`, `swr` (compat), `satori` + `@resvg/resvg-wasm` (+ `@resvg/resvg-js` opt-in), `geist` (optional), `ai` (adapter lane), `flags` core |
| `port-to-rust` | `next.js` conventions (`crates/yolo-next`) |
| `shim` | `next/*` import surface, `@vercel/analytics`, `@vercel/speed-insights`, `flags/next` |
| `reject` | everything else in A.1/A.2, including `turborepo`, `edge-runtime`, `nft`, `ncc`, `pkg`, `styled-jsx`, `og-image`, `@vercel/og`, `next-view-transitions`, `hyper`, `micro`, `serve`, `ms`, `async-sema`, `examples`, `next-learn` |

## B. OXC (`oxc-project`)

### B.1 Repositories

Query: `https://api.github.com/search/repositories?q=org:oxc-project+fork:false&sort=stars&order=desc&per_page=40` (64 non-fork repositories, quoted 2026-11-04).

| Repository | Stars | License | Last push | Lang | Purpose | yolo-stack relevance |
| --- | --- | --- | --- | --- | --- | --- |
| `oxc-project/oxc` | 23,322 | MIT | 2026-11-04 | Rust | Parser, semantic, transformer, codegen, minifier, mangler, isolated declarations, linter (`oxlint`), formatter (`oxfmt`), napi packages | Core: `cargo-dep` 0.157.0 + `npm-dep` 0.157.0 |
| `oxc-project/tsrs` | 2,774 | Apache-2.0 | 2026-11-04 | Rust | Rust port of the TypeScript 7 type checker | Watch; future `bun check` parity and type-aware lint without Go |
| `oxc-project/tsgolint` | 1,476 | MIT | 2026-10-26 | Go | Type-aware linting for oxlint (ships as npm `oxlint-tsgolint` 7.0.2005 with `@oxlint-tsgolint/win32-x64` and `linux-x64`) | Optional CI lint (Go binary, not built here) |
| `oxc-project/javascript-parser-in-rust` | 504 | none | 2024-08-07 | JS | Tutorial; archived | none |
| `oxc-project/eslint-plugin-oxlint` | 460 | MIT | 2026-11-01 | TS | Disable ESLint rules covered by oxlint | none (no ESLint here) |
| `oxc-project/oxc-zed` | 342 | MIT | 2026-11-03 | Rust | Zed extension | editor only |
| `oxc-project/oxc-resolver` | 300 | MIT | 2026-11-04 | Rust | Rust port of enhanced-resolve (crate `oxc_resolver` 11.24.3, npm `oxc-resolver` 11.24.2) | `cargo-dep` for `crates/yolo-next` tsconfig-paths resolution; `npm-dep` for the Bun plugin |
| `oxc-project/oxc-node` | 275 | none | 2026-11-04 | TS | Run TypeScript in Node.js (`@oxc-node/core` 1.0.1) | `reject` (Bun runs TS natively) |
| `oxc-project/oxlint-migrate` | 193 | MIT | 2026-11-01 | TS | ESLint flat config -> `.oxlintrc.json` | one-shot tool |
| `oxc-project/oxc-walker` | 127 | MIT | 2026-11-01 | TS | estree-walker over oxc ASTs (npm `oxc-walker` 1.1.1, peers `rolldown`, `oxc-parser`, `@oxc-project/types`) | optional for `scripts/*.ts` AST passes |
| `oxc-project/oxc-vscode` | 126 | MIT | 2026-11-01 | TS | VS Code extension | editor only |
| `oxc-project/bench-javascript-parser-written-in-rust` | 112 | none | 2026-11-03 | JS | Parser benchmark (oxc 3x swc, 5x Biome per README); this is the "javascript-parser-benchmark" repository | quoted numbers only |
| `oxc-project/oxc-intellij-plugin` | 110 | MIT | 2026-10-22 | Kotlin | IntelliJ plugin | editor only |
| `oxc-project/website` | 101 | none | 2026-11-04 | Vue | oxc.rs | docs |
| `oxc-project/bench-linter` | 85 | none | 2026-11-04 | JS | oxlint vs ESLint benchmark | quoted numbers only |
| `oxc-project/playground` | 74 | MIT | 2026-11-03 | Vue | Playground | none |
| `oxc-project/fast-glob` | 73 | MIT | 2026-11-01 | Rust | Glob matching crate | `Bun.Glob` exists; Rust side may use it |
| `oxc-project/oxc-browserslist` | 62 | MIT | 2026-11-04 | Rust | browserslist port (crate `oxc-browserslist` 5.0.1, 2026-08-12) | transitive via `oxc_transformer` targets |
| `oxc-project/bench-formatter` | 30 | none | 2026-11-04 | JS | Formatter benchmark | none |
| `oxc-project/sort-package-json` | 25 | MIT | 2026-11-01 | Rust | package.json sorter | none |
| `oxc-project/bench-transformer` | 25 | none | 2026-11-03 | TS | Transformer and isolated-declarations benchmark | quoted numbers only |
| `oxc-project/oxc-sourcemap` | 25 | NOASSERTION (crate: BSD-3-Clause) | 2026-11-03 | Rust | Forked rust-sourcemap (crate `oxc_sourcemap` 9.0.0, 2026-09-26) | transitive via `oxc_codegen` (`CodegenReturn.map`) |
| `oxc-project/coc-oxc` | 25 | MIT | 2026-10-22 | TS | coc.nvim extension | editor only |
| `oxc-project/oxc-webpack-loader` | 18 | MIT | 2026-11-02 | JS | webpack loader | none |
| `oxc-project/oxlint-action` | 16 | none | 2026-10-10 | Shell | GitHub Action | CI option |
| `oxc-project/oxc-yaml-parser` | 15 | MIT | 2026-11-03 | Rust | YAML parser | `Bun.YAML` exists |
| `oxc-project/oxc-css-parser` | 15 | MIT | 2026-11-04 | Rust | CSS parser | watch (yolo-web has Servo Stylo) |
| `oxc-project/oxc-ecosystem-ci` | 15 | none | 2026-11-03 | JS | Oxlint ecosystem CI | none |
| `oxc-project/json-strip-comments` | 14 | Apache-2.0 | 2026-11-01 | Rust | JSONC stripping | `Bun.JSONC` exists |
| `oxc-project/monitor-oxc` | 13 | none | 2026-11-02 | JS | E2E monitoring of oxc | none |
| `oxc-project/javascript-globals` | 13 | MIT | 2026-11-03 | Rust | Rust fork of npm `globals` | transitive |
| `oxc-project/oxc-jest` | 13 | none | 2025-11-11 | JS | Jest transformer; archived | none (`bun:test`) |
| `oxc-project/oxc-miette` | 11 | Apache-2.0 | 2026-08-11 | Rust | Forked miette (crate `oxc-miette` 4.0.0, 2026-08-10); repository archived | transitive via `oxc_diagnostics` |
| `oxc-project/forked-react-compiler` | 10 | MIT | 2026-09-25 | Rust | archived | none (Bun ships `src/react_compiler`) |
| `oxc-project/aria-query` | 10 | MIT | 2025-01-10 | Rust | archived | none |
| `oxc-project/oxc-graphql-parser` | 9 | Apache-2.0 | 2026-11-02 | Rust | GraphQL parser | none |
| `oxc-project/mirrors-oxlint` | 9 | none | 2026-10-19 | - | pre-commit mirror | none |
| `oxc-project/oxc-toml` | 9 | MIT | 2026-11-01 | Rust | Taplo fork for oxfmt | `Bun.TOML` exists |
| `oxc-project/oxfmt-action` | 8 | none | 2026-10-10 | Shell | GitHub Action | CI option |
| `oxc-project/setup-rust` | 8 | none | 2026-11-02 | - | GitHub Action | none |
| `oxc-project/backlog` | 7 | none | 2026-08-14 | - | "backlog for collaborators only" | none |

Repositories named in the brief that do not exist under these names: `oxc-project/javascript-parser-benchmark` (it is `bench-javascript-parser-written-in-rust`), `oxc-project/oxc_sourcemap` (it is `oxc-sourcemap`), `oxc-project/oxfmt` (oxfmt lives in the `oxc` monorepo; releases tagged `oxfmt_v0.76.0` on 2026-11-03), `oxc-project/oxc_index` (crate `oxc_index` 5.0.0 comes from `oxc-project/oxc-index-vec`). Release cadence in the monorepo: `crates_v0.157.0` 2026-11-02, `crates_v0.156.0` 2026-10-27, `oxlint_v1.91.0` 2026-11-03, `oxfmt_v0.76.0` 2026-11-03.

### B.2 crates.io versions (quoted 2026-11-04)

| Crate | Current | Published | License | rust-version | yolo-ui / yolo-stack pin | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `oxc` (umbrella) | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | not used | features: `default = ["regular_expression"]`, `full = [codegen, mangler, minifier, semantic, transformer, isolated_declarations, ast_visit, regular_expression]`, `serialize`, `cfg`, `conformance`; `transformer` also enables `oxc_transformer_plugins` |
| `oxc_parser` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | 0.155.0 | 0.155 is two weekly minors behind (0.156.0 on 2026-10-27, 0.157.0 on 2026-11-02); not current |
| `oxc_ast` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | 0.155.0 | same |
| `oxc_ast_visit` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | add (`VisitMut`) |
| `oxc_semantic` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | add |
| `oxc_transformer` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | add |
| `oxc_codegen` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | add |
| `oxc_minifier` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | add (production builds) |
| `oxc_mangler` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | with minifier |
| `oxc_isolated_declarations` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | `.d.ts` emit for `@yolo-rs/next` without `tsc` |
| `oxc_traverse` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | optional; see B.4 note on stability |
| `oxc_span` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | 0.155.0 | bump |
| `oxc_allocator` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | 0.155.0 | bump |
| `oxc_diagnostics` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | transitive | `Diagnostics` derefs to `Vec<OxcDiagnostic>` |
| `oxc_linter` | not on crates.io (HTTP 404) | - | - | - | - | oxlint is distributed as a binary/npm only |
| `oxc_language_server` | not on crates.io (HTTP 404) | - | - | - | - | same |
| `oxc_resolver` | 11.24.3 | 2026-08-24 | MIT | 1.95.0 | - | add for tsconfig `paths` in `crates/yolo-next` |
| `oxc_sourcemap` | 9.0.0 | 2026-09-26 | BSD-3-Clause | 1.95.0 | - | transitive; `cargo deny` must allow BSD-3-Clause |
| `oxc_index` | 5.0.0 | 2026-06-06 | MIT | 1.85.0 | transitive | - |
| `oxc_napi` / `oxc_transform_napi` / `oxc_parser_napi` | 0.157.0 | 2026-11-02 | MIT | 1.97.0 | - | the Rust side of the npm bindings; useful template for `crates/yolo-next-bun-plugin` |
| `oxc-browserslist` | 5.0.1 | 2026-08-12 | MIT | 1.95.0 | transitive | - |
| `oxc-miette` | 4.0.0 | 2026-08-10 | Apache-2.0 | 1.85.0 | transitive | repository archived 2026-08-11; crate still consumed by `oxc_diagnostics` |
| `oxc_module_lexer` | 0.38.0 | 2024-11-26 | MIT | 1.76 | - | stale; do not use |

Consequences for the bump 0.155 -> 0.157: (1) `oxc` 0.157 declares `rust-version = 1.97.0` while `Cargo.toml` here sets `rust-version = "1.95"`; raise the workspace MSRV to 1.97 (toolchain 1.99.1 is installed). (2) The `oxc_parser`, `oxc_ast`, and `oxc_transformer` CHANGELOGs list no `BREAKING` entries for 0.156.0 or 0.157.0; the last breaking batch was 0.154.0 (node IDs assigned during parsing, comment attachment storage) and is already absorbed by the 0.155 pin. (3) `yolo-ui/crates/yolo-migrator/src/tsx.rs` and `yolo-ui/crates/yolo-react/src/compiler.rs` already read `parsed.diagnostics` (the field name in 0.157 is `diagnostics`, not `errors`), so they compile unchanged in principle; run `cargo check` after the bump.

### B.3 npm packages (quoted 2026-11-04) and Bun 1.4.4 behaviour (measured)

| Package | Version | License | `engines` | win32-x64-msvc prebuilt | linux-x64-gnu prebuilt | Runs under Bun 1.4.4 (win32 x64, stable and canary) |
| --- | --- | --- | --- | --- | --- | --- |
| `oxc-parser` | 0.157.0 | MIT | `node ^20.19.0 \|\| >=22.12.0` | `@oxc-parser/binding-win32-x64-msvc` 0.157.0 | `@oxc-parser/binding-linux-x64-gnu` 0.157.0 (19 targets total, plus wasm32-wasi) | yes: `parseSync("x.ts", src, { lang: "ts" })` -> 2 body statements, 0 errors, `module.staticExports` populated |
| `oxc-transform` | 0.157.0 | MIT | same | `@oxc-transform/binding-win32-x64-msvc` 0.157.0 | `@oxc-transform/binding-linux-x64-gnu` 0.157.0 | yes: `await transform(...)` and `transformSync(...)` (B.5); `transform` returns a `Promise` since this line, the sync variant is `transformSync` |
| `oxc-minify` | 0.157.0 | MIT | same | `@oxc-minify/binding-win32-x64-msvc` 0.157.0 | `@oxc-minify/binding-linux-x64-gnu` 0.157.0 | yes: `await minify()` -> `export function f(){return 42}`; `minifySync()` also works |
| `oxc-resolver` | 11.24.2 (crate is 11.24.3) | MIT | - | `@oxc-resolver/binding-win32-x64-msvc` 11.24.2 | `@oxc-resolver/binding-linux-x64-gnu` 11.24.2 (19 targets, wasm32-wasi with browser fs) | yes: `new ResolverFactory({ conditionNames: ["bun", "import", "default"] }).sync(cwd, "satori").path` resolves |
| `oxlint` | 1.91.0 | MIT | same as oxc-parser | `@oxlint/binding-win32-x64-msvc` 1.91.0 (legacy `@oxlint/win32-x64` stopped at 1.43.0) | `@oxlint/binding-linux-x64-gnu` 1.91.0 | yes: `bunx oxlint@1.91.0 --version` -> `Version: 1.91.0`; lints a TS file; peers `oxlint-tsgolint`, `vite-plus` optional |
| `oxfmt` | 0.76.0 | MIT | same | `@oxfmt/binding-win32-x64-msvc` 0.76.0 (legacy `@oxfmt/win32-x64` at 0.28.0) | `@oxfmt/binding-linux-x64-gnu` 0.76.0 | yes: `bunx oxfmt@0.76.0 --check file.ts` -> "Finished in 3ms on 1 files using 24 threads" |
| `oxlint-tsgolint` | 7.0.2005 | MIT | - | `@oxlint-tsgolint/win32-x64` | `@oxlint-tsgolint/linux-x64` | not run (Go binary; optional type-aware lint) |
| `@oxc-project/types` | 0.157.0 | MIT | - | - | - | types only |
| `@oxc-project/runtime` | 0.157.0 | MIT | same | - | - | helper runtime when `helpers: { mode: "External" }` |
| `oxc-walker` | 1.1.1 (2026-07-31) | MIT | - | - | - | not run |

Bun ignores `engines.node` by default, so the `node >= 20.19` constraint is not enforced; Bun 1.4.4 reports `process.versions.napi = "10"`. The `bun check`, `bun lint`, and `bun fmt` commands listed in `AGENTS.md` are Bun built-ins (`src/sema`, `src/lint/oxlint`, `src/format` in the Bun workspace, section C.1); `oxlint`/`oxfmt` from npm remain the cross-check in CI.

### B.4 Using `oxc_transformer` + `oxc_codegen` from Rust (0.157.0, compiled and run 2026-11-04)

Probe crate: `oxc_allocator`, `oxc_ast`, `oxc_ast_visit`, `oxc_parser`, `oxc_span`, `oxc_semantic`, `oxc_transformer`, `oxc_codegen`, `oxc_traverse`, `oxc_diagnostics` all `= "0.157"`; rustc 1.99.1; `cargo build` succeeded; the binary produced the expected output for all four passes. Entry points as they exist in 0.157.0 sources:

| Step | API (verified in `~/.cargo/registry/src/*/oxc_*-0.157.0`) |
| --- | --- |
| Parse | `oxc_parser::Parser::new(&allocator, source_text, SourceType::from_path(path)?).parse()` -> `ParserReturn { program, diagnostics: oxc_diagnostics::Diagnostics, .. }` (`diagnostics`, not `errors`; `Diagnostics: Deref<Target = Vec<OxcDiagnostic>>`, `into_vec()`) |
| Semantic | `oxc_semantic::SemanticBuilder::new().build(&program).semantic.into_scoping()` -> `Scoping` (`SemanticBuilder::new_compiler()`, `new_linter()`, `with_check_syntax_error`, `with_cfg`, `with_build_nodes` also exist) |
| Transform | `oxc_transformer::Transformer::new(&allocator, source_path: &Path, &TransformOptions).build_with_scoping(scoping, &mut program)` -> `TransformerReturn { diagnostics, scoping, helpers_used (deprecated) }` (`#[non_exhaustive]`) |
| Options | `TransformOptions { cwd, assumptions: CompilerAssumptions, typescript: TypeScriptOptions, decorator, plugins: PluginsOptions, jsx: JsxOptions, env: EnvOptions, proposals, helper_loader }`; constructors `TransformOptions::enable_all()`, `TransformOptions::from_target("es2022")`, `TransformOptions::from_target_list(&[..])` |
| TS strip | `TypeScriptOptions { jsx_pragma, jsx_pragma_frag, only_remove_type_imports, allow_namespaces, allow_declare_fields, remove_class_fields_without_initializer, optimize_const_enums, rewrite_import_extensions: Option<RewriteExtensionsMode> }` |
| JSX | `JsxOptions { jsx_plugin, display_name_plugin, jsx_self_plugin, jsx_source_plugin, runtime: JsxRuntime::{Classic, Automatic}, development, throw_if_namespace, pure, import_source, pragma, pragma_frag, refresh: Option<ReactRefreshOptions> }`; `JsxOptions::default()` == `enable()`; `JsxOptions::disable()`; `development = true` forces `jsx_self_plugin`/`jsx_source_plugin` |
| Refresh | `ReactRefreshOptions { refresh_reg: String ("$RefreshReg$"), refresh_sig: String ("$RefreshSig$"), emit_full_signatures: bool }` |
| Codegen | `oxc_codegen::Codegen::new().with_options(CodegenOptions { single_quote, minify, comments, source_map_path: Option<PathBuf>, indent_width, .. }).with_scoping(Option<Scoping>).build(&program)` -> `CodegenReturn<'a> { code: String, map: Option<oxc_sourcemap::SourceMap<'a>>, legal_comments }` |
| AST mutation | `oxc_ast_visit::{VisitMut, walk_mut}` (`fn visit_import_declaration(&mut self, it: &mut ImportDeclaration<'a>)` + `walk_mut::walk_import_declaration(self, it)`), or `oxc_traverse::traverse_mut(&mut traverser, &allocator, &mut program, scoping, state) -> Scoping` with `trait Traverse<'a, State>` and `traverse_mut_with_ctx(&mut tr, &mut program, &mut ReusableTraverseCtx<'a, State>)` |
| Builders | `oxc_ast::builder::AstBuilder::new(&allocator)` (module path `oxc_ast::builder`, not the crate root); constructors are associated functions on node types, e.g. `Statement::new_export_declaration(span, Declaration, &ast)`, `Expression::new_string_literal(span, value, raw, &ast)`, `StringLiteral::boxed(..)`; `oxc_allocator::Vec::new_in(&impl GetAllocator)` |
| AST facts | `StringLiteral { node_id, span, value: JSStr<'a>, raw: Option<Str<'a>> }` with `JSStr::as_str(self) -> Option<&'a str>` and `JSStr: From<&'a str>`; `export function f() {}` is `Statement::ExportDeclaration(Box<ExportDeclaration { declaration: Declaration }>)` (distinct from `ExportNamedDeclaration { specifiers, export_kind }` and `ExportFromDeclaration`); `Directive { expression: StringLiteral, directive: Str<'a> }`; `FunctionBody { directives, statements }` |

Verified probe (abridged; the full file compiled as `src/main.rs`):

```rust
use std::path::Path;
use oxc_allocator::Allocator;
use oxc_ast::{ast::*, builder::AstBuilder};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_codegen::{Codegen, CodegenOptions};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{SPAN, SourceType};
use oxc_transformer::{JsxOptions, JsxRuntime, ReactRefreshOptions, TransformOptions, Transformer, TypeScriptOptions};

/// (3) rewrite `next/*` import specifiers in place
struct ImportRewriter { rewritten: usize }
impl<'a> VisitMut<'a> for ImportRewriter {
    fn visit_import_declaration(&mut self, decl: &mut ImportDeclaration<'a>) {
        let target: Option<&'static str> = match decl.source.value.as_str() {
            Some("next/font/google") => Some("@yolo-rs/next/shims/font-google"),
            Some("next/image") => Some("@yolo-rs/next/shims/image"),
            _ => None,
        };
        if let Some(t) = target { decl.source.value = t.into(); decl.source.raw = None; self.rewritten += 1; }
        walk_mut::walk_import_declaration(self, decl);
    }
}

fn is_use_server(f: &Function<'_>) -> bool {
    f.body.as_ref().is_some_and(|b| b.directives.iter().any(|d| d.directive.as_str() == "use server"))
}

/// (4) hoist inline `"use server"` function declarations into module-level exports
fn hoist_use_server<'a>(ast: AstBuilder<'a>, program: &mut Program<'a>) -> usize {
    let mut hoisted: Vec<oxc_allocator::Box<'a, Function<'a>>> = Vec::new();
    for stmt in program.body.iter_mut() {
        let func: Option<&mut Function<'a>> = match stmt {
            Statement::FunctionDeclaration(f) => Some(&mut **f),
            Statement::ExportDefaultDeclaration(e) => match &mut e.declaration {
                ExportDefaultDeclarationKind::FunctionDeclaration(f) => Some(&mut **f), _ => None },
            Statement::ExportDeclaration(e) => match &mut e.declaration {
                Declaration::FunctionDeclaration(f) => Some(&mut **f), _ => None },
            _ => None,
        };
        let Some(body) = func.and_then(|f| f.body.as_mut()) else { continue };
        let mut i = 0;
        while i < body.statements.len() {
            if matches!(&body.statements[i], Statement::FunctionDeclaration(f) if is_use_server(f)) {
                let Statement::FunctionDeclaration(f) = body.statements.remove(i) else { unreachable!() };
                hoisted.push(f);
            } else { i += 1; }
        }
    }
    let n = hoisted.len();
    for f in hoisted {
        program.body.push(Statement::new_export_declaration(SPAN, Declaration::FunctionDeclaration(f), &ast));
    }
    n
}

fn main() {
    let allocator = Allocator::default();
    let path = Path::new("app/page.tsx");
    let parsed = Parser::new(&allocator, SRC, SourceType::from_path(path).unwrap()).parse();
    assert!(parsed.diagnostics.is_empty());
    let mut program = parsed.program;
    let ast = AstBuilder::new(&allocator);
    let mut rw = ImportRewriter { rewritten: 0 };
    rw.visit_program(&mut program);
    let hoisted = hoist_use_server(ast, &mut program);
    // scoping is built after our own mutations and consumed by the transformer
    let scoping = SemanticBuilder::new().build(&program).semantic.into_scoping();
    // (1) strip TypeScript + (2) JSX automatic runtime with react-refresh registration
    let mut options = TransformOptions::from_target("es2022").unwrap();
    options.typescript = TypeScriptOptions::default();
    options.jsx = JsxOptions { runtime: JsxRuntime::Automatic, development: true,
        refresh: Some(ReactRefreshOptions::default()), ..JsxOptions::default() };
    let t = Transformer::new(&allocator, path, &options).build_with_scoping(scoping, &mut program);
    assert!(t.diagnostics.is_empty());
    let out = Codegen::new()
        .with_options(CodegenOptions { source_map_path: Some(path.to_path_buf()), ..CodegenOptions::default() })
        .build(&program);
    println!("{}", out.code); // out.map.is_some() == true
}
```

Measured output for the fixture (`import { Inter } from "next/font/google"; import Image from "next/image"; type Props = {...}; export default function Page({ n }: Props) { async function save(formData: FormData) { "use server"; ... } return <form action={save} ...>...</form>; }`):

```js
import { Inter } from "@yolo-rs/next/shims/font-google";
import Image from "@yolo-rs/next/shims/image";
var _jsxFileName = "app/page.tsx";
import { jsxDEV as _jsxDEV } from "react/jsx-dev-runtime";
const inter = Inter({ subsets: ["latin"] });
export default function Page({ n }) {
	return /* @__PURE__ */ _jsxDEV("form", { action: save, className: inter.className, children: [...] }, void 0, true, { fileName: _jsxFileName, lineNumber: 11, columnNumber: 10 }, this);
}
_c = Page;
export async function save(formData) {
	"use server";
	console.log(formData.get("x"));
}
var _c;
$RefreshReg$(_c, "Page");
// rewritten_imports=2 hoisted_server_fns=1 sourcemap=true
```

Notes and unverified points:

- The hoist shown is the closure-free case. Next.js encrypts captured variables for inline actions; `crates/yolo-next/src/rsc.rs` must reject or encrypt closures (`Bun.CryptoHasher` + AES-GCM via WebCrypto on the Bun side) before shipping. Unverified here.
- `oxc_traverse` compiled, and its `traverse_mut` signature is as listed, but oxc documents it as an internal crate for the transformer; prefer `oxc_ast_visit::VisitMut` for yolo-stack passes. Semver stability of `oxc_traverse` is unverified.
- `TransformOptions::from_preset_env`, `EnvOptions::from_browserslist_query`, and the `plugins.styled_components` option exist in the sources but were not exercised.
- `JsxOptions.refresh` emits `$RefreshReg$`/`$RefreshSig$` calls only; the dev server must inject the `react-refresh` 0.19.0 runtime (`react-refresh/runtime`) and define those globals, which Bun's HMR (`development: { hmr: true }`) does for its own React Fast Refresh path. How the two interact when a custom plugin pre-transforms JSX is unverified; the native-plugin route (C.2) sidesteps it because Bun still owns the final parse.

### B.5 `oxc-transform` inside a Bun plugin `onLoad` (measured 2026-11-04)

Call verified under both Bun 1.4.4 builds:

```ts
import { transform } from "oxc-transform"; // 0.157.0: transform() is async, transformSync() is sync
const r = await transform("app/page.tsx", source, {
  typescript: {},
  jsx: { runtime: "automatic", development: true, refresh: {} },
  sourcemap: true,
});
// r = { code, map: { mappings, ... }, errors: [], helpersUsed }   (10.7 ms cold on the first call)
```

Emitted code contained `import { jsxDEV as _jsxDEV } from "react/jsx-dev-runtime"`, `_c = Page; $RefreshReg$(_c, "Page")`, and `var _jsxFileName = "app/page.tsx"`; `transformSync` with `jsx: { runtime: "automatic" }` emitted the production `react/jsx-runtime` form. `TransformOptions` (from `index.d.ts`): `lang`, `sourceType`, `cwd`, `assumptions`, `typescript`, `decorator`, `plugins` (`styledComponents`, `taggedTemplateEscape`), `jsx: "preserve" | JsxOptions`, `target: string | string[]`, `helpers`, `inject`, `define`, `sourcemap`. `JsxOptions`: `runtime`, `development`, `throwIfNamespace`, `pure`, `importSource`, `pragma`, `pragmaFrag`, `refresh?: boolean | ReactRefreshOptions`. `TransformResult`: `code`, `map?`, `declaration?`, `declarationMap?`, `errors`, `helpersUsed`.

Plugin shape for `packages/yolo-next/src/plugin.ts` (Bun `onLoad` callbacks may be async; `[serve.static] plugins` and `Bun.build({ plugins })` accept the same object):

```ts
import type { BunPlugin } from "bun";
import { transform } from "oxc-transform";
import { ResolverFactory } from "oxc-resolver";

const NEXT_SHIMS: Record<string, string> = {
  "next/font/google": "@yolo-rs/next/shims/font-google",
  "next/font/local": "@yolo-rs/next/shims/font-local",
  "next/image": "@yolo-rs/next/shims/image",
  "next/link": "@yolo-rs/next/link",
  "next/navigation": "@yolo-rs/next/navigation",
  "next/headers": "@yolo-rs/next/headers",
  "next/cache": "@yolo-rs/next/cache",
  "next/og": "@yolo-rs/next/og",
};

export function yoloNextPlugin(opts: { dev: boolean; cwd?: string } = { dev: false }): BunPlugin {
  const resolver = new ResolverFactory({ tsconfig: { configFile: `${opts.cwd ?? process.cwd()}/tsconfig.json` }, conditionNames: ["bun", "import", "default"] });
  return {
    name: "yolo-next-oxc",
    setup(build) {
      build.onResolve({ filter: /^next\// }, ({ path, importer }) => {
        const target = NEXT_SHIMS[path];
        return target ? { path: resolver.sync(importer, target).path ?? target } : undefined;
      });
      build.onLoad({ filter: /\.[cm]?tsx?$/ }, async ({ path }) => {
        const source = await Bun.file(path).text();
        const { code, map, errors } = await transform(path, source, {
          typescript: {},
          jsx: { runtime: "automatic", development: opts.dev, refresh: opts.dev ? {} : false },
          target: "es2022",
          sourcemap: true,
        });
        if (errors.length) throw new AggregateError(errors.map((e) => new Error(e.message)), path);
        const inline = map ? `\n//# sourceMappingURL=data:application/json;base64,${btoa(JSON.stringify(map))}` : "";
        return { contents: code + inline, loader: "js" };
      });
    },
  };
}
```

Register it either in `bunfig.toml` (`[serve.static] plugins = ["@yolo-rs/next/plugin"]`, which requires the module to export a plugin object with `name` and `setup`) or in `Bun.build({ plugins: [yoloNextPlugin({ dev })] })`. The native alternative (`onBeforeParse`, section C.2) runs the same passes in Rust on Bun's parser threads and avoids the UTF-8 to UTF-16 copy; use it for the import rewrite and the Server Action hoist, keep the JS `onLoad` as a fallback when the `.node` artifact is absent.

## C. Bun native (verified against Bun 1.4.4)

### C.1 Bun is a Rust workspace with JavaScriptCore (C++) bindings

Evidence (quoted 2026-11-04 from `oven-sh/bun` `main`, 96,546 stars):

| Fact | Evidence |
| --- | --- |
| Root `Cargo.toml` is a Cargo workspace | `[workspace] resolver = "2"`, 134 `src/*` members, `edition = "2024"`, `[profile.release]`, `[profile.shim]`; `[workspace.dependencies]` lists every `bun_*` crate plus `hashbrown`, `allocator-api2`, `rustix`, `encoding_rs`, `lol_html = { path = "vendor/lolhtml" }`, `rust-argon2`, `bcrypt`, `getrandom` |
| Language bytes | Rust 85,343,647; C++ 13,434,510; TypeScript 9,367,752; C 1,439,448; JavaScript 757,627 (`GET /repos/oven-sh/bun/languages`) |
| Zig is gone | 0 `.zig` paths in the git tree (57,368 entries returned, listing truncated by GitHub, so treat as "none seen"); 4,291 `.rs` files; 1,385 `.cpp/.h` files |
| JavaScriptCore boundary | crates `src/jsc`, `src/jsc_macros`, and the `*_jsc` crates (`ast_jsc`, `bundler_jsc`, `css_jsc`, `http_jsc`, `install_jsc`, `js_parser_jsc`, `patch_jsc`, `semver_jsc`, `sourcemap_jsc`, `sql_jsc`, `sys_jsc`); `src/runtime/napi` implements Node-API; `src/runtime/webview`, `src/runtime/image`, `src/runtime/server`, `src/runtime/shell`, `src/runtime/test_runner` |
| Compiler stack | `src/js_parser`, `src/js_printer`, `src/ast`, `src/bundler` (with `ServerComponentParseTask.rs`, `HTMLImportManifest.rs`, `aot/`), `src/transpiler`, `src/css` (own CSS parser), `src/sema` (TypeScript checker for `bun check`), `src/lint/*` (`oxlint`, `eslint`, `typescript`, `react`, `react_compiler`, `jest`, `unicorn`, `json`, `markdown`, `plugins`), `src/format/*` (`bun fmt`: js, css, html, json, markdown, yaml, toml, svelte, graphql, handlebars, tailwind), `src/react_compiler`, `src/aot` (ahead-of-time compilation), `src/standalone_graph` and `src/exe_format` (PE/ELF/Mach-O for `--compile`) |
| Not using oxc crates | `grep -c oxc Cargo.toml` on the root manifest returns 0; `src/lint/oxlint` is Bun's own oxlint-compatible rule set |
| Native plugin crate in-tree | `packages/bun-native-plugin-rs/Cargo.toml` is `bun-native-plugin` 0.2.0 (deps `anyhow 1.0.94`, `bun-macro 0.1.0` at `./bun-macro`), `headers/`, `wrapper.h`, `copy_headers.ts`; C header API in `packages/bun-native-bundler-plugin-api`; `packages/bun-build-mdx-rs` is a second in-tree Rust plugin example |
| Other `packages/` | `bun-types`, `bun-plugin-svelte` (0.0.6), `bun-plugin-yaml` (0.0.1 sample), `bun-debug-adapter-protocol`, `bun-inspector-frontend`, `bun-inspector-protocol`, `bun-vscode`, `bun-lambda`, `bun-release`, `bun-error`, `bun-usockets`, `bun-uws`, `h3blast` |

Rust sits beneath Bun, oxc, and yolo-stack alike; the C ABI of the native plugin hook and Node-API are the two stable seams between them.

### C.2 `bun-native-plugin` crate (verified build and load on Windows x64, 2026-11-04)

| Item | Value |
| --- | --- |
| crates.io | `bun-native-plugin` 0.2.0 (2025-02-05, MIT); `bun-macro` 0.1.0 (2025-01-15, MIT); no newer version in-tree (`packages/bun-native-plugin-rs/Cargo.toml` also says 0.2.0) |
| Crate deps | `anyhow = "1.0.94"`, `bun-macro = "0.1.0"`; napi is not a dependency of the crate; the plugin cdylib brings `napi` + `napi-derive` because `define_bun_plugin!` expands to `#[napi] fn bun_plugin_register() {}` and `pub static BUN_PLUGIN_NAME: BunPluginName` |
| napi versions used | `napi` 3.16.1 (crates.io 2026-10-31, `default-features = false, features = ["napi4"]`), `napi-derive` 3.7.0 (2026-10-30), `napi-build` 2.6.0 (2026-10-01), `napi-sys` 3.4.1; cargo also resolved `napi` 2.16.17 transitively during the build and the final cdylib loaded without conflict |
| Public API (`src/lib.rs`) | `pub use bun_macro::bun;` `pub mod sys;` `define_bun_plugin!("name")`; `OnBeforeParse<'a>` with `from_raw(args, result)`, `path()`, `namespace()`, `input_source_code() -> PluginResult<Cow<str>>`, `set_output_source_code(String, BunLoader)`, `set_output_loader(BunLoader)`, `output_loader() -> BunLoader`, `log(message, BunLogLevel)`, `log_error(message)`; `TaggedObject<T>` for `external` state; `BunLoader::{BUN_LOADER_JS, JSX, TS, TSX, CSS, JSON, TOML, TEXT, FILE, WASM, NAPI, BASE64, DATAURL}`; `type Result<T> = anyhow::Result<T>` |
| `#[bun]` macro | Takes `pub fn name(handle: &mut OnBeforeParse) -> Result<()>` and generates `pub unsafe extern "C" fn name(args_raw: *mut sys::OnBeforeParseArguments, result: *mut sys::OnBeforeParseResult)` wrapped in `std::panic::catch_unwind`, logging returned errors |
| Only hook | `onBeforeParse` (docs: "These lifecycle hooks are available to native plugins: onBeforeParse()") |
| Threading | "Bun can call this callback from any thread, so the NAPI module implementation must be thread-safe"; state via `External<T>` must be `Sync` |
| JS registration | `build.onBeforeParse({ filter: /\.tsx?$/, namespace?: "file" }, { napiModule: require("./plugin.node"), symbol: "fn_name", external?: state })` (typed in `bun.d.ts`: `onBeforeParse(constraints: PluginConstraints, callback: OnBeforeParseCallback): this`) |

Verified example (Cargo project `crate-type = ["cdylib"]`, `build.rs` calling `napi_build::setup()`):

```rust
use bun_native_plugin::{bun, define_bun_plugin, OnBeforeParse, Result};
use napi_derive::napi;

define_bun_plugin!("yolo-rewrite-next-imports");

#[bun]
pub fn rewrite_next_imports(handle: &mut OnBeforeParse) -> Result<()> {
    let input = handle.input_source_code()?;
    let loader = handle.output_loader();
    let output = input
        .replace("\"next/font/google\"", "\"@yolo-rs/next/shims/font-google\"")
        .replace("\"next/image\"", "\"@yolo-rs/next/shims/image\"");
    handle.set_output_source_code(output, loader);
    Ok(())
}
```

```ts
const napiModule = require("./plugin.node"); // exports: bunPluginRegister
const r = await Bun.build({
  entrypoints: ["./entry.ts"], target: "bun", external: ["@yolo-rs/*", "next/*"],
  plugins: [{ name: "yolo-native-rewrite", setup(build) {
    build.onBeforeParse({ filter: /\.ts$/ }, { napiModule, symbol: "rewrite_next_imports" });
  } }],
});
```

Result on Windows x64: `cargo build` 24.7 s cold (10.9 s with a warm registry), the DLL exports `rewrite_next_imports`, `BUN_PLUGIN_NAME`, `bun_plugin_register`, `napi_register_module_v1`; `Bun.build` returned `success: true` and the output chunk contained `import { Inter } from "@yolo-rs/next/shims/font-google"` on both Bun 1.4.4 stable and 1.4.4-canary.1. Conclusion: usable on Windows (MSVC toolchain, `x86_64-pc-windows-msvc`), no Node binary required at build or load time. The production version of this plugin (`crates/yolo-next-bun-plugin`) replaces the string replace with the `oxc_parser` + `VisitMut` passes of B.4 and returns `BunLoader::BUN_LOADER_TSX` so Bun's parser still performs the final JSX/TS lowering.

### C.3 Built-in modules and APIs relevant to the framework

All names below were present on `Bun` in both 1.4.4 builds (`Object.getOwnPropertyNames(Bun)`), and the `bun:*` modules imported successfully. Status is taken from `bun-types@1.4.4` doc comments and `docs/*.mdx`.

| API | Exact name | Status in 1.4.4 | Framework role | Docs (`bun-types@1.4.4/docs/...`, mirrors `https://bun.com/docs/...`) |
| --- | --- | --- | --- | --- |
| SQLite | `import { Database } from "bun:sqlite"` (6 exports) | stable | OG/image cache, analytics collector, route manifests, flag overrides | `runtime/sqlite.mdx` |
| FFI | `import { dlopen, cc, ptr, JSCallback } from "bun:ffi"` (15 exports) | stable | optional bridge to `crates/yolo-next` without napi (TinyCC `cc` for C shims) | `runtime/ffi.mdx` |
| JSC | `import { heapStats, serialize, deserialize } from "bun:jsc"` (38 exports) | low-level | heap snapshots in `bench/`, structured clone of RSC payload caches | `https://bun.com/reference/bun/jsc` |
| Test | `import { test, expect, mock } from "bun:test"` (22 exports) | stable | all package tests (`bun test`) | `test/` |
| HTTP server + routes | `Bun.serve({ routes: { "/": indexHtml, "/users/:id": (req: BunRequest<"/users/:id">) => Response, "/api/x": { GET, POST } }, development: { hmr, console }, websocket, fetch })`; route values `Response | false | HTMLBundle | BunFile | DirectoryRouteOptions | handler | per-method map` | stable | App Router host: file routes from `crates/yolo-stack-routes` compile to `routes`, HTML imports for shells, `server.reload({ routes })` for hot route reloading | `runtime/http/server.mdx`, `runtime/http/routing.mdx`, `bundler/fullstack.mdx` |
| Static plugins | `bunfig.toml` `[serve.static] plugins = ["bun-plugin-tailwind" \| "./my-plugin.ts"]`, `env = "PUBLIC_*"`, `sourcemap = "linked"` | stable | loads `@yolo-rs/next/plugin` for every HTML/route bundle in dev and `bun build` | `bundler/fullstack.mdx` ("Plugins"), `bundler/html-static.mdx` |
| Bundler | `Bun.build({ entrypoints, outdir, target: "bun" \| "browser" \| "node", splitting, minify, sourcemap, plugins, define, env, bytecode, compile: boolean \| CompileTarget \| CompileBuildOptions })`; `CompileBuildOptions { target, outfile, autoloadTsconfig, windows: { hideConsole, icon }, ... }` | stable | three-pass build (server, client, RSC) in `packages/yolo-next/src/build.ts`; `--compile` for single-binary deploys | `bundler/index.mdx`, `bundler/executables.mdx` |
| Plugins | `Bun.plugin({ name, setup(build) { build.onStart / onResolve / onLoad / onEnd / onBeforeParse } })`, `bunfig.toml` `preload = ["./plugin.ts"]`; namespaces (`file` default) | stable (`onBeforeParse` native: experimental per README) | runtime loader for `next/*` shims in tests; bundler plugin in builds | `runtime/plugins.mdx`, `bundler/plugins.mdx` |
| Transpiler | `new Bun.Transpiler({ loader: "tsx", target, define, macro })`, `.transformSync()`, `.scanImports()` | stable | fast import scanning for `"use client"` / `"use server"` boundary detection without oxc | `runtime/transpiler.mdx` |
| S3 | `Bun.S3Client`, `Bun.s3.file("bucket/key")`, presigned URLs | stable | CDN output target of `crates/yolo-next/src/cdn.rs` (`@yolo-rs/next/cdn`) | `runtime/s3.mdx` |
| Cookies | `Bun.Cookie`, `Bun.CookieMap` (`req.cookies` on `BunRequest`) | stable | `@yolo-rs/next/headers` `cookies()` | `runtime/cookies.mdx` |
| CSRF | `Bun.CSRF.generate(secret, { sessionId, expiresIn })`, `Bun.CSRF.verify(token, { secret, sessionId })` (HMAC, timestamped) | stable | Server Action POST protection, flags toolbar endpoint | `runtime/csrf.mdx` |
| Image | `new Bun.Image(path \| bytes \| Blob, options?)`, `Blob#image()`; `.resize(w, h?, { fit: "fill" \| "inside", withoutEnlargement, filter: "lanczos3" \| "lanczos2" \| "mitchell" \| "cubic" \| "mks2013" \| "mks2021" \| "bilinear" \| "box" \| "nearest" })`, `.rotate(deg)`, `.flip()`, `.flop()`, `.modulate({...})`, `.jpeg({ quality, progressive })`, `.png({ compressionLevel, palette, colors, dither })`, `.webp({ quality, lossless })`, `.heic({ quality })`, `.avif({ quality })` (HEIC/AVIF: macOS and Windows only), terminals `.bytes()`, `.buffer()`, `.toBuffer()`, `.blob()`, `.dataurl()`, `.placeholder()` (ThumbHash LQIP <= 32 px), `.write(dest)`, `.metadata()`; `static backend: "system" \| "bun"`, `fromClipboard()`, `hasClipboardImage()`, `clipboardChangeCount()`; built on libjpeg-turbo, spng, libwebp, SIMD kernels, work runs off the JS thread | stable (documented, no unstable tag) | `next/image` replacement (`@yolo-rs/next/image` + `/_yolo/image` route): resize, format negotiation, blur placeholders; pin `backend = "bun"` for byte-identical output across Linux/Windows | `runtime/image.mdx` |
| Markdown | `Bun.markdown.html(md, opts)`, `Bun.markdown.ansi()`, `Bun.markdown.render(md, callbacks)`, `Bun.markdown.react(md)`; GFM by default; options `tables`, `strikethrough`, `tasklists`, `hardSoftBreaks`, `wikiLinks`, `underline`, `latexMath`, `collapseWhitespace`, `permissiveAtxHeaders`, `noIndentedCodeBlocks`, `noHtmlBlocks`, `noHtmlSpans` | unstable ("Unstable API" banner) | docs routes and MDX-free content pages; replaces `streamdown`/`react-markdown` | `runtime/markdown.mdx` |
| YAML / TOML / JSON5 / JSONC / JSONL / XML | `Bun.YAML.parse/stringify`, `Bun.TOML.parse/stringify`, `Bun.JSON5`, `Bun.JSONC`, `Bun.JSONL`, `Bun.XML`; `import cfg from "./x.yaml"` works natively | stable | `fusion.toml`, `ecosystem.toml` loaders (`scripts/fusion.ts` already uses `Bun.TOML.parse`) | `runtime/yaml.mdx`, `runtime/toml.mdx`, `runtime/json5.mdx`, `runtime/jsonl.mdx`, `runtime/xml.mdx` |
| Color | `Bun.color(input, "css" \| "ansi" \| "ansi-16" \| "ansi-256" \| "ansi-16m" \| "number" \| "rgb" \| "rgba" \| "hsl" \| "hex" \| "HEX" \| "[rgb]" \| "[rgba]" \| "{rgb}" \| "{rgba}")` (uses Bun's CSS parser) | stable | normalizing theme input before HCT/CAM16 conversion in `yolo-tokens` emitters | `runtime/color.mdx` |
| WebView | `new Bun.WebView({ width, height, url, backend: "webkit" \| "chrome" \| { type: "chrome", url \| path \| argv }, dataStore })`, `navigate`, `click`, `type`, `press`, `scroll`, `evaluate`, `screenshot({ encoding: "blob" \| "buffer" \| "base64" \| "shmem", format: "png" \| "jpeg" \| "webp", quality })`, `await using`; WKWebView on macOS, CDP-driven Chrome/Chromium/Edge/Brave on Linux and Windows (preinstalled Edge suffices) | documented (new) | screenshot gate in `bench/` and skill reviews; overlaps `agent-browser` and yolo-web Obscura | `runtime/webview.mdx` |
| Cron | `Bun.cron("0 * * * *", cb)` (in-process job with `.stop()`, `.unref()`), `Bun.cron.parse(expr, from?)`, OS-level `Bun.cron("./worker.ts", "30 2 * * MON", "weekly-report")` (crontab / launchd / Task Scheduler), `Bun.cron.remove(name)` | documented (new) | ISR-style revalidation, OG cache sweeps, sitemap rebuilds | `runtime/cron.mdx` |
| Terminal | `new Bun.Terminal({ cols, rows, ... })`, `Bun.spawn({ terminal })` (PTY) | documented | `yolo-next dev` TUI panes, `bench/` runners | `runtime/child-process.mdx` |
| Redis / Valkey | `Bun.redis`, `new Bun.RedisClient(url)` | stable | session store option | `runtime/redis.mdx` |
| SQL | `Bun.SQL`, `Bun.sql`, `Bun.postgres` (Postgres and MySQL drivers in `src/sql/{postgres,mysql}`) | stable | data layer for Server Actions; no ORM | `runtime/sql.mdx` |
| HTMLRewriter | `new HTMLRewriter().on(selector, handlers).transform(response)` (lol-html, vendored) | stable | streaming injection of RSC payload, `<link rel=preload>` for fonts and images, nonce insertion | `runtime/html-rewriter.mdx` |
| Password / hashing | `Bun.password.hash/verify` (argon2id, bcrypt), `Bun.hash`, `Bun.CryptoHasher`, `Bun.sha` | stable | auth helpers, cache keys | `runtime/hashing.mdx` |
| Semver | `Bun.semver.satisfies(version, range)`, `Bun.semver.order(a, b)` | stable | `ecosystem.toml` doctor | `runtime/semver.mdx` |
| Glob | `new Bun.Glob("app/**/page.tsx").scan({ cwd })` | stable | App Router discovery in `packages/yolo-next/src/router.ts` | `runtime/glob.mdx` |
| ModuleGraph | `new Bun.ModuleGraph({ globals, onError })`, `graph.import()`, `graph.run()`, `graph.dispose()`, `ModuleGraph.current` | experimental (`@experimental`, docs warning) | per-tenant or per-request isolation of RSC renders and Server Actions without `vm`; replaces `@edge-runtime/vm` | `runtime/module-graph.mdx` |
| Secrets | `Bun.secrets.get/set/delete({ service, name })` (OS credential store) | stable | CDN and S3 credentials on developer machines | `runtime/secrets.mdx` |
| Compile / AOT | `bun build --compile --target=bun-{linux-x64, linux-arm64, windows-x64, windows-arm64, darwin-x64, darwin-arm64}[-musl] --bytecode --minify --sourcemap --outfile app`; `Bun.build({ compile: { target, outfile, windows: { icon, hideConsole } }, bytecode: true })`; `-baseline`/`-modern` suffixes accepted for compatibility | stable | single-binary deploys to yolo-vps; the `src/aot` crate exists in the Bun workspace but no public `aot` flag is documented in 1.4.4 (unverified; do not rely on it) | `bundler/executables.mdx`, `bundler/bytecode.mdx` |
| Other present globals | `Bun.FetchSession`, `Bun.Archive`, `Bun.FileSystemRouter`, `Bun.embeddedFiles`, `Bun.isStandaloneExecutable`, `Bun.registerMacro`, `Bun.openInEditor`, `Bun.stripANSI`, `Bun.wrapAnsi`, `Bun.sliceAnsi`, `Bun.stringWidth`, `Bun.zstdCompress*`, `Bun.udpSocket`, `Bun.dns`, `Bun.inspect`, `Bun.deepEquals`, `Bun.randomUUIDv7` | mixed | as needed | `runtime/utils.mdx` |

`bun:internal-for-testing` is not importable from user code (ENOENT), as expected.

### C.4 Official and semi-official Bun plugins (quoted 2026-11-04)

| Package | Version | Date | Publisher / repo | Status | yolo-stack |
| --- | --- | --- | --- | --- | --- |
| `bun-plugin-tailwind` | 0.1.2 | 2025-10-09 | npm publisher `zackradisic` (Bun team); `package.json` repository field points to `tailwindlabs/tailwindcss`; the package is no longer in `oven-sh/bun/packages` | referenced by the Bun docs as the `[serve.static] plugins` example | `reject` (Tailwind is banned here); the plugin remains the reference implementation of a static-route plugin |
| `bun-plugin-svelte` | 0.0.6 | 2025-03-26 | `oven-sh/bun/packages/bun-plugin-svelte` | official | `reject` (no Svelte) |
| `bun-plugin-yaml` | 0.0.1 | 2023-05-03 (npm, third party, `js-yaml`); an unrelated 0.0.1 sample lives in `oven-sh/bun/packages/bun-plugin-yaml` | - | obsolete: `.yaml` imports and `Bun.YAML` are built in | `reject` |
| `bun-plugin-dts` | 0.4.0 | 2026-02-10 | `wobsoriano` (third party) | community | `reject`; `.d.ts` come from `oxc_isolated_declarations` / `isolatedDeclaration()` |
| `@bun/*` scope | none | - | npm search `scope:bun` returns 0 packages | - | nothing to pin |
| `bun-types` / `@types/bun` | 1.4.4 / 1.4.4 | - | `oven-sh/bun` | official | already in `package.json` `devDependencies` |

### C.5 Bun facts that change framework design

- Isolated installs are Bun's default layout here (`node_modules/.bun/<name>@<version>`), so plugins must resolve binding packages through the package itself (`oxc-transform` does; direct `require("@oxc-transform/binding-win32-x64-msvc")` fails).
- `Bun.serve` HTML imports plus `[serve.static] plugins` give the dev loop (HMR, console forwarding) without Vite; production uses `bun build` ahead of time (docs recommend AOT bundling).
- `bun check`, `bun lint`, `bun fmt` are native (Rust) and already mandated by `AGENTS.md`; `oxlint`/`oxfmt` npm binaries remain the second opinion.
- `Bun.markdown` and `Bun.ModuleGraph` are marked unstable/experimental; gate them behind feature flags in `@yolo-rs/next` and keep fallbacks (`marked`-free: render on the Rust side via `crates/yolo-next` if needed).

## D. Combination architecture

### D.1 Layers

```text
+---------------------------------------------------------------------------------------------+
| L7  Rust UI lanes            crates/yolo-stack-dioxus (Dioxus 0.8 islands, desktop, Blitz)   |
|                              crates/yolo-stack-leptos (optional, feature-gated)              |
|                              json-render spec -> RSX renderer (later)                        |
+---------------------------------------------------------------------------------------------+
| L6  Quality gates            repos/agent-skills: react-best-practices, web-design-guidelines, |
|     (vercel-labs skills)     writing-guidelines, composition-patterns, react-view-transitions, |
|                              react-native-skills; repos/web-interface-guidelines; agent-browser|
|                              + Bun.WebView screenshots; oxlint/oxfmt cross-check in CI        |
+---------------------------------------------------------------------------------------------+
| L5  Client data + routing    @yolo-rs/stack-tanstack: TanStack Router 1.170.46, Query 5.106.0,|
|                              swr 2.5.1 compat export, React 19.3.1 <ViewTransition>           |
+---------------------------------------------------------------------------------------------+
| L4  @yolo-rs/next App Router packages/yolo-next: server.ts (Bun.serve routes + HTMLRewriter), |
|                              rsc.ts (react-server-dom-parcel 19.3.1 Flight), actions.ts       |
|                              (Bun.CSRF + hoisted "use server"), fonts.ts / shims/font-*.ts    |
|                              (self-hosted, geist optional), image.ts (Bun.Image), og.ts       |
|                              (satori + resvg-wasm), cdn.ts (Bun.S3Client), cache.ts           |
|                              (bun:sqlite + Bun.cron), mcp.ts (mcp-handler on Bun.serve),      |
|                              shims/{analytics,speed-insights,flags,swr}.ts                    |
+---------------------------------------------------------------------------------------------+
| L3  OXC transforms           Rust: crates/yolo-next/src/transform.rs (oxc_parser/semantic/    |
|                              transformer/codegen 0.157): TS strip, JSX automatic + refresh,   |
|                              next/* import rewrite, Server Action hoist; shipped to Bun as    |
|                              crates/yolo-next-bun-plugin (bun-native-plugin 0.2.0 + napi 3,   |
|                              onBeforeParse, multithreaded)                                    |
|                              TS: packages/yolo-next/src/plugin.ts (oxc-transform 0.157.0 napi |
|                              in onLoad, oxc-resolver 11.24.2 for tsconfig paths) as fallback  |
|                              and for Bun.build pipelines; oxc_isolated_declarations for .d.ts |
+---------------------------------------------------------------------------------------------+
| L2  crates/yolo-next         Next 16 conventions compiler (routes, segments, metadata, rules,  |
|                              turbopack -> Bun.build, fonts, images, cdn, proxy) emitting Bun, |
|                              Dioxus or Leptos targets; oxc_* 0.157; oxc_resolver 11.24.3      |
+---------------------------------------------------------------------------------------------+
| L1  Bun 1.4.4 runtime        Bun.serve routes + HTML imports, Bun.build (+compile/bytecode),  |
|     (Rust + JSC)             Bun.plugin/onBeforeParse, Bun.Transpiler, Bun.Image, Bun.markdown,|
|                              Bun.CSRF, Bun.CookieMap, Bun.S3Client, Bun.SQL, Bun.redis,        |
|                              bun:sqlite, bun:ffi, bun:test, Bun.cron, Bun.WebView,             |
|                              Bun.ModuleGraph (experimental), HTMLRewriter, Bun.secrets         |
+---------------------------------------------------------------------------------------------+
```

Data flow for one request in dev: `bun --hot server.ts` -> `Bun.serve` route from `crates/yolo-stack-routes` -> `@yolo-rs/next/rsc` renders Server Components to a Flight stream -> `HTMLRewriter` injects the shell, preloads, and nonce -> client bundle built by `Bun.build` with the OXC plugin (native when `yolo_next.node` is present) -> TanStack Router hydrates, `<ViewTransition>` animates navigations -> Dioxus islands hydrate where `data-component` is present. Server Actions POST back to `/_yolo/action/<id>` guarded by `Bun.CSRF`; images go through `/_yolo/image` (`Bun.Image`); OG routes through `@yolo-rs/next/og` (satori + resvg); MCP through `/api/mcp` (mcp-handler).

### D.2 Prioritized integration tasks

| # | Task | Files | Done when |
| --- | --- | --- | --- |
| 1 | Bump OXC to 0.157.0 and raise MSRV | `C:\src\yolo-ui\Cargo.toml` (`oxc_allocator`, `oxc_ast`, `oxc_parser`, `oxc_span` -> `"0.157"`), `C:\src\yolo-stack\crates\yolo-next\Cargo.toml` (same + add `oxc_semantic`, `oxc_transformer`, `oxc_codegen`, `oxc_ast_visit`, `oxc_isolated_declarations`, `oxc_resolver = "11.24"`), `C:\src\yolo-stack\Cargo.toml` (`rust-version = "1.97"`, `[workspace.dependencies]` entries), `C:\src\yolo-stack\deny.toml` (allow BSD-3-Clause for `oxc_sourcemap`) | `cargo check --workspace` green on both checkouts; `cargo tree -i oxc_ast` shows a single 0.157.0 |
| 2 | Rust transform passes | new `C:\src\yolo-stack\crates\yolo-next\src\transform.rs` (code of B.4: `ImportRewriter`, `hoist_use_server`, `transform_tsx(path, source, TransformProfile) -> Transformed { code, map, server_actions }`), wire into `lib.rs`, `rsc.rs` (closure detection), `emit_bun.rs`; tests `C:\src\yolo-stack\crates\yolo-next\tests\transform.rs` using fixtures under `C:\src\yolo-stack\apps\next-fixture` | fixture page produces the B.4 output; closure-capturing inline actions yield a diagnostic |
| 3 | Native Bun plugin | new `C:\src\yolo-stack\crates\yolo-next-bun-plugin\{Cargo.toml,build.rs,src\lib.rs}` (`crate-type = ["cdylib"]`, `bun-native-plugin = "0.2.0"`, `napi = { version = "3.16", default-features = false, features = ["napi4"] }`, `napi-derive = "3.7"`, `napi-build = "2.6"`; allow `unsafe_code` for this FFI crate only), `justfile` recipe `build-bun-plugin` copying the artifact to `C:\src\yolo-stack\packages\yolo-next\native\yolo_next.node`; CI matrix step in `.github\workflows\ci.yml` | `bun run packages/yolo-next/tests/native-plugin.test.ts` rewrites imports through `onBeforeParse` on ubuntu-latest and windows-latest |
| 4 | TS plugin and resolver | `C:\src\yolo-stack\packages\yolo-next\src\plugin.ts` (B.5), `packages\yolo-next\package.json` dependencies `oxc-transform@0.157.0`, `oxc-resolver@11.24.2`, `oxc-parser@0.157.0`; `bunfig.toml` `[serve.static] plugins = ["@yolo-rs/next/plugin"]` in `apps/showcase-tanstack` | `bun test packages/yolo-next` passes a transform snapshot; `bun build` of the showcase uses the plugin |
| 5 | `ImageResponse` on satori + resvg | `C:\src\yolo-stack\packages\yolo-next\src\og.ts`, `src\shims\og.ts` re-export, deps `satori@0.63.2`, `@resvg/resvg-wasm@2.6.2`, optional `@resvg/resvg-js@2.6.2`; fonts from `C:\src\yolo-ui\crates\yolo-tokens` presets; cache in `bun:sqlite`; test `packages\yolo-next\tests\og.test.ts` (PNG magic, 1200x630) | PNG served from `/opengraph-image` under `bun build --compile` output (wasm lane) |
| 6 | MCP route | `C:\src\yolo-stack\packages\yolo-next\src\mcp.ts` (`createMcpRoutes`, `withMcpAuth`, `protectedResourceHandler`), `package.json` `exports["./mcp"]`, deps `mcp-handler@2.3.0`, `@modelcontextprotocol/server@2.4.0`, `zod@^4.2.0`; wire into `apps/showcase-tanstack` `server.ts` as `/api/mcp` | MCP inspector handshake over Streamable HTTP against `Bun.serve` |
| 7 | `next/image` on `Bun.Image` | `C:\src\yolo-stack\packages\yolo-next\src\image.ts` (route `/_yolo/image?src&w&q&f` with `resize(w, undefined, { fit: "inside", withoutEnlargement: true })`, `webp`/`avif` by `Accept`, `.placeholder()` for `placeholder="blur"`, `Image.backend = "bun"`), `src\shims\image.ts` (`<Image>` component emitting `srcset`), `crates\yolo-next\src\images.rs` keeps the manifest and remote-pattern rules | pixel-size and content-type tests in `packages\yolo-next\tests\image.test.ts`; `bench/` adds an image route |
| 8 | Fonts without Google | `C:\src\yolo-stack\packages\yolo-next\src\fonts.ts`, `src\shims\font-google.ts` (maps `Inter(...)`-style calls to a local manifest and emits a diagnostic naming the replacement preset), `src\shims\font-local.ts` (`@font-face` CSS from TTF/WOFF2, `geist@1.9.1` optional via `geist/dist/fonts/**`), `crates\yolo-next\src\fonts.rs` produces the manifest | no network font fetch at build or runtime; CSS emitted through Bun's CSS bundler |
| 9 | Vercel shims and compat exports | `C:\src\yolo-stack\packages\yolo-next\src\shims\{analytics,speed-insights,flags,swr}.ts`, collector route `/_yolo/collect` (`bun:sqlite`), `flags@4.4.0` core as dependency, `swr@2.5.1` re-export; remove `styled-jsx` usages in fixtures with a `yolo-next migrate` rule in `crates\yolo-next\src\rules.rs` | migrated fixtures build without `@vercel/*` packages; `flags` overrides work through `Bun.CookieMap` |
| 10 | Quality gates and manifest doctor | `C:\src\yolo-stack\fusion.toml` entries for `agent-skills` and `web-interface-guidelines` (`repos/`), `justfile` recipes `skills-install` (`bunx skills add vercel-labs/agent-skills -a claude-code`), `skills-check` (`bunx skills check`), `ecosystem-doctor` (new `scripts\ecosystem.ts` reading `ecosystem.toml` with `Bun.TOML.parse`, comparing registry/crates.io versions with `Bun.semver`), CI job running `bunx oxlint@1.91.0` and `bunx oxfmt@0.76.0 --check` next to `bun lint`/`bun fmt` | `just ecosystem-doctor` reports drift; review skills run on `packages/yolo-next` before each release |

### D.3 Open risks and unverified items

| Item | Status | Mitigation |
| --- | --- | --- |
| React Fast Refresh when a custom plugin pre-transforms JSX under `Bun.serve` HMR | unverified | prefer the native `onBeforeParse` plugin (Bun still parses JSX); test the JS `onLoad` path with `development: { hmr: true }` |
| Inline Server Actions with closures | unverified (closure-free case verified) | diagnostic now, encryption later (`rsc.rs`) |
| `Bun.ModuleGraph` for request isolation | experimental API | feature flag; `Worker` fallback |
| `Bun.markdown` | unstable API | feature flag; keep Rust-side renderer option |
| `@vercel/og` 1.0.3 | broken tarball (`hb.wasm` missing) | own `ImageResponse` on satori + resvg (verified) |
| `oxc_traverse` semver | internal crate | use `oxc_ast_visit::VisitMut` |
| `agent-skills` license file | missing on GitHub (README: MIT) | attribution note; re-check on each `skills check` |
| `bun-native-plugin` 0.2.0 age (2025-02) vs Bun 1.4.4 | verified compatible on win32 x64 (stable and canary) | pin 0.2.0; re-verify on Linux in CI (task 3) |
| oxc MSRV 1.97 vs workspace 1.95 | fact | task 1 raises MSRV |
| Timing numbers in A.4/B.5/C.2 | single samples, no receipt file | re-run section 0.3 commands into `docs/receipts/` |
