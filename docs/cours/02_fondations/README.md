# Chapitre 2 — Primitifs, variables, mutabilité

Les bases syntaxiques. **Immuabilité par défaut**, deux types de chaînes (`&str` et `String`, source n°1 de confusion pour les débutants), shadowing, et conversions explicites.

## 1. `let`, `mut`, shadowing

```rust
let x = 5;             // immuable
x = 6;                 // ❌ erreur de compilation

let mut y = 5;         // mutable
y = 6;                 // ✅

const MAX: u32 = 1000; // constante compile-time
```

- Immuable par défaut, mutable si nécessaire. Force à réfléchir aux invariants.
- `const` : valeur connue à la compilation, type **obligatoire**, convention `SCREAMING_SNAKE_CASE`.
- `static` : existe au même endroit mémoire pendant toute la durée du programme. Rare. Préférez `const`.

### Shadowing — redéfinir le même nom

```rust
let x = 5;
let x = x + 1;         // 6
let x = "five";        // OK : shadowing change même le type
```

Différent de `mut` : le shadowing **crée une nouvelle variable**. Utile pour transformer une valeur tout en gardant un nom court.

## 2. Primitifs numériques

| Taille | Signés | Non-signés | Notes |
|---|---|---|---|
| 8 | `i8` | `u8` | |
| 16 | `i16` | `u16` | |
| 32 | `i32` | `u32` | défaut des littéraux entiers |
| 64 | `i64` | `u64` | |
| 128 | `i128` | `u128` | rares |
| arch | `isize` | `usize` | taille d'un pointeur (indices, longueurs) |

Flottants : `f32`, `f64` (défaut).

### Littéraux

```rust
let decimal = 98_222;             // underscores pour lisibilité
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A';                  // u8 littéral

let suffixed = 42_u64;            // spécifie le type
let float = 3.14_f32;
```

### Overflow

- **Debug** : panic au runtime si overflow.
- **Release** : wraps silencieusement (comportement défini, pas UB).

Contrôle explicite :

```rust
x.checked_add(1)      // Option<T> : None si overflow
x.wrapping_add(1)     // wrap
x.saturating_add(1)   // sature à MAX
x.overflowing_add(1)  // (valeur, bool)
```

**Toujours** utiliser ces variantes quand l'overflow est plausible (arithmétique modulaire, compteurs non bornés). `+` tout nu est pour le cas "je garantis que ça ne déborde jamais".

## 3. `bool` et `char`

```rust
let actif: bool = true;
let lettre: char = 'é';     // Unicode scalar, 4 octets
let emoji: char = '🦀';     // 1 char Rust = 1 scalaire Unicode (pas 1 byte)
```

**Attention** : `char` ≠ `u8`. Un `char` est un **Unicode scalar value** (4 octets). Pour manipuler des octets bruts, utilisez `u8`.

## 4. Tuples et arrays

```rust
// Tuple
let pair: (i32, &str) = (42, "answer");
let (n, s) = pair;                  // déstructuration
pair.0;                              // accès indexé

// Array (taille fixe, stack)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];                // dix zéros
let first = arr[0];
// Borne vérifiée à l'exécution : arr[99] panique
```

Les arrays sont **de taille fixe compile-time**. Pour une taille dynamique, `Vec` (Ch. 5).

## 5. `&str` vs `String` — le piège n°1

Rust a **deux** types pour manipuler du texte. Comprendre leur différence est critique.

### `String` — propriétaire, mutable

```rust
let mut s = String::from("bonjour");
s.push_str(", monde");        // s : "bonjour, monde"
s.push('!');
```

- **Heap allocated**, `Drop` libère la mémoire automatiquement.
- Croissable (capacité réalloue à la demande).
- Vous en êtes **propriétaire**.

### `&str` — vue (slice de string)

```rust
let litteral: &str = "bonjour";    // pointe vers de la RAM read-only
let s = String::from("hello world");
let tranche: &str = &s[0..5];      // "hello" — référence dans s
```

- **Pas propriétaire** : référence vers un `String` ou un littéral.
- Immuable.
- Sa durée de vie est liée à celle de son "propriétaire".

### Conversions

```rust
let s: String = String::from("a");
let r: &str = &s;                   // &String → &str : deref coercion, gratuit

let s2: String = r.to_string();     // &str → String : alloue
let s3: String = r.to_owned();      // idem, plus explicite sémantiquement
```

### Règle de signatures

```rust
// ❌ Inflexible : force à passer un String
fn traiter(s: String) { ... }

// ✅ Accepte &str ET &String (via coercion)
fn traiter(s: &str) { ... }
```

**Règle** : paramètres en `&str`, retours en `String` (si on crée).

### Indexer une string ?

```rust
let s = "bonjour";
s[0];        // ❌ erreur de compilation
```

Rust **refuse** `s[i]` car les strings sont UTF-8, et indexer par octet n'a pas de sens ("é" fait 2 octets). Utilisez :

```rust
s.chars().nth(0)            // Option<char>
s.bytes().nth(0)            // Option<u8>
&s[0..3]                    // slice de bytes — PANIQUE si vous coupez un char multi-byte
```

## 6. Expressions vs statements

Rust est **expression-oriented**. Presque tout est une expression qui retourne une valeur.

