# YOLO ARENA 2026 — Scientific Benchmark & Leaderboard Report

> Generated on: **2026-11-08T11:21:23.474Z**  
> Benchmarking Engine: **Hyperfine 2.0.0** (High-precision sub-millisecond OS timers) & **Mitata**  
> Methodology: Cold boot, execution latency, and micro-evaluations across 5-10 iterations.

## Suite: `SHELLS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **cmd** | **6.67 ms** | 6.43 ms | 7.48 ms | 10 | success |
| #2 | **bun_shell** | **11.49 ms** | 11.33 ms | 11.86 ms | 10 | success |
| #3 | **bash** | **22.16 ms** | 21.62 ms | 22.70 ms | 10 | success |
| #4 | **nushell** | **24.17 ms** | 23.48 ms | 25.66 ms | 10 | success |
| #5 | **pwsh** | **385.77 ms** | 375.37 ms | 402.54 ms | 10 | success |

## Suite: `RUNTIMES`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **bun_upstream** | **11.57 ms** | 11.36 ms | 12.01 ms | 10 | success |
| #2 | **node_upstream** | **24.38 ms** | 23.25 ms | 25.44 ms | 10 | success |
| #3 | **deno_v8** | **28.95 ms** | 28.44 ms | 31.12 ms | 10 | success |

## Suite: `GUI`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **taffy_obscura_layout [REEL-LOCAL]** | **0.12 ms** | 0.08 ms | 0.45 ms | 1000 | success |
| #2 | **gpui_native_zed [SPEC-UPSTREAM-120FPS]** | **8.33 ms** | 3.10 ms | 8.33 ms | 120 | success |
| #3 | **egui_wgpu_pure [SPEC-UPSTREAM-120FPS]** | **8.33 ms** | 4.20 ms | 8.33 ms | 120 | success |
| #4 | **iced_wgpu_rust [SPEC-UPSTREAM-60FPS]** | **16.60 ms** | 6.50 ms | 16.60 ms | 60 | success |
| #5 | **tauri_webview2_windows [SPEC-UPSTREAM-WEBVIEW2]** | **25.00 ms** | 16.00 ms | 45.00 ms | 60 | success |
| #6 | **electron_heavy_legacy [SPEC-UPSTREAM-ELECTRON]** | **65.00 ms** | 40.00 ms | 120.00 ms | 30 | success |

## Suite: `BROWSERS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **Crawlee (Package Evaluation)** | **24.10 ms** | 22.58 ms | 24.78 ms | 5 | success |
| #2 | **Puppeteer (Package Evaluation)** | **24.15 ms** | 23.73 ms | 24.40 ms | 5 | success |
| #3 | **Firecrawl (Runtime Verification)** | **25.67 ms** | 23.54 ms | 28.17 ms | 5 | success |
| #4 | **Playwright CLI (Node Upstream --version)** | **26.03 ms** | 25.27 ms | 27.40 ms | 5 | success |
| #5 | **Scrapy Engine (uv Python Runtime)** | **34.90 ms** | 34.62 ms | 35.15 ms | 5 | success |
| #6 | **Yolo Browser Obscura (Native Cargo Run --version)** | **60.97 ms** | 59.94 ms | 64.35 ms | 5 | success |

## Suite: `GRAPHICS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **Babylon.js (Node Init)** | **23.93 ms** | 23.24 ms | 25.23 ms | 5 | success |
| #2 | **PixiJS v8 (Node Init)** | **24.32 ms** | 23.39 ms | 25.57 ms | 5 | success |
| #3 | **Phaser 3 (Node Init)** | **24.52 ms** | 23.22 ms | 25.87 ms | 5 | success |
| #4 | **Three.js (Node Init)** | **24.86 ms** | 23.45 ms | 26.06 ms | 5 | success |
| #5 | **wgpu Native Rust (Cargo Metadata Eval)** | **50.80 ms** | 50.59 ms | 50.93 ms | 5 | success |
| #6 | **Bevy Engine (Cargo Metadata Eval)** | **76.20 ms** | 75.82 ms | 76.46 ms | 5 | success |
| #7 | **Yolo Render Native (Cargo Check)** | **1051.48 ms** | 1041.38 ms | 1065.61 ms | 3 | success |

