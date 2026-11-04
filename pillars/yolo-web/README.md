# Pilier yolo-web

## Rôle

Reconnaissance et extraction web haute fidélité : pilotage de navigateur Obscura via CDP,
moteur de style Servo Stylo, scraping, extraction DOM, métriques de layout, audit de trackers.

## Frontière

- Fournit : clients CDP, extraction DOM / CSSOM, mesures de layout, rapports de trackers, fixtures
  pour la rétro-conception d'interfaces.
- Ne fournit pas : rendu UI (c'est `yolo-ui`), désassemblage binaire (c'est `yolo-re`).
- Dépend de : `yolo-standards` (W3C), `yolo-bun` (runtime des scripts). Aucune dépendance vers
  `yolo-ui`.
- Interdit : Puppeteer / Playwright sur Node ; uniquement Rust ou Bun natif.

## Source et fusion

- Dépôt source : `C:\src\yolo-web` (branche `main`, remote `https://github.com/yolo-rs/yolo-web.git`).
- Commande : `just merge-pillar yolo-web C:/src/yolo-web`
