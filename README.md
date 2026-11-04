# yolo-stack

Monorepo unifié de la pile YOLO : **Rust** (Cargo, édition 2024) et **Bun** (1.4.4+), rien d'autre.
Node.js, npm, yarn, pnpm, Next.js, Tailwind, Base UI et shadcn sont bannis. Aucun HTML, CSS ou JS
brut : composants Rust RSX (Dioxus 0.8 / GPUI / WGPU) ou TSX typé Bun, layout analytique SDF et
Taffy 0.15, couleur HCT / CAM16 via `yolo-ui`.

## Vision

Chaque dépôt `C:\src\yolo-*` devient un **pilier** de ce monorepo, fusionné par `git subtree` dans
`pillars/<nom>/` en conservant ses frontières documentées. Les applications consommatrices vivent
dans `apps/<nom>/` ; la première est **Shenron** (dragonballfr.com), reconstruite en Dioxus + wgpu.
Un seul `Cargo.lock`, un seul `bun.lock`, une seule CI, une seule politique de dépendances.

## Arborescence

```
yolo-stack/
  Cargo.toml                 workspace Cargo (resolver 3, lints partagés, versions épinglées)
  package.json               workspace Bun (pillars/*/packages/*, apps/*/packages/*, packages/*)
  bunfig.toml  tsconfig.json rust-toolchain.toml  rustfmt.toml  clippy.toml  deny.toml
  justfile                   recettes : setup, check, test, fmt, lint, build-web, build-desktop,
                             bench, merge-pillar, pull-pillar, release
  .github/workflows/ci.yml   fmt + clippy + cargo test + bun test, matrice ubuntu / windows
  pillars/
    yolo-ui/                 framework UI (Dioxus RSX, GPUI/WGPU, Taffy, SDF, HCT, 7 presets, Phosphor)
      crates/yolo-ui-color/  crate HCT / CAM16 / palettes tonales (compile et testée)
    yolo-web/                Obscura CDP, Servo Stylo, scraping, extraction DOM
    yolo-bun/                pin Bun upstream, frontières runtime
      packages/yolo-bun-tools/  utilitaire de vérification de version Bun (testé)
    yolo-standards/          normes W3C, WebGPU, POSIX.1-2024, Linux Foundation, standards doc
    yolo-plugins/            signature Ed25519, Merkle tree, packaging Claude / Codex / Gemini / AGY
    yolo-re/                 iced-x86, triage PE / ELF / Mach-O, studio Replica
    yolo-cross/              compilation croisée, MSVC, shims FFI
    yolo-os/                 politique noyau / initramfs
    yolo-ai/  yolo-vendor/  yolo-vps/  yolo-zig/
  apps/
    shenron/                 dragonballfr.com en Dioxus + wgpu (PoC lecteur dans crates/)
    showcase-*/              vitrines du harnais de benchmark (TanStack, Dioxus, Leptos)
  crates/  packages/  bench/ harnais de benchmark des stacks (table de routes partagée, mesures,
                             compilateur de migration yolo-next)
  fusion.toml  scripts/      manifeste de la flotte yolo-* et outil `bun run scripts/fusion.ts`
  docs/
    DECISION.md BENCHMARKS.md  décision et mesures de l'agent de recherche (stack web)
    architecture/            vue d'ensemble et règles de dépendance entre piliers
    decisions/               ADR (0000 gabarit, 0001 monorepo, 0002 Dioxus, 0003+ réservés)
    standards/               conventions de code, de documentation et de revue
    research/                réservé à l'agent de recherche (benchmark des frameworks)
```

## Fusionner un pilier

Chaque pilier est importé depuis son dépôt local `C:\src\<nom>` (ou son URL GitHub) avec
historique écrasé, dans le préfixe `pillars/<nom>` :

```sh
just merge-pillar yolo-ui C:/src/yolo-ui            # git subtree add --prefix pillars/yolo-ui ... main --squash
just pull-pillar  yolo-ui C:/src/yolo-ui            # resynchronisation ultérieure
```

Après fusion : déplacer les crates sous `pillars/<nom>/crates/*` (absorbées automatiquement par le
glob du workspace), les packages Bun sous `pillars/<nom>/packages/*`, supprimer le `Cargo.lock`
et le `bun.lock` du pilier, puis `just lint && just test`. Le README du pilier décrit ses
frontières ; les violations sont des bugs.

