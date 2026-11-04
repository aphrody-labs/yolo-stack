# Pilier yolo-zig

## Rôle

Outillage Zig au service de Rust et Bun : `zig cc` comme compilateur / linker C portable,
builds reproductibles, aide à la compilation croisée.

## Frontière

- Fournit : wrappers `zig cc` / `zig ar`, intégration `cargo-zigbuild`, scripts de build C pour
  les shims FFI.
- Ne fournit pas : code applicatif en Zig (Rust et Bun restent les seuls langages applicatifs).
- Dépend de : `yolo-vendor` (toolchain Zig épinglée).
- Interdit : logique métier en Zig.

## Source et fusion

- Dépôt source : `C:\src\yolo-zig` (branche `main`, remote `https://github.com/yolo-rs/yolo-zig.git`).
- Commande : `just merge-pillar yolo-zig C:/src/yolo-zig`
