# ADR 0002 : Dioxus retenu comme framework UI

- **Statut** : Proposé, en attente du benchmark
- **Date** : 2026-11-04
- **Décideurs** : aphrody-dev, agent de recherche (benchmark), agent de scaffolding
- **Pilier(s) concerné(s)** : yolo-ui, apps/shenron

## Contexte

La doctrine impose des composants Rust RSX sans HTML / CSS / JS brut, un rendu GPU (wgpu 30),
un layout Taffy 0.15 et une cible double web (wasm32) et desktop. Trois candidats sont en lice
dans le harnais de benchmark (`bench/`, `apps/showcase-*`, `crates/yolo-stack-routes`) :

- **Dioxus 0.8** (0.8.0-alpha.3 sur crates.io ; fork vendorisé dans `C:\src\yolo-ui\packages\dioxus`) :
  RSX, fullstack, desktop natif, déjà intégré à `yolo-ui`.
- **Leptos 0.8.23** : signaux fins, ilots wasm réputés plus compacts, pas de cible desktop native.
- **TanStack sur Bun** : coquille TypeScript, exclue comme solution principale par la règle
  « zéro JS brut », conservée uniquement comme référence de mesure.

Un PoC `apps/shenron/crates/shenron-reader-poc` (Dioxus 0.7 + wgpu + Taffy) existe pour valider
l'intégration d'une surface wgpu dans une fenêtre Dioxus.

## Décision (provisoire)

Nous retenons Dioxus 0.8 comme framework UI de `yolo-ui` et de Shenron, sous réserve que le
benchmark confirme les critères ci-dessous. L'agent de recherche finalise cet ADR (statut
`Accepté` ou `Rejeté`, résultats chiffrés dans `docs/research/`).

## Critères d'acceptation du benchmark

| Critère                                  | Seuil                                              |
| ---------------------------------------- | -------------------------------------------------- |
| Taille wasm gzip de la route `island-counter` | Dioxus <= 1,5 x Leptos                        |
| TTI de `table-1k` (1000 lignes SSR + hydratation) | <= 150 ms sur la machine de référence     |
| Rendu desktop natif avec surface wgpu    | Fonctionnel sur Windows MSVC et Linux GNU           |
| Intégration Taffy 0.15 et HCT (`yolo-ui-color`) | Sans couche HTML/CSS intermédiaire           |
| Maturité 0.8                             | Alpha acceptable si `yolo-ui` maintient le fork    |

## Options considérées

| Option   | Avantages                                                     | Inconvénients                                           |
| -------- | ------------------------------------------------------------- | ------------------------------------------------------- |
| Dioxus   | RSX, desktop + web + fullstack, déjà dans `yolo-ui`           | 0.8 en alpha ; wasm plus lourd que Leptos               |
| Leptos   | Wasm compact, réactivité fine                                 | Pas de desktop natif ; doublon avec `yolo-ui`           |
| GPUI seul | Performance native maximale                                  | Pas de cible web ; écosystème restreint                 |
| TanStack | Outillage web mature                                          | JS brut : contraire à la doctrine                       |

## Conséquences

- Positives : un seul modèle de composant pour web et desktop ; réutilisation directe des crates
  `yolo-ui`.
- Négatives : dépendance à une version alpha ; surveillance de la taille wasm obligatoire
  (`profile.wasm-size`).
- Travaux induits : aligner `shenron-reader-poc` sur Dioxus 0.8 et le réintégrer au workspace ;
  conserver Leptos derrière un feature flag tant que le critère de taille n'est pas tranché.

## Vérification

```sh
just bench
cargo build -p yolo-ui-color --target wasm32-unknown-unknown --release
```

## Références

- `docs/BENCHMARKS.md`, `docs/receipts/`, `docs/research/` (agent de recherche).
- `crates/README.md`, `apps/README.md` (harnais).