```rust
let x = if condition { 5 } else { 10 };    // if est une expression

let y = {
    let tmp = 2;
    tmp * 10                  // pas de ;  → c'est la valeur du bloc
};

fn carre(n: i32) -> i32 {
    n * n                     // pas de return ; dernière expression = retour
}
```

**Règle de syntaxe** : un `;` à la fin transforme une expression en statement (valeur jetée, retourne `()`, le "unit type").

```rust
fn f() -> i32 {
    let x = 5;
    x + 1                  // ✅ retourne 6
}

fn g() -> i32 {
    let x = 5;
    x + 1;                 // ❌ retourne () alors que la signature promet i32
}
```

## 7. Conversions : `as`, `From`/`Into`, `TryFrom`

### `as` — cast explicite entre primitifs

```rust
let x: i32 = 1000;
let y: i16 = x as i16;       // peut perdre des bits, silencieux
let z: f64 = x as f64;       // OK
let u: u8 = 300 as u8;       // 44 (300 mod 256)
```

`as` est simple mais **ne vérifie pas** l'overflow. À utiliser quand vous savez que la conversion est sûre.

### `From` / `Into` — conversions entre types

Pattern idiomatique pour la plupart des conversions :

```rust
let s = String::from("hello");   // From<&str> for String
let n: i64 = i64::from(42_i32);  // From<i32> for i64 (toujours safe)

// Into est le dual, implicite
let s: String = "hello".into();
```

Règle : si `impl From<A> for B`, alors `A.into()` retourne `B` (gratuit).

### `TryFrom` — conversions faillibles

```rust
use std::convert::TryFrom;

let x: i32 = 300;
let y: u8 = u8::try_from(x)?;       // Result<u8, TryFromIntError>
```

Pour des conversions pouvant échouer (overflow, format invalide...).

**Règle** : préférez `From`/`Into` à `as`. Utilisez `as` uniquement entre primitifs quand la troncature est intentionnelle.

## 8. Inférence de type

```rust
let x = 5;                    // i32 par défaut
let y = 5.0;                  // f64 par défaut
let v = vec![1, 2, 3];        // Vec<i32>
let parsed: u32 = "42".parse().unwrap();   // annotation obligatoire ici
```

Rust infère le type depuis l'usage. Parfois il a besoin d'aide (ex. `parse` peut retourner n'importe quel numérique).

```rust
let parsed = "42".parse::<u32>().unwrap();   // turbofish ::<> sur la méthode
```

## 9. `Option<T>` — absence de valeur

Rust n'a **pas de `null`**. L'absence se modélise explicitement :

```rust
enum Option<T> {
    Some(T),
    None,
}

let age: Option<u32> = Some(30);
let rien: Option<u32> = None;
```

Pour utiliser la valeur, il faut **explicitement** gérer le cas `None` :

```rust
if let Some(n) = age {
    println!("age : {n}");
}

match age {
    Some(n) => println!("age : {n}"),
    None => println!("inconnu"),
}

age.unwrap_or(0);       // 0 si None
age.unwrap();           // PANIC si None (à éviter sauf dans les tests / code-garanti)
```

Plus de détails au Ch. 6 (Result/Option) et Ch. 10 (méthodes d'Option/Result).

## 10. `println!`, `format!`, `dbg!`

```rust
println!("Hello, {}!", "world");              // positionnel
println!("x = {}", x);                         // Display
println!("x = {:?}", x);                       // Debug
println!("x = {:#?}", x);                      // Pretty-print Debug
println!("{x}");                               // capture la variable (1.58+)
println!("{x:?}");                             // Debug avec capture

let s = format!("Hello, {name}");              // retourne String
eprintln!("error!");                           // stderr

dbg!(x);     // imprime "[src/main.rs:5] x = 42" et retourne x (pass-through)
```

`dbg!` est l'outil de debug rapide. Sa sortie inclut le fichier, la ligne, l'expression et sa valeur.

---

### Piège courant : `String` partout par défaut

Débutant venant de Python/Java :

```rust
fn saluer(nom: String) -> String {    // ❌
    format!("hello {nom}")
}

saluer(String::from("Alice"));         // ok
saluer("Bob");                         // ❌ erreur de compilation
```

Préférez `&str` :

```rust
fn saluer(nom: &str) -> String {       // ✅ accepte les deux
    format!("hello {nom}")
}
```

---

### Sous le capot : un `&str` fait 16 octets

Sur une machine 64 bits, un `&str` est une paire `(pointer, length)` de 16 octets. C'est un **slice**, pas un pointeur nu. Le `length` permet à Rust de garantir les bornes sans traverser la string.

Un `String` est (en interne) `Vec<u8>` : `(pointer, length, capacity)` = 24 octets, plus le buffer heap. Toujours UTF-8 valide — Rust refuse d'en construire un avec des bytes invalides.

---

## À retenir

- `let` immuable par défaut, `mut` pour muter.
- Shadowing = redéfinir (utile pour transformer et garder un nom court).
- `String` possède ; `&str` référence.
- **Paramètres en `&str`, retours en `String`**.
- `{x}` dans `println!` depuis 1.58.
- Pas de `null` : `Option<T>` explicite.
- `as` : cast simple entre primitifs ; `From`/`Into` : conversions entre types ; `TryFrom` : faillible.
- Overflow : debug = panic, release = wrap (pour changer : `checked_*`, `wrapping_*`, `saturating_*`).

---

➡️ [Chapitre 3 — Flux de contrôle](../03_flux_controle/README.md)
