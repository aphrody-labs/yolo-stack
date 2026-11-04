# Pilier yolo-os

## Rôle

Politique noyau et initramfs : configuration du noyau Linux, composition de l'initramfs,
contraintes de démarrage et de sécurité pour les cibles YOLO (postes, VPS, machines de build).

## Frontière

- Fournit : fragments de configuration noyau, générateurs d'initramfs, politiques (modules,
  cgroups, namespaces), documentation de conformité POSIX.1-2024 / Linux Foundation.
- Ne fournit pas : provisioning distant (c'est `yolo-vps`), compilation croisée (c'est `yolo-cross`).
- Dépend de : `yolo-standards`, `yolo-cross`.

## Source et fusion

- Dépôt source : `C:\src\yolo-os` (branche `main`, remote `https://github.com/yolo-rs/yolo-os.git`).
- Commande : `just merge-pillar yolo-os C:/src/yolo-os`
