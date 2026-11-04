# Standards de yolo-stack

Source normative : pilier `yolo-standards` (`pillars/yolo-standards/`). Ce document résume les
règles appliquées dans le monorepo ; en cas de divergence, le pilier fait foi.

## Langages et outils

- Rust édition 2024, canal `stable` (`rust-toolchain.toml`), MSRV 1.95. `unsafe_code = "forbid"`
  sauf crate FFI documentée. `clippy::pedantic` (sous-ensemble défini dans `Cargo.toml`) sans
  avertissement.
- Bun 1.4.4+, TypeScript 5.9 `strict`, `types: ["bun-types"]`, `verbatimModuleSyntax`. Aucun
  Node.js, npm, yarn, pnpm.
- Formatage : `rustfmt.toml` (largeur 100, style 2024), `.editorconfig` (LF, UTF-8, 2 espaces,
  4 pour Rust).

## Normes externes

| Domaine        | Référence                                  | Application                                 |
| -------------- | ------------------------------------------ | ------------------------------------------- |
| Web            | W3C DOM, CSSOM, WAI-ARIA 1.2, WCAG 2.2     | `yolo-web` (extraction), `yolo-ui` (a11y)    |
| GPU            | WebGPU, WGSL                               | `yolo-ui` (wgpu 30), Shenron                 |
| Système        | POSIX.1-2024                               | `yolo-os`, `yolo-cross`, scripts Bun         |
| Linux          | Linux Foundation : FHS 3.0, OCI, systemd   | `yolo-os`, `yolo-vps`                        |
| Couleur        | CIE 1931 XYZ, CAM16 (CIE 248:2022), HCT    | `yolo-ui-color`                              |
| Cryptographie  | RFC 8032 (Ed25519), FIPS 180-4 (SHA-2)     | `yolo-plugins`                               |

## Documentation

- Langue : français pour les documents, anglais pour les identifiants de code et les messages de
  commit (`type: description`, Conventional Commits).
- Zéro emoji. Pas de formules de politesse ni de remplissage : faits, chemins, commandes, statuts.
- Chaque pilier a un `README.md` avec les sections Rôle, Frontière, Source et fusion.
- Toute décision structurante est un ADR dans `docs/decisions/` suivant `0000-template.md`.
- `docs/research/` et `docs/decisions/0003-*` sont réservés à l'agent de recherche.

## Design

- Polices et icônes génériques interdites (Inter, Roboto, packs SVG). Système HCT / CAM16, 7
  presets de marque (Neutral, M3, Fluent, Apple, Omarchy, Hyprland, Aphrody), moteur vectoriel
  Phosphor unifié.
- Layout analytique SDF et Taffy 0.15 ; aucun HTML / CSS / JS brut.

## Revue et CI

- Avant commit : `just lint && just test`.
- CI : `.github/workflows/ci.yml` (ubuntu, windows). Un avertissement clippy est une erreur.
- Dépendances : `deny.toml` (licences autorisées, sources, bannissements Node / OpenSSL).
