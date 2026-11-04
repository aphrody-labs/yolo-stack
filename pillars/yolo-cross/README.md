# Pilier yolo-cross

## Rôle

Compilation croisée et interopérabilité : cibles MSVC, Linux GNU / musl, wasm32, shims FFI,
images et scripts de build reproductibles.

## Frontière

- Fournit : configurations de cibles, wrappers de linkers, shims FFI (C ABI), recettes de build.
- Ne fournit pas : code applicatif, politique noyau (c'est `yolo-os`).
- Dépend de : `yolo-zig` (cc / linker zig quand pertinent), `yolo-vendor`.
- Interdit : toolchains téléchargées hors `yolo-vendor`.

## Source et fusion

- Dépôt source : `C:\src\yolo-cross` (branche `main`, remote `https://github.com/cross-rs/cross.git`,
  fork local de cross-rs/cross).
- Commande : `just merge-pillar yolo-cross C:/src/yolo-cross`
- Note : historique volumineux (2000+ commits) ; `--squash` est obligatoire.
