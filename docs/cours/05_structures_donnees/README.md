# Chapitre 5 — Structs, enums et types algébriques

Les types composites de Rust. Les **structs** regroupent des données ; les **enums** expriment des alternatives exclusives. Ensemble, ils forment les **types algébriques** qui font de Rust un langage à typage très expressif.

## 1. Structs — produit de types

Un struct regroupe des champs nommés :

```rust
struct Point {
    x: f64,
    y: f64,
}

let p = Point { x: 1.0, y: 2.0 };
println!("{}", p.x);
```

### Struct tuple

Quand les noms de champs sont moins importants que leur position :

```rust
struct Couleur(u8, u8, u8);   // RGB

let blanc = Couleur(255, 255, 255);
println!("{}", blanc.0);
```

### Struct unitaire

Struct sans champs — utile pour les traits (Ch. 9) :

```rust
struct Marqueur;
```

### `#[derive]` — implémentations automatiques

```rust
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

let r = Rectangle { largeur: 10.0, hauteur: 5.0 };
println!("{r:?}");                       // Debug
let r2 = r.clone();                      // Clone
assert_eq!(r, r2);                       // PartialEq
```

`#[derive]` génère des implémentations automatiques pour les traits courants. Uniquement si tous les champs les implémentent aussi.

### Méthodes avec `impl`

```rust
impl Rectangle {
    // Méthode associée (constructeur conventionnel)
    fn new(largeur: f64, hauteur: f64) -> Rectangle {
        Rectangle { largeur, hauteur }    // shorthand si nom = nom du champ
    }

    // Méthode d'instance — &self : emprunt immuable
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    // Méthode mutante — &mut self
    fn agrandir(&mut self, facteur: f64) {
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }

    // Consomme self (move)
    fn vers_carre(self) -> Rectangle {
        let cote = self.largeur.min(self.hauteur);
        Rectangle::new(cote, cote)
    }
}

let mut r = Rectangle::new(10.0, 5.0);
println!("{}", r.aire());                // 50.0
r.agrandir(2.0);
let carre = r.vers_carre();
```

**Convention** : `new` (ou `with_*`) pour les constructeurs, pas de constructeur spécial en Rust.

## 2. Enums — somme de types

Un enum est l'une **ou** l'autre variante, mutuellement exclusive. Chaque variante peut avoir des données associées différentes.

```rust
enum Forme {
    Cercle(f64),                        // rayon
    Rectangle { largeur: f64, hauteur: f64 },
    Triangle(f64, f64, f64),            // trois côtés
    Point,                              // sans donnée
}
```

### `match` exhaustif sur enum

```rust
fn aire(f: &Forme) -> f64 {
    match f {
        Forme::Cercle(r) => std::f64::consts::PI * r * r,
        Forme::Rectangle { largeur, hauteur } => largeur * hauteur,
        Forme::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        },
        Forme::Point => 0.0,
    }
}
```

Si vous ajoutez `Forme::Ellipse` plus tard, chaque `match` qui ne la couvre pas devient une **erreur de compilation**. Votre refactoring est guidé par le compilateur.

### Méthodes sur enum

```rust
impl Forme {
    fn est_reguliere(&self) -> bool {
        matches!(self, Forme::Cercle(_) | Forme::Point)
    }
}
```

## 3. `Option<T>` et `Result<T, E>` — enums de la stdlib

Ces deux types sont des enums ordinaires. Tout ce qu'on a vu s'applique.

```rust
// Dans la stdlib :
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Méthodes clés d'`Option`

```rust
let opt: Option<i32> = Some(42);

opt.unwrap()                    // i32, panic si None
opt.unwrap_or(0)                // i32, 0 si None
opt.unwrap_or_else(|| calcul()) // lazy default
opt.is_some()                   // bool
opt.is_none()                   // bool
opt.map(|n| n * 2)              // Option<i32> → transforme la valeur si Some
opt.and_then(|n| if n > 0 { Some(n) } else { None })  // flatMap
opt.filter(|n| *n > 0)          // None si prédicat échoue
opt.as_ref()                    // Option<&i32> — emprunte sans consommer
```

### Méthodes clés de `Result`

```rust
let res: Result<i32, String> = Ok(42);

res.unwrap()                    // i32, panic si Err
res.unwrap_or(0)                // i32, 0 si Err
res.is_ok()                     // bool
res.is_err()                    // bool
res.map(|n| n * 2)              // Result<i32, E> → transforme Ok
res.map_err(|e| format!("{e}")) // Result<T, String> → transforme Err
res.ok()                        // Option<i32> — ignore l'erreur
```

### L'opérateur `?`

L'étoile de la gestion d'erreurs en Rust. Dans une fonction retournant `Result<T, E>` (ou `Option<T>`), `?` :

1. Si `Ok(v)` / `Some(v)` : extrait `v` et continue.
2. Si `Err(e)` / `None` : **retourne immédiatement** avec `Err(e.into())` / `None`.

```rust
use std::num::ParseIntError;

