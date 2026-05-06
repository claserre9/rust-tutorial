# Tutoriel Rust — Du débutant à l'expert

Parcours progressif en **3 niveaux** pour passer de la syntaxe de base à la maîtrise réelle de l'ownership, du typage, de l'async, de l'unsafe et du packaging crates.io.

Cible : **Rust stable 1.85+**, édition **2024**. Outils : `cargo`, `rustfmt`, `clippy`, `rust-analyzer`.

## Structure du projet

```
rust-tutorial/
├── docs/
│   ├── cours/          # Leçons théoriques avec exemples exécutables
│   ├── exercices/      # Exercices pratiques par chapitre
│   ├── solutions/      # Corrections annotées
│   └── annexes/        # Références et ressources complémentaires
```

## Sommaire

### Niveau 1 — Fondamentaux

| # | Chapitre | Thèmes abordés |
|---|----------|----------------|
| 1 | [Écosystème Rust](cours/01_ecosysteme/README.md) | `rustup`, `cargo`, édition 2024, Hello World |
| 2 | [Primitifs & variables](cours/02_fondations/README.md) | Types primitifs, `&str` vs `String`, shadowing, mutabilité, conversions |
| 3 | [Flux de contrôle](cours/03_flux_controle/README.md) | `match` en profondeur, `if let`, `let else`, boucles |
| 4 | [Ownership & borrowing](cours/04_ownership/README.md) | Move, copy, borrow, `&`, `&mut`, règles du borrow checker |
| 5 | [Structs, enums, collections](cours/05_structures_donnees/README.md) | `struct`, `enum`, `Vec`, `HashMap`, itérateurs de base |
| 6 | [Gestion des erreurs](cours/06_gestion_erreurs/README.md) | `Option`, `Result`, `?`, `thiserror`, `anyhow` |
| 7 | [Fonctions & modules](cours/07_fonctions_modules/README.md) | Modules, `pub`, `use`, `crate`, `Result`/`Option` — **Projet : CLI todo** |

### Niveau 2 — Intermédiaire

| # | Chapitre | Thèmes abordés |
|---|----------|----------------|
| 8 | [Structs avancés & newtype](cours/08_structs_enums/README.md) | Newtype, builder pattern, enums à données, pattern matching |
| 9 | [Lifetimes](cours/09_lifetimes/README.md) | Annotations explicites, `'static`, lifetime élision, structs avec références |
| 10 | [Smart pointers](cours/10_smart_pointers/README.md) | `Box`, `Rc`, `RefCell`, `Arc`, `Weak`, interior mutability |
| 11 | [Traits](cours/11_traits/README.md) | `derive`, `dyn Trait` vs `impl Trait`, associated types, object safety |
| 12 | [Génériques & itérateurs](cours/12_generics_iterators/README.md) | Generics, closures (`Fn`/`FnMut`/`FnOnce`), `Iterator`, adaptateurs |
| 13 | [Tests & qualité](cours/13_tests_qualite/README.md) | `#[test]`, doc-tests, benchmarks, `clippy`, `proptest` — **Projet : parseur** |

### Niveau 3 — Expert

| # | Chapitre | Thèmes abordés |
|---|----------|----------------|
| 14 | [Concurrence sync](cours/14_concurrence_sync/README.md) | `Arc`/`Mutex`, atomics, `Send`/`Sync`, rayon |
| 15 | [Async & Tokio](cours/15_async_tokio/README.md) | `async/await`, `select!`, channels, cancellation safety |
| 16 | [Unsafe & FFI](cours/16_unsafe_ffi/README.md) | `unsafe`, raw pointers, FFI C ↔ Rust, `bindgen` |
| 17 | [Macros & patterns](cours/17_macros_patterns/README.md) | `macro_rules!`, proc macros, const generics, patterns avancés |
| 18 | [Web avec Axum](cours/18_dev_web_axum/README.md) | `axum`, `sqlx`, JWT, middleware, tests tower |
| 19 | [Packaging & CI](cours/19_packaging/README.md) | Workspace, crates.io, CI, MSRV — **Projet : API publiée** |

## Prérequis

- [`rustup`](https://rustup.rs) — installe Rust et gère les toolchains
- Un IDE avec `rust-analyzer` (VS Code, RustRover, Zed, Helix...)

## Installation rapide

```bash
# Installer rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vérifier
rustc --version       # >= 1.85
cargo --version

# Cloner le tutoriel
git clone https://github.com/claserre9/rust-tutorial.git
cd rust-tutorial

# Servir la documentation localement
pip install -r requirements.txt
mkdocs serve
```

La documentation est ensuite disponible sur `http://localhost:8000`.

## Parcours recommandé

1. Lire le cours du chapitre
2. Faire les exercices sans regarder les solutions
3. Comparer avec les solutions annotées
4. Ne pas sauter les encadrés "Piège courant" et "Sous le capot"

## Licence

MIT — Clifford
