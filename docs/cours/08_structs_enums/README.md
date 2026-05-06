# Chapitre 7 — Structs avancés, Display et newtype

Aller plus loin avec les structs : impl de traits standards, pattern newtype, builders, et l'art d'écrire des types Rust idiomatiques.

## 1. Implémenter `Display`

`println!("{}", valeur)` appelle `Display`. Pour vos propres types :

```rust
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1.0, y: 2.5 };
println!("{p}");                      // "(1, 2.5)"
let s = p.to_string();                // Display → ToString automatique
```

`Debug` est dérivable (`#[derive(Debug)]`). `Display` est pour les humains — implémentez-le manuellement avec la sémantique souhaitée.

## 2. Implémenter `PartialEq`, `Eq`, `PartialOrd`, `Ord`

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

let v1 = Version { major: 1, minor: 2, patch: 0 };
let v2 = Version { major: 1, minor: 3, patch: 0 };
assert!(v1 < v2);
```

`derive(Ord)` compare les champs dans l'ordre de déclaration (lexicographique). Pour un ordre personnalisé, implémentez manuellement.

## 3. Implémenter des opérateurs avec `std::ops`

```rust
use std::ops::{Add, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

let a = Vec2 { x: 1.0, y: 2.0 };
let b = Vec2 { x: 3.0, y: 4.0 };
let c = a + b;                        // appelle Add::add
let d = -a;                           // appelle Neg::neg
```

## 4. Pattern newtype — typage fort

Encapsuler un type primitif pour éviter les confusions sémantiques :

```rust
struct Metres(f64);
struct Secondes(f64);

fn vitesse(d: Metres, t: Secondes) -> f64 {
    d.0 / t.0
}

// vitesse(Secondes(10.0), Metres(5.0));  // ❌ erreur de compilation
vitesse(Metres(100.0), Secondes(10.0));   // ✅
```

Le newtype est **zéro coût** — compilé identiquement au type interne. Mais il force le compilateur à vérifier la sémantique.

```rust
impl Metres {
    fn en_km(&self) -> f64 { self.0 / 1000.0 }
}

impl fmt::Display for Metres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}
```

## 5. Pattern builder

Quand un struct a beaucoup de champs optionnels :

```rust
#[derive(Debug)]
struct Requete {
    url: String,
    methode: String,
    timeout_secs: u64,
    corps: Option<String>,
    headers: Vec<(String, String)>,
}

struct RequeteBuilder {
    url: String,
    methode: String,
    timeout_secs: u64,
    corps: Option<String>,
    headers: Vec<(String, String)>,
}

impl RequeteBuilder {
    fn new(url: impl Into<String>) -> RequeteBuilder {
        RequeteBuilder {
            url: url.into(),
            methode: String::from("GET"),
            timeout_secs: 30,
            corps: None,
            headers: Vec::new(),
        }
    }

    fn methode(mut self, m: impl Into<String>) -> RequeteBuilder {
        self.methode = m.into();
        self
    }

    fn timeout(mut self, secs: u64) -> RequeteBuilder {
        self.timeout_secs = secs;
        self
    }

    fn corps(mut self, body: impl Into<String>) -> RequeteBuilder {
        self.corps = Some(body.into());
        self
    }

    fn header(mut self, cle: impl Into<String>, val: impl Into<String>) -> RequeteBuilder {
        self.headers.push((cle.into(), val.into()));
        self
    }

    fn build(self) -> Requete {
        Requete {
            url: self.url,
            methode: self.methode,
            timeout_secs: self.timeout_secs,
            corps: self.corps,
            headers: self.headers,
        }
    }
}

let req = RequeteBuilder::new("https://api.example.com/data")
    .methode("POST")
    .timeout(60)
    .header("Content-Type", "application/json")
    .corps(r#"{"key": "value"}"#)
    .build();
```

Chaque méthode du builder prend `self` par valeur et retourne `self` — permet le chaînage. Pas de `&mut self` ici car chaque étape est une transformation.

## 6. `impl Into<String>` — ergonomie des API

Dans les signatures, `impl Into<String>` accepte à la fois `String` et `&str` :

```rust
fn saluer(nom: impl Into<String>) -> String {
    format!("Bonjour, {}!", nom.into())
}

saluer("Alice");                        // &str → String automatique
saluer(String::from("Bob"));            // String direct
```

Coût : un `.into()` alloue si nécessaire. Utiliser `&str` en paramètre est souvent préférable si vous n'avez pas besoin d'owned String.

## 7. Méthodes `const`

Depuis Rust 1.61, les méthodes peuvent être `const` si leur corps est évaluable à la compilation :

```rust
struct Rgb(u8, u8, u8);

impl Rgb {
    pub const BLANC: Rgb = Rgb(255, 255, 255);
    pub const NOIR: Rgb = Rgb(0, 0, 0);

    pub const fn luminance(&self) -> u8 {
        (self.0 / 3) + (self.1 / 3) + (self.2 / 3)
    }
}

const LUM: u8 = Rgb::BLANC.luminance();   // évalué à la compilation
```

---

### Piège courant : `Clone` vs `Copy`

```rust
#[derive(Clone)]              // clone() explicite
struct Texte(String);

#[derive(Clone, Copy)]        // copie implicite à l'affectation
struct Coord(f32, f32);       // possible seulement si tous les champs sont Copy
```

`String` n'est pas `Copy` → vous ne pouvez pas dériver `Copy` pour un struct qui la contient.

---

### Sous le capot : struct layout

Par défaut, Rust peut réordonner les champs pour minimiser le padding. Utilisez `#[repr(C)]` pour forcer l'ordre déclaré (nécessaire pour FFI — Ch. 14).

```rust
#[repr(C)]
struct CStruct { a: u8, b: u32 }   // 8 bytes avec padding comme en C
struct RStruct { a: u8, b: u32 }   // peut être 5 bytes si Rust réordonne
```

---

## À retenir

- `Display` pour l'affichage humain, `Debug` pour le debug. `Display` → `to_string()` gratuit.
- Newtype = zéro coût + typage sémantique fort.
- Builder pattern pour les structs complexes à configuration variable.
- `impl Into<String>` pour des APIs flexibles.
- `derive(Ord)` compare les champs dans l'ordre de déclaration.

---

➡️ [Chapitre 9 — Lifetimes : durées de vie explicites](../09_lifetimes/README.md)
