# Décision moteur UI/fullstack Yolo : Dioxus vs Leptos vs TanStack

Date d'arbitrage : 2026-11-04. Périmètre : moteur principal du framework fullstack Rust + Bun v1.4.4 de `yolo-stack` remplaçant Next.js (App Router, Server Components, Server Actions, streaming SSR, `next.config`, `next/font`, `next/image` + CDN, Turbopack). Doctrine : Bun >= 1.4.4 et Rust purs, zéro Node/npm/Vite/webpack. Tout chiffre est daté et sourcé ; `UNVERIFIED` marque ce qui n'a pas pu être confirmé.

## 1. Verdict

- Moteur principal : **Dioxus** (fork aplati 0.8.0-alpha.3 déjà présent dans `C:\src\yolo-ui`, workspace `version = "0.8.0-alpha.3"`), runtime Rust RSX web + desktop + mobile + natif (Blitz/Taffy), serveur Axum (server functions, SSR streaming, SSG/ISR, manganis).
- Lane secondaire : **TanStack sous Bun** (Router, Query, Table, Form, Virtual, DB) + `yolo-react`/`@yolo-rs/fullstack` pour RSC Flight, Server Actions, `next/font`, `next/image`. TanStack Start est exclu (Vite/Rsbuild/Nitro obligatoires).
- Leptos : **rejeté comme runtime** (deuxième runtime UI Rust et deuxième graphe réactif = dette), **conservé comme référence de conception** (modes SSR, islands, `StaticRoute::regenerate`) et comme challenger de réévaluation.
- Fusion : saine uniquement aux frontières de processus/protocole (HTTP, flux Flight, JSON server-fn). Malsaine au niveau du runtime réactif (reactive_graph/tachys dans Dioxus).
- Score pondéré : Dioxus 75,25 ; Leptos 72,50 ; TanStack 70,00 (section 7). Marge faible (2,75 points) : le verdict bascule vers Leptos seulement si l'on dégrade simultanément Dioxus en performance (6,0) et en intégration (8,5) et que l'on relève Leptos sur les deux. Déclencheur de réouverture : Dioxus 0.8 stable non publié au 2027-03-31 ou budget de bundle (section 10, action 3) non tenu.

## 2. Performance runtime (krausest js-framework-benchmark)

Source des données : table officielle Chrome 156, la plus récente listée sur https://krausest.github.io/js-framework-benchmark/index.html (observée 2026-11-04 ; release `chrome156` du 2026-10-25, https://github.com/krausest/js-framework-benchmark/releases/tag/chrome156). Les pages 2026 sont rendues en JavaScript ; les valeurs ci-dessous ont été recalculées à partir de https://raw.githubusercontent.com/krausest/js-framework-benchmark/master/webdriver-ts-results/src/results.ts (commit 2026-11-02 « incremental run ») avec la formule officielle de `Common.ts` (médianes, facteur = médiane / médiane la plus rapide parmi les implémentations keyed, moyenne géométrique pondérée CPU avec les 9 poids codés en dur, mode CPU `total`). Les séries leptos, dioxus et react-hooks de ce fichier sont identiques au bundle officiel Chrome 156 (`2026/results-Ch_eDRpy.js`) ; seul le dénominateur (207 vs 201 implémentations) peut décaler la deuxième décimale. La catégorie « startup » historique n'existe plus : elle est remplacée par `41_size-uncompressed`, `42_size-compressed`, `43_first-paint`.

