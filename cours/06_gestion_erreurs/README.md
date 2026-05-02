# Chapitre 6 — Gestion des erreurs

Rust n'a pas d'exceptions. À la place, il utilise deux types : `Option<T>` pour l'absence de valeur, et `Result<T, E>` pour les opérations faillibles. Ce chapitre couvre la philosophie, les outils, et les patterns idiomatiques — de `unwrap` à `thiserror` en passant par `anyhow`.

## 1. Philosophie : les erreurs sont des valeurs

En Python/Java, une exception peut surgir de n'importe quelle fonction et remonter silencieusement la pile. En Rust, **si une fonction peut échouer, son type de retour le dit explicitement** :

```rust
// Cette fonction ne peut pas échouer
fn additionner(a: i32, b: i32) -> i32 { a + b }

// Cette fonction peut échouer
fn diviser(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// Cette fonction peut échouer avec un message d'erreur
fn lire_entier(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse()
}
```

Le compilateur force à traiter les cas d'erreur. Vous ne pouvez pas accidentellement ignorer un `Result`.

## 2. `Option<T>` — valeur optionnelle

Utilisé quand l'absence est **normale** (pas une erreur) : chercher dans une collection, lire un champ optionnel, etc.

```rust
fn premier(v: &[i32]) -> Option<i32> {
    if v.is_empty() { None } else { Some(v[0]) }
}
```

### Méthodes essentielles

```rust
let opt: Option<i32> = Some(42);

// Extraire (avec risque)
opt.unwrap()                      // panique si None — à éviter hors tests
opt.expect("message de bug")      // panique avec message — pour les invariants garantis

// Extraire (sûr)
opt.unwrap_or(0)                  // valeur par défaut
opt.unwrap_or_else(|| calcul())   // lazy — n'évalue que si nécessaire
opt.unwrap_or_default()           // valeur Default::default()

// Transformer sans extraire
opt.map(|n| n * 2)                // Option<i32> : applique f si Some
opt.filter(|n| *n > 0)            // None si le prédicat échoue
opt.and_then(|n| autre_option(n)) // flatMap — enchaîner des fonctions qui retournent Option
opt.or(Some(0))                   // valeur de repli si None
opt.or_else(|| Some(calcul()))    // lazy repli

// Informations
opt.is_some()
opt.is_none()

// Référence sans consommer
opt.as_ref()                      // Option<&i32>
```

### Pattern : `if let` et `while let`

```rust
if let Some(n) = opt {
    println!("{n}");
}

// Equivalent moderne (Rust 1.82+)
if opt.is_some_and(|n| n > 0) { ... }
```

## 3. `Result<T, E>` — succès ou erreur

Utilisé pour les opérations qui peuvent échouer avec une raison : I/O, parsing, réseau, validation.

```rust
use std::fs;
use std::io;

fn lire_fichier(chemin: &str) -> Result<String, io::Error> {
    fs::read_to_string(chemin)
}
```

### Méthodes essentielles

```rust
let res: Result<i32, String> = Ok(42);

// Extraire (avec risque)
res.unwrap()                      // panique si Err
res.expect("invariant garanti")   // panique avec message

// Extraire (sûr)
res.unwrap_or(0)
res.unwrap_or_else(|_| 0)
res.unwrap_or_default()

// Transformer
res.map(|n| n * 2)                // Result<i32, E> : transforme Ok
res.map_err(|e| format!("{e}"))   // Result<T, String> : transforme Err
res.and_then(|n| autre_result(n)) // flatMap sur Ok
res.or_else(|_| Ok(0))            // repli sur Err

// Convertir
res.ok()                          // Option<i32> — ignore l'erreur
res.err()                         // Option<String> — ignore le succès

// Informations
res.is_ok()
res.is_err()
```

## 4. L'opérateur `?` — propagation élégante

