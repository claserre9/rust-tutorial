# Chapitre 11 — Tests, benchmarks et projet parseur de logs

Rust intègre les tests directement dans le langage. Pas de framework externe nécessaire pour l'essentiel — `cargo test` suffit pour 90% des besoins. Ce chapitre couvre les tests unitaires, d'intégration, la gestion des panics, et les benchmarks, puis le **projet fil rouge niveau 2** : un parseur de logs Nginx.

## 1. Tests unitaires

```rust
fn additionner(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;         // importe tout du module parent

    #[test]
    fn test_addition_positive() {
        assert_eq!(additionner(2, 3), 5);
    }

    #[test]
    fn test_addition_negative() {
        assert_eq!(additionner(-1, -1), -2);
    }
}
```

`#[cfg(test)]` — le module est compilé uniquement pour `cargo test`, pas dans le binaire final.

### Macros d'assertion

```rust
assert!(expr)                      // panique si false
assert_eq!(a, b)                   // panique si a != b (affiche les deux valeurs)
assert_ne!(a, b)                   // panique si a == b
assert!(expr, "message {}", val)   // avec message formaté
```

`assert_eq!` et `assert_ne!` requièrent `Debug + PartialEq`.

## 2. Tester les panics et les erreurs

```rust
#[test]
#[should_panic(expected = "division par zéro")]
fn test_division_par_zero() {
    let _ = 10 / 0;   // panique attendue
}

#[test]
fn test_result() -> Result<(), String> {
    let n: i32 = "42".parse().map_err(|e| format!("{e}"))?;
    assert_eq!(n, 42);
    Ok(())             // les tests peuvent retourner Result
}
```

`#[should_panic(expected = "...")]` vérifie que le message de panique contient la chaîne attendue.

## 3. Tests d'intégration

Les tests d'intégration vivent dans `tests/` à la racine du projet (même niveau que `src/`). Ils testent l'API publique comme un consommateur externe.

```
projet/
├── src/
│   └── lib.rs
└── tests/
    ├── integration_1.rs
    └── integration_2.rs
```

```rust
// tests/integration_1.rs
use mon_crate::fonctionnalite_publique;

#[test]
fn test_integration() {
    let résultat = fonctionnalite_publique(42);
    assert_eq!(résultat, 84);
}
```

Chaque fichier dans `tests/` est compilé comme une crate séparée.

## 4. Fixtures et helpers

```rust
#[cfg(test)]
mod tests {
    // Helper partagé entre les tests du module
    fn creer_donnees_test() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn test_somme() {
        let data = creer_donnees_test();
        assert_eq!(data.iter().sum::<i32>(), 15);
    }

    #[test]
    fn test_longueur() {
        assert_eq!(creer_donnees_test().len(), 5);
    }
}
```

Pour les fixtures plus complexes (fichiers temporaires, bases de données), utilisez `tempfile` (crate externe).

## 5. Ignorer des tests

```rust
#[test]
#[ignore = "trop lent en CI"]
fn test_tres_long() {
    // ...
}
```

```bash
cargo test                  # saute les #[ignore]
cargo test -- --ignored     # exécute seulement les ignorés
cargo test -- --include-ignored   # tous les tests
```

## 6. Tests en parallèle et isolation

Par défaut, `cargo test` lance les tests en parallèle. Si un test modifie une ressource partagée (fichier, variable globale), il peut interférer.

```bash
cargo test -- --test-threads=1   # séquentiel
```

Pour les tests qui accèdent à des fichiers, utilisez des chemins uniques :

```rust
use std::env;

fn chemin_test_unique(nom: &str) -> std::path::PathBuf {
    env::temp_dir().join(format!("test_{}_{}", nom, std::process::id()))
}
```

## 7. Benchmarks avec `criterion`

