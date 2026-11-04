# Pilier yolo-bun

## Rôle

Pin de la version Bun upstream (1.4.4+) et définition des frontières runtime : ce qui peut être
écrit en TypeScript typé Bun, ce qui doit rester en Rust, conventions d'interopérabilité (Bun FFI,
`Bun.build`, `Bun.serve`, `bun test`).

## Frontière

- Fournit : outils de vérification de version, scripts de build et de test Bun, règles de
  compatibilité, gabarits `package.json` / `tsconfig.json`.
- Ne fournit pas : composants UI, logique métier.
- Dépend de : rien dans le monorepo.
- Interdit : Node.js, npm, yarn, pnpm, polyfills Node, `node:` modules hors compatibilité Bun
  documentée.

## Contenu actuel

- `packages/yolo-bun-tools/` : `parseVersion`, `compareVersions`, `bunVersion`,
  `satisfiesMinimum`, `assertBunVersion` (basé sur `Bun.version`). 8 tests `bun test`.

## Source et fusion

- Dépôt source : `C:\src\yolo-bun` (branche `main`, remote `https://github.com/yolo-rs/yolo-bun.git`).
- Commande : `just merge-pillar yolo-bun C:/src/yolo-bun`
