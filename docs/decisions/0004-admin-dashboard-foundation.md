# ADR 0004 : Fondation des tableaux de bord d'administration : Dioxus + dioxus-primitives + couche `yolo-dashboard`

- **Statut** : Accepté
- **Date** : 2026-11-05
- **Décideurs** : aphrody-dev (demande explicite : « décide, sans question »), session de recherche ADR dashboard
- **Pilier(s) concerné(s)** : yolo-ui (nouvelle couche `yolo-dashboard`), yolo-bun, apps/shenron (admin dragonballfr.com/admin)
- **Dépend de** : ADR 0003 (Leptos site SEO, Dioxus natif et admin, wgpu 30 + Vello + Parley + Taffy 0.15.1,
  tokens `yolo-tokens`, TanStack seulement comme bibliothèques Bun)
- **Sources** : relevés `gh api` et API crates.io du 2026-11-05 (tableau « Faits »), `docs/research/notes/dioxus_ecosystem.md`,
  inventaire en lecture seule de `C:\src\shenron\apps\site\src\app\admin` (81 `page.tsx`) et `src\app\api\admin` (20 `route.ts`),
  `C:\src\yolo-ui` (crates `yolo-tokens`, `yolo-layout`, `yolo-render`, `yolo-shared`, `submodules/dioxus-components`,
  `skills/yolo-ui-pro-max/data`), crate `apps/shenron/crates/shenron-admin-ai` (branche `claude-admin-ai`, Cargo.toml du 2026-11-05)

## Contexte

L'admin Shenron compte **81 pages** (`page.tsx`) et **20 routes API** (`/api/admin/**/route.ts`), en Next.js 16 + React 19
+ Tailwind v4 + Base UI. Les dépendances réellement importées sous `src/app/admin` sont : `next` (83 imports),
`@aphrody/shenron-ui` (52), `react` (51), `@tanstack/react-query` (39 fichiers), `lucide-react` (19), `next-intl` (16),
`zod` (3), `drizzle-orm` (4), `@tiptap/react` (1, `posts`), `@uiw/react-color` (1). Le kit admin partagé
(`packages/ui/src/admin`) contient déjà exactement les briques à reproduire : `AdminPageHeader`, `AdminSidebarView`,
`ConfirmDialog`, `DataTable` + `data-table-model.ts`, `FormGrid`, `Metrics`, `RythmeSparkline`, `StatusBadge`,
`QueryProvider`, `useHotkeys`. Le registre `components/admin/admin-pages.ts` classe les pages en 7 familles
(pilotage, site, système, encyclopédie, communauté, économie, sécurité). Le temps réel actuel est un sondage
`/api/admin/dashboard-live` (cache 6 s, sondes HTTP 1,5 s, ping Redis) et un `EventSource` sur `/admin/events`.

L'ADR 0003 place l'admin hors du site SEO : pages `noindex`, authentifiées, sans contrainte de first paint public.
Le coût de bundle de Dioxus (114,9 kB gzip contre 48,8 kB pour Leptos, krausest Chrome 156) y est acceptable, et
Dioxus est le seul choix qui sert la même arborescence en web et en desktop (Shenron Studio). Le premier écran porté,
le cockpit IA (`shenron-admin-ai`), fixe déjà le gabarit : **Dioxus 0.7.10 de crates.io** (features `web` et `desktop`),
un **hub Axum 0.8 séparé** (« one upstream poller, SSE fan-out, admin gate, action proxy, fixture mode »),
`EventSource` via `web-sys` côté web, `reqwest` en flux côté desktop, et une palette HCT/CAM16 dérivée **au build**
par `build.rs` depuis `yolo-tokens`. Le fork Dioxus de `C:\src\yolo-ui` est en `0.8.0-alpha.3` et son README indique que
la feature `primitives` attend la migration du paquet 0.7 vers l'API 0.8.

Doctrine imposée : Rust + Bun purs, ni Node/npm/pnpm, ni Tailwind, ni Base UI/shadcn en code, pas de HTML/CSS/JS brut
écrits à la main hors composants RSX, tokens HCT/CAM16, icônes Phosphor, ni Inter ni Roboto.

