# ADR 0001 : Monorepo yolo-stack et fusion par git subtree

- **Statut** : Accepté
- **Date** : 2026-11-04
- **Décideurs** : aphrody-dev, agents de scaffolding
- **Pilier(s) concerné(s)** : tous

## Contexte

Douze dépôts `C:\src\yolo-*` évoluent séparément avec des toolchains, des lockfiles et des CI
distincts. Les applications (Shenron) doivent consommer plusieurs piliers à la fois ; les
dépendances de chemin entre dépôts voisins (`../yolo-ui/...`) ne sont ni reproductibles ni
vérifiables en CI. La doctrine impose Rust + Bun exclusivement et des frontières de pilier
documentées.

## Décision

Nous retenons un monorepo unique `yolo-stack` (workspace Cargo `resolver = "3"` + workspace Bun)
dans lequel chaque dépôt existant est fusionné comme pilier `pillars/<nom>/` par
`git subtree add --prefix pillars/<nom> <url> main --squash`, et où les applications vivent dans
`apps/<nom>/`.

## Options considérées

| Option                      | Avantages                                                          | Inconvénients                                                            |
| --------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------ |
| Dépôts séparés + path deps  | Aucun travail de migration                                         | Non reproductible, CI fragmentée, versions divergentes                   |
| Submodules git              | Historique intact, dépôts indépendants                             | Pointeurs fragiles, double commit, mauvaise expérience `bun`/`cargo`     |
| Subtree avec `--squash`     | Un seul arbre, un seul lock, `pull-pillar` possible, pas d'outillage | Historique écrasé (conservé dans le dépôt source), gros premier commit |
| Subtree sans `--squash`     | Historique complet                                                 | 7000+ commits de yolo-ui et 2000+ de yolo-cross pollueraient `main`      |

## Conséquences

- Positives : une version par dépendance (`[workspace.dependencies]`), une CI, un `just lint`
  pour tout ; les frontières de piliers deviennent vérifiables dans un seul graphe.
- Négatives : premier commit volumineux par pilier ; l'historique détaillé reste dans les dépôts
  sources, qui passent en lecture seule après fusion.
- Travaux induits : par pilier, déplacer les crates sous `crates/` et les packages sous
  `packages/`, supprimer les lockfiles locaux, aligner les versions sur la racine.

## Vérification

```sh
just merge-pillar <nom> C:/src/<nom>
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace && bun install && bun test
```

## Références

- `README.md` racine (tableau de fusion), `justfile` (`merge-pillar`, `pull-pillar`).
- `docs/architecture/README.md` (règles de dépendance).