`?` est l'outil central. Dans une fonction retournant `Result<T, E>`, `?` sur un `Result<V, F>` :
- Si `Ok(v)` : extrait `v`
- Si `Err(e)` : convertit via `From` et retourne immédiatement `Err(e.into())`

```rust
use std::io;
use std::fs;
use std::num::ParseIntError;

// Sans ? — verbeux
fn lire_age_v1(chemin: &str) -> Result<u32, io::Error> {
    let contenu = match fs::read_to_string(chemin) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    // ...
}

// Avec ? — idiomatique
fn lire_age_v2(chemin: &str) -> Result<String, io::Error> {
    let contenu = fs::read_to_string(chemin)?;  // retourne Err si échec
    let ligne = contenu.lines().next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "vide"))?;
    Ok(ligne.trim().to_string())
}
```

### `?` dans `main`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contenu = std::fs::read_to_string("config.toml")?;
    println!("{contenu}");
    Ok(())
}
```

`Box<dyn std::error::Error>` accepte n'importe quelle erreur — pratique pour les scripts et prototypes.

### `?` avec `Option`

```rust
fn premier_chiffre(s: &str) -> Option<u32> {
    let c = s.chars().next()?;  // None si s est vide
    c.to_digit(10)              // None si pas un chiffre
}
```

## 5. Types d'erreur personnalisés

Pour les bibliothèques et les projets structurés, définissez vos propres types d'erreur.

### Version manuelle

```rust
#[derive(Debug)]
enum MonErreur {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    ValeurInvalide(String),
}

impl std::fmt::Display for MonErreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonErreur::Io(e) => write!(f, "erreur I/O : {e}"),
            MonErreur::Parse(e) => write!(f, "erreur de parsing : {e}"),
            MonErreur::ValeurInvalide(s) => write!(f, "valeur invalide : {s}"),
        }
    }
}

impl std::error::Error for MonErreur {}

// Pour que ? convertisse automatiquement io::Error → MonErreur
impl From<std::io::Error> for MonErreur {
    fn from(e: std::io::Error) -> MonErreur {
        MonErreur::Io(e)
    }
}

impl From<std::num::ParseIntError> for MonErreur {
    fn from(e: std::num::ParseIntError) -> MonErreur {
        MonErreur::Parse(e)
    }
}