## Faits (relevés `gh api` et crates.io, 2026-11-05 ; commits = commits sur la branche par défaut depuis le 2026-08-07)

| Dépôt / crate | Étoiles | Issues+PR ouvertes | Licence | Dernière release | Commits 90 j | Rôle évalué |
|---|---|---|---|---|---|---|
| DioxusLabs/dioxus | 39 606 | 896 | Apache-2.0 | v0.7.10 (2026-07-30) | 71 | runtime admin |
| DioxusLabs/dioxus-components | 355 | 51 | Apache-2.0/MIT | aucune (crate `dioxus-primitives` 0.0.0) | 3 | primitives ARIA |
| rust-ui/ui (Rust/UI, Dioxus) | 787 | 11 | MIT | aucune | 320 | registre shadcn-like |
| longbridge/gpui-component (→ gpui-kit) | 19 177 | 147 | « Other » (API GitHub) | v0.9.0 (2026-11-02) | 775 | kit GPUI desktop |
| thaw-ui/thaw (Leptos) | 614 | 33 | MIT | v0.4.8 (2025-08-03) | 0 | kit Leptos |
| lpotthast/leptonic | 302 | 36 | Apache-2.0 | v0.5.0 (2024-02-19) | 0 | kit Leptos |
| cloud-shuttle/leptos-shadcn-ui | 74 | 9 | MIT | v0.6.0 (2025-09-04) | 0 | port shadcn Leptos |
| Synphonyte/leptos-struct-table | 261 | 7 | Apache-2.0 | crate 0.19.1 (2026-09-02, 209 594 téléch.) | 11 | table Leptos |
| zernst3/rust-chorale | 5 | 15 | non déclarée | aucune | 0 | table headless Dioxus |
| opensass/table-rs | 22 | 7 | MIT | — | 0 | table pagination |
| emilk/egui | 32 004 | 1 137 | Apache-2.0 | 0.37.0 (2026-10-15) | 216 | UI immédiate |
| emilk/egui_plot | 484 | 88 | Apache-2.0 | 0.38.0 (2026-10-19) | 15 | graphes egui |
| iced-rs/iced | 31 916 | 519 | MIT | 0.14.0 (2025-12-07) | 337 | UI Elm |
| pop-os/libcosmic | 966 | 219 | MPL-2.0 | aucune | 97 | widgets COSMIC |
| yuankunzhang/charming | 2 608 | 22 | Apache-2.0 | crate 0.6.0 (2025-06-17) | 10 | ECharts |
| plotters-rs/plotters | 4 644 | 187 | MIT | crate 0.3.7 (2024-09-08) | 0 | graphes |
| jagtesh/lumen-charts | 19 | 0 | Apache-2.0 | crate 2.0.2 (2026-03-05) | 0 | graphes Vello |
| linebender/vello | 4 467 | 213 | Apache-2.0 | 0.11.0 (2026-10-02, wgpu 30) | 164 | rendu 2D GPU |
| leeoniya/uPlot | 10 616 | 133 | MIT | 1.6.32 (2025-03-14) | 173 | îlot séries denses |
| TanStack/table | 28 548 | 80 | MIT | 2026-10-12 | 62 | table Bun |
| TanStack/query | 50 527 | 113 | MIT | 2026-11-04 | 1 109 | cache Bun |
| TanStack/router | 15 283 | 814 | MIT | 2026-10-22 | 274 | routeur Bun |
| TanStack/virtual | 7 187 | 86 | MIT | 2026-11-03 | 69 | virtualisation Bun |
| ueberdosis/tiptap | 38 962 | 910 | MIT | v3.32.0 (2026-10-13) | 311 | îlot texte riche |
| juanma-dev/taino-edit | 16 | 8 | Apache-2.0 | crate 0.7.0 (2026-09-05) | 5 | texte riche Rust |
| dioxus_codemirror (crate) | — | — | — | 0.3.1 (2026-06-09, 786 téléch.) | — | îlot code |
| codemirror/dev | 7 806 | 12 | non déclarée (API GitHub) | — | 0 (dépôt GitHub **archivé**) | éditeur code |
| tokio-rs/axum | 27 695 | 117 | MIT | axum-v0.8.9 (2026-04-14) | 55 | serveur, SSE |
| redis-rs/redis-rs | 4 274 | 98 | BSD-3 | redis-1.8.1 (2026-10-30) | 222 | fan-out |
| jprochazk/garde | 906 | 32 | Apache-2.0 (API GitHub) | v0.23.0 (2026-05-23) | 0 | validation |
| Keats/validator | 2 526 | 67 | MIT | crate 0.21.0 (2026-07-27) | 0 | validation |
| shadcn-ui/ui · satnaing/shadcn-admin · TailAdmin free · tremorlabs/tremor · refinedev/refine | 127 386 · 16 553 · 1 297 · 3 689 · 35 997 | — | MIT · MIT · MIT · Apache-2.0 · MIT | — | 269 · 0 · 31 · 0 · 4 | références UX seulement |