| Implémentation (keyed) | Version testée | Durée (géom. pondérée) | Mémoire (géom.) | Taille + first paint (géom.) | Taille brute / gzip (KB) | First paint (ms) | Mémoire run 1k (MB) |
|---|---|---|---|---|---|---|---|
| vanillajs | - | 1,02 | 1,06 | 2,07 | 11,3 / 2,5 | 42,1 | 1,86 |
| solid | 1.9.3 | 1,14 | 1,29 | 2,74 | 11,5 / 4,5 | 53,5 | 2,68 |
| dioxus | 0.7.2 | 1,30 | 1,83 | 57,75 | 419,6 / 114,9 | 536,9 | 2,12 |
| sycamore | 0.9.0-beta.2 | 1,31 | 1,24 | 22,84 | 157,5 / 47,2 | 215,3 | 2,09 |
| leptos | 0.7.0 (csr, nightly, delegation) | 1,32 | 1,46 | 25,34 | 189,6 / 48,8 | 236,4 | 2,69 |
| react-hooks | 19.2.0 | 1,56 | 2,64 | 25,51 | 190,3 / 51,4 | 228,1 | 4,42 |

Lecture : Dioxus et Leptos sont à égalité CPU (1,30 vs 1,32), tous deux devant React 19 (1,56). Dioxus paie son bundle : 2,2x la taille de Leptos et 2,3x son first paint ; Leptos est le meilleur compromis mémoire/taille des trois candidats. React (lane TanStack) consomme 1,8x la mémoire des runtimes Rust. Les implémentations testées datent (leptos 0.7.0, dioxus 0.7.2) : les versions 0.8/0.9 ne sont pas mesurées (gains annoncés UNVERIFIED).

## 3. Tailles de bundle et vitesse d'itération

