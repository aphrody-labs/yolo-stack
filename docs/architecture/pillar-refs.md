# Références de fusion des piliers (branches auditées, 2026-11-05)

| Pilier | Dépôt | Branches auditées | Commande |
|---|---|---|---|
| yolo-ui | https://github.com/yolo-rs/yolo-ui.git | main uniquement (secrets dans le working tree, codex/claude non poussées) | `just merge-pillar yolo-ui https://github.com/yolo-rs/yolo-ui.git main` |
| yolo-web | https://github.com/yolo-rs/yolo-web.git | main uniquement | `just merge-pillar yolo-web https://github.com/yolo-rs/yolo-web.git main` |
| yolo-bun | https://github.com/yolo-rs/yolo-bun.git | main, codex, claude | `just merge-pillar yolo-bun https://github.com/yolo-rs/yolo-bun.git main` |
| yolo-standards | https://github.com/yolo-rs/yolo-standards.git | main, codex, claude | idem |
| yolo-plugins | https://github.com/yolo-rs/yolo-plugins.git | main, codex, claude ; purge des clés Ed25519 préalable | idem après purge |
| yolo-re | https://github.com/yolo-rs/yolo-re.git | main, codex, claude | idem |
| yolo-cross | https://github.com/yolo-rs/yolo-cross.git | main, codex, claude | idem |
| yolo-os | https://github.com/yolo-rs/yolo-os.git | main, codex, claude, claude-fleet (intégration flotte) | idem |
| yolo-ai | https://github.com/yolo-rs/yolo-ai.git | main, codex, claude | idem |
| yolo-vendor | https://github.com/yolo-rs/yolo-vendor.git | main, codex, claude | idem |
| yolo-vps | https://github.com/yolo-rs/yolo-vps.git | main, codex, claude | idem |
| yolo-zig | https://github.com/yolo-rs/yolo-zig.git | main, codex, claude | idem |

Règles : `gitleaks detect --source pillars/<nom>` bloquant avant commit de fusion ; les dépôts frères
`C:\src\yolo-*` restent intacts ; une tranche de travail se nomme `claude-<tranche>`.