Constats décisifs :

- `dioxus-primitives` couvre 37 modules WAI-ARIA (accordion, alert_dialog, calendar, checkbox, combobox, context_menu,
  date_picker, dialog, dropdown_menu, menubar, popover, select, tabs, toast, tooltip, **virtual_list**, etc.) ; la
  version vendorisée dans `yolo-ui/submodules/dioxus-components` expose aussi 43 composants stylés (dont `sidebar`,
  `pagination`, `form`, `sheet`, `skeleton`). Distribution par copie (`dx components add`), pas par crates.io.
  Faible cadence (3 commits en 90 j) : on copie et on possède le code, on ne dépend pas d'un rythme amont.
- Rust/UI est le catalogue le plus complet (Data Table, Data Grid, Command, Charts, AutoForm, MultiSelect) mais son
  README le déclare « Early stage, experimental », « Built with Tailwind CSS v4 », `tw_merge`, et exige
  « Node.js and pnpm for Tailwind tooling » : incompatible avec la doctrine.
- gpui-component a le meilleur `DataTable` Rust (défilement virtuel, colonnes fixes et redimensionnables, tri,
  « hundreds of thousands of rows »), des graphes et un Dock, mais GPUI ne produit pas de page web : il ne peut pas
  servir `/admin/*` dans un navigateur.
- Dioxus 0.7 fullstack fournit `ServerEvents`/`SseTx`, `Websocket`/`use_websocket` et `Streaming<T>` sur Axum ;
  les messages SSE sont texte (binaire en base64 ou `Streaming<T>`).
- Bun sert nativement des imports HTML via `Bun.serve({ routes })` avec HMR depuis 1.3 ; TanStack Router tourne
  sans Vite en routes codées ou avec `@tanstack/router-generator` en plugin Bun ; TanStack Start exige Vite + Nitro.

## Décision

Nous retenons **Dioxus 0.7.10 (puis 0.8 stable) + `dioxus-primitives` copiés et possédés + une couche `yolo-dashboard`
du pilier yolo-ui stylée exclusivement par `yolo-tokens`**, avec un hub temps réel **Axum SSE** (`watch` + `broadcast`,
Redis Streams pour la reprise), des **graphes SVG RSX** (Canvas 2D au-delà de 1 000 points), une **table headless
maison** inspirée du modèle d'état de TanStack Table, des **formulaires `garde`**, et **CodeMirror 6 / Tiptap 3 en îlots**
tant qu'aucun équivalent Rust n'atteint la parité. Migration par tranches verticales derrière les mêmes URL `/admin`.