`cargo bench` (built-in) utilise des benchmarks instables. Pour la production, utilisez `criterion` :

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "mon_bench"
harness = false
```

```rust
// benches/mon_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_tri(c: &mut Criterion) {
    let mut v: Vec<i32> = (0..1000).rev().collect();
    c.bench_function("sort_1000", |b| {
        b.iter(|| {
            let mut copy = black_box(v.clone());
            copy.sort();
            black_box(copy)
        })
    });
}

criterion_group!(benches, bench_tri);
criterion_main!(benches);
```

```bash
cargo bench                     # exécute et génère HTML dans target/criterion/
cargo bench -- --save-baseline main    # sauvegarde pour comparaison
```

`black_box` empêche le compilateur d'optimiser le code benchmarké hors de l'existence.

## 8. Couverture de code

```bash
cargo install cargo-tarpaulin    # outil de couverture pour Linux
cargo tarpaulin --out Html       # génère coverage.html

# Alternative multi-plateforme :
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

---

## Projet fil rouge — Parseur de logs Nginx

Construisez un parseur de logs Nginx avec analyse statistique.

### Format d'un log Nginx (Combined Log Format)

```
127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326
```

Champs : `ip - utilisateur [timestamp] "méthode chemin protocole" statut taille`

### Structure du projet

```
log-parser/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── parser.rs       # parsing d'une ligne
│   ├── models.rs       # struct LogEntry
│   └── analytics.rs   # statistiques
└── tests/
    └── integration.rs
```

### `Cargo.toml`

```toml
[package]
name = "log-parser"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror = "2"
chrono = { version = "0.4", default-features = false, features = ["alloc"] }

[dev-dependencies]
```

### `src/models.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub ip: String,
    pub utilisateur: Option<String>,
    pub statut: u16,
    pub taille: u64,
    pub methode: String,
    pub chemin: String,
}
```

### `src/parser.rs`

```rust
use crate::errors::ParseError;
use crate::models::LogEntry;

pub fn parser_ligne(ligne: &str) -> Result<LogEntry, ParseError> {
    // Format : IP - USER [DATE] "METHOD PATH PROTO" STATUS SIZE
    let parties: Vec<&str> = ligne.splitn(10, ' ').collect();
    if parties.len() < 9 {
        return Err(ParseError::FormatInvalide(ligne.to_string()));
    }

    let ip = parties[0].to_string();

    let utilisateur = match parties[2] {
        "-" => None,
        u   => Some(u.to_string()),
    };

    // Retirer les guillemets de la requête
    let requete = parties[6].trim_start_matches('"');
    let req_parties: Vec<&str> = requete.split(' ').collect();
    if req_parties.len() < 2 {
        return Err(ParseError::FormatInvalide(ligne.to_string()));
    }

    let methode = req_parties[0].to_string();
    let chemin = req_parties[1].to_string();

    let statut_str = parties[8];
    let statut = statut_str.parse::<u16>()
        .map_err(|_| ParseError::StatutInvalide(statut_str.to_string()))?;

    let taille_str = parties.get(9).unwrap_or(&"0").trim_end_matches('"');
    let taille = if taille_str == "-" { 0 } else {
        taille_str.parse::<u64>()
            .map_err(|_| ParseError::TailleInvalide(taille_str.to_string()))?
    };

    Ok(LogEntry { ip, utilisateur, statut, taille, methode, chemin })
}
```

### `src/analytics.rs`

```rust
use std::collections::HashMap;
use crate::models::LogEntry;

pub struct Statistiques {
    pub total_requetes: usize,
    pub total_octets: u64,
    pub codes_statut: HashMap<u16, usize>,
    pub ips_frequentes: Vec<(String, usize)>,
    pub chemins_populaires: Vec<(String, usize)>,
    pub taux_erreurs: f64,
}

