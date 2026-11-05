# Migration report: @aphrody/shenron-site

Generated 2026-11-05T04:55:01Z by yolo-next (Next.js 16.4.5 baseline, Bun 1.4.4).

## Summary

| Metric | Value |
|---|---|
| Parity score | 91% |
| Targets | leptos |
| cache functions | 0 |
| client components | 355 |
| diagnostics errors | 0 |
| fonts | 0 |
| images | 54 |
| metadata routes | 4 |
| pages | 200 |
| route handlers | 158 |
| routes | 362 |
| server actions | 33 |
| Diagnostics | 23 (0 errors, 2 warnings) |
| Emitted files | 16 |

## Features

| Feature | Support | Note | Target |
|---|---|---|---|
| output | mapped | Bun.serve server (default) | all |
| headers (dynamic) | partial | awaited at startup; no static manifest | all |
| app/layout.tsx | mapped | nested layouts -> router outlet chain | all |
| app/page.tsx | mapped | route component | all |
| app/loading.tsx | mapped | Suspense fallback / pending component | all |
| app/error.tsx | mapped | error boundary | all |
| app/not-found.tsx | mapped | 404 boundary | all |
| app/global-error.tsx | mapped | root error boundary | all |
| app/default.tsx | mapped | parallel slot fallback | all |
| route handlers | mapped | GET/POST/... exports -> Bun.serve routes | all |
| parallel routes (@slot) | mapped | slots rendered as layout props | all |
| intercepting routes | mapped | soft navigation renders the interceptor, hard navigation the target | bun |
| intercepting routes | partial | rendered as the target page (no interception) | dioxus |
| intercepting routes | partial | rendered as the target page (no interception) | leptos |
| generateStaticParams | mapped | prerender list at build time | all |
| generateMetadata | mapped | async head tags | all |
| metadata files | mapped | icon/opengraph/sitemap/robots/manifest routes | all |
| proxy.ts | partial | wrapped for Bun.serve; Node runtime semantics | all |
| next/image | mapped | /_yolo/image optimizer with srcset algorithm | all |
| "use client" | mapped | client reference manifest + hydration boundary | all |
| "use server" | mapped | Server Actions RPC (Next-Action id) | all |
| inline server actions | mapped | hoisted and registered by id | all |

## Diagnostics

### Errors

- none

### Warnings

- `next-config-spread-unsupported` spread `...resolved` cannot be evaluated statically; its entries are missing (next.config.ts)
- `next-config-dynamic-headers` `headers()` does not return a literal; no static manifest (next.config.ts) -- the function is awaited at startup by @yolo-rs/next; static manifest unavailable

### Info

- `next-config-dynamic-value` `resolveConfig(...args)` is not statically evaluable (function call); value omitted (next.config.ts)
- `next-config-dynamic-value` `((await headers?.()) ?? []).filter((route) => !route.source....` is not statically evaluable (function call); value omitted (next.config.ts)
- `next-config-phase-function` config is a phase function; the returned object literal was evaluated for every phase (next.config.ts)
- `next-image-usage-summary` 54 <Image> usages: 0 remote across 0 hosts, 17 static imports, 0 unmatched remote
- `next-image-priority-deprecated` `src/app/dashboard/page.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/dashboard/page.tsx)
- `next-image-priority-deprecated` `src/app/news/[slug]/page.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/news/[slug]/page.tsx)
- `next-image-priority-deprecated` `src/app/news/_journal.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/news/_journal.tsx)
- `next-image-priority-deprecated` `src/app/studio/video/StudioVideoWorkspace.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/studio/video/StudioVideoWorkspace.tsx)
- `next-image-priority-deprecated` `src/app/wiki/games/[slug]/page.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/wiki/games/[slug]/page.tsx)
- `next-image-priority-deprecated` `src/app/wiki/manga/volume/[id]/page.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/app/wiki/manga/volume/[id]/page.tsx)
- `next-image-priority-deprecated` `src/components/PageHero.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/PageHero.tsx)
- `next-image-priority-deprecated` `src/components/feed/FeedStream.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/feed/FeedStream.tsx)
- `next-image-priority-deprecated` `src/components/home/DbsBeerusSpotlight.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/home/DbsBeerusSpotlight.tsx)
- `next-image-priority-deprecated` `src/components/home/SceneBackdrop.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/home/SceneBackdrop.tsx)
- `next-image-priority-deprecated` `src/components/kit/MangaCard.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/kit/MangaCard.tsx)
- `next-image-priority-deprecated` `src/components/manga/VignetteTome.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/manga/VignetteTome.tsx)
- `next-image-priority-deprecated` `src/components/media/BackgroundImage.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/media/BackgroundImage.tsx)
- `next-image-priority-deprecated` `src/components/media/PageBanner.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/media/PageBanner.tsx)
- `next-image-priority-deprecated` `src/components/stream/EpisodeCard.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/stream/EpisodeCard.tsx)
- `next-image-priority-deprecated` `src/components/stream/QuickLook.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/stream/QuickLook.tsx)
- `next-image-priority-deprecated` `src/components/wiki/GamesExplorer.tsx` sets `priority`/`preload` on <Image>; Next.js 16 names the prop `preload` and emits a `<link rel=preload>` (src/components/wiki/GamesExplorer.tsx)

## Emitted files

- `Cargo.toml`
- `src/lib.rs`
- `src/main.rs`
- `src/app.rs`
- `src/content.rs`
- `src/i18n.rs`
- `src/seo.rs`
- `src/tokens.rs`
- `src/server/mod.rs`
- `src/server/redirects.rs`
- `src/server_fns.rs`
- `src/proxy.rs`
- `messages/fr.json`
- `messages/en.json`
- `messages/ja.json`
- `README.md`
