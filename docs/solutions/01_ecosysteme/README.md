# Solutions — Chapitre 1

## 1.1 — Diagnostic

Pas de "bonne" réponse absolue. En 2026, stable devrait être **1.85+** pour l'édition 2024. Si votre `rustc` est < 1.85, faites `rustup update`.

`rustup component list --installed` doit afficher au minimum :
- `rustc-*`
- `cargo-*`
- `rustfmt-*`
- `clippy-*`

Si l'un manque : `rustup component add rustfmt clippy`.

## 1.2 — Premier projet

```rust
const VERSION: &str = "1.85";

fn main() {
    println!("Bonjour Rust {VERSION} !");
}
```

Notez la **interpolation dans la macro `println!`** (`{VERSION}` au lieu de `{}, VERSION`). Depuis Rust 1.58, on peut capturer directement des variables dans la string de format.

## 1.3 — check vs build

`cargo check` saute la phase de code generation (génération du binaire). Seul le type-checking est fait. Typiquement **3 à 5× plus rapide**.

L'erreur `let x: i32 = "hello";` est attrapée par le type-checker, donc `check` la remonte aussi — rendant `check` suffisant pour vérifier la compilation pendant le dev.

## 1.4 — Ajouter une dépendance

```toml
[dependencies]
rand = "0.8"
```

`rand` elle-même tire des dépendances transitives (`getrandom`, `rand_core`, `rand_chacha`, `ppv-lite86`, selon la version). Le `Cargo.lock` contient en général 5-10 entrées après `cargo add rand`. C'est normal.

## 1.5 — `rust-toolchain.toml`

```toml
[toolchain]
channel = "1.85.0"
components = ["rustfmt", "clippy"]
```

Si 1.85 n'est pas installé, rustup l'installe automatiquement à la prochaine invocation de `cargo`. Utile pour garantir la même version partout.

## 1.6 — debug vs release

Typique : debug ~2-4s pour 100M itérations, release ~50-100ms. **Facteur 20-40×**.

Causes :
- Debug : aucune optimisation (`-O0`), overflow checks activés.
- Release : `-O3`, LLVM vectorize/unroll, overflow checks désactivés.

**Toujours benchmarker en release** — jamais tirer de conclusion sur la perf depuis un build debug.

## 1.7 — cargo tree

Exemple de sortie (varie selon la version de rand) :

```
hello_rust v0.1.0 (/Users/…/hello_rust)
└── rand v0.8.5
    ├── libc v0.2.155
    ├── rand_chacha v0.3.1
    │   ├── ppv-lite86 v0.2.17
    │   └── rand_core v0.6.4
    │       └── getrandom v0.2.15
    │           ├── cfg-if v1.0.0
    │           └── libc v0.2.155
    └── rand_core v0.6.4 (*)
```

Environ 6 crates transitives pour juste tirer un nombre aléatoire. Rust n'a pas de random dans sa stdlib — vous payez ce coût.

## 1.8 — cargo doc

`cargo doc --open` génère un site HTML statique dans `target/doc/` et l'ouvre. Vous voyez **votre crate** plus **toutes les dépendances**.

Pour un commentaire :

```rust
/// Retourne un salut personnalisé.
pub fn saluer(nom: &str) -> String {
    format!("Bonjour {nom}")
}
```

Les `///` sont des **doc comments** (Markdown). `cargo doc` les parse et en fait une API documentation complète.

## 1.9 — Format et lint

`cargo fmt` applique `rustfmt` — reformate en place selon les conventions officielles. Aucune option pour débattre : la convention est celle que `rustfmt` applique.

`cargo clippy` sort typiquement des warnings du style :

```
warning: redundant closure
   --> src/main.rs:5:30
    |
5   |     let doubled: Vec<_> = v.iter().map(|x| f(x)).collect();
    |                                        ^^^^^^^^ help: try: `f`
```

Config recommandée dans `Cargo.toml` pour transformer les warnings en erreurs en CI :

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
```

Ou via `rust-toolchain.toml` pour tous les projets.
