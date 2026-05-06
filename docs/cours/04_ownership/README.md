# Chapitre 4 — Ownership & borrowing

Le chapitre le plus important du tutorial. **Ownership** est le mécanisme qui permet à Rust d'être memory-safe sans garbage collector. Une fois ce modèle mental bien ancré, tout le reste du langage devient logique.

## 1. La règle fondamentale

Chaque valeur a **un seul propriétaire** à la fois. Quand le propriétaire sort de portée, la valeur est **droppée** (mémoire libérée automatiquement).

```rust
{
    let s = String::from("hello");   // s est propriétaire
    // utilise s...
}                                    // s sort de portée → drop automatique
// s n'existe plus
```

Pas de `free()`, pas de GC, pas de double-free. Le compilateur génère le `drop` au bon endroit.

## 2. Move — le transfert de propriété

En Rust, l'affectation **déplace** la propriété (move) pour les types heap-allocated :

```rust
let s1 = String::from("hello");
let s2 = s1;                  // move : s1 n'existe plus

println!("{s1}");             // ❌ erreur : s1 a été moved
println!("{s2}");             // ✅
```

Pourquoi ? Si `s1` et `s2` pointaient vers le même buffer, qui ferait le `drop` ? Rust interdit l'ambiguïté au moment de la compilation.

### Move dans les appels de fonction

```rust
fn consomme(s: String) {
    println!("{s}");
}                             // s est droppée ici

let s = String::from("hello");
consomme(s);                  // move → s appartient maintenant à la fonction
println!("{s}");              // ❌ erreur
```

### Move au retour de fonction

```rust
fn produire() -> String {
    let s = String::from("hello");
    s                         // move vers l'appelant
}

let s = produire();           // s possède maintenant "hello"
```

## 3. Clone — copie profonde explicite

Quand vous voulez vraiment deux copies indépendantes :

```rust
let s1 = String::from("hello");
let s2 = s1.clone();          // copie profonde du buffer heap

println!("{s1} et {s2}");     // ✅ les deux sont valides
```

`.clone()` est **coûteux** (allocation + copie). Préférez le borrowing (§4) quand possible.

## 4. Types `Copy` — pas de move

Les types entièrement stack-allocated implémentent `Copy`. Pour eux, l'affectation **copie** silencieusement la valeur (comme en C/Python) :

```rust
let x = 5;
let y = x;                    // copie, pas move
println!("{x}");              // ✅ x est toujours valide
```

Types `Copy` : `i8`–`i128`, `u8`–`u128`, `f32`, `f64`, `bool`, `char`, tuples de types `Copy`, arrays de types `Copy`.

**Non-Copy** : `String`, `Vec<T>`, `Box<T>`, tout ce qui alloue sur le heap.

## 5. Borrowing — emprunter sans transférer

Le borrowing permet de **référencer** une valeur sans en prendre ownership. La valeur n'est pas droppée quand la référence sort de portée.

### Référence immuable `&T`

```rust
fn longueur(s: &String) -> usize {
    s.len()
}                             // s est une référence, pas droppée

let s = String::from("hello");
let n = longueur(&s);         // emprunt immuable
println!("{s}");              // ✅ s est toujours valide
```

La référence `&s` ne possède pas la valeur — elle l'**emprunte**. Comme Python vous passe un objet sans transférer la variable.

### Référence mutable `&mut T`

```rust
fn ajouter(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
ajouter(&mut s);
println!("{s}");              // "hello world"
```

## 6. Les règles du borrow checker

Ces deux règles sont vérifiées à la **compilation** :

> **Règle 1** : vous pouvez avoir soit **plusieurs `&T`** (lecteurs), soit **exactement un `&mut T`** (éditeur), jamais les deux en même temps.
>
> **Règle 2** : une référence ne peut pas **survivre à sa source**.

### Règle 1 — pas de lecteur + éditeur simultanés

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;                  // ✅ deux lecteurs
println!("{r1} {r2}");

let r3 = &mut s;              // ✅ maintenant les lecteurs ne sont plus utilisés
r3.push_str("!");
```

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;              // ❌ r1 (lecteur) est encore vivant
println!("{r1}");
```

Le compilateur calcule la **durée de vie effective** (NLL — Non-Lexical Lifetimes, stabilisé en 2018). Une référence "meurt" à sa dernière utilisation, pas à la fermeture du bloc.

