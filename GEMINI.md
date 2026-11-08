# GEMINI.md — YOLO-Stack Multi-Target Meta-Framework & Arena (`C:\src\yolo-stack`)

> **Doctrine**: No Human In The Loop (NHITL) | Fork `cloudflare/vinext` + Next.js Upstream on Bun v1.4.4+

## 1. Architecture (`C:\src\yolo-stack`)
- **Multi-Target Meta-Framework**: Compiles a single codebase into 6 deployment targets: Discord Bot, SSR/RSC Website, 120 FPS Native Desktop App (`gpui-kit` v0.8 / Dioxus 0.8), API/Streaming Server (`Bun.serve`), Nginx/Pingora/Traefik v3 Edge Config, and Docker/Dokploy Blueprint.
- **Universal Provider Compatibility**: Runs VPS-first (`yolo-vps`), Cloudflare `workerd` (`yolo-cloudflare`), Google Cloud Run (`google`), and Vercel.
- **Absorbed Benchmark Leaderboard (`yolo-stack/arena`)**: Universal 2026 LLM, Agent, Browser, and Runtime benchmark leaderboard.

## 2. Operating Directives
- Zero emojis in all code, commits, and logs.
- Validate changes with `bun fmt`, `bun check`, and `bun test`.