| Pilier         | Source                 | Commande                                             |
| -------------- | ---------------------- | ---------------------------------------------------- |
| yolo-ui        | `C:\src\yolo-ui`       | `just merge-pillar yolo-ui C:/src/yolo-ui`           |
| yolo-web       | `C:\src\yolo-web`      | `just merge-pillar yolo-web C:/src/yolo-web`         |
| yolo-bun       | `C:\src\yolo-bun`      | `just merge-pillar yolo-bun C:/src/yolo-bun`         |
| yolo-standards | `C:\src\yolo-standards`| `just merge-pillar yolo-standards C:/src/yolo-standards` |
| yolo-plugins   | `C:\src\yolo-plugins`  | `just merge-pillar yolo-plugins C:/src/yolo-plugins` |
| yolo-re        | `C:\src\yolo-re`       | `just merge-pillar yolo-re C:/src/yolo-re`           |
| yolo-cross     | `C:\src\yolo-cross`    | `just merge-pillar yolo-cross C:/src/yolo-cross`     |
| yolo-os        | `C:\src\yolo-os`       | `just merge-pillar yolo-os C:/src/yolo-os`           |
| yolo-ai        | `C:\src\yolo-ai`       | `just merge-pillar yolo-ai C:/src/yolo-ai`           |
| yolo-vendor    | `C:\src\yolo-vendor`   | `just merge-pillar yolo-vendor C:/src/yolo-vendor`   |
| yolo-vps       | `C:\src\yolo-vps`      | `just merge-pillar yolo-vps C:/src/yolo-vps` (dépôt sans commit : initialiser d'abord) |
| yolo-zig       | `C:\src\yolo-zig`      | `just merge-pillar yolo-zig C:/src/yolo-zig`         |

## Matrice de toolchain

| Outil        | Version / canal                                   | Source de vérité          |
| ------------ | ------------------------------------------------- | ------------------------- |
| Rust         | `stable` (1.99.1 au 2026-11-04), MSRV 1.95        | `rust-toolchain.toml`     |
| Cibles       | `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, `wasm32-unknown-unknown` | `rust-toolchain.toml` |
| Composants   | `clippy`, `rustfmt`, `rust-src`                    | `rust-toolchain.toml`     |
| Bun          | 1.4.4 minimum (`engines.bun`, `packageManager`)   | `package.json`            |
| TypeScript   | 5.9.3, `strict`, `types: ["bun-types"]`           | `tsconfig.json`           |
| Dioxus       | 0.8.0-alpha.3                                     | `[workspace.dependencies]`|
| wgpu / Taffy | 30.0.1 / 0.15.1                                   | `[workspace.dependencies]`|
| vello / parley / accesskit | 0.11.0 / 0.12.0 / 0.25.2            | `[workspace.dependencies]`|
| tokio / axum | 1.54.1 / 0.8.9                                    | `[workspace.dependencies]`|
| ed25519-dalek / sha2 | 3.0.0 / 0.11.1                            | `[workspace.dependencies]`|
| just         | 1.58+                                             | `justfile`                |
| cargo-deny   | 0.20+ (optionnel en local, `just lint`)           | `deny.toml`               |

Les versions crates.io ont été vérifiées le 2026-11-04 avec `cargo search <crate> --limit 1`.

## Commandes

```sh
just setup            # toolchains, cibles, composants, bun install
just check            # cargo check --all-targets + tsc
just lint             # cargo fmt --check, clippy -D warnings, cargo deny (si présent)
just test             # cargo test --workspace + bun test
just fmt              # cargo fmt
just build-web        # build wasm32 des crates portables
just build-desktop    # build release natif
just bench            # criterion + harnais bench/
just release 0.1.0    # lint + test + tag annoté
```

Vérification minimale avant tout commit :

```sh
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings \
  && cargo test --workspace && bun install && bun test
```

## Règles

- Zéro emoji, zéro bavardage : les documents sont factuels, en français ; les identifiants de code
  sont en anglais.
- Pas de polices ni d'icônes génériques (Inter, Roboto, packs SVG) : système HCT / CAM16, 7 presets
  de marque (Neutral, M3, Fluent, Apple, Omarchy, Hyprland, Aphrody), moteur vectoriel Phosphor.
- Toute décision structurante passe par un ADR dans `docs/decisions/`.
- Licence : Apache-2.0 (`LICENSE-APACHE`).
