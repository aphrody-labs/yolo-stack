# repos/

Mount point for the YOLO fleet. Each entry is declared in `../fusion.toml`.

- `bun fusion link` creates a directory junction (Windows) or symlink (POSIX) per repository pointing at its local sibling checkout under `C:\src`. Links are gitignored.
- `bun fusion submodule` replaces a link by a pinned git submodule.
- `bun fusion subtree` merges the repository history into this monorepo under the same path (`git subtree add --prefix repos/<name>`), which is the final fusion step.

Cargo and Bun paths never change between the three modes, so `crates/*` and `packages/*` keep building before, during, and after fusion.
