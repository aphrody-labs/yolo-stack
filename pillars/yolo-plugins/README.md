# Pilier yolo-plugins

## Rôle

Distribution sécurisée des plugins : signature Ed25519 native (`ed25519-dalek` 3), arbre de Merkle
(`sha2` 0.11) pour l'intégrité des bundles, packaging universel compatible Claude, Codex,
Gemini Web et AGY CLI.

## Frontière

- Fournit : format de manifeste, signature / vérification, construction et vérification de
  Merkle tree, générateurs de bundles par cible.
- Ne fournit pas : contenu des plugins (skills, agents), hébergement.
- Dépend de : `yolo-bun` (scripts de packaging), `yolo-standards`.
- Interdit : OpenSSL (rustls uniquement), clés en clair dans le dépôt.

## Source et fusion

- Dépôt source : `C:\src\yolo-plugins` (branche `main`, remote `https://github.com/yolo-rs/yolo-plugins.git`).
- Commande : `just merge-pillar yolo-plugins C:/src/yolo-plugins`
