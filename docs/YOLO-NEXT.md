# yolo-next : migration Next.js 16 vers Bun, Dioxus et Leptos

Statut : outil de migration et de mesure. Selon l'ADR 0003, il n'entre jamais dans l'image Shenron. Le site Shenron vise Leptos 0.8 SSR, les applications natives visent Dioxus, et TanStack ne sert que de bibliothèques Bun.

Base de référence : Next.js 16.4.5, Bun 1.4.4, OXC 0.155 (Rust) et 0.157 (npm), React 19.3.1.

## Composants

| Chemin | Rôle |
|---|---|
| `crates/yolo-next` | Compilateur Rust. Analyse `next.config.*`, l'arbre `app/` et `pages/`, les conventions de segment, les métadonnées, `next/font`, `next/image`, le graphe RSC (`"use client"`, `"use server"`, `"use cache"`) et `proxy.ts`, puis émet trois lanes. |
| `packages/yolo-next` (`@yolo-rs/next`) | Runtime Bun de compatibilité : serveur `Bun.serve`, RSC Flight, server actions, `cache` et `headers`, plus les shims `next/*` (20 au total), `Image`, `Font`, `ImageResponse` (satori + resvg) et l'adaptateur TanStack Router/Query. |
| `apps/next-fixture` | Projet Next 16 App Router de test : groupes, slots `@`, interception, catch-all, route handlers, fichiers de métadonnées, actions, `use cache`, et le répertoire `pages/` historique. |
| `scripts/yolo-next-verify.ts` | Exécute les gates de la CI et écrit un reçu daté dans `docs/receipts/`. |

## Les trois lanes

| Lane | Sortie | Limites connues |
|---|---|---|
| `bun` | `Bun.serve` + React 19 + arbre TanStack Router, et un RPC de server actions qui suit le protocole `Next-Action`. | `proxy.ts` tourne avec la sémantique du runtime Node. |
| `dioxus` | `#[derive(Routable)]` avec layouts et nest, `#[get]`/`#[post]` pour les handlers, `document::*` et `asset!()`. | Les intercepting routes sont rendues comme la page cible. |
| `leptos` | `leptos_router` avec le préfixe `:lang`, `SsrMode::Static` et `prerender_params`, plus `content.rs`, `i18n.rs`, `seo.rs`, `tokens.rs`, une table de redirections 308 et `messages/{fr,en,ja}.json`. Aucun CSS n'est émis. | Mêmes limites d'interception ; les `headers` dynamiques sont résolus au démarrage. |

## Moteur RSC

`src/rsc-engine.ts` choisit le moteur dans cet ordre :

1. `@yolo-rs/fullstack` (yolo-ui, react-server-dom-parcel), chargé par import dynamique s'il est résolvable ;
2. `webpack-worker` (`bun --conditions=react-server rsc-worker.ts`) ;
3. `inline`.

On peut forcer un moteur avec `YOLO_RSC_ENGINE` ou `experimental.rscEngine`.

## CLI

```bash
cargo run -p yolo-next -- analyze <projet> [--json]
cargo run -p yolo-next -- migrate <projet> --out <dir> [--target bun|dioxus|leptos|all] [--dry-run]
cargo run -p yolo-next -- config|routes|match|fonts|metrics|image-url|rsc|rule ...
bun packages/yolo-next/src/cli.ts dev|build|start|analyze|migrate
just next-analyze DIR | just next-migrate DIR OUT [TARGET] | just next-test
```

## Mesures (2026-11-05, Windows 11, Bun 1.4.4)

Gates de `scripts/yolo-next-verify.ts` (reçu : `docs/receipts/yolo-next-2026-11-05.json`) :

| Gate | Résultat |
|---|---|
| `cargo fmt --check` | ok |
| `cargo clippy --all-targets -D warnings` | ok |
| `cargo test -p yolo-next` | 120 réussis, 0 échec |
| `yolo-next analyze apps/next-fixture` | ok |
| `bun test` (`@yolo-rs/next`) | 262 réussis, 0 échec |
| `bun check` et `tsc` racine | ok |

Migrations :

| Projet | Lanes | Parité | Routes | Diagnostics | Fichiers émis |
|---|---|---|---|---|---|
| `apps/next-fixture` | toutes | 85 % | 20 | 4 | 48 |
| `shenron/apps/site` (Next 16.5 canary) | leptos | 91 % | 362 (200 pages, 158 handlers, 4 métadonnées) | 23 (0 erreur) | 18 |

Le rapport Shenron complet se trouve dans `docs/research/notes/shenron-site-next-migration-report.md` et `shenron-site-next-analysis.json`.

## Limites hors ligne

- Google Fonts : sans les métadonnées de Google, l'URL CSS2 d'une police variable ne porte pas de plage `wght`. Les métriques de fallback sont calculées au build, avec `parse_sfnt_metrics` sur des données SFNT décompressées.
- `Bun.Image` encode WebP, PNG et JPEG. L'encodage AVIF échoue sur Windows, et l'optimiseur se replie alors sur le format accepté suivant.
- `@vercel/og` 1.0.3 ne se charge pas sous Bun (son tarball ne contient pas `hb.wasm`). `ImageResponse` repose donc directement sur satori et resvg.
- Les wildcards d'hôte suivent la sémantique documentée (`*` couvre un label, `**` un ou plusieurs). Le matcher réel de Next est plus permissif.

## Écosystème

Les sources Vercel Labs, OXC et Bun natives, avec la décision prise pour chacune, sont dans `docs/ECOSYSTEM-COMBINATION.md` et `ecosystem.toml` (137 sources).
