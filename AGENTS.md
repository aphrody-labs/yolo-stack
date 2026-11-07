# yolo-stack contributor rules

- This repository is the yolo web framework: adapters, showcase apps, benchmark harness and decision
  records. UI crates and the vendored Dioxus 0.8 tree live in `C:/src/yolo-ui`; the Bun runtime
  integration lives in `C:/src/yolo-bun`; browser engines in `yolo-web`; toolchains in `yolo-cross`.
  Reference those by path or dependency. Never copy their code here.
- Bun 1.4.4+ only: `bun install`, `bun run`, `bun test`, `bun check`, `bun lint`, `bun fmt`,
  `bun build`. No Node, npm, npx, yarn or pnpm anywhere, including in app scripts and CI.
- Rust 2024 edition, `unsafe_code = "forbid"` unless wrapping FFI. Keep the upstream release profile;
  measure any LTO or flag change separately with a receipt.
- UI is Dioxus 0.8 RSX or typed TSX. No raw `.html`, `.css` or `.js` source files; HTML imports are
  the Bun entry convention, not hand-written markup. Theme through `yolo-tokens` presets; no Inter,
  Roboto or generic icon packs.
- Stack roles are fixed by `docs/DECISION.md`: TanStack on Bun is the shell, Dioxus 0.8 is islands
  and native, Leptos is an optional feature-gated islands adapter. Changing a role requires a new
  dated decision record with a receipt.
- Every number in `docs/` is tagged measured, quoted or missing. Measured means a file in
  `docs/receipts/` with date, host, versions and raw samples. Quoted means URL and retrieval date.
  Do not infer parity from the baseline harness; it shares a host with the load generator.
- Do not claim a build, benchmark or app as working unless a receipt or test exists. `planned` in
  `bench/targets.json` and in READMEs means exactly that.
- Preserve untracked files and directories created by other sessions (`benches/`, `repos/`,
  `scripts/`, `tests/` existed empty at init). Do not reset or clean the tree.
- No emoji. Concise English. Commits are authored by `aphrody-dev`.

---

## Autonomous NHITL Agent & Context / Token Optimization Best Practices (24/7 Operations)

### 1. Context Window & Token Optimization
- **High-Precision Targeting**: Never dump large raw files into the model context. Use exact byte/line range reads or AST/symbol lookups.
- **Zero-Polling Reactive Wakeup**: When launching background tasks or subagents, do not poll in loops. Yield execution immediately and wait for reactive system events.
- **Compact & High Information Density**: Output concise, fact-based reports with clickable markdown file links (`[file.ext](file:///path/to/file)`). Do not narrate obvious actions.
- **Strict Zero-Emoji Policy**: Zero emojis in all code, comments, commit messages, PRs, logs, and agent communications.

### 2. Autonomous Execution (NHITL - No Human In The Loop)
- **Continuous Grind & Auto-Goal**: Independently identify objectives, resolve compile errors, apply fixes, format (`bun fmt`), lint (`bun lint`), typecheck (`bun check`), test (`bun test` / `cargo test`), commit, and push.
- **Pure Tech Stack**: 100% Upstream Bun v1.4.4+, Rust 2024, Python uv v0.13.7+, Go 1.27+. Zero Node.js, npm, yarn, or pnpm.
- **Strict Isolation & Safe Execution**: Heavy Rust builds run on vps (12 cores) with sccache under shared lock; web and fast tests run on bunos or local Bun.