| Donnée | Valeur | Source | Date |
|---|---|---|---|
| Dioxus hello world web | « about 50kb, comparable to React » (affirmation projet) | https://github.com/DioxusLabs/dioxus (README) | 2026-11-04 |
| Dioxus benchmark krausest | 419,6 KB brut / 114,9 KB compressé | results.ts ci-dessus | 2026-11-02 |
| Leptos benchmark krausest | 189,6 KB brut / 48,8 KB compressé | results.ts ci-dessus | 2026-11-02 |
| Leptos islands | « 24kb of WASM (uncompressed), compared to 274kb in non-islands mode » ; « 166kb uncompressed » avec une island ; démo tabs « about 200kb » vs « over 400kb » | https://book.leptos.dev/islands.html | 2026-11-04 |
| Comparaison tierce (non reproductible) | Dioxus 210 KB, Yew 180 KB, Leptos 86 KB gzip (même dashboard, versions 0.7, sans code source) | https://www.pistack.xyz/posts/2026-08-29-dioxus-vs-yew-vs-leptos-rust-wasm-frameworks-comparison/ | 2026-08-29 |
| Dioxus hot-patch (subsecond) | 6 plateformes (Android, iOS simulateur, Linux, macOS, Windows x86_64/arm64, wasm32) ; « ThinLink can shrink an incremental build to less than 500ms » ; « does not support hot-reloading of structs » ; `dx serve --hotpatch` requis en 0.7, par défaut en 0.8-alpha.0 (PR #5506) | https://docs.rs/subsecond/latest/subsecond/ ; https://github.com/DioxusLabs/dioxus/releases/tag/v0.8.0-alpha.0 | 2026-11-04 |
| Leptos hot reload | patch des macros `view!` (crate `leptos_hot_reload`), CSS hot-reload, sinon recompilation + redémarrage serveur ; « hot reloading does not currently work in --release mode » ; `--cfg=erase_components` « significant improvements to compile times » sans chiffre | https://github.com/leptos-rs/cargo-leptos ; https://docs.rs/crate/cargo-leptos/0.2.47/source/src/command/watch.rs ; https://github.com/leptos-rs/leptos/releases/tag/v0.8.0 | 2026-11-04 |
| Bun dev server | HMR natif (`development: { hmr: true }`), imports HTML, `bun build --target=bun --production` ; « Server-side rendering (SSR) is not built-in » ; RSC absent de la doc | https://bun.sh/docs/bundler/fullstack | 2026-11-04 |
| Temps de compilation incrémentale chiffrés Leptos vs Dioxus | ABSENCE de mesure publiée comparable | - | - |

## 4. SSR et fullstack

| Capacité | Dioxus 0.7/0.8 | Leptos 0.8/0.9-rc | TanStack (Router seul sous Bun / Start) |
|---|---|---|---|
| Streaming HTML | opt-in `enable_out_of_order_streaming` ; sans streaming le serveur attend toutes les suspenses ; contenu streamé « not visible to users unless they have JavaScript enabled » ; head uniquement dans le premier chunk | défaut out-of-order ; in-order, async, synchrone, partially-blocked (`SsrMode::PartiallyBlocked`) | `renderRouterToStream` / `defaultStreamHandler` (React 19) ; APIs « should be regarded as experimental until Start reaches stable status » ; Start : streaming intégré |
| Suspense / hydration | `SuspenseBoundary`, `use_server_future` ; ordre d'hydratation non documenté (UNVERIFIED) | `Suspense`, `Transition`, hydratation ordonnée en in-order ; 0.8.21 à 0.9 : correctifs hydratation `For`/islands | Suspense React 19, hydratation sélective React |
| RSC ou équivalent | aucun (tout est hydraté) ; issue « Partial hydration » #1758 fermée 2023-12-29, résolution UNVERIFIED | islands `#[island]` + islands router (navigation côté client en mode islands) | Start : « Server Components are experimental » (React 19, Vite 8 ou Rsbuild 2 obligatoires) ; Router seul : non |
| Server functions / actions | `#[get]`/`#[post]` routes Axum typées, `Form<T>`, multipart, `FileStream` | `#[server]` + `ActionForm`, encodages Json/Cbor/MsgPack/Postcard/Rkyv/Multipart/Streaming (server_fn 0.8.13) ; pas de WebSocket dans `server_fn::codec` | `createServerFn` réservé à Start ; Router seul : aucun |
| WebSockets / SSE | `use_websocket`, `ServerEvents<T>`, `Streaming<T>` | manuel via Axum | manuel via `Bun.serve` websocket |
| SSG / ISR | `dx bundle --web --ssg` + server fn `static_routes` ; fork local : `IncrementalRendererConfig::invalidate_after(Duration)`, `static_dir`, `clear_cache`, `map_path` (`packages/fullstack-server/src/isrg/config.rs`) ; revalidation non documentée côté site (UNVERIFIED) | `StaticRoute::prerender_params` + `regenerate(Stream<Item=()>)` (leptos_router 0.8.17) | Start : `prerender { crawlLinks, pages, filter }` (plugin Vite/Rsbuild) ; ISR absent de la doc |
| Débit SSR req/s publié | ABSENCE de preuve | anecdote : Leptos 0.6 + Axum, vegeta, « Average latency remains around 3-5ms all the way up to 2000rps » (https://benw.is/posts/full-stack-rust-with-leptos, 2024) ; non comparable | ABSENCE de preuve pour Start sous Bun |
| Micro-benchmark serveur non-SSR | Dioxus 87 638 RPS JSON vs Leptos 77 135 (Axum 100 620), 125 connexions / 10 s, auteur promouvant « Rullst », endpoints JSON | https://dev.to/venelouis/a-benchmark-of-23-web-frameworks-the-top-5-were-rust-based-17fb | « Jun 23 », année non affichée |

## 5. Santé des projets (GitHub API, observé 2026-11-04)

| Projet | Étoiles | Contributeurs | Issues ouvertes | Licence | Dernière stable | Dernière pré-release | Financement / équipe |
|---|---|---|---|---|---|---|---|
| DioxusLabs/dioxus | 39 598 | 463 | 896 | Apache-2.0 | v0.7.10 (2026-07-30) ; v0.7.0 (2025-10-31) ; v0.6.0 (2024-12-09) | v0.8.0-alpha.3 (2026-10-16) ; jalon 0.8.0 : 41 ouvertes / 13 fermées, échéance 2025-06-30 dépassée | « Full-time core team » financée par FutureWei, Satellite.im, GitHub Accelerator (README) ; pré-amorçage ~500 k$ YC/Eight Capital selon agrégateurs : UNVERIFIED |
| leptos-rs/leptos | 21 461 | 398 | 108 | MIT | v0.8.23 (2026-10-31) ; v0.8.0 (2025-05-01) ; v0.7.0 (2024-11-30) | v0.9.0-rc1 (2026-10-31) | communautaire (gbj) ; Open Collective total 169,16 $ US, 2 contributeurs financiers ; GitHub Sponsors individuels UNVERIFIED |
| TanStack/router (+Start) | 15 281 | 758 | 812 | MIT | `@tanstack/react-start` 1.169.4 (2026-10-22) ; Start v1 annoncé 2025-09-23 ; `@tanstack/react-router` 1.170.46 | `@tanstack/react-start-rsc` 0.1.65 ; `solid-start` 2.0.0-rc.10 | partenaires Cloudflare, Netlify, Neon, Clerk, Convex, Sentry, Prisma, Strapi, CodeRabbit, AG Grid ; sponsors or Vercel, Render, Railway, Lovable |
| TanStack/query | 50 525 | 1 151 | 112 | MIT | 5.106.0 (2026-11-04) | - | idem |
| TanStack/table, form, virtual, db | 28 546 / 6 762 / 7 188 / 3 977 | 503 / 225 / 171 / 95 | 80 / 230 / 84 / 148 | MIT | 9.3.1 / 1.33.5 / 3.14.26 / 1.0.0-rc.1.3 | - | idem |
| DioxusLabs/blitz, taffy, components, sdk | 4 341 / 3 732 / 355 / 308 | 55 / 90 / 28 / 17 | 370 / 189 / 51 / 34 | Apache-2.0 / MIT | blitz-dom 0.2.4 (0.3.0-beta.4 en pré-release) ; taffy 0.15.1 | - | Blitz « still very young », « work in progress » (blog 0.7) |
| leptos-rs/cargo-leptos, thaw, leptonic, leptos-use, leptos_i18n | 548 / 614 / 302 / 496 / 168 | 117 / 25 / - / 60 / - | 54 / 33 / 36 / 16 / - | MIT / Apache-2.0 | cargo-leptos 0.3.11 (2026-10-02) | - | - |

Téléchargements crates.io (2026-11-04) : leptos 5,77 M (1,63 M récents), dioxus 3,53 M (1,33 M), dioxus-fullstack 2,46 M, server_fn 6,56 M, reactive_graph 3,62 M (1,51 M), taffy 18,75 M.

## 6. Matrice de parité Next.js

| Fonction Next.js | Dioxus | Leptos | TanStack sous Bun (sans Start) |
|---|---|---|---|
| Routage par fichiers | non : enum `#[derive(Routable)]` typée, vérifiée à la compilation | non : `<Routes/>`, `<Route/>`, `<ParentRoute/>` | oui : `@tanstack/router-cli` 1.167.46 (`tsr generate`), plugins Vite/Rspack/Webpack/Esbuild non requis |
| Layouts imbriqués | oui : `#[layout(Wrapper)]`, `#[nest("/:name")]`, `Outlet::<Route> {}` | oui : `<ParentRoute>` + `<Outlet/>` | oui : `__root.tsx`, layouts sans chemin `_layout` |
| loading / error boundaries | `SuspenseBoundary`, `ErrorBoundary`, pages d'erreur personnalisées | `Suspense`, `ErrorBoundary` (0.9 : Suspense dans ErrorBoundary corrigé) | `pendingComponent` (1 000 ms, `pendingMinMs` 500 ms), `errorComponent`, `notFoundComponent` |
| Streaming SSR | opt-in, JS requis | défaut, 5 modes | oui (Router SSR) |
| RSC | non | islands | expérimental (Start, Vite) ; lane locale : `@yolo-rs/fullstack` sur `react-server-dom-parcel` + `Bun.build` |
| Server Actions | server functions Axum | `#[server]` + `ActionForm` | locale : modules `"use server"` (RSC Flight) ; `createServerFn` exclu |
| ISR / SSG | `dx bundle --web --ssg`, `invalidate_after` | `StaticRoute::regenerate` | `prerender.ts` local ; ISR à implémenter |
| metadata / head | `document::Title/Meta/Link` (premier chunk) | `leptos_meta` | `head()` + `<HeadContent/>`, déduplication dernier gagnant |
| fonts / images / CDN | manganis `asset!()` : `with_format(ImageFormat::Avif)`, `with_size`, `with_preload`, noms hachés « infinitely cache » | aucun pipeline image ; hachage de fichiers cargo-leptos | aucun natif ; local : `font/google`, `font/local`, `image/optimizer` (sharp en peer dep), `assetPrefix` CDN |
| i18n | dioxus-i18n (69 étoiles) | leptos_i18n (168 étoiles) | aucun officiel |
| Hot reload | hot-patch Rust + RSX | macro `view!` + CSS | HMR Bun |
| Devtools | `dx serve` TUI, devtools websocket | aucun officiel | `@tanstack/react-router-devtools` 1.167.2, `react-query-devtools` 5.106.0 |
| Desktop / mobile | oui (webview, natif Blitz WIP, Android/iOS) ; « Portable <3mb binaries » (README) | non (Tauri tiers) | web seulement |
| Bibliothèques de composants | dioxus-primitives « adheres to the WAI-ARIA Authoring Practices », `dx components add` | thaw, leptonic | shadcn/Radix/Base UI (React 19) |
| Tailwind | oui (dx) | oui (cargo-leptos `tailwind-input-file`, UNVERIFIED pour 0.3.11) | `bun-plugin-tailwind` |
| Intégration Bun | Axum derrière `Bun.serve` : `dioxusProxy`, `callServerFn` (xxh64 compatible `concatcp!`), `dioxusWebRoutes` déjà présents dans `packages/yolo-fullstack/src/dioxus/` | Axum ; aucun pont local | libs OK ; Start nécessite Vite + Nitro (`nitro({ preset: 'bun' })`, React 19) |

## 7. Matrice de score pondérée

Barème 0-10 par critère, total = somme(score x poids) / 10.

| Critère (poids) | Dioxus | Leptos | TanStack | Justification courte |
|---|---|---|---|---|
| Performance runtime (20) | 6,5 | 8,0 | 5,5 | CPU 1,30 / 1,32 / 1,56 ; mémoire 1,83 / 1,46 / 2,64 ; taille 419,6 / 189,6 / 190,3 KB ; first paint 537 / 236 / 228 ms |
| SSR et parité Next (25) | 7,5 | 8,5 | 6,0 | Leptos : 5 modes de streaming, islands, `StaticRoute` ; Dioxus : streaming opt-in, pas d'islands, mais assets/WS/SSG CLI ; TanStack : Start exclu, RSC expérimental |
| DX et vitesse d'itération (15) | 8,5 | 6,0 | 9,0 | hot-patch Rust < 500 ms incrémental vs recompilation Leptos vs HMR Bun |
| Écosystème et santé (15) | 7,5 | 6,5 | 9,5 | étoiles, contributeurs, cadence, financement, bibliothèques |
| Intégration Bun + Rust pur (15) | 9,0 | 7,0 | 6,5 | fork aplati + pont Bun existants ; Leptos = second runtime ; TanStack = libs OK, framework Vite |
| Risque (10, 10 = faible) | 6,0 | 6,0 | 6,5 | 0.8 alpha, jalon en retard, 896 issues, Blitz WIP / cassures annuelles 0.7-0.8-0.9, financement nul / CVE-2026-102989 critique, churn hebdomadaire, Vite |
| **Total /100** | **75,25** | **72,50** | **70,00** | |

## 8. Risques retenus

- Dioxus : 0.8.0-alpha.2/3 cassent `dioxus_elements` (plus d'espace de noms), `keyboard-types` 0.8, `WriteMutations::set_attribute`, features desktop, Windows 7 abandonné ; subsecond « experimental », pas de migration de structs ; jalon 0.8.0 en retard de 16 mois ; bundle 2,2x Leptos ; accessibilité native via feature `accessibility` (AccessKit) non qualifiée.
- Leptos : cassures 0.7 (tachys, 2024-11-30) -> 0.8 (Axum 0.8, 2025-05-01) -> 0.9-rc1 (traits de signaux, feature `lazy`, limites de corps 2 Mo) ; bus factor et financement quasi nul ; islands « cutting edge » sans navigation client complète.
- TanStack : CVE-2026-102989 (XSS réfléchi dans les réponses de server functions, versions 1.143.12 -> 1.168.60, publié 2026-09-30) ; RSC « experimental » et dépendant d'APIs React 19 sans semver ; Start impossible sans Vite/Rsbuild ; releases quasi quotidiennes.
- Commun : Windows pris en charge par les trois (subsecond x86_64/arm64, cargo-leptos `CTRL_BREAK_EVENT`, Bun 1.4.4 observé localement sur Windows 11). SEO : SSR complet chez les trois ; Dioxus perd le head hors premier chunk.

## 9. Décision détaillée

1. Moteur principal : Dioxus (fork `yolo-ui` 0.8.0-alpha.3, Blitz 0.3.0-beta.4, Taffy 0.15.1). Cible : `dx` + Axum, SSR streaming activé par défaut dans Yolo, SSG/ISR via `IncrementalRendererConfig`, assets manganis, desktop/mobile/natif.
2. Lane TS : TanStack Router 1.170.x + Query 5.106.x + Table 9.3.x + Form 1.33.x + Virtual 3.14.x + DB 1.0.0-rc sous Bun 1.4.4 ; routage fichiers via `router-generator` lancé par Bun ; SSR via `@tanstack/react-router/ssr/server` (API Request/Response) ; RSC et actions via `@yolo-rs/fullstack` (`react-server-dom-parcel`) et `yolo-react` (encodeur Flight Rust, analyseur OXC `"use client"`/`"use server"`).
3. Fusion : couches Bun.serve (edge, HTML imports, Flight, proxy) -> Axum/Dioxus (runtime Rust) -> Bun.build/TanStack (lane TS). Rejet explicite de reactive_graph/tachys dans Dioxus (deux graphes réactifs, deux ordonnanceurs, double hydratation), de `server_fn` Leptos en parallèle des server functions Dioxus (deux protocoles) et de TanStack Start.
4. Rejets : Leptos runtime, TanStack Start, Next.js, Node/npm/Vite/webpack/Nitro.
5. Réévaluation : si Dioxus 0.8 stable n'est pas publié au 2027-03-31, ou si le budget de bundle n'est pas tenu après l'action 3, rouvrir avec Leptos 0.9 stable comme challenger (pont Bun à construire).

## 10. Cinq premières actions d'implémentation

1. Geler le runtime Rust sur le fork Dioxus 0.8.0-alpha.3 (SHA amont épinglé, porte de montée « 0.8 stable »), activer `enable_out_of_order_streaming` et `invalidate_after` par défaut dans `@yolo-rs/fullstack/dioxus`, ajouter une matrice de tests de parité (streaming, head premier chunk, SSG, ISR, 404 sous suspense).
2. Faire de `@yolo-rs/fullstack` l'unique point d'entrée Bun (`Bun.serve` + imports HTML + `dioxusProxy` + `dioxusWebRoutes`), stabiliser sa suite (44 tests / 3 échecs puis 112 tests / 0 échec observés le 2026-11-04 pendant la refonte), brancher CI `bun test` + `cargo test`.
3. Budget de bundle : hello world Dioxus <= 60 KB compressé et vitrine <= 150 KB (référence krausest : 114,9 KB / 537 ms) via `wasm-split`, `opt-level = "z"`, `wasm-opt -Oz`, `panic = "abort"` ; ajouter `yolo-ui` comme implémentation keyed locale du harnais krausest dans `C:\src\yolo-stack\benches`.
4. Hydratation partielle côté Dioxus : prototype de composants serveur sans WASM (marqueurs `data-component`/`data-props` à la Leptos islands) dans le fork (`vdom-template`, hydratation), repli : drapeau `static` par route qui saute l'hydratation ; mesurer contre les 24 KB / 274 KB de Leptos.
5. Lane TS durcie : épingler les versions TanStack ci-dessus, exécuter `@tanstack/router-generator` sous Bun sans plugin Vite, relier l'analyseur OXC de `yolo-react` au découpage `Bun.build` client/serveur, ajouter un test de non-régression dérivé de CVE-2026-102989 (type de contenu et échappement des réponses d'actions).

## 11. Sources principales

- krausest : https://krausest.github.io/js-framework-benchmark/index.html ; https://krausest.github.io/js-framework-benchmark/2026/chrome156.html ; https://raw.githubusercontent.com/krausest/js-framework-benchmark/master/webdriver-ts-results/src/results.ts ; https://raw.githubusercontent.com/krausest/js-framework-benchmark/master/webdriver-ts-results/src/Common.ts ; https://github.com/krausest/js-framework-benchmark/releases/tag/chrome156
- Dioxus : https://github.com/DioxusLabs/dioxus ; https://github.com/DioxusLabs/dioxus/releases ; https://github.com/DioxusLabs/dioxus/releases/tag/v0.8.0-alpha.2 ; https://github.com/DioxusLabs/dioxus/releases/tag/v0.8.0-alpha.3 ; https://dioxuslabs.com/blog/release-070/ ; https://dioxuslabs.com/learn/0.7/essentials/fullstack/streaming ; https://dioxuslabs.com/learn/0.7/essentials/fullstack/static_site_generation ; https://dioxuslabs.com/learn/0.7/essentials/router/layouts ; https://docs.rs/subsecond/latest/subsecond/ ; https://docs.rs/manganis/latest/manganis/macro.asset.html ; https://github.com/DioxusLabs/components
- Leptos : https://github.com/leptos-rs/leptos ; https://github.com/leptos-rs/leptos/releases/tag/v0.8.0 ; https://github.com/leptos-rs/leptos/releases/tag/v0.9.0-rc1 ; https://book.leptos.dev/ssr/23_ssr_modes.html ; https://book.leptos.dev/islands.html ; https://book.leptos.dev/router/17_nested_routing.html ; https://book.leptos.dev/deployment/binary_size.html ; https://docs.rs/leptos_router/latest/leptos_router/static_routes/struct.StaticRoute.html ; https://docs.rs/server_fn/latest/server_fn/codec/index.html ; https://github.com/leptos-rs/cargo-leptos ; https://opencollective.com/leptos
- TanStack : https://tanstack.com/blog/announcing-tanstack-start-v1 ; https://tanstack.com/start/latest/docs/framework/react/overview ; https://tanstack.com/start/latest/docs/framework/react/guide/hosting ; https://tanstack.com/start/latest/docs/framework/react/guide/server-components ; https://tanstack.com/start/latest/docs/framework/react/guide/static-prerendering ; https://tanstack.com/router/latest/docs/framework/react/guide/ssr ; https://tanstack.com/router/latest/docs/framework/react/routing/file-based-routing ; https://tanstack.com/router/latest/docs/framework/react/guide/data-loading ; https://tanstack.com/router/latest/docs/framework/react/guide/document-head-management ; https://tanstack.com/blog/tanstack-start-security-update-cve-2026-102989 ; https://registry.npmjs.org/@tanstack/react-start
- Bun : https://bun.sh/docs/bundler/fullstack ; https://thenewstack.io/the-problems-with-react-server-components/ (2025-08-04, Jarred Sumner : RSC « not documented and is in a very incomplete state »)
- Local : `C:\src\yolo-ui\Cargo.toml`, `C:\src\yolo-ui\packages\fullstack-server\src\isrg\config.rs`, `C:\src\yolo-ui\packages\yolo-fullstack\`, `C:\src\yolo-ui\crates\yolo-react\src\rsc.rs`
