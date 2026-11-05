# ADR 0005 : Thèmes en bundles, couleurs calculées

- **Statut** : Accepté
- **Date** : 2026-11-05
- **Décideurs** : propriétaire du produit, agents yolo-stack (claude-theme-bundle) et yolo-ui (claude-theme-packs)
- **Pilier(s) concerné(s)** : yolo-ui (consommé), apps/shenron (site et admin)

## Contexte

Exigence du propriétaire : « un thème doit tout changer d'un coup, pas seulement les couleurs. Wallpapers, icônes, logo, titre, compagnon (pet) et assets suivent le thème. Les couleurs sont calculées et réglables par l'utilisateur, au lieu d'être codées en dur. »

Faits :

- Les couleurs de `apps/shenron/crates/shenron-site/src/tokens.rs` étaient deux tables de valeurs hexadécimales écrites à la main.
- Le moteur HCT/CAM16 existe déjà dans `yolo-tokens` (yolo-ui), avec 50 rôles (49 M3 plus `focus-ring`) et neuf variantes de schéma.
- yolo-ui définit déjà le format de thème complet dans `crates/yolo-theme-pack` : manifeste `theme.toml` (titres, logo, icônes, wallpapers avec licence, pet, polices OFL, forme du shell, densité, rayons, verre, mouvement), `UserPrefs`, résolution HCT vers CSS, attributs `data-*`, wallpaper SVG génératif, importeur Omarchy.
- Les presets `YoloThemePreset` (neutral, m3, fluent, apple, omarchy et ses 23 sous thèmes, hyprland, aphrody, chromium) et `ByronLovePetSpec` restent la référence de forme pour les autres produits ; Shenron n'en reprend que le principe (graine, pet, tokens), pas les valeurs.
- `shenron-admin-ai` (branche `claude-admin-ai`) calcule aujourd'hui ses propres couleurs dans un module privé.

## Décision

Nous retenons qu'un thème est un bundle au format `yolo-theme-pack`, complété côté yolo-stack par une couche fine `crates/yolo-stack-theme`, et que toute couleur affichée est calculée à partir d'une graine HCT, contrôlée par des portes de contraste, jamais stockée en hexadécimal dans un composant.

## Ce que cet ADR ne re spécifie pas

Le manifeste `theme.toml` est défini par yolo-theme-pack ; se référer à `crates/yolo-theme-pack/src/manifest.rs` dans yolo-ui. Les quatre bundles Shenron l'utilisent tel quel : `id`, `name`, `[title]`, `[logo]`, `[color]` (graine, source, variante, contraste, mode, surcharges), `[icons]` (Phosphor, style, graisse), `[[wallpapers]]`, `[pet]`, `[typography]`, `[shell]`.

## Ce que yolo-stack ajoute

Un bundle Shenron est un dossier `apps/shenron/themes/<id>/` :

| Fichier | Rôle |
| ------- | ---- |
| `theme.toml` | Manifeste yolo-theme-pack, inchangé. |
| `bundle.toml` | Extras versionnés (`schema = 1`) : logo sombre et masque monochrome, jeux de wallpapers par scheme et par point de rupture avec dimensions et couleur dominante, nom du pet par thème, sprite vectoriel, animations d'attente et comportements, sons et curseur optionnels, police d'en-tête, enregistrement de licence de chaque fichier. |
| `assets.lock.json` | `sha256`, taille et type MIME de chaque fichier, produit par `themes/scripts/gen-placeholders.ts`. |
| `brand/`, `wallpapers/`, `pet/` | Fichiers référencés. |

