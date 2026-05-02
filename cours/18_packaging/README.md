# Chapitre 17 — Packaging, CI/CD et distribution

Publier une crate sur crates.io, configurer une CI robuste avec GitHub Actions, distribuer des binaires cross-platform, et documenter une API publique.

## 1. `Cargo.toml` complet pour une crate publiable

```toml
[package]
name = "mon-outil"
version = "1.0.0"
edition = "2024"
rust-version = "1.85"            # MSRV (Minimum Supported Rust Version)
description = "Un outil Rust exemplaire"
license = "MIT OR Apache-2.0"
repository = "https://github.com/utilisateur/mon-outil"
homepage = "https://mon-outil.rs"
documentation = "https://docs.rs/mon-outil"
keywords = ["cli", "outil", "exemple"]
categories = ["command-line-utilities"]
readme = "README.md"
exclude = [".github", "tests/fixtures/*.log"]

[dependencies]
clap = { version = "4", features = ["derive"] }

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"

[[bin]]
name = "mon-outil"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true                       # Link-Time Optimization — binaire plus petit et plus rapide
codegen-units = 1                # meilleure optimisation (compilation plus lente)
strip = true                     # supprime les symboles de debug — binaire 2-3x plus petit

[profile.dev]
opt-level = 0
debug = true

[lints.rust]
unsafe_code = "forbid"          # interdit unsafe dans ce crate
unused_imports = "warn"

[lints.clippy]
pedantic = "warn"
```

## 2. Features conditionnels

```toml
[features]
default = ["json"]
json = ["serde/derive", "serde_json"]
yaml = ["serde_yaml"]
full = ["json", "yaml"]

[dependencies]
serde = { version = "1", optional = true }
serde_json = { version = "1", optional = true }
serde_yaml = { version = "0.9", optional = true }
```

```rust
#[cfg(feature = "json")]
pub fn serialiser_json<T: serde::Serialize>(val: &T) -> String {
    serde_json::to_string(val).unwrap()
}
```

```bash
cargo build --features yaml
cargo build --all-features
cargo build --no-default-features
```

## 3. Documentation

```rust
//! # Mon Outil
//!
//! Documentation au niveau crate (dans lib.rs ou main.rs).

/// Additionne deux nombres entiers.
///
/// # Arguments
///
/// * `a` - premier nombre
/// * `b` - second nombre
///
/// # Exemples
///
/// ```
/// use mon_outil::additionner;
/// assert_eq!(additionner(2, 3), 5);
/// ```
pub fn additionner(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
cargo doc --open            # génère et ouvre dans le navigateur
cargo test --doc            # exécute les exemples dans les docstrings comme tests !
```

Les exemples dans `///` sont de vrais tests — `cargo test --doc` les exécute. Jamais d'exemple cassé si vous testez en CI.

## 4. Publier sur crates.io

```bash
# Vérification préliminaire
cargo publish --dry-run

# Authentification
cargo login <token>   # token sur https://crates.io/me

# Publication
cargo publish

# Publier une nouvelle version
# 1. Bump la version dans Cargo.toml (semver)
# 2. git tag v1.1.0
# 3. cargo publish
```

### Semver en Rust

- **Patch** (1.0.x) : bugfix sans breaking change
- **Minor** (1.x.0) : nouvelle fonctionnalité backwards-compatible
- **Major** (x.0.0) : breaking change (type supprimé, signature changée)

Cargo respecte semver : `"^1.2"` accepte 1.2.x et 1.3.x mais pas 2.0.

## 5. GitHub Actions — CI complète

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"    # toutes les warnings = erreurs

jobs:
  test:
    name: Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2     # cache des dépendances
      - run: cargo fmt --check
      - run: cargo clippy --all-targets --all-features
      - run: cargo test --all-features
      - run: cargo test --doc

  msrv:
    name: MSRV check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.85
      - run: cargo build --all-features
```

## 6. Distribution de binaires avec `cargo-dist`

`cargo-dist` génère des archives pour GitHub Releases et les installe via shell :

```bash
cargo install cargo-dist
cargo dist init          # configure .github/workflows/release.yml
```

```toml
# Cargo.toml — section ajoutée par cargo-dist
[workspace.metadata.dist]
cargo-dist-version = "0.22.1"
ci = ["github"]
installers = ["shell", "powershell"]
targets = ["aarch64-apple-darwin", "x86_64-apple-darwin",
           "x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"]
```

Au push d'un tag `v*`, la release CI génère automatiquement les binaires, les sha256, et les publie sur GitHub Releases.

## 7. Cross-compilation

```bash
rustup target add aarch64-unknown-linux-gnu
cargo install cross        # wrapper docker pour cross-compilation

cross build --release --target aarch64-unknown-linux-gnu
```

`cross` utilise des conteneurs Docker pré-configurés — pas besoin d'installer les toolchains C croisées manuellement.

## 8. `cargo-nextest` — runner de tests plus rapide

```bash
cargo install cargo-nextest
cargo nextest run           # parallèle par défaut, rapport plus lisible
cargo nextest run --retries 2  # réessaye les tests flaky
```

`nextest` est 2-3x plus rapide que `cargo test` sur les grandes suites, et son rapport est bien plus lisible.

---

### Checklist avant publication

- [ ] `cargo test --all-features` passe
- [ ] `cargo clippy -- -D warnings` passe
- [ ] `cargo fmt --check` passe
- [ ] `cargo doc` génère sans erreurs
- [ ] `cargo test --doc` passe (exemples dans les docs)
- [ ] `cargo publish --dry-run` réussit
- [ ] CHANGELOG.md mis à jour
- [ ] Version bumpée selon semver
- [ ] Git tag créé

---

## À retenir

- `[profile.release]` : `lto = true`, `strip = true` pour des binaires optimisés et compacts.
- `[features]` pour la compilation conditionnelle — `default` pour les features activées par défaut.
- Les exemples dans les docstrings sont des tests — `cargo test --doc`.
- GitHub Actions : matrix sur os/rust, `RUSTFLAGS="-D warnings"`, `Swatinem/rust-cache`.
- `cargo-dist` pour la distribution multi-plateforme automatisée.

---

➡️ [Annexes](../../annexes/A_debug_tooling.md)
