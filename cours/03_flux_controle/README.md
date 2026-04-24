# Chapitre 3 — Flux de contrôle

`if` comme expression, boucles avec valeur de retour, et surtout **`match`** — le super-outil de Rust qui combine switch, pattern matching et exhaustivité compile-time.

## 1. `if`, `else if`, `else` — expressions

```rust
let n = 7;

if n < 5 {
    println!("petit");
} else if n < 10 {
    println!("moyen");
} else {
    println!("grand");
}
```

Parce que `if` est une **expression**, on peut assigner sa valeur :

```rust
let categorie = if n < 5 { "petit" } else { "grand" };
```

Les deux branches doivent retourner le **même type**.

## 2. Boucles

### `loop` — boucle infinie

```rust
loop {
    println!("hello");
    break;                   // sort de la boucle
}
```

`loop` peut retourner une valeur via `break` :

```rust
let resultat = loop {
    let candidat = calcul();
    if candidat > 100 {
        break candidat;      // valeur retournée par la boucle
    }
};
```

### `while`

```rust
let mut n = 3;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### `for` — la plus utilisée

```rust
for i in 0..5 {               // 0, 1, 2, 3, 4
    println!("{i}");
}

for i in 0..=5 {              // 0..=5 inclus
    println!("{i}");
}

let v = vec![10, 20, 30];
for n in &v {                 // itère par référence, v reste utilisable
    println!("{n}");
}

for (i, x) in v.iter().enumerate() {
    println!("{i}: {x}");
}
```

### Étiquettes de boucle

```rust
'outer: for i in 0..5 {
    for j in 0..5 {
        if i * j > 10 {
            break 'outer;       // sort de outer, pas juste inner
        }
    }
}
```

## 3. `match` — pattern matching exhaustif

Le vrai outil de contrôle de Rust. Aussi puissant que le `match` Python, avec en plus l'**exhaustivité vérifiée à la compilation**.

```rust
let x = 3;

match x {
    0 => println!("zéro"),
    1 | 2 => println!("un ou deux"),
    3..=9 => println!("entre 3 et 9"),
    _ => println!("autre"),
}
```

Si vous oubliez un cas, le compilateur **refuse** de compiler. C'est ce qui rend les enums incroyablement safe.

### `match` avec enum

```rust
enum Etat {
    Actif,
    Inactif,
    Suspendu(String),
}

let e = Etat::Suspendu(String::from("paiement"));

match e {
    Etat::Actif => println!("ok"),
    Etat::Inactif => println!("désactivé"),
    Etat::Suspendu(raison) => println!("suspendu : {raison}"),
}
```

Si vous ajoutez `Etat::Banni` plus tard, le compilateur vous avertit de chaque `match` qu'il faut mettre à jour. **C'est une des fonctionnalités préférées des rustacés.**

### Déstructuration de tuples, structs

```rust
let pair = (1, -1);
match pair {
    (0, _) => println!("premier est zéro"),
    (_, 0) => println!("second est zéro"),
    (x, y) if x == y => println!("égaux"),
    (x, y) if x + y == 0 => println!("opposés"),
    _ => println!("autre"),
}
```

### Bindings avec `@`

Capturer **et** matcher :

```rust
let n = 42;

match n {
    x @ 0..=9 => println!("chiffre : {x}"),
    x @ 10..=99 => println!("nombre à deux chiffres : {x}"),
    _ => println!("grand"),
}
```

### Patterns sur références

```rust
let v = vec![1, 2, 3];
match v.first() {               // Option<&i32>
    Some(&n) => println!("{n}"),         // déstructure la référence
    None => println!("vide"),
}
```

## 4. `if let` — raccourci pour un seul cas

Quand vous ne voulez matcher **qu'un cas** :

```rust
let age: Option<u32> = Some(30);

// Verbose
match age {
    Some(n) => println!("{n}"),
    None => (),
}

// Idiomatique
if let Some(n) = age {
    println!("{n}");
}
```

Avec `else` :

```rust
if let Some(n) = age {
    println!("{n}");
} else {
    println!("inconnu");
}
```

## 5. `while let` — boucle conditionnelle sur pattern

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {    // pop retourne Option<T>
    println!("{top}");
}
// boucle s'arrête quand pop retourne None
```

## 6. `let ... else` (1.65+) — early return idiomatique

```rust
fn parse_age(s: &str) -> u32 {
    let Ok(age) = s.parse::<u32>() else {
        println!("age invalide, défaut à 0");
        return 0;
    };
    age        // age est un u32 ici, sorti du pattern
}
```

Beaucoup plus lisible que la version avec `match`. `let else` exige que le bloc `else` **diverge** (`return`, `break`, `panic!`, `continue`).

## 7. Exhaustivité et le wildcard `_`

```rust
let x: Option<i32> = Some(5);

match x {
    Some(n) => println!("{n}"),
    // si on oublie None : compile error !
}
```

Le wildcard `_` matche tout :

```rust
match code {
    200 => "ok",
    404 => "not found",
    _ => "autre",
}
```

Préférez `_` à la fin plutôt que des `else if` en cascade.

## 8. `match` vs `if/else if`

| | `match` | `if/else if` |
|---|---|---|
| Exhaustivité vérifiée | ✅ | ❌ |
| Pattern matching structurel | ✅ | ❌ |
| Lisibilité sur enums | ✅ | ❌ |
| Conditions hétérogènes | ❌ | ✅ |

**Règle** : `match` dès qu'on test sur la **structure** (enum, Option, Result, tuple, struct). `if/else` pour des conditions booléennes hétérogènes.

---

### Piège courant : confondre `==` et `=` dans `if let`

```rust
if let Some(x) = option { ... }    // ✅ pattern matching
if let Some(x) == option { ... }   // ❌ erreur compile
```

`let` dans `if let`/`while let` n'est pas une comparaison, c'est un binding. Pas de `==`.

---

### Piège courant : match qui "consomme"

```rust
let s = String::from("hello");

match s {
    s => println!("{s}"),         // move de s dans ce bras
}

println!("{s}");                    // ❌ s a été moved
```

Pour matcher sans consommer :

```rust
match &s {                           // emprunte
    s => println!("{s}"),
}
```

ou utiliser `ref` dans le pattern. En pratique, prenez le réflexe de matcher sur `&value` pour les types non-Copy.

---

### Sous le capot : `match` est compilé en jump table

Pour les entiers contigus, `match` est compilé en **jump table** (aussi efficace qu'un switch C). Pour les enums, c'est une série de comparaisons + sauts optimisés. Pas de surcoût par rapport à un if/else chaîné — souvent meilleur.

---

## À retenir

- `if` et `loop` sont des expressions, retournent des valeurs.
- `match` exhaustif → la stdlib et les libs l'exploitent pour garantir la sûreté.
- `if let` / `while let` : sucre pour matcher un seul cas.
- `let else` pour early return.
- Étiquettes `'outer:` pour sortir de boucles imbriquées.
- Préférez matcher sur `&value` pour ne pas consommer.

---

➡️ [Chapitre 4 — Ownership & borrowing (bases)](../04_ownership/README.md)