| Volet | Choix | Rejeté et pourquoi |
|---|---|---|
| Runtime | Dioxus 0.7.10 crates.io, `web` + `desktop` ; bascule 0.8 quand stable via le fork yolo-ui | Leptos : pas de desktop, cockpit déjà Dioxus |
| Kit | `dioxus-primitives` (comportements ARIA, focus, clavier) copiés à un commit épinglé dans `yolo-dashboard`, styles réécrits | `dx-components-theme.css` remplacé ; Rust/UI (Tailwind) |
| Table | `yolo_dashboard::table` : modèle pur Rust (colonnes, tri multi-colonnes, filtres, pagination, sélection, visibilité), requête serveur typée `TableQuery { sort, range, filters }`, rendu virtualisé sur `virtual_list` | rust-chorale (5 étoiles, licence absente) ; leptos-struct-table (mauvais framework, sert de modèle pour `TableQuery`) |
| Formulaires | Structs `serde` + `garde` 0.23 partagées wasm/serveur, `FormGrid`, suivi « sale », erreurs serveur mappées par champ | `validator` (pas de validation contextuelle), zod (TS) |
| Graphes | SVG RSX générés en Rust (sparkline, ligne, aire, barres, jauge) avec tableau de données accessible ; Canvas 2D `web-sys` + décimation LTTB en Rust au-delà de 1 000 points ; Vello/`yolo-render` en desktop | charming (ECharts JS, crate figée 2025-06), plotters (0 commit, 2024), lumen-charts (0 commit, financier) ; uPlot gardé comme îlot de secours |
| Temps réel | Hub Axum : un sondeur amont par source, `tokio::sync::watch` pour les instantanés, `broadcast` borné pour les journaux, SSE avec `id`, `retry`, keep-alive 15 s, reprise `Last-Event-ID` ; Redis Streams (`XADD MAXLEN ~10000`) comme bus entre Bun et hubs | Dioxus `ServerEvents` réservé aux pages qui n'ont pas besoin d'être lues par Next pendant la migration ; WebSocket seulement pour le bidirectionnel (console, command-runner) |
| Éditeurs | Code : îlot CodeMirror 6 vendorisé (modèle `dioxus_codemirror` : asset local, canal `document::eval`, messages typés JSON). Texte riche : îlot Tiptap 3.32 pour `posts` ; cible de sortie `taino-edit` 0.7 (Dioxus, Apache-2.0) après preuve de parité | Monaco (poids), réécriture d'éditeur maison |
| Thème | `build.rs` → `yolo_tokens::sheets::{scheme_css, tokens_css}` → `assets/yolo-dashboard.css` (variables `--yd-*`, `data-theme` clair/sombre, 7 presets + preset Shenron) ; aucune classe utilitaire | Tailwind v4, `tw_merge` |
| Icônes | Phosphor (`yolo-ui/skills/yolo-ui-pro-max/data/phosphor-icons-upstream.json`) en composants RSX | lucide-react, dioxus-free-icons |
| Côté Bun | Aucune nouvelle page Next. Exception documentée seulement : `Bun.serve` + imports HTML + React 19 + TanStack Router (routes codées) / Query / Table / Virtual, montée sous `/admin/<slug>` par la passerelle | TanStack Start (Vite + Nitro), Next |

## Options considérées (barème sur 100)

Critères et poids : doctrine Rust + Bun purs (25), adéquation ADR 0003 et cockpit (web + desktop, Dioxus 0.7.10) (20),
couverture admin (table, formulaires, overlays, graphes, palette de commandes) (20), santé du projet (15),
accessibilité (10), thémabilité `yolo-tokens` (10).

| Option | Doctrine | Adéquation | Couverture | Santé | A11y | Thème | **Total** |
|---|---|---|---|---|---|---|---|
| **Dioxus + dioxus-primitives copiés + `yolo-dashboard`** | 24 | 20 | 13 | 8 | 9 | 10 | **84** |
| gpui-component (gpui-kit 0.9.0) | 22 | 4 | 20 | 15 | 4 | 6 | 71 |
| Leptos : thaw 0.4.8 + leptos-struct-table 0.19 | 23 | 8 | 15 | 8 | 6 | 7 | 67 |
| Bun : React 19 + TanStack Router/Query/Table/Virtual | 8 | 6 | 20 | 15 | 8 | 8 | 65 |
| egui 0.37 + egui_plot 0.38 | 25 | 3 | 12 | 14 | 4 | 5 | 63 |
| Rust/UI (rust-ui/ui, Dioxus) | 5 | 16 | 19 | 11 | 6 | 4 | 61 |
| Iced 0.14 / libcosmic + iced_aw | 25 | 2 | 9 | 12 | 3 | 5 | 56 |

Justification des écarts :

- **Retenu (84)** : seul candidat qui remplit à la fois la doctrine, le web `/admin/*`, le desktop et le gabarit du
  cockpit. La couverture (13/20) est faible en table, graphes et palette de commandes : c'est précisément ce que
  `yolo-dashboard` ajoute. Le point « doctrine » perdu vient du petit asset `focus-trap.js` des primitives sur le web.
