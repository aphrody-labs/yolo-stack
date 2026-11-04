# Pilier yolo-vendor

## Rôle

Dépendances tierces vendorisées et épinglées : toolchains, bibliothèques natives, assets sous
licence compatible, avec empreintes et provenance.

## Frontière

- Fournit : archives vérifiées (SHA-256), manifestes de provenance, scripts de mise à jour.
- Ne fournit pas : code YOLO original.
- Dépend de : rien. `yolo-cross`, `yolo-os`, `yolo-zig` en dépendent.
- Interdit : binaires sans empreinte ni licence documentée.

## Source et fusion

- Dépôt source : `C:\src\yolo-vendor` (branche `main`, remote `https://github.com/yolo-rs/yolo-vendor.git`).
- Commande : `just merge-pillar yolo-vendor C:/src/yolo-vendor`
