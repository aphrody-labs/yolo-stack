# ADR 0003 : Verdict framework : Leptos pour le site SEO, Dioxus pour le natif, TanStack hors doctrine

- **Statut** : Accepté (remplace `docs/DECISION.md` et finalise l'ADR 0002)
- **Date** : 2026-11-05
- **Décideurs** : aphrody-dev (demande explicite : « tranche »), session sheron-cf (coordination recherche)
- **Pilier(s) concerné(s)** : yolo-ui, yolo-bun, apps/shenron
- **Sources** : `docs/research/refonte-shenron-rust-bun.md`, `docs/research/notes/framework_benchmarks_verdict.md`,
  `docs/research/notes/dioxus_ecosystem.md`, `docs/research/notes/gpu_rendering_stack.md`,
  `docs/BENCHMARKS.md`, `docs/DECISION-UI-STACK-BENCHMARKS.md`

## Contexte

Trois arbitrages contradictoires ont été produits le 2026-11-04 dans ce dépôt :
`docs/DECISION.md` (TanStack sur Bun primaire, 430/500), `docs/DECISION-UI-STACK-BENCHMARKS.md`
(Dioxus primaire, 75,25/100) et le rapport de recherche (Leptos pour le site, Dioxus pour le natif).
La demande du propriétaire fixe les contraintes qui départagent : refonte complète de Shenron
en Rust + Bun, sans Next.js, Tailwind, Base UI ni shadcn ; WebGPU/wgpu réel ; Taffy ; un seul choix.

## Faits retenus (krausest Chrome 156, publié 2026-10-25, recalculé depuis `results.ts`)

| Keyed | Version | CPU géom. | Mémoire géom. | Taille brute / gzip | First paint |
|---|---|---|---|---|---|
| dioxus | 0.7.2 | 1,30 | 1,83 | 419,6 / 114,9 kB | 536,9 ms |
| leptos | 0.7.0 | 1,32 | 1,46 | 189,6 / 48,8 kB | 236,4 ms |
| react-hooks (lane TanStack) | 19.2.0 | 1,56 | 2,64 | 190,3 / 51,4 kB | 228,1 ms |

- Leptos 0.8.23 stable (2026-10-31), 0.9.0-rc1 publié ; `SsrMode::Static` + `RegenerationFn` (ISR),
  îlots (plancher 24 kB wasm), `leptos_i18n`, cargo-leptos. Aucune cible native.
- Dioxus 0.7.10 stable (2026-07-30), 0.8.0-alpha.3 ; pas d'îlots (PR #1758 fermée) ; desktop, mobile,
  Blitz (Stylo + Taffy + anyrender/vello_gpu sur wgpu 30, AccessKit 0.25) ; équipe recrutée par Cognition
  le 2026-09-10 ; fork aplati déjà dans `C:\src\yolo-ui`.
- TanStack Start 1.0 (2026-10-13) exige Vite 8 ou Rsbuild + Nitro et React 19 : TypeScript et
  Node-shaped, hors doctrine « Rust + Bun purs ». Router/Query/Virtual seuls restent utilisables sous Bun.
- Le critère « coût de migration Next » qui faisait gagner TanStack dans `docs/DECISION.md` est nul ici :
  le propriétaire impose la réécriture des 272 895 lignes liées à Next (`apps/site`) et des 47 k lignes
  React de `packages/ui`.

## Décision

1. **Site public SEO (dragonballfr.com, ~55 000 pages fr/en/ja)** : **Leptos 0.8.x sur Axum**
   (SSR + `SsrMode::Static` par locale + îlots), passage à 0.9 quand stable. Squelette :
   `apps/shenron/crates/shenron-site`.
2. **Applications natives (Shenron Studio, lecteur desktop, outils)** : **Dioxus** (0.7.10 stable
   aujourd'hui, 0.8 dès stabilisation) depuis le pilier yolo-ui, avec Blitz/Taffy 0.15.1 et anyrender.
   Squelette : `apps/shenron/crates/shenron-reader-poc`.
3. **Rendu GPU partagé** : wgpu 30.0.1, vello_gpu 0.3 (repli WebGL2 natif) et vello_cpu 0.3
   (headless, OG images), Parley 0.12 (`complex-scripts`), Taffy 0.15.1, AccessKit 0.25.2 natif.
   rust-skia exclu du web (emscripten + WebGL seulement), autorisé en option native via `anyrender_skia`.
4. **Couleur et tokens** : `yolo-tokens` (HCT/CAM16, 8 presets) du pilier yolo-ui ;
   `pillars/yolo-ui/crates/yolo-ui-color` sert d'oracle de parité en test.
5. **TanStack** : exclu comme coquille applicative. Autorisé uniquement comme bibliothèques Bun
   (Query, Table, Virtual) dans l'outillage interne TypeScript qui subsiste (bot, admin Bun).
6. **Frontière entre les deux runtimes Rust** : HTTP, JSON server-fn et contrat d'îlot `data-component`
   (`crates/yolo-stack-routes`). Aucun partage de graphe réactif.

## Conséquences

- `docs/DECISION.md` passe en statut « remplacé par ADR 0003 ». `docs/DECISION-UI-STACK-BENCHMARKS.md`
  reste la référence chiffrée ; son verdict « Dioxus moteur principal » vaut pour le natif uniquement.
- ADR 0002 passe en « Accepté, périmètre natif » (voir en-tête).
- `apps/next-fixture`, `crates/yolo-next` et `packages/yolo-stack-next-compat` sont des outils de
  migration et de mesure, pas des cibles de production ; ils ne doivent jamais être livrés dans l'image
  Shenron.
- Déclencheur de réouverture : Dioxus livre des îlots ou un bundle gzip inférieur à 60 kB sur
  `island-counter` avant le 2027-03-31, ou Leptos 0.9 stable régresse en SSR.

## Sécurité de la fusion des piliers (avertissements session yolo-eb, audit gitleaks 2026-11-04)

- Fusionner (`just merge-pillar`) uniquement depuis les branches auditées de `github.com/yolo-rs/<pilier>` :
  `main`, `codex`, `claude-<tranche>` (tiret, jamais `claude/<tranche>`). Jamais depuis un working tree.
- yolo-ui et yolo-web sont publics et leurs working trees contiennent des secrets (apps intégrées) :
  pointer `origin/main` seulement.
- yolo-plugins versionne 7 clés privées Ed25519 (`**/.keys/ed25519.key`) : régénérer et purger
  l'historique (`git filter-repo`) avant tout passage public.
- yolo-web : renommage `crates/yolo-standards` -> `yolo-web-standards` à faire côté yolo-web dans un
  commit dédié (collision avec le pilier yolo-standards) ; ne pas le faire deux fois.

## Couche transitoire (réconciliation avec `docs/DECISION.md`, session yolo-bun, 2026-11-05)

`@yolo-rs/fullstack` (`C:\src\yolo-ui\packages\yolo-fullstack`, commit 875dc8bd1) exécute aujourd'hui
l'`app/` Next 16 de Shenron sur Bun (RSC, server actions, `next.config`, `next/font`, `next/image`,
`proxy.ts`). Il est **autorisé comme pont de migration uniquement** : il sert les pages non encore
réécrites pendant que Leptos les remplace route par route, derrière le même reverse proxy Axum.
Conditions : aucune nouvelle page n'y est écrite ; il disparaît de l'image Shenron quand la dernière
route Next est portée (porte de la phase 4 du rapport) ; il n'est jamais la cible finale. Le fork
Next 16.5.0-canary.40 patché Bun + Rust LTO + wasm + WebGPU (`C:\src\yolo-next`, branche
`yolo-bun-rust`) relève de la même couche transitoire et ne doit pas être cloné une seconde fois.
Couches finales : Leptos (site SEO), Dioxus (natif), services Bun (bot, API, pipeline).