pub fn analyser(entrées: &[LogEntry]) -> Statistiques {
    let total_requetes = entrées.len();
    let total_octets: u64 = entrées.iter().map(|e| e.taille).sum();

    let mut codes_statut: HashMap<u16, usize> = HashMap::new();
    let mut ip_counts: HashMap<&str, usize> = HashMap::new();
    let mut chemin_counts: HashMap<&str, usize> = HashMap::new();

    for e in entrées {
        *codes_statut.entry(e.statut).or_insert(0) += 1;
        *ip_counts.entry(&e.ip).or_insert(0) += 1;
        *chemin_counts.entry(&e.chemin).or_insert(0) += 1;
    }

    let mut ips_frequentes: Vec<(String, usize)> = ip_counts
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    ips_frequentes.sort_by(|a, b| b.1.cmp(&a.1));
    ips_frequentes.truncate(10);

    let mut chemins_populaires: Vec<(String, usize)> = chemin_counts
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    chemins_populaires.sort_by(|a, b| b.1.cmp(&a.1));
    chemins_populaires.truncate(10);

    let erreurs = entrées.iter().filter(|e| e.statut >= 400).count();
    let taux_erreurs = if total_requetes > 0 {
        erreurs as f64 / total_requetes as f64 * 100.0
    } else { 0.0 };

    Statistiques {
        total_requetes,
        total_octets,
        codes_statut,
        ips_frequentes,
        chemins_populaires,
        taux_erreurs,
    }
}
```

### `src/lib.rs`

```rust
pub mod analytics;
pub mod models;
pub mod parser;
mod errors;

pub use errors::ParseError;
pub use models::LogEntry;

pub fn parser_fichier(contenu: &str) -> (Vec<LogEntry>, Vec<ParseError>) {
    let mut entrées = Vec::new();
    let mut erreurs = Vec::new();

    for ligne in contenu.lines() {
        if ligne.is_empty() { continue; }
        match parser::parser_ligne(ligne) {
            Ok(e) => entrées.push(e),
            Err(e) => erreurs.push(e),
        }
    }

    (entrées, erreurs)
}
```

### Tests dans `tests/integration.rs`

```rust
use log_parser::{parser_fichier, analytics::analyser};

const LOG_EXEMPLE: &str = r#"
127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 2326
192.168.1.1 - - [10/Oct/2000:13:55:37 -0700] "POST /api/data HTTP/1.1" 404 512
127.0.0.1 - - [10/Oct/2000:13:55:38 -0700] "GET /style.css HTTP/1.0" 200 1024
10.0.0.1 - - [10/Oct/2000:13:55:39 -0700] "GET /index.html HTTP/1.1" 500 0
"#;

#[test]
fn test_parser_plusieurs_lignes() {
    let (entrées, erreurs) = parser_fichier(LOG_EXEMPLE);
    assert_eq!(entrées.len(), 4);
    assert!(erreurs.is_empty());
}

#[test]
fn test_statistiques() {
    let (entrées, _) = parser_fichier(LOG_EXEMPLE);
    let stats = analyser(&entrées);

    assert_eq!(stats.total_requetes, 4);
    assert_eq!(stats.codes_statut[&200], 2);
    assert_eq!(stats.codes_statut[&404], 1);
    assert!((stats.taux_erreurs - 50.0).abs() < 0.01);
}

#[test]
fn test_ligne_invalide() {
    let (entrées, erreurs) = parser_fichier("ligne invalide");
    assert!(entrées.is_empty());
    assert_eq!(erreurs.len(), 1);
}
```

---

## À retenir

- Les tests vivent dans `#[cfg(test)] mod tests { ... }` — compilés seulement pour `cargo test`.
- `tests/` pour l'intégration, `#[cfg(test)]` dans `src/` pour les unitaires.
- `#[should_panic]` pour tester les panics, `-> Result<>` pour propager les erreurs de test.
- `criterion` pour des benchmarks statistiquement rigoureux.
- Pattern idiomatique : `parser_fichier` retourne `(Vec<T>, Vec<Erreur>)` pour séparer succès et erreurs.

---

➡️ [Chapitre 13 — Concurrence sync : threads, Arc, Mutex](../13_concurrence_sync/README.md)