fn parser_et_doubler(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;   // retourne Err si échec
    Ok(n * 2)
}

fn chaîne(s: &str) -> Result<i32, ParseIntError> {
    let a = parser_et_doubler(s)?;
    let b = parser_et_doubler("21")?;
    Ok(a + b)
}
```

Sans `?`, il faudrait un `match` ou `.unwrap_or_else` à chaque appel. `?` permet d'écrire du code heureux (happy path) lisible.

### `?` avec `Option`

```rust
fn premier_chiffre(s: &str) -> Option<u32> {
    let c = s.chars().next()?;    // None si vide
    c.to_digit(10)                // Option<u32>
}
```

## 4. `Vec<T>` — tableau dynamique

`Vec<T>` est la collection dynamique la plus utilisée.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v2 = vec![1, 2, 3];          // macro de création

println!("{}", v[1]);             // accès par index — panique si hors bornes
println!("{:?}", v.get(1));       // Option<&i32> — sûr
v.pop();                          // Option<i32>
v.len();
v.is_empty();
v.contains(&2);
v.sort();
v.dedup();                        // supprime les doublons consécutifs
```

### Itération

```rust
for x in &v {
    println!("{x}");              // &i32
}

for x in &mut v {
    *x *= 2;                      // mutation par déréférence
}

for x in v {                      // consomme v
    println!("{x}");
}
```

## 5. `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, u32> = HashMap::new();
scores.insert(String::from("Alice"), 10);
scores.insert(String::from("Bob"), 20);

scores.get("Alice")               // Option<&u32>
scores.contains_key("Bob")        // bool
scores.remove("Bob")              // Option<u32>

// Insérer seulement si absent
scores.entry(String::from("Alice")).or_insert(0);
// Incrémenter ou initialiser
*scores.entry(String::from("Alice")).or_insert(0) += 1;

for (nom, score) in &scores {
    println!("{nom}: {score}");
}
```

### Ownership dans HashMap

Quand vous insérez une valeur non-Copy (comme `String`), la hashmap en devient propriétaire :

```rust
let cle = String::from("Alice");
scores.insert(cle, 10);
// println!("{cle}");  // ❌ cle a été moved dans la hashmap
```

## 6. Autres collections stdlib

| Collection | Cas d'usage |
|---|---|
| `Vec<T>` | tableau dynamique, accès par index |
| `VecDeque<T>` | file double (push/pop des deux côtés) |
| `LinkedList<T>` | rare — préférez `VecDeque` |
| `HashMap<K,V>` | clé-valeur non ordonné |
| `BTreeMap<K,V>` | clé-valeur ordonné par clé |
| `HashSet<T>` | ensemble non ordonné |
| `BTreeSet<T>` | ensemble ordonné |
| `BinaryHeap<T>` | file de priorité (max-heap) |

```rust
use std::collections::{HashSet, BTreeMap};

let mut ensemble = HashSet::new();
ensemble.insert("rust");
ensemble.insert("rust");           // doublon ignoré
assert_eq!(ensemble.len(), 1);

let mut ordonné: BTreeMap<&str, i32> = BTreeMap::new();
ordonné.insert("z", 1);
ordonné.insert("a", 2);
for (k, v) in &ordonné {          // itère en ordre lexicographique
    println!("{k}: {v}");
}
```

---

### Piège courant : enum non-exhaustif dans un match

```rust
enum Couleur { Rouge, Vert, Bleu }

fn nom(c: Couleur) -> &'static str {
    match c {
        Couleur::Rouge => "rouge",
        Couleur::Vert  => "vert",
        // ❌ oubli de Bleu → erreur de compilation
    }
}
```

Le compilateur vous **force** à traiter tous les cas. Jamais de surprise en prod.

---

### Sous le capot : taille des enums

Un enum Rust est un **tagged union** : un tag (discriminant, généralement 1 byte) + l'espace pour la plus grande variante. Le compilateur peut optimiser : `Option<&T>` fait la même taille qu'un `&T` car `None` est représenté par l'adresse nulle (null pointer optimization).

```rust
use std::mem::size_of;
println!("{}", size_of::<Option<&i32>>());  // 8 (même taille qu'un pointeur)
println!("{}", size_of::<Option<i32>>());   // 8 (4 pour i32 + 4 pour le tag)
```

---

## À retenir

- **Struct** : produit de types (tous les champs présents). `impl` pour les méthodes.
- **Enum** : somme de types (une variante active). `match` exhaustif obligatoire.
- `#[derive(Debug, Clone, PartialEq)]` pour 90% des besoins courants.
- `Option<T>` et `Result<T,E>` sont des enums. Maîtrisez `.map()`, `.and_then()`, `.unwrap_or()`.
- **`?`** : propagation élégante des erreurs/absences.
- `Vec<T>` pour les listes, `HashMap<K,V>` pour les dictionnaires.

---

➡️ [Chapitre 6 — Gestion des erreurs](../06_gestion_erreurs/README.md)
