# Chapitre 6 — Fonctions, modules et projet CLI todo

Fonctions avancées, système de modules, gestion d'erreurs idiomatique avec `thiserror`, puis le **projet fil rouge niveau 1** : une CLI todo complète avec `clap` + `serde_json`.

## 1. Fonctions avancées

### Paramètres génériques simples

```rust
fn plus_grand<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

println!("{}", plus_grand(5, 10));
println!("{}", plus_grand(3.14, 2.71));
```

Les génériques sont détaillés au Ch. 10. Pour l'instant, retenez `T: Trait` comme "T doit implémenter ce trait".

### Closures

Une closure capture son environnement :

```rust
let seuil = 10;
let filtre = |n: &i32| *n > seuil;   // capture seuil par référence

let v = vec![5, 15, 3, 20];
let grands: Vec<_> = v.iter().filter(filtre).collect();
```

Types de capture :
- `|x| ...` → capture par référence (si possible)
- `move |x| ...` → prend ownership des variables capturées

```rust
let s = String::from("bonjour");
let afficher = move || println!("{s}");   // s est moved dans la closure
// println!("{s}");                        // ❌ s moved
afficher();
```

### Fonctions retournant des fonctions (`impl Fn`)

```rust
fn multiplicateur(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n                        // n est capturé par move
}

let triple = multiplicateur(3);
assert_eq!(triple(7), 21);
```

### Fonctions comme paramètres

```rust
fn appliquer(f: impl Fn(i32) -> i32, valeur: i32) -> i32 {
    f(valeur)
}

appliquer(|x| x * x, 5);           // 25
appliquer(triple, 4);               // 12
```

## 2. Système de modules

Rust organise le code en modules, fichiers et crates.

### Modules inline

```rust
mod math {
    pub fn carre(n: i32) -> i32 { n * n }

    pub mod avance {
        pub fn racine(n: f64) -> f64 { n.sqrt() }
    }
}

println!("{}", math::carre(4));
println!("{}", math::avance::racine(16.0));
```

`pub` rend les items accessibles depuis l'extérieur. Tout est **privé par défaut**.

### Modules dans des fichiers

```
src/
├── main.rs           // racine de la crate (binary)
├── lib.rs            // racine d'une lib crate (optionnel)
├── math.rs           // mod math { ... }
└── math/
    ├── mod.rs        // ancien style
    └── avance.rs     // sous-module
```

Dans `main.rs` :
```rust
mod math;             // charge src/math.rs (ou src/math/mod.rs)

use math::carre;
println!("{}", carre(5));
```

### `use` — imports

```rust
use std::collections::HashMap;
use std::collections::{HashMap, HashSet};
use std::collections::*;              // glob import (à éviter sauf prelude)

use crate::math::carre;               // chemin absolu depuis la racine
use super::math::carre;               // chemin relatif au parent
use self::avance::racine;             // chemin relatif au module courant
```

### Visibilité fine

```rust
pub(crate) fn interne() { ... }       // visible dans la crate entière
pub(super) fn restreinte() { ... }    // visible dans le module parent
```

## 3. Gestion d'erreurs avec `thiserror`

Pour les bibliothèques et les projets structurés, on définit un type d'erreur dédié. `thiserror` génère les implémentations boilerplate.

```toml
# Cargo.toml
[dependencies]
thiserror = "2"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum TodoError {
    #[error("tâche {0} introuvable")]
    NonTrouvee(u32),

    #[error("fichier corrompu : {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON invalide : {0}")]
    Json(#[from] serde_json::Error),
}
```

- `#[error("...")]` : message affiché par `Display` (et donc `println!("{e}")`)
- `#[from]` : implémente `From<std::io::Error> for TodoError` — le `?` convertit automatiquement

```rust
fn charger(path: &str) -> Result<Vec<Todo>, TodoError> {
    let contenu = std::fs::read_to_string(path)?;  // io::Error → TodoError::Io
    let todos: Vec<Todo> = serde_json::from_str(&contenu)?; // serde_json::Error → TodoError::Json
    Ok(todos)
}
```

## 4. `clap` — arguments CLI

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "Gestionnaire de tâches")]
struct Cli {
    #[command(subcommand)]
    commande: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Ajouter une tâche
    Ajouter {
        /// Description de la tâche
        description: String,
    },
    /// Lister les tâches
    Lister,
    /// Marquer une tâche comme faite
    Terminer {
        /// ID de la tâche
        id: u32,
    },
    /// Supprimer une tâche
    Supprimer {
        id: u32,
    },
}
```

L'utilisation génère automatiquement `--help` :

```
Usage: todo <COMMAND>

