# Tutoriel Rust — Du débutant à l'expert

Parcours progressif en **3 niveaux** pour passer de la syntaxe de base à la maîtrise réelle de l'ownership, du typage, de l'async, de l'unsafe et du packaging crates.io.

Cible : **Rust stable 1.85+**, édition **2024**. Outils : `cargo`, `rustfmt`, `clippy`, `rust-analyzer`.

## Structure

- `cours/` — théorie commentée avec exemples exécutables
- `exercices/` — énoncés à compléter
- `solutions/` — corrigés commentés

Chaque niveau se termine par un **projet fil rouge** qui consolide les chapitres précédents.

## Sommaire

### Niveau 1 — Fondamentaux
*Projet fil rouge : CLI de gestion de tâches (clap + serde)*

1. Écosystème Rust moderne (`rustup`, `cargo`, edition 2024)
2. Primitifs, variables, mutabilité (`&str` vs `String`, shadowing, conversions)
3. Flux de contrôle (`match` en profondeur, `if let`, `let else`)
4. Ownership & borrowing — bases
5. Structures de données + introduction aux itérateurs
6. Fonctions, modules, `Result`/`Option` + `?` — **+ projet CLI todo**

### Niveau 2 — Intermédiaire
*Projet fil rouge : parseur de logs typé publié sur crates.io*

7. Structs & enums
8. Ownership avancée & lifetimes explicites
9. Traits (`derive`, `dyn Trait` vs `impl Trait`, associated types)
10. Generics & itérateurs avancés, closures (`Fn`/`FnMut`/`FnOnce`)
11. Tests, doc-tests, benchmarks, `clippy`, `proptest` — **+ projet parseur**

### Niveau 3 — Expert
*Projet fil rouge : API axum async avec auth JWT, publiée*

12. Concurrence synchrone (`Arc`/`Mutex`, atomics, `Send`/`Sync`)
13. Async & tokio (`select!`, channels, cancellation safety)
14. `unsafe`, smart pointers, FFI C ↔ Rust
15. Macros (`macro_rules!`, proc macros intro), const generics, patterns avancés
16. Dev web async avec axum (`sqlx`, JWT, tests tower)
17. Packaging & distribution (workspace, crates.io, CI, MSRV) — **+ projet API**

### Annexes
- **A.** Debug & tooling (`gdb`/`lldb`, `dbg!`, Miri, sanitizers, `tracing`)
- **B.** Regex & parsing (`regex`, `nom`)
- **C.** Bases de données (`sqlx`, `diesel`)
- **D.** WebAssembly (`wasm-bindgen`, intro)

## Prérequis

- [`rustup`](https://rustup.rs) — installe Rust et gère les toolchains
- Un IDE avec `rust-analyzer` (VS Code, RustRover, Zed, Helix...)

## Installation

```bash
# Installer rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vérifier
rustc --version       # >= 1.85
cargo --version
```

## Comment étudier

1. Lire `cours/NN_.../README.md`
2. Faire les exercices dans `exercices/NN_.../`
3. Comparer avec `solutions/NN_.../` **après** avoir tenté
4. Ne pas sauter les encadrés "Piège courant" et "Sous le capot"

Commencez par [Chapitre 1 — Écosystème Rust moderne](cours/01_ecosysteme/README.md).

## Valeur ajoutée vs "The Rust Programming Language"

Ce tuto est complémentaire à [The Book](https://doc.rust-lang.org/book/) officiel :

- **Projet-orienté** : 3 projets fil rouge publiables.
- **Écosystème moderne** : `tokio`, `axum`, `sqlx`, `serde`, `tracing` — non couverts par The Book.
- **Opinioné** sur les patterns (newtype, type-state, builder, anti-patterns).
- **Profondeur inhabituelle** sur `unsafe`, FFI, async cancellation safety.
