# yolo-stack : recettes de developpement (Rust + Bun uniquement).

set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Installe les toolchains et dependances (rustup, composants, Bun).
setup:
    rustup show active-toolchain
    rustup target add wasm32-unknown-unknown x86_64-pc-windows-msvc x86_64-unknown-linux-gnu
    rustup component add clippy rustfmt rust-src
    bun install
    @echo "setup: OK"

# Verification rapide (sans build complet).
check:
    cargo check --workspace --all-targets
    bun run typecheck

# Tests Rust + Bun.
test:
    cargo test --workspace
    bun test

# Formatage (ecriture).
fmt:
    cargo fmt

# Formatage (verification) + clippy strict + cargo-deny si present.
lint:
    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    if command -v cargo-deny >/dev/null 2>&1; then cargo deny check; else echo "cargo-deny absent : etape ignoree"; fi

# Build web (wasm32) des crates portables.
build-web:
    cargo build -p yolo-ui-color --target wasm32-unknown-unknown --release
    @echo "build-web: apps/shenron rejoindra cette recette apres le benchmark (ADR 0002)"

# Build desktop (hote natif).
build-desktop:
    cargo build --workspace --release

# Benchmarks (criterion + harnais Bun).
bench:
    cargo bench --workspace --no-run
    if [ -f bench/bench.ts ]; then bun run bench/bench.ts; else echo "aucun bench bun"; fi

# Fusionne un depot pilier existant dans pillars/NAME via git subtree (historique ecrase).
# Exemple : just merge-pillar yolo-ui C:/src/yolo-ui
merge-pillar NAME URL BRANCH="main":
    test ! -e "pillars/{{NAME}}/.git" || { echo "pillars/{{NAME}} est deja un depot"; exit 1; }
    git subtree add --prefix "pillars/{{NAME}}" "{{URL}}" "{{BRANCH}}" --squash

# Synchronise un pilier deja fusionne avec son depot source.
pull-pillar NAME URL BRANCH="main":
    git subtree pull --prefix "pillars/{{NAME}}" "{{URL}}" "{{BRANCH}}" --squash

# Release : verifications completes puis tag annote.
release VERSION:
    just lint
    just test
    git tag -a "v{{VERSION}}" -m "yolo-stack v{{VERSION}}"
    @echo "Tag v{{VERSION}} cree. Pousser avec : git push origin v{{VERSION}}"