Commands:
  ajouter   Ajouter une tâche
  lister    Lister les tâches
  terminer  Marquer une tâche comme faite
  supprimer Supprimer une tâche
  help      Print this message or the help of the given subcommand(s)
```

## 5. `serde` + `serde_json` — sérialisation

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    description: String,
    fait: bool,
}

// Sérialiser
let todo = Todo { id: 1, description: String::from("Apprendre Rust"), fait: false };
let json = serde_json::to_string_pretty(&todo)?;
println!("{json}");

// Désérialiser
let todo2: Todo = serde_json::from_str(&json)?;
assert_eq!(todo2.description, "Apprendre Rust");
```

---

## Projet fil rouge — CLI todo

Construisez une CLI todo complète avec persistance JSON.

### Structure du projet

```
todo-cli/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── cli.rs       # définition clap
    ├── models.rs    # struct Todo
    ├── storage.rs   # lecture/écriture JSON
    └── errors.rs    # enum TodoError
```

### `Cargo.toml`

```toml
[package]
name = "todo-cli"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
```

### `src/models.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub fait: bool,
}

impl Todo {
    pub fn nouveau(id: u32, description: String) -> Todo {
        Todo { id, description, fait: false }
    }
}
```

### `src/errors.rs`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("tâche #{0} introuvable")]
    NonTrouvee(u32),

    #[error("erreur I/O : {0}")]
    Io(#[from] std::io::Error),

    #[error("erreur JSON : {0}")]
    Json(#[from] serde_json::Error),
}
```

### `src/storage.rs`

```rust
use std::path::Path;
use crate::errors::TodoError;
use crate::models::Todo;

const FICHIER: &str = "todos.json";

pub fn charger() -> Result<Vec<Todo>, TodoError> {
    if !Path::new(FICHIER).exists() {
        return Ok(Vec::new());
    }
    let contenu = std::fs::read_to_string(FICHIER)?;
    let todos = serde_json::from_str(&contenu)?;
    Ok(todos)
}

pub fn sauvegarder(todos: &[Todo]) -> Result<(), TodoError> {
    let json = serde_json::to_string_pretty(todos)?;
    std::fs::write(FICHIER, json)?;
    Ok(())
}
```

### `src/cli.rs`

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "Gestionnaire de tâches CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub commande: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Ajouter une nouvelle tâche
    Ajouter { description: String },
    /// Lister toutes les tâches
    Lister,
    /// Marquer une tâche comme faite
    Terminer { id: u32 },
    /// Supprimer une tâche
    Supprimer { id: u32 },
}
```

### `src/main.rs`

```rust
mod cli;
mod errors;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Cmd};
use errors::TodoError;
use models::Todo;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erreur : {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut todos = storage::charger()?;

    match cli.commande {
        Cmd::Ajouter { description } => {
            let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            todos.push(Todo::nouveau(id, description));
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} ajoutée.");
        }
        Cmd::Lister => {
            if todos.is_empty() {
                println!("Aucune tâche.");
            } else {
                for t in &todos {
                    let etat = if t.fait { "✓" } else { "○" };
                    println!("[{etat}] #{}: {}", t.id, t.description);
                }
            }
        }
        Cmd::Terminer { id } => {
            let t = todos.iter_mut().find(|t| t.id == id)
                .ok_or(TodoError::NonTrouvee(id))?;
            t.fait = true;
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} marquée comme faite.");
        }
        Cmd::Supprimer { id } => {
            let avant = todos.len();
            todos.retain(|t| t.id != id);
            if todos.len() == avant {
                return Err(TodoError::NonTrouvee(id));
            }
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} supprimée.");
        }
    }
    Ok(())
}
```

### Utilisation

```bash
cargo run -- ajouter "Apprendre Rust"
cargo run -- ajouter "Lire le chapitre 7"
cargo run -- lister
cargo run -- terminer 1
cargo run -- lister
cargo run -- supprimer 2
```

Sortie :
```
[○] #1: Apprendre Rust
[○] #2: Lire le chapitre 7

[✓] #1: Apprendre Rust
[○] #2: Lire le chapitre 7
```

---

## À retenir

- Les closures capturent leur environnement. `move` transfère l'ownership.
- Le système de modules est basé sur les fichiers. `pub` pour exporter.
- `thiserror` élimine le boilerplate des types d'erreur. `#[from]` pour les conversions automatiques avec `?`.
- `clap` derive génère toute la CLI depuis des structs annotées.
- `serde` + `#[derive(Serialize, Deserialize)]` : sérialisation JSON en quelques lignes.
- Pattern idiomatique : `fn main() → void` + `fn run() → Result<_, _>`.

---

➡️ [Chapitre 7 — Structs avancés, Display et newtype](../07_structs_enums/README.md)
