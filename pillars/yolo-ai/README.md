# Pilier yolo-ai

## Rôle

Intégrations IA : clients de modèles, orchestration d'agents, protocole MCP, évaluation et
traçabilité des réponses.

## Frontière

- Fournit : clients typés (Rust et Bun), schémas d'outils, harnais d'évaluation.
- Ne fournit pas : interface utilisateur (c'est `yolo-ui`), packaging des plugins (c'est
  `yolo-plugins`).
- Dépend de : `yolo-bun`, `yolo-standards`.
- Interdit : clés d'API dans le dépôt, SDK Node.

## Source et fusion

- Dépôt source : `C:\src\yolo-ai` (branche `main`, remote `https://github.com/yolo-rs/yolo-ai.git`).
- Commande : `just merge-pillar yolo-ai C:/src/yolo-ai`
