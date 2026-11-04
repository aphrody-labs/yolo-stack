# Pilier yolo-vps

## Rôle

Provisioning et exploitation des serveurs : images, déploiement, observabilité, sauvegardes,
politiques réseau pour les services YOLO (dont Shenron).

## Frontière

- Fournit : définitions d'infrastructure, scripts de déploiement (Bun / Rust), runbooks.
- Ne fournit pas : politique noyau (c'est `yolo-os`), code applicatif.
- Dépend de : `yolo-os`, `yolo-bun`, `yolo-standards`.
- Interdit : secrets en clair ; utiliser des références vers un coffre.

## Source et fusion

- Dépôt source : `C:\src\yolo-vps` (remote `https://github.com/yolo-rs/yolo-vps.git`).
- État au 2026-11-04 : dépôt local sans commit. Créer un premier commit sur `main` avant la fusion.
- Commande : `just merge-pillar yolo-vps C:/src/yolo-vps`
