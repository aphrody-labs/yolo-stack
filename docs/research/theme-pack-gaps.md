# Écarts de yolo-theme-pack constatés par yolo-stack

Constat fait en intégrant `yolo-theme-pack` (yolo-ui, branche `claude-theme-packs`, 5fdee4a13) dans `crates/yolo-stack-theme`. Chaque ligne indique le contournement actuel côté yolo-stack et ce qui peut être repris par yolo-ui.

| # | Écart | Contournement yolo-stack | Proposition pour yolo-ui |
| - | ----- | ------------------------ | ------------------------ |
| 1 | Surcharges par rôle : `resolve` applique les surcharges et signale `contrast_warnings` ou ajuste les rôles `on-*` avec `clamp_text`, mais ne couvre pas les paires UI (3:1) ni les fonds surchargés, et ne peut pas refuser une surcharge | `gate.rs` : paires texte 4,5:1 et UI 3:1, déplacement de ton de l'avant plan, rejet d'une surcharge irréconciliable (`OverrideOutcome.rejected`) | Reprendre `gate::PAIRS` et la boucle de réparation dans `resolve`, exposer `rejected_overrides` |
| 2 | Sons : aucun champ | `bundle.toml` : `[sound.events]` événement vers fichier | Ajouter `[sounds]` au manifeste, avec licence par fichier |
| 3 | Police d'en-tête : `typography` n'a que display, body, mono | `bundle.toml` : `header_font` | Ajouter `typography.header` (optionnel, défaut display) |
| 4 | Wallpapers : pas de variantes par point de rupture, ni de dimensions, ni de couleur dominante par fichier | `bundle.toml` : `[[wallpaper]]` avec `[[wallpaper.variant]]` (scheme, `min_width`, `width`, `height`, `dominant`) | Étendre `Wallpaper` avec `variants` ; `seed` existe déjà pour la dominante |
| 5 | Empreintes d'assets : `Wallpaper.sha256` seulement | `assets.lock.json` (sha256, taille, MIME de tout fichier) et URL `{cdn}/{sha16}.{ext}` | Généraliser `sha256` à logo, pet, sons ; ajouter une aide de chemin adressé par contenu |
| 6 | Licence par fichier : seulement pour les wallpapers | `bundle.toml` : `[[asset]]` (licence, source, attribution, `placeholder`) | Licence et source sur logo, pet, sons ; drapeau `placeholder` refusé en distribution |
| 7 | Pet : `asset_dir` suppose `pets/<provider>/<id>/<state>.webp ou .json` ; pas de sprite vectoriel, de nom par thème, d'animations d'attente ni de comportements | `bundle.toml` : `[pet]` nom, `sprite` SVG, `idle`, `behaviors` | Autoriser `.svg` par état, ajouter `name` et `behaviors` |
| 8 | Réglage HCT par l'utilisateur : `UserPrefs.seed` est un hexadécimal, pas une paire teinte et chroma | `UserThemePrefs.hue` et `chroma` convertis en graine par `Hct::new(hue, chroma, 50)` | Ajouter `seed_hue` et `seed_chroma` à `UserPrefs` |
| 9 | Variantes : aucune liste recommandée pour un sélecteur (neuf noms dans `Variant::ALL`) | Liste de cinq noms dans le panneau | Exposer une liste « sélecteur » documentée |
| 10 | Émetteurs : `ResolvedTheme::css` ne produit qu'un schéma à la fois, sans `data-theme`, `data-scheme`, `forced-colors` ni wallpaper responsive | `emit::css` : deux schemes, `prefers-color-scheme`, `forced-colors`, `data-motion`, variables de wallpaper par point de rupture | Reprendre le format de sélecteurs `:root[data-theme]` |
| 11 | DTCG et typé natif : absents | `emit::dtcg`, `ResolvedBundle::native` | Reprendre ou référencer |
| 12 | Titre : `Titles` n'a pas de champ pour le slogan ou la description | Textes du site via i18n | Optionnel |

Priorité suggérée pour yolo-ui : 1, 7, 4, 5, 6, puis 2, 3, 8.