fn traiter(chemin: &str) -> Result<i32, MonErreur> {
    let s = std::fs::read_to_string(chemin)?;  // io::Error → MonErreur::Io via From
    let n: i32 = s.trim().parse()?;             // ParseIntError → MonErreur::Parse via From
    if n < 0 { return Err(MonErreur::ValeurInvalide(format!("{n} est négatif"))); }
    Ok(n * 2)
}
```

### Avec `thiserror` — éliminer le boilerplate

```toml
[dependencies]
thiserror = "2"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum MonErreur {
    #[error("erreur I/O : {0}")]
    Io(#[from] std::io::Error),           // #[from] génère impl From automatiquement

    #[error("erreur de parsing : {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("valeur invalide : {0}")]
    ValeurInvalide(String),

    #[error("config manquante pour la clé '{cle}'")]
    ConfigManquante { cle: String },       // champs nommés dans le message
}
```

`thiserror` génère `Display`, `Error`, et tous les `From` — rien d'autre à écrire.

## 6. `anyhow` — erreurs dynamiques pour les binaires

`thiserror` est pour les **bibliothèques** (types précis, convertibles). `anyhow` est pour les **binaires et scripts** où vous voulez juste propager des erreurs sans définir un type.

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result, bail, ensure};

fn lire_config(chemin: &str) -> Result<Config> {
    let contenu = std::fs::read_to_string(chemin)
        .with_context(|| format!("impossible de lire '{chemin}'"))?;  // ajoute du contexte

    let config: Config = toml::from_str(&contenu)
        .context("configuration TOML invalide")?;

    ensure!(config.port > 1024, "le port doit être > 1024, trouvé {}", config.port);

    Ok(config)
}

fn main() -> Result<()> {
    let config = lire_config("config.toml")?;
    println!("port = {}", config.port);
    Ok(())
}
```

### Macros `anyhow`

```rust
// bail! — retourner immédiatement une erreur
if valeur < 0 {
    bail!("valeur négative : {valeur}");
}

// ensure! — assertion qui retourne Err si fausse
ensure!(valeur > 0, "valeur doit être positive, trouvé {valeur}");

// anyhow! — créer une erreur ad hoc
return Err(anyhow::anyhow!("quelque chose s'est mal passé"));
```

### Afficher la chaîne d'erreurs

```rust
fn main() -> anyhow::Result<()> {
    lire_config("manquant.toml")?;
    Ok(())
}
// Affiche :
// Error: impossible de lire 'manquant.toml'
// Caused by:
//     No such file or directory (os error 2)
```

`anyhow` conserve la chaîne complète de contexte — bien meilleure UX que `unwrap`.

## 7. Quand utiliser quoi

| Situation | Outil |
|---|---|
| Valeur optionnelle (pas une erreur) | `Option<T>` |
| Erreur récupérable avec type précis | `Result<T, MonErreur>` |
| Bibliothèque publiée | `thiserror` (type explicite) |
| Binaire / script / prototype | `anyhow` (type dynamique) |
| Invariant garanti par la logique | `unwrap()` / `expect()` |
| Test | `unwrap()` acceptable |
| `main` simple | `fn main() -> Result<(), Box<dyn Error>>` |

### Règle : ne jamais `unwrap` en prod (sauf invariants)

```rust
// ❌ : peut paniquer si la config change
let port = config["port"].as_integer().unwrap();

// ✅ : erreur explicite
let port = config["port"].as_integer()
    .ok_or_else(|| anyhow::anyhow!("clé 'port' manquante ou invalide"))?;
```

## 8. Conversions entre `Option` et `Result`

```rust
// Option → Result
let opt: Option<i32> = Some(42);
let res: Result<i32, &str> = opt.ok_or("valeur absente");
let res2: Result<i32, String> = opt.ok_or_else(|| format!("absent à {}", 42));

// Result → Option (ignore l'erreur)
let opt2: Option<i32> = res.ok();

// Option dans une fonction Result avec ?
fn f() -> Result<i32, String> {
    let n = chercher_valeur().ok_or("introuvable")?;
    Ok(n * 2)
}
```

---

### Piège courant : mélanger les types d'erreur sans `From`

```rust
fn mélange() -> Result<(), String> {
    std::fs::read_to_string("f")?;  // ❌ io::Error ≠ String, pas de From
    Ok(())
}

// Fix 1 : .map_err
std::fs::read_to_string("f").map_err(|e| e.to_string())?;

// Fix 2 : anyhow
fn mélange() -> anyhow::Result<()> {
    std::fs::read_to_string("f")?;  // ✅ anyhow accepte tout
    Ok(())
}
```

---

### Sous le capot : `?` et `From`

`expr?` est du sucre syntaxique pour :

```rust
match expr {
    Ok(val) => val,
    Err(e) => return Err(From::from(e)),
}
```

C'est le `From::from(e)` qui permet la conversion automatique. Tant que `impl From<SourceError> for TargetError` existe, `?` convertit silencieusement.

---

## À retenir

- `Option<T>` pour l'absence normale. `Result<T, E>` pour les échecs.
- `?` propage les erreurs idiomatiquement — plus jamais de `match Err => return Err`.
- `thiserror` pour les bibliothèques (types précis). `anyhow` pour les binaires (ergonomie).
- `unwrap()` / `expect()` sont acceptables pour les invariants et les tests, pas en prod.
- `.context()` d'anyhow ajoute du contexte à chaque niveau — les messages d'erreur deviennent exploitables.

---

➡️ [Chapitre 7 — Fonctions, modules et projet CLI todo](../07_fonctions_modules/README.md)
