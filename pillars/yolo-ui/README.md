# Pilier yolo-ui

## Rôle

Framework UI de la pile YOLO : composants Rust RSX (Dioxus 0.8), rendu GPUI / WGPU, layout
Taffy 0.15 et layout analytique SDF, système colorimétrique HCT / CAM16, 7 presets de marque
(Neutral, M3, Fluent, Apple, Omarchy, Hyprland, Aphrody), moteur vectoriel Phosphor unifié,
accessibilité AccessKit, texte Parley, rendu 2D Vello.

## Frontière

- Fournit : tokens, couleur, layout, composants, rendu, icônes vectorielles, presets.
- Ne fournit pas : accès réseau, scraping, persistance, logique métier applicative.
- Dépend de : `yolo-standards` (normes), `yolo-bun` (outillage web). Aucune dépendance vers les
  applications ni vers `yolo-web`, `yolo-re`, `yolo-plugins`.
- Interdit : HTML / CSS / JS brut, polices et packs d'icônes génériques, Tailwind, shadcn, Base UI.

## Contenu actuel

- `crates/yolo-ui-color/` : sRGB <-> XYZ <-> CAM16 <-> HCT, résolution inverse itérative,
  palettes tonales 0..100, graines des 7 presets (`BrandPreset`). 21 tests unitaires.

## Source et fusion

- Dépôt source : `C:\src\yolo-ui` (branche `main`, remote `https://github.com/yolo-rs/yolo-ui.git`).
- Commande : `just merge-pillar yolo-ui C:/src/yolo-ui`
- Après fusion : les crates existantes (`yolo-tokens`, `yolo-layout`, `yolo-render`, `yolo-gpu`,
  `yolo-3d`, ...) passent sous `crates/`, les packages Bun sous `packages/`. Les dépendances de
  chemin `../yolo-ui/...` du `Cargo.toml` racine deviennent des membres du workspace.
