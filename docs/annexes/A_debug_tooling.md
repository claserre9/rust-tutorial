# Annexe A — Debug, profiling et outils de diagnostic

## Débogage

### `dbg!` — le couteau suisse

```rust
let x = dbg!(2 * 3 + 4);   // affiche "[src/main.rs:1] 2 * 3 + 4 = 10"
                             // et retourne 10 — passthrough
let v = vec![1, 2, 3];
dbg!(&v);
```

### `eprintln!` — stderr sans alloc

```rust
eprintln!("DEBUG: état = {:?}", état);
```

### `RUST_LOG` et `tracing` / `log`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```rust
use tracing::{info, warn, error, debug};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("mon_crate=debug,warn")
        .init();

    info!(utilisateur = "alice", "connexion réussie");
    warn!("taux d'erreur élevé : {:.1}%", 15.3);
    debug!(?données, "données reçues");   // ? → format Debug
}
```

```bash
RUST_LOG=debug cargo run
RUST_LOG=mon_crate::module=trace cargo run
```

## Profiling

### `cargo flamegraph`

```bash
cargo install flamegraph
cargo flamegraph --bin mon-binaire   # génère flamegraph.svg
```

Nécessite `perf` sur Linux ou `dtrace` sur macOS. Le flamegraph SVG est interactif.

### `cargo-profdata` + LLVM instruments

```bash
# macOS
CARGO_INCREMENTAL=0 RUSTFLAGS="-C instrument-coverage" cargo build --release
./target/release/mon-binaire
llvm-profdata merge -sparse *.profraw -o default.profdata
llvm-cov report --use-color
```

### `criterion` avec flamegraph

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
pprof = { version = "0.13", features = ["criterion", "flamegraph"] }
```

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn bench(c: &mut Criterion) {
    c.bench_function("ma_fn", |b| b.iter(|| ma_fn()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);
```

## Détection de bugs mémoire

### Miri — interpréteur pour détecter l'UB

```bash
rustup +nightly component add miri
cargo +nightly miri test       # exécute les tests sous Miri
cargo +nightly miri run        # exécute le binaire sous Miri
```

Détecte :
- Déréférencement de pointeur invalide
- Use-after-free
- Data races
- Alignement incorrect
- Accès hors bornes dans unsafe

### AddressSanitizer

```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test
```

### Valgrind (Linux)

```bash
cargo build
valgrind --leak-check=full ./target/debug/mon-binaire
```

## Analyse statique

### `cargo clippy` — linter

```bash
cargo clippy -- -D warnings          # warnings = erreurs
cargo clippy --all-targets --all-features
cargo clippy --fix                    # applique les suggestions automatiquement
```

Quelques lints utiles activables :

```toml
[lints.clippy]
pedantic = "warn"
nursery = "warn"
cargo = "warn"
```

### `cargo audit` — vulnérabilités

```bash
cargo install cargo-audit
cargo audit                           # vérifie les CVE dans les dépendances
cargo audit fix                       # tente de mettre à jour automatiquement
```

### `cargo deny` — politique de dépendances

```bash
cargo install cargo-deny
cargo deny check                      # vérifie licences, duplicates, advisories
```

```toml
# deny.toml
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]

[bans]
multiple-versions = "warn"
```

## Outils de productivité

### `cargo watch`

```bash
cargo install cargo-watch
cargo watch -x check             # relance cargo check à chaque changement
cargo watch -x "test -- --nocapture"
```

### `cargo expand` — voir les macros expansées

```bash
cargo install cargo-expand
cargo expand                     # expand tout
cargo expand mon_module::ma_fn   # expand spécifiquement
```

### `cargo tree` — graphe de dépendances

```bash
cargo tree                       # tout l'arbre
cargo tree --duplicates          # packages en plusieurs versions
cargo tree -i serde              # qui dépend de serde ?
cargo tree --features all        # avec toutes les features
```

### `rust-analyzer` — LSP

Disponible dans VSCode (`rust-analyzer` extension), Neovim (nvim-lspconfig), IntelliJ IDEA (plugin Rust).

Fonctionnalités clés :
- Complétion contextuelle
- Inlay hints (types inférés)
- `Go to definition` sur tout
- Refactoring : rename, inline variable
- Actions rapides : impl trait, add match arm
