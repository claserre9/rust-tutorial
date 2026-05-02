# Chapitre 15 — Macros et patterns avancés

Les macros Rust sont hygiéniques et opèrent sur l'AST — bien plus puissantes que les macros C. Ce chapitre couvre les macros déclaratives (`macro_rules!`) et procédurales (derive, attribute), plus les patterns de conception idiomatiques.

## 1. `macro_rules!` — macros déclaratives

Correspondance de pattern sur les tokens :

```rust
macro_rules! dire {
    ($msg:expr) => {
        println!("{}", $msg);
    };
    ($fmt:expr, $($arg:expr),+) => {
        println!($fmt, $($arg),+);
    };
}

dire!("bonjour");
dire!("x = {}", 42);
```

### Fragments

| Fragment | Correspond à |
|---|---|
| `$e:expr` | expression |
| `$t:ty` | type |
| `$i:ident` | identifiant |
| `$p:pat` | pattern |
| `$s:stmt` | statement |
| `$b:block` | bloc `{ ... }` |
| `$l:literal` | littéral |
| `$tt:tt` | token tree (tout) |

### Répétitions

```rust
macro_rules! vecteur {
    ($($elem:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($elem);)*
            v
        }
    };
}

let v = vecteur![1, 2, 3];
assert_eq!(v, vec![1, 2, 3]);
```

`$(...)*` → zéro ou plusieurs. `$(...)+` → un ou plusieurs. `$(...)?` → zéro ou un.

### Exemple pratique : `assert_matches!`

```rust
macro_rules! assert_matches {
    ($val:expr, $pat:pat) => {
        match $val {
            $pat => {},
            other => panic!("expected {}, got {:?}", stringify!($pat), other),
        }
    };
}

let opt: Option<i32> = Some(42);
assert_matches!(opt, Some(_));
```

## 2. Macros procédurales

Les macros procédurales reçoivent du code Rust sous forme de `TokenStream` et retournent du code transformé.

Trois types :
- **Custom derive** : `#[derive(MonTrait)]`
- **Attribute macros** : `#[ma_macro]` sur items
- **Function-like macros** : `ma_macro!(...)` comme les déclaratives mais plus puissantes

Elles vivent dans une crate séparée avec `proc-macro = true` :

```toml
# macros/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

### Exemple : derive `Describe`

```rust
// macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let nom = &input.ident;
    let description = format!("Je suis un {}", nom);

    let expanded = quote! {
        impl #nom {
            pub fn description() -> &'static str {
                #description
            }
        }
    };

    TokenStream::from(expanded)
}
```

```rust
// Dans votre crate principale
use macros::Describe;

#[derive(Describe)]
struct Robot;

println!("{}", Robot::description());  // "Je suis un Robot"
```

## 3. Patterns de conception idiomatiques Rust

### State machine par types

Encoder les états comme des types distincts pour invalider les transitions illégales à la compilation :

```rust
struct Brouillon;
struct EnRevue;
struct Publié;

struct Article<État> {
    contenu: String,
    _état: std::marker::PhantomData<État>,
}

impl Article<Brouillon> {
    fn new(contenu: String) -> Article<Brouillon> {
        Article { contenu, _état: std::marker::PhantomData }
    }

    fn soumettre(self) -> Article<EnRevue> {
        Article { contenu: self.contenu, _état: std::marker::PhantomData }
    }
}

impl Article<EnRevue> {
    fn approuver(self) -> Article<Publié> {
        Article { contenu: self.contenu, _état: std::marker::PhantomData }
    }
    fn rejeter(self) -> Article<Brouillon> {
        Article { contenu: self.contenu, _état: std::marker::PhantomData }
    }
}

impl Article<Publié> {
    fn contenu(&self) -> &str { &self.contenu }
}

// Article<Brouillon> n'a pas de méthode contenu() → impossible d'accéder au contenu non publié
```

### Extension trait

Ajouter des méthodes à des types existants sans les modifier :

```rust
trait IterExt: Iterator {
    fn moyenne(self) -> Option<f64>
    where
        Self: Sized,
        Self::Item: Into<f64>,
    {
        let mut sum = 0f64;
        let mut count = 0usize;
        for val in self {
            sum += val.into();
            count += 1;
        }
        if count > 0 { Some(sum / count as f64) } else { None }
    }
}

impl<I: Iterator> IterExt for I {}   // implémente pour tous les itérateurs

let v = vec![1.0, 2.0, 3.0, 4.0];
println!("{:?}", v.iter().copied().moyenne());  // Some(2.5)
```

### `PhantomData<T>` — marqueur de type fantôme

Quand un struct générique ne contient pas directement T mais doit se comporter comme s'il le possédait :

```rust
use std::marker::PhantomData;

struct Proprietaire<T> {
    pointeur: *const T,
    _fantôme: PhantomData<T>,   // indique au compilateur que Proprietaire possède T
}

// Sans PhantomData, la variance et les drop checks seraient incorrects
```

## 4. `Deref` et `DerefCoercions`

```rust
use std::ops::Deref;

struct MonBox<T>(T);

impl<T> Deref for MonBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

let b = MonBox(String::from("hello"));
println!("{}", b.len());    // deref coercion : MonBox<String> → String → str
```

Chaîne de coercions automatiques : `Box<T>` → `T`, `String` → `str`, `Vec<T>` → `[T]`.

---

### Sous le capot : hygiène des macros

Rust garantit que les identifiants introduits par une macro ne "fuient" pas dans le code appelant :

```rust
macro_rules! macro_propre {
    () => {
        let x = 42;   // ce x n'entre pas en conflit avec un x de l'appelant
    };
}

let x = 1;
macro_propre!();
println!("{x}");   // affiche 1, pas 42
```

Contrairement aux macros C (`#define`), les macros Rust `macro_rules!` sont hygiéniques par défaut.

---

## À retenir

- `macro_rules!` pour les transformations syntaxiques simples et répétitives.
- Les macros procédurales opèrent sur l'AST — `syn` + `quote` pour les écrire.
- State machine par types : invalider les transitions illégales à la compilation.
- Extension traits : ajouter des méthodes à des types existants.
- `PhantomData<T>` pour les structs génériques sans champ T.

---

➡️ [Chapitre 16 — Web avec Axum — Projet API REST](../16_dev_web_axum/README.md)