- **gpui-component (71)** : meilleur kit Rust brut (DataTable virtualisé, graphes, Dock, 775 commits en 90 j) mais
  GPUI est desktop seulement, ne produit pas de DOM et ne peut pas servir les URL `/admin` ; licence déclarée « Other ».
  Conservé comme **référence d'API** pour `DataTable` et pour un futur Shenron Studio GPUI éventuel, pas comme fondation.
- **Leptos thaw + leptos-struct-table (67)** : leptos-struct-table est la meilleure table Rust web (virtualisation par
  défaut, tri multi-colonnes, fournisseur de données serveur avec sqlx), mais thaw n'a aucun commit en 90 j ni release
  depuis 2025-08, Leptos n'a pas de cible desktop et le cockpit est déjà en Dioxus. Son modèle `TableDataProvider`
  inspire `TableQuery`.
- **Bun + TanStack (65)** : couverture et santé maximales, mais React/TS est hors doctrine finale (ADR 0003, point 5).
  Gardé comme voie d'exception pour une page bloquée, jamais comme cible.
- **egui (63) / Iced (56)** : rendu canvas immédiat ou Elm sans DOM, accessibilité web faible, pas d'URL par page ;
  egui reste admis pour des outils internes natifs (inspecteurs, profils).
- **Rust/UI (61)** : le plus proche fonctionnellement (Dioxus 0.7, Data Table, Command, Charts, AutoForm, 320 commits
  en 90 j), éliminé par Tailwind v4 + `tw_merge` + pnpm et son statut expérimental. Licence MIT : ses compositions peuvent
  servir de référence de structure, sans reprendre ses classes Tailwind.

## Couche `yolo-dashboard` (pilier yolo-ui, réutilisable par toute la yolo stack)

