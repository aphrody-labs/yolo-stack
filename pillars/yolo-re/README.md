# Pilier yolo-re

## Rôle

Rétro-ingénierie et décomposition : désassemblage iced-x86, triage multi-format (PE, ELF, Mach-O),
extraction AST, modélisation structurelle avec le studio Replica.

## Frontière

- Fournit : parseurs de formats binaires, désassembleur, extracteurs de symboles et de chaînes,
  modèles structurels, interface du studio Replica (rendue avec `yolo-ui`).
- Ne fournit pas : pilotage de navigateur (c'est `yolo-web`), compilation croisée (c'est `yolo-cross`).
- Dépend de : `yolo-ui` (studio uniquement), `yolo-standards`.
- Interdit : exécution de binaires analysés hors bac à sable.

## Source et fusion

- Dépôt source : `C:\src\yolo-re` (branche `main`, remote `https://github.com/yolo-rs/yolo-re.git`).
- Commande : `just merge-pillar yolo-re C:/src/yolo-re`
