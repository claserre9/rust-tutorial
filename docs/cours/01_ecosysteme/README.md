# Chapitre 1 — Écosystème Rust moderne

Avant d'écrire une ligne de Rust, maîtrisez l'outillage. Contrairement à beaucoup de langages, Rust a **un** gestionnaire officiel (`cargo`) qui couvre build, test, doc, bench, format, lint et publication. Apprenez-le une bonne fois, vous l'utilisez pour tous vos projets.

## 1. `rustup` — gestionnaire de toolchains

Rust suit un cycle de release toutes les **6 semaines** sur 3 canaux :

| Canal | Description | Usage |
|---|---|---|
| `stable` | Dernière version validée | ← **défaut**, tout le code prod |
| `beta` | Preview de la prochaine stable | tests CI anticipés |
| `nightly` | Latest, features non stabilisées | libs qui ont besoin de `#![feature(...)]` |

### Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Commandes clés

```bash
rustup update                          # mettre à jour la toolchain
rustup toolchain list
rustup default stable
rustup install nightly                 # ajouter un canal
rustup override set nightly            # forcer nightly dans le dossier courant
```

### `rust-toolchain.toml` — épingler la version d'un projet

```toml
[toolchain]
channel = "1.85.0"
components = ["rustfmt", "clippy"]
```

À la racine d'un projet, force la toolchain à utiliser. Reproductibilité garantie.

## 2. Éditions

Une **édition** Rust est un changement majeur de syntaxe (2015, 2018, 2021, 2024). Contrairement à une version, les éditions **coexistent** : chaque crate en choisit une.

- **Edition 2024** (stabilisée fin 2024) est le défaut recommandé en 2026.
- Différences notables : nouveaux usages d'`async`, changements dans les captures de closures, `unsafe` plus strict dans les macros.

Déclarée dans `Cargo.toml` :

```toml
[package]
edition = "2024"
```

Un projet peut dépendre d'une crate d'édition plus ancienne sans souci : c'est géré par le compilateur.

## 3. `cargo` — l'outil unique

`cargo` couvre ~100% du quotidien. Apprenez ces commandes :

| Commande | Rôle |
|---|---|
| `cargo new foo` | Créer un nouveau binaire |
| `cargo new --lib foo` | Créer une lib |
| `cargo init` | Init dans le dossier courant |
| `cargo build` | Compiler (mode debug) |
| `cargo build --release` | Compiler optimisé |
| `cargo run` | Build + exécute |
| `cargo test` | Exécute les tests |
| `cargo bench` | Benchmarks (nightly ou `criterion` sur stable) |
| `cargo doc --open` | Génère et ouvre la doc |
| `cargo check` | Compile sans produire d'exécutable — **rapide** |
| `cargo add <crate>` | Ajoute une dép à `Cargo.toml` |
| `cargo remove <crate>` | Retire |
| `cargo update` | Met à jour `Cargo.lock` dans les bornes |
| `cargo fmt` | Formate le code (rustfmt) |
| `cargo clippy` | Linter avancé — **à passer avant chaque commit** |
| `cargo tree` | Arbre des dépendances |
| `cargo publish` | Publier sur crates.io |

### Le réflexe à acquérir : `cargo check`

`cargo check` fait le type-checking sans générer de binaire. **~3x plus rapide** que `cargo build`. Utilisez-le en boucle pendant le dev ; `build` / `run` uniquement quand vous voulez exécuter.

## 4. Structure d'un projet

```
mon-projet/
├── Cargo.toml           # manifeste
├── Cargo.lock           # versions épinglées (commit pour les binaires, .gitignore pour les libs)
├── src/
│   ├── main.rs          # point d'entrée du binaire
│   └── lib.rs           # racine de la lib (si applicable)
├── tests/               # tests d'intégration
│   └── integration.rs
├── benches/             # benchmarks
├── examples/            # exemples exécutables
└── target/              # build artifacts — dans .gitignore
```

### `Cargo.toml` — l'anatomie

```toml
[package]
name = "mon-projet"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"        # MSRV : minimum supported Rust version
description = "Description courte."
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/mon-projet"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
proptest = "1"

[profile.release]
lto = true                   # link-time optimization
codegen-units = 1            # plus long à compiler, plus rapide à l'exécution
```

### Bin + lib dans un même crate

```
src/
├── lib.rs                  # code réutilisable
└── main.rs                 # binaire fin, utilise lib
```

Pattern recommandé : logique dans `lib.rs`, `main.rs` fait juste l'entrée CLI. Testable, réutilisable.

### Multiples binaires

```
src/
├── lib.rs
└── bin/
    ├── serveur.rs
    └── migration.rs
```

```bash
cargo run --bin serveur
```

## 5. `Cargo.lock` — commit ou pas ?

- **Binaire / application** : **commit** le lockfile. Garantit que tout le monde (dev, CI, prod) build avec les mêmes versions.
- **Lib publiée sur crates.io** : **.gitignore** le lockfile (les consommateurs auront le leur).

## 6. Semver et résolution de versions

Dans `Cargo.toml` :