### Règle 2 — pas de dangling reference

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s                         // ❌ s sera droppée, &s serait un pointeur fantôme
}
```

En C, ce code compile et donne un dangling pointer. En Rust, erreur de compilation garantie.

## 7. Slices — références vers une partie d'une collection

Une **slice** est une vue (référence) sur une séquence contiguë en mémoire.

### String slices `&str`

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];   // vue sur les bytes 0..5
let world: &str = &s[6..11];
```

`&str` que vous avez vu au chapitre 2 **est** une string slice. Les littéraux `"hello"` sont des `&'static str` — vues vers de la ROM compile-time.

```rust
fn premier_mot(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

let s = String::from("hello world");
let mot = premier_mot(&s);
// s.clear();                 // ❌ interdit : mot emprunte s, donc on ne peut pas modifier s
println!("{mot}");
```

Le borrow checker empêche `s.clear()` tant que `mot` est utilisé — exactement ce qu'on veut.

### Array slices `&[T]`

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];  // [2, 3]

fn somme(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

somme(&a);                     // passe toute la slice
somme(&a[1..3]);               // passe une sous-slice
```

**Règle** : préférez `&[T]` à `&Vec<T>` en paramètre — plus flexible (accepte tout ce qui peut se coercer en slice).

## 8. Ownership et les structures

Quand un `struct` contient des champs qui possèdent des données, il les possède lui aussi. Move/clone s'appliquent de la même façon.

```rust
struct Etudiant {
    nom: String,
    age: u32,
}

let e1 = Etudiant { nom: String::from("Alice"), age: 20 };
let e2 = e1;                   // move : e1 n'est plus accessible
// println!("{}", e1.nom);     // ❌
```

### Struct update syntax

```rust
let e3 = Etudiant {
    nom: String::from("Bob"),
    ..e2                       // copie les autres champs de e2
};
// Attention : si e2 a des champs non-Copy, ils sont moved
// e2.age est Copy (u32), ok. e2.nom serait moved si on ne l'overridait pas.
```

## 9. Résumé du modèle mental

| Opération | Effet sur l'original | Coût |
|---|---|---|
| `let b = a` (type non-Copy) | `a` invalide (move) | ~gratuit |
| `let b = a.clone()` | `a` valide | allocation |
| `let b = &a` | `a` emprunté immuable | ~gratuit |
| `let b = &mut a` | `a` emprunté mutable | ~gratuit |
| `let b = a` (type Copy) | `a` valide (copie) | ~gratuit |

## 10. Erreurs typiques et comment les lire

### "use of moved value"

```
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:5:20
   |
3  |     let s = String::from("hello");
4  |     let _s2 = s;                     // move ici
5  |     println!("{s}");                 // utilisation après move
   |               ^ value borrowed here after move
```

**Fix** : soit `s.clone()` à la ligne 4, soit passer `&s` là où vous n'avez pas besoin de posséder.

### "cannot borrow as mutable because it is also borrowed as immutable"

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

**Fix** : s'assurer que la référence immuable est morte (dernière utilisation) avant de créer la référence mutable.

### "returns a reference to data owned by the current function"

```
error[E0515]: cannot return reference to local variable `s`
```

**Fix** : retourner le `String` directement (transfert d'ownership) au lieu d'une référence.

---

### Sous le capot : le borrow checker est purement statique

Pas d'overhead runtime. Le borrow checker analyse le graphe de flot du programme à la compilation et calcule les durées de vie. Aucune instruction supplémentaire n'est générée — les `&T` et `&mut T` deviennent de simples pointeurs en assembleur. La sécurité est entièrement à la compilation.

---

### Pourquoi pas un GC ?

Un GC interrompt le programme pour scanner la mémoire (pause GC), consomme de la RAM supplémentaire, et n'est pas prévisible. Les systèmes temps-réel et les bibliothèques C-compatibles ne peuvent pas se permettre de GC. L'ownership donne les mêmes garanties de sécurité que Java/Go, avec un coût proche du C.

---

## À retenir

- **Chaque valeur a un propriétaire unique**. Drop automatique à la sortie de portée.
- **Move** par défaut pour les types heap. **Copy** pour les primitifs.
- **`&T`** : emprunt immuable. **`&mut T`** : emprunt mutable exclusif.
- **Règle borrow checker** : N lecteurs OR 1 éditeur, jamais les deux.
- **Slices** (`&str`, `&[T]`) : références vers une partie d'une collection. Pas d'allocation.
- Préférez `&str` / `&[T]` aux `&String` / `&Vec<T>` en signature de fonction.

---

➡️ [Chapitre 5 — Structs, enums et types algébriques](../05_structures_donnees/README.md)