Règles appliquées par `ThemeBundle::validate` (toutes les erreurs sont listées d'un coup) :

1. Chaque fichier référencé existe dans `assets.lock.json`, avec un `sha256` bien formé, et possède un enregistrement `[[asset]]` avec licence vérifiée (`unverified` refusé).
2. Les chemins sont relatifs au bundle (ni `..`, ni absolu, ni URL).
3. Chaque jeu de wallpapers couvre le clair et le sombre à `min_width = 0`.
4. La graine est un `#rrggbb`, la variante est connue, le contraste est dans -1..1, les surcharges nomment un rôle existant.
5. Le pet est déclaré à la fois dans `theme.toml` et dans `bundle.toml`.

### Assets

Chaque fichier est adressé par son contenu : `{cdn}/{sha256 sur 16 caractères}.{ext}`. L'URL ne change jamais de sens, donc elle est servie avec `Cache-Control: public, max-age=31536000, immutable` (constante `IMMUTABLE_CACHE_CONTROL`) ; remplacer une illustration change l'empreinte, donc l'URL, sans purge. Le build copie les fichiers sous `public/theme-assets/` ; en production le même dossier part sur le CDN et `SHENRON_THEME_CDN` repointe les URL. Formats finaux : AVIF et WebP pour les wallpapers ; SVG pour logo, wordmark, masque et sprite. Les fichiers actuels sont des SVG générés, marqués `placeholder = true`.

### Couleurs

- Aucune couleur stockée : la graine (teinte, chroma) et la variante (`tonal-spot`, `vibrant`, `expressive`, `fidelity`, `monochrome`, ...) produisent les 50 rôles clairs et sombres par `yolo-tokens`.
- Réglages utilisateur (`UserThemePrefs`) : bundle, teinte, chroma, niveau de contraste, variante, clair, sombre ou système, mouvement réduit, pet activé, choix du wallpaper, surcharge par rôle.
- Zéro hexadécimal dans les composants : test `tests/no_hex.rs` du site.

### Portes de contraste

Appliquées à chaque `compute`, dans les deux schemes :

- Texte : 4,5:1. Interface (contours, anneau de focus, icônes) : 3:1.
- Une paire qui échoue voit son avant plan déplacé sur l'axe du ton (teinte et chroma conservées) jusqu'au seuil, mesuré en sRGB 8 bits.
- Une surcharge utilisateur est recontrôlée. Si un déplacement de ton suffit, elle est ajustée et signalée ; si aucun ton ne la rend compatible (une surface forcée à une couleur extrême), elle est refusée (`OverrideOutcome.rejected`) et la valeur calculée est conservée.
- Le build du site échoue si un bundle ne passe pas ; `style/contrast-audit.txt` liste chaque ratio.
- Un test de propriétés (teinte, chroma, contraste, variante, mode, surcharge aléatoires) vérifie que chaque paire passe.

### Bundle de repli

`ThemeBundle::fallback()` : graine neutre, wallpaper génératif, aucun asset externe. `Registry::get_or_default` retombe sur le bundle par défaut (`shenron`) pour un identifiant inconnu.

### Émetteurs

`emit::css` : variables `--yolo-<rôle>` sous `:root[data-theme="<id>"]`, mode sombre via `prefers-color-scheme` et `data-scheme`, `forced-colors: active` vers les couleurs système, `prefers-reduced-motion` et `data-motion="reduce"`. `emit::dtcg` : JSON DTCG. `ResolvedBundle::native` : structure typée pour Dioxus, wgpu et vello.

## Application au site Shenron

- `build.rs` de `shenron-site` charge les bundles, calcule, vérifie les portes, génère `style/main.css` et copie les assets sous empreinte. `tokens.rs` est supprimé.
- Un script en tête de `<head>` lit cookie puis localStorage et pose `data-theme`, `data-scheme`, `data-motion` et la feuille personnalisée avant le premier rendu.
- Le site passe en mode îlots Leptos : `ThemeRuntime`, `ThemeBrand`, `ThemeSwitcher`, `ThemePet` et `ThemePanel` sont les seuls composants hydratés. Le pet est masqué par le mouvement réduit du système, par la préférence utilisateur et par la préférence « pet » ; il est aussi masqué en CSS sans script.
- Route `/{lang}/theme` : réglages (graine, chroma, contraste, variante, surcharges par rôle) avec aperçu vivant et audit de contraste.
- Les règles d'accessibilité (`:focus-visible`, `forced-colors`, `prefers-reduced-motion`) vivent dans `src/structure.rs`. La branche `claude-site-a11y` modifie la même zone : à la fusion, garder les règles de cette branche dans `structure.rs`.

## Dépendance à yolo-ui

Comme pour `yolo-tokens`, `yolo-theme-pack` (v1.1, rôle `focus-ring`, 50 rôles, `pet.files`, licences `unverified` et `pending-*` non distribuables) est une dépendance par chemin `../yolo-ui/crates/yolo-theme-pack` (workspace racine). Le crate est sur le `main` de yolo-ui (989690c) : aucun checkout séparé de la branche `claude-theme-packs` n'est plus nécessaire sur vps. Les licences non distribuables sont rejetées par `is_distributable_license` à la validation du bundle.