```toml
serde = "1.0"       # équivaut à "^1.0" : >=1.0.0, <2.0.0
serde = "=1.0.203"  # exact (pour debug ou reproductibilité ponctuelle)
serde = "~1.0"      # >=1.0.0, <1.1.0
tokio = { version = "1", features = ["rt", "macros"] }
```

**Règle** : `cargo update` est sûr tant que vous restez sur le même `major`. Les breaking changes exigent un bump majeur sur PyPI... pardon, crates.io. Les auteurs respectent largement semver en pratique, plus qu'en Python.

## 7. Features

Les **features** activent des parties conditionnelles de code. Exemple côté consommateur :

```toml
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Côté producteur (`Cargo.toml` d'une lib) :

```toml
[features]
default = ["std"]
std = []
async = ["dep:tokio"]
```

Utile pour garder les deps minimales. Approfondi au Ch. 17.

## 8. Outillage indispensable

Installez ces compléments dès maintenant :

```bash
rustup component add rustfmt clippy
cargo install cargo-edit       # cargo add / rm / upgrade (déjà inclus depuis 1.62)
cargo install cargo-watch      # rebuild à chaque sauvegarde
cargo install cargo-nextest    # test runner 60% plus rapide
cargo install cargo-expand     # voit le code généré après macros
```

### `cargo-watch` — workflow itératif

```bash
cargo watch -x check          # cargo check à chaque save
cargo watch -x test           # tests en boucle
cargo watch -x "clippy -- -D warnings"
```

### `cargo-nextest` — tests

```bash
cargo nextest run
```

Parallélise mieux que `cargo test`, output plus lisible. À adopter.

## 9. IDE et rust-analyzer

**`rust-analyzer`** est le LSP de référence. Sans lui, vous perdez 70% de la productivité.

| Éditeur | Setup |
|---|---|
| VS Code | Extension officielle "rust-analyzer" |
| RustRover | JetBrains, gratuit pour usage perso, très abouti |
| Zed / Helix | Support natif |
| Neovim | LSP + `rust-tools.nvim` |

Fonctionnalités : inlay hints (types inférés affichés), auto-import, code action (`cargo expand` inline), tests cliquables, debug intégré.

## 10. Compiler Explorer ([godbolt.org](https://godbolt.org))

Pour comprendre **ce que génère le compilateur**. Collez du Rust à gauche, l'assembleur (ou l'IR LLVM) apparaît à droite. Utile pour :
- Débusquer une optimisation qui n'a pas lieu.
- Comprendre le coût réel d'une abstraction (`Vec::push`, `Option::map`, ...).
- Montrer qu'un `for` sur un slice est compilé en SIMD.

## 11. `docs.rs` — doc auto pour toutes les crates

Toutes les crates publiées sur crates.io sont documentées sur [docs.rs/<crate>](https://docs.rs). Auto-généré par `cargo doc` à chaque publication.

Votre propre crate publiée apparaît automatiquement — pas de MkDocs ou Sphinx à configurer.

## 12. Le "Hello World" moderne

```bash
cargo new hello
cd hello
cargo run
```

`src/main.rs` généré :

```rust
fn main() {
    println!("Hello, world!");
}
```

`cargo run` → compile, link, exécute. En ~2 secondes sur une machine récente.

---

### Piège courant : compiler lent

Rust a la réputation d'être lent à compiler. **Mesures** pour mitiger :

- `cargo check` au lieu de `build` pendant le dev.
- Incremental compilation : **activé par défaut** en debug ; désactivé en release (normal).
- Réduire les generics exposés (chaque instanciation recompile).
- Partager un dossier `target/` entre workspaces : utiliser `[build]` `target-dir` dans `.cargo/config.toml`.
- `cargo-nextest` pour paralléliser les tests plus efficacement.

Les release builds restent lents (optimisations LLVM). Accepter.

---

### Sous le capot : pourquoi Rust est si typé au runtime

Contrairement à Python qui porte le type à l'exécution, Rust **efface** les types à la compilation : pas de `type(obj)` en Rust. Tout est connu à la compilation.

Conséquences :
- Pas de dispatch dynamique par défaut (statique).
- Monomorphisation : chaque instanciation générique est compilée séparément → binaire rapide mais potentiellement gros.
- Pas de reflection au runtime (sauf via `Any`, approché par approximation).

C'est pourquoi Rust produit des binaires **autonomes** (pas de "Python interpreter" à installer) et rapides.

---

## À retenir

- **Une** toolchain par projet via `rust-toolchain.toml`.
- `cargo` couvre build, test, doc, bench, publish, format, lint.
- `cargo check` pendant le dev, `cargo build --release` pour mesurer la vitesse.
- Edition 2024 recommandée par défaut.
- `Cargo.lock` : commit pour les binaires, ignore pour les libs.
- `rust-analyzer` + `clippy` + `rustfmt` = stack minimale.
- `cargo-watch` et `cargo-nextest` : productivité immédiate.

---

➡️ [Chapitre 2 — Primitifs, variables, mutabilité](../02_fondations/README.md)