Deux crates dans `C:\src\yolo-ui\crates\`, publiées dans yolo-stack via `just merge-pillar` depuis `origin/main` :

1. **`yolo-dashboard`** (UI, features `web`, `desktop`, `native` ; dépend de `dioxus` 0.7.10, primitives copiées
   sous `src/primitives/` avec `LICENSE-MIT` et `LICENSE-APACHE` et le commit amont en en-tête) :
   - `shell` : `DashboardShell` (barre latérale repliable par familles, barre supérieure, fil d'Ariane, zone de travail,
     feuille mobile), alimenté par un **manifeste JSON** de navigation (`id`, `family`, `route`, `engine`) ;
   - `nav` : correspondance de route (statique avant paramétrée, comme `adminPageForPath`), palette de commandes ⌘K
     (`combobox` + `dialog` + raccourcis clavier), liens inter-moteurs en navigation pleine page ;
   - `table` : modèle headless + vue virtualisée, état synchronisé dans l'URL, sélection et actions groupées,
     visibilité des colonnes, export CSV, états vide, erreur et squelette ;
   - `stat` : `StatCard`, `Metric` (delta, tendance), `Sparkline`, `StatusBadge` ;
   - `live` : `use_live<T>(url)` (EventSource web, flux `reqwest` desktop, backoff exponentiel plafonné à 30 s avec
     gigue, reprise `Last-Event-ID`), `LiveStatus` (connecté, reconnexion, périmé au-delà de 2 intervalles), horodatage
     de fraîcheur ;
   - `form` : `FormGrid`, `Field`, intégration `garde`, `ConfirmDialog` pour toute action destructive, toasts ;
   - `chart` : SVG (ligne, aire, barres, barres empilées, donut, jauge) avec motifs et libellés directs en plus de la
     couleur, tableau de données accessible, Canvas 2D + LTTB au-delà de 1 000 points (seuils de `charts.csv`) ;
   - `island` : contrat d'îlot `data-component` (ADR 0003, point 6) pour CodeMirror, Tiptap et uPlot, assets vendorisés
     et servis localement, aucune CDN ;
   - `theme` : CSS généré au build depuis `yolo-tokens`, preset par application, `prefers-color-scheme` et `data-theme`.
2. **`yolo-dashboard-server`** (Axum 0.8, généralise le serveur du cockpit) : `SseHub<T>` (sondeur unique, `watch`
   instantané, `broadcast` borné à 256 messages, en cas de retard `Lagged` envoi d'un événement `resync` avec
   l'instantané courant au lieu de bloquer), anneau de reprise des 1 024 derniers événements, keep-alive 15 s,
   `retry: 3000`, extracteur `AdminGate` (session better-auth validée auprès de l'API Bun, jeton d'admin pour les
   agents), `ActionProxy` vers l'API Bun avec journal d'audit, mode fixture pour les tests, consommateur Redis Streams.

Le cockpit `shenron-admin-ai` est le premier client : son `build.rs` `yolo-tokens`, son hub SSE et ses composants
deviennent la base de `yolo-dashboard` et `yolo-dashboard-server` (extraction, pas réécriture).

## Plan de migration des 81 pages admin

`C:\src\.wt\shenron\admin-purge\docs\audits\admin-inventory-2026-11-05.md` n'existait pas à la rédaction : le découpage
suit les répertoires de `src/app/admin`, le registre `admin-pages.ts` et la taille des fichiers (lignes `.ts/.tsx` du
répertoire de la page).

### Coexistence

- **Passerelle unique** : le reverse proxy Axum (`crates/yolo-stack-routes`) lit une table `admin-routes.toml`
  (préfixe → `dioxus` | `next` | `bun`). Basculer une tranche = changer une ligne ; retour arrière en moins d'une minute.
- Pages non portées servies par le pont `@yolo-rs/fullstack` (ADR 0003, couche transitoire) ; aucune nouvelle page Next.
- Une seule session : cookie better-auth vérifié par `AdminGate` côté Rust et par `requireAdmin` côté Next.
- Une seule navigation : `admin-pages.ts` devient `admin-pages.json` (avec champ `engine`) lu par la barre latérale Next
  et par `DashboardShell` ; passage d'un moteur à l'autre en navigation pleine page.
- Un seul thème : la feuille `yolo-dashboard.css` est aussi injectée dans le layout Next admin pendant la transition.
- Écritures : la base reste possédée par l'API Bun (Drizzle) ; les routes `/api/admin/*` purement agrégatrices passent
  dans le hub Rust, celles qui écrivent passent dans l'API Bun, contrat JSON inchangé.
- Le hub SSE est lisible par les pages Next restantes : `/admin/events` et le tableau de bord Next peuvent consommer
  le même flux avant leur portage.

### Ordre des tranches (valeur décroissante, risque croissant)

| # | Tranche | Pages (nb) | API `/api/admin` | Lignes | Pourquoi à ce rang |
|---|---|---|---|---|---|
| 0 | Socle | `/admin` (redirection) (1) | — | — | passerelle, `AdminGate`, manifeste, shell, thème ; aucune page métier |
| 1 | Cockpit IA (en cours, `claude-admin-ai`) | `ai`, `evaluations`, `agents`, `a2a` (4) | `ai`, `a2a` | ~1 400 | déjà commencé, lecture et SSE, fixe le gabarit |
| 2 | Pilotage temps réel | `dashboard`, `live`, `services`, `events`, `logs`, `stats`, `activity`, `sync` (8) | `dashboard-live` | ~2 200 | plus forte valeur quotidienne, surtout lecture, réutilise le hub ; remplace le sondage 6 s par SSE |
| 3 | Données en lecture | `database`, `database/[table]`, `audit`, `audit-internal`, `reports`, `api-docs` (6) | `reports` | ~2 250 | éprouve `table` (CRUD générique `DbCrud`, virtualisation) sur des écrans à faible risque d'écriture |
| 4 | Bot et automatisation | `bot`, `bot/[id]`, `console`, `send`, `command-runner`, `commands`, `cron`, `triggers`, `webhooks`, `discord` (10) | — (proxy `bot-admin`) | ~3 200 | effets Discord réels : exige `ConfirmDialog`, audit, WebSocket pour la console |
| 5 | Communauté, modération, sécurité | `moderation`, `moderation/comments`, `tickets`, `messages`, `giveaways`, `profile`, `hierarchy`, `account` (8) | `account` | ~3 200 | écritures sur utilisateurs et rôles ; `account` (2FA, passkey) testé contre better-auth |
| 6 | Économie | `economy`, `levels`, `shop`, `leaderboards`, `taxes`, `contests` (6) | — | ~4 200 | formulaires lourds (`levels` 1 703, `shop` 1 052), argent virtuel : validation `garde` stricte |
| 7 | Encyclopédie et catalogue | `db-universe/{sources,assets,sagas,episodes,movies,games,databooks}`, `launch`, `visibility`, `bxc-campaigns`, `timeline`, `canon` (12) | `catalog/sources/import`, `databooks/campaign`, `databooks/status`, `transcriptions`, `bxc-harvest/[key]`, `manga/revalidate` | ~4 550 | touche le contenu public et la revalidation Leptos ; `canon` (1 455) et `timeline` (993) en dernier |
| 8 | Wiki | `wiki`, `wiki/{history,todo,characters,contributions}`, `wiki/category/{new,[id]}`, `wiki/page/{new,[id]}`, `wiki/studio/[table]/[id]` (10) | `wiki-contributions`, `contribution-rights`, `entities` | ~1 900 (+ `WikiStudio`, `CharacterPageEditor`) | éditeurs structurés, îlot CodeMirror (`JsonEditor`), workflow éditorial |
| 9 | Site, éditorial, médias | `home`, `design`, `settings`, `posts`, `posts/{new,[id]}`, `banners`, `monetization`, `cards`, `canvas`, `canvas/themes`, `tierlists`, `tierlists/{new,[id]}`, `dbxv2`, `dbxv2/immersion` (16) | `banner-library`, `media-library`, `upload`, `cards/images`, `monetization`, `video/crunchyroll/[...path]` | ~9 100 | plus gros risque : Tiptap, sélecteur de couleur, recadrage d'images, éditeur canvas, téléversements, paiements |

Total : 1 + 4 + 8 + 6 + 10 + 8 + 6 + 12 + 10 + 16 = **81 pages** ; 2 + 1 + 1 + 0 + 1 + 0 + 6 + 3 + 6 = **20 routes API**.
Les tranches 3 à 6 peuvent avancer en parallèle une fois la tranche 2 sortie ; 7 à 9 restent séquentielles.

### Critères de sortie d'une tranche (tous obligatoires)

1. **Parité** : liste des fonctions de chaque page Next (actions, filtres, états vide, erreur, chargement) cochée ;
   aucun écart non signé par le propriétaire.
2. **Contrat** : réponses JSON des routes portées identiques à celles de Next sur un jeu enregistré (tests instantané).
3. **Parcours** : scénarios de bout en bout via yolo-web (CDP) sur les mêmes URL, web et desktop pour les tranches 1 et 2.
4. **Accessibilité** : navigation clavier complète, focus visible, contraste vérifié par `yolo-tokens::contrast` (AA),
   graphes avec tableau de données.
5. **Performance** : wasm initial de l'admin ≤ 350 kB gzip, interaction tableau ≤ 100 ms sur 10 000 lignes,
   reprise SSE ≤ 5 s après coupure, zéro fuite mémoire sur 1 h de flux.
6. **Exploitation** : 7 jours en production derrière la passerelle sans retour arrière ni erreur 5xx imputable.
7. **Retrait** : la page Next, ses composants exclusifs et la route API Next sont supprimés dans le même lot ;
   la ligne `admin-routes.toml` passe à `dioxus` définitivement.

La phase 5 du rapport (`refonte-shenron-rust-bun.md`) est atteinte quand les dix tranches sont sorties : zéro import
`next`, `@tanstack/react-query`, `@tiptap/react`, `lucide-react` sous l'admin, et le pont `@yolo-rs/fullstack` ne sert
plus aucune route `/admin`.

## Conséquences

- Positives : un seul arbre RSX pour `/admin` web et Shenron Studio desktop ; une couche `yolo-dashboard` réutilisable
  par tous les projets de la yolo stack ; temps réel par poussée au lieu du sondage 6 s ; plus de Tailwind ni de Base UI
  dans l'admin ; thème HCT/CAM16 cohérent avec le site Leptos.
- Négatives : maintenance propre des primitives copiées (amont à 3 commits en 90 j) ; table, graphes et palette de
  commandes à écrire ; deux îlots JS (CodeMirror, Tiptap) persistent jusqu'à preuve d'un remplaçant Rust ; bundle
  Dioxus plus lourd que Leptos, accepté pour une zone authentifiée.
- Travaux induits : extraction de `yolo-dashboard` et `yolo-dashboard-server` depuis le cockpit ; `admin-routes.toml`
  et `admin-pages.json` ; consommateur Redis Streams côté hub et producteur côté bot Bun ; suivi de la branche `0.8`
  de dioxus-components pour la bascule Dioxus 0.8 ; vérification de l'hébergement amont de CodeMirror (dépôt GitHub
  `codemirror/dev` archivé) avant vendorisation.
- Déclencheurs de réouverture : Rust/UI abandonne Tailwind ; gpui-component livre une cible web ; `taino-edit` atteint
  la parité Tiptap sur `posts` ; Dioxus 0.8 stable change l'API des primitives au point d'imposer une réécriture.

## Vérification

- `rg -n "tailwind|tw_merge|@base-ui|shadcn" C:\src\yolo-ui\crates\yolo-dashboard` : aucun résultat.
- `cargo tree -p shenron-admin-ai -e normal | rg "dioxus v0.7.10"` puis, après extraction, `rg yolo-dashboard` dans le
  Cargo.toml du cockpit.
- `rg -c "engine = \"dioxus\"" admin-routes.toml` : progression du nombre de préfixes portés ; objectif 81 pages.
- Sous `apps/site/src/app/admin` : `rg -l "@tanstack/react-query|@tiptap/react|lucide-react"` décroît à chaque tranche.
- Tests du hub : coupure réseau simulée en mode fixture, reprise par `Last-Event-ID` sans perte ni doublon.

## Références

- DioxusLabs/dioxus-components : https://github.com/DioxusLabs/dioxus-components (README, `primitives/src/lib.rs`)
- Dioxus 0.7 fullstack (SSE, WebSocket, streaming) : https://dioxuslabs.com/learn/0.7/essentials/fullstack/ ;
  https://github.com/DioxusLabs/dioxus/blob/main/examples/07-fullstack/server_sent_events.rs ;
  https://docs.rs/dioxus/latest/dioxus/prelude/dioxus_fullstack/index.html
- Rust/UI : https://github.com/rust-ui/ui ; https://rust-ui.com/components/data-table
- gpui-component : https://github.com/longbridge/gpui-component ; https://longbridge.github.io/gpui-component/docs/components/table
- leptos-struct-table : https://github.com/Synphonyte/leptos-struct-table (exemple `serverfn_sqlx`)
- rust-chorale : https://users.rust-lang.org/t/code-review-headless-virtualized-dioxus-table-library/140450
- charming : https://github.com/yuankunzhang/charming ; lumen-charts : https://github.com/jagtesh/lumen-charts ;
  Vello 0.11 : https://lib.rs/crates/vello ; uPlot : https://github.com/leeoniya/uPlot
- dioxus_codemirror : https://docs.rs/dioxus_codemirror/latest/dioxus_codemirror/ ; taino-edit : https://github.com/juanma-dev/taino-edit ;
  tiptap-rs : https://github.com/dsplce-co/tiptap-rs ; Tiptap : https://github.com/ueberdosis/tiptap
- Bun fullstack (imports HTML, `Bun.serve` routes) : https://bun.com/docs/bundler/fullstack ;
  TanStack Router sans Vite sous Bun : https://gist.github.com/roderik/50c4b07a452cc0546b76d66fc957757c
- Références UX seulement (aucun code ni asset copié) : https://ui.shadcn.com/blocks , https://github.com/satnaing/shadcn-admin ,
  https://github.com/TailAdmin/free-react-tailwind-admin-dashboard , https://github.com/tremorlabs/tremor , https://github.com/refinedev/refine
- Données internes : `C:\src\yolo-ui\skills\yolo-ui-pro-max\data\charts.csv` (seuils SVG/Canvas, accessibilité des graphes),
  `stacks\dioxus.csv` (règles Dioxus 0.8), `crates\yolo-tokens\src\sheets.rs` (`scheme_css`, `tokens_css`)
