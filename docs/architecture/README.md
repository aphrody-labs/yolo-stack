# Architecture de yolo-stack

## Principe

Un monorepo, deux outils de build (Cargo et Bun), des piliers aux frontières explicites et des
applications qui ne font que composer ces piliers. Le graphe de dépendances est orienté et sans
cycle ; il est vérifié à la revue et, à terme, par un lint de workspace.

## Couches

```
apps/            Shenron, vitrines                       (composent, ne définissent pas)
pillars/ UI      yolo-ui                                  (rendu, couleur, layout, icônes)
pillars/ Web     yolo-web, yolo-ai, yolo-plugins, yolo-re (acquisition, IA, distribution, RE)
pillars/ Runtime yolo-bun, yolo-cross, yolo-zig           (runtime, compilation, FFI)
pillars/ Infra   yolo-os, yolo-vps, yolo-vendor           (noyau, serveurs, dépendances)
pillars/ Normes  yolo-standards                           (W3C, WebGPU, POSIX, LF, documentation)
```

Règles de dépendance :

1. Une couche ne dépend que des couches situées en dessous ou de `yolo-standards`.
2. `yolo-ui` ne dépend d'aucun pilier Web.
3. Les applications ne sont jamais des dépendances d'un pilier.
4. `yolo-vendor` et `yolo-standards` ne dépendent de rien.

## Build

- **Cargo** : `Cargo.toml` racine, `resolver = "3"`, membres `crates/*`, `pillars/*/crates/*`
  et crates applicatives listées explicitement. `[workspace.dependencies]` est la seule source
  de versions ; une crate n'épingle jamais une version localement. `[workspace.lints]` impose
  `clippy::pedantic` (sous-ensemble) et `unsafe_code = "forbid"` ; une crate FFI documente son
  exception dans son README.
- **Bun** : `package.json` racine, workspaces `pillars/*/packages/*`, `apps/*/packages/*`,
  `packages/*`, `bench`. `tsconfig.json` strict, `types: ["bun-types"]`. Aucun `node_modules`
  n'est commité ; `bun.lock` texte unique.
- **Profils** : `release` (LTO, 1 unité de codegen, O3) et `wasm-size` (Oz) pour les cibles web.

## Harnais de benchmark

`crates/yolo-stack-routes`, `packages/yolo-stack-routes`, `bench/` et `apps/showcase-*`
constituent le harnais qui compare les stacks candidates sur un jeu de routes identique. Ses
résultats alimentent l'ADR 0002 ; il est conservé après décision comme garde-fou de régression.

## Fusion des piliers

Voir `README.md` racine et ADR 0001 : `git subtree add --prefix pillars/<nom> <url> main --squash`.
Chaque pilier conserve son README de frontière ; la fusion ne modifie pas ses limites.

## CI

`.github/workflows/ci.yml` : `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`,
`bun install --frozen-lockfile`, `tsc`, `bun test`, sur `ubuntu-latest` et `windows-latest`, avec
`dtolnay/rust-toolchain@stable` et `oven-sh/setup-bun@v2`.
