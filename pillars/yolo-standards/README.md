# Pilier yolo-standards

## Rôle

Référentiel des normes appliquées par la pile : W3C (DOM, CSS, WAI-ARIA), WebGPU / WGSL,
POSIX.1-2024, Linux Foundation (FHS, systemd, OCI), standards de documentation (ADR, README,
conventions de commit).

## Frontière

- Fournit : textes normatifs résumés, matrices de conformité, listes de contrôle de revue,
  gabarits de documentation.
- Ne fournit pas : code exécutable autre que des vérificateurs (lint de conformité).
- Dépend de : rien. Tous les autres piliers peuvent en dépendre.

## Source et fusion

- Dépôt source : `C:\src\yolo-standards` (branche `main`, remote `https://github.com/yolo-rs/yolo-standards.git`).
- Commande : `just merge-pillar yolo-standards C:/src/yolo-standards`
- Après fusion : `docs/standards/README.md` renvoie vers ce pilier comme source unique.