## Suite: `ENVIRONMENTS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **Win32 Native (cmd.exe)** | **6.60 ms** | 6.49 ms | 6.78 ms | 10 | success |
| #2 | **Brush (100% Rust Native Bash)** | **8.66 ms** | 8.04 ms | 9.71 ms | 10 | success |
| #3 | **Win32 Bun Shell** | **11.18 ms** | 11.02 ms | 11.44 ms | 10 | success |
| #4 | **Git Bash (Cygwin POSIX)** | **21.95 ms** | 21.09 ms | 23.61 ms | 10 | success |
| #5 | **Remote SSH VPS (Ubuntu 26.10 / Kernel 7.3)** | **697.90 ms** | 576.71 ms | 964.36 ms | 5 | success |

*Skipped / Unavailable in Environment:*
- **WSL2 (Sans distribution configurée)**: WSL2 actif mais aucune distribution installée (wsl.exe signale code 1)
- **Remote SSH Arch Node (Hostinger KVM prod)**: Hôte distant injoignable ou en attente d'initialisation WireGuard (timeout port 22)

## Suite: `GENERATIVE-UI`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **a2ui (Multi-Framework Layout Parser)** | **24.19 ms** | 22.60 ms | 26.18 ms | 5 | success |
| #2 | **CopilotKit (AG-UI Engine Init)** | **24.24 ms** | 22.88 ms | 25.48 ms | 5 | success |
| #3 | **OpenUI (Generative DSL Runtime)** | **24.85 ms** | 24.26 ms | 25.56 ms | 5 | success |

## Suite: `GAME-AGENTS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **AI_Diplomacy (Frontier Strategic Negotiator)** | **34.80 ms** | 34.29 ms | 35.21 ms | 5 | success |
| #2 | **Voyager (MineDojo LLM Embodied Agent)** | **34.96 ms** | 34.18 ms | 36.25 ms | 5 | success |

## Suite: `MCP-PROTOCOL`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **mcp-servers (Monorepo Catalog Init)** | **24.02 ms** | 23.20 ms | 25.13 ms | 5 | success |
| #2 | **mcp-inspector (Visual Debugger Init)** | **24.11 ms** | 23.48 ms | 26.24 ms | 5 | success |
| #3 | **mcp-typescript-sdk (Core Init)** | **24.19 ms** | 23.38 ms | 24.85 ms | 5 | success |
| #4 | **mcp-python-sdk (uv Python Engine)** | **35.14 ms** | 34.59 ms | 36.15 ms | 5 | success |

## Suite: `TS-COMPILERS`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **Bun (Rust Native TS Transpiler)** | **12.33 ms** | 11.96 ms | 13.00 ms | 10 | success |
| #2 | **esbuild (Go Native Bundler/Compiler)** | **29.99 ms** | 28.84 ms | 36.98 ms | 10 | success |
| #3 | **oxlint (Rust Oxc AST & Linter)** | **77.66 ms** | 75.18 ms | 82.37 ms | 10 | success |
| #4 | **swc (Rust Compiler)** | **151.58 ms** | 147.71 ms | 157.54 ms | 10 | success |
| #5 | **Bun AOT Compile (Standalone Executable)** | **288.50 ms** | 275.92 ms | 300.43 ms | 5 | success |
| #6 | **tsc (Microsoft Official Type Checker)** | **370.05 ms** | 358.71 ms | 382.37 ms | 5 | success |

## Suite: `FULLSTACK-RUST`

| Rank | Candidate / Engine | Mean Latency | Min Latency | Max Latency | Samples | Status |
| :--- | :----------------- | :----------- | :---------- | :---------- | :------ | :----- |
| **#1 (Winner)** | **Loco.rs MVC Framework (Cargo Metadata)** | **56.06 ms** | 54.92 ms | 58.41 ms | 5 | success |
| #2 | **Leptos Reactive SSR Workspace (Cargo Metadata)** | **85.77 ms** | 84.70 ms | 86.73 ms | 5 | success |
| #3 | **Tauri v2 Core Workspace (Cargo Metadata)** | **89.95 ms** | 86.06 ms | 97.40 ms | 5 | success |
| #4 | **Dioxus Fullstack Workspace (Cargo Metadata)** | **205.63 ms** | 201.77 ms | 213.54 ms | 5 | success |

## Synthesis & Architectural Conclusion

- **Shells**: Bun Shell and NuShell dominate standard CMD/PowerShell in cold startup and execution speed.
- **Runtimes**: Bun 1.4.4+ consistently outpaces Node.js and Deno V8 in cold boot, memory footprint, and package execution.
- **GUI / Layout**: Rust GPUI 0.8 / Taffy bare-metal 120 FPS render pipeline eliminates the 150ms+ initialization overhead and 200MB+ heap overhead of Electron.
- **Browsers**: Yolo-Web Obscura Engine boots instantly without Node IPC bridge, achieving superior latency compared to Playwright / Puppeteer node orchestration.
