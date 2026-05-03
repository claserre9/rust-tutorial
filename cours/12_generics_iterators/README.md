# Chapitre 10 — Génériques et itérateurs avancés

Les génériques permettent d'écrire du code qui fonctionne sur plusieurs types sans sacrifier la performance. Les itérateurs sont le mécanisme idiomatique pour transformer des collections — et l'un des points où Rust surpasse souvent Python en clarté ET performance.

## 1. Fonctions et structs génériques

```rust
fn identite<T>(x: T) -> T { x }

struct Paire<T> {
    premier: T,
    second: T,
}

impl<T: std::fmt::Display> Paire<T> {
    fn afficher(&self) {
        println!("{} et {}", self.premier, self.second);
    }
}

// Implémentation conditionnelle
impl<T: std::fmt::Display + PartialOrd> Paire<T> {
    fn plus_grand(&self) -> &T {
        if self.premier >= self.second { &self.premier } else { &self.second }
    }
}
```

## 2. Contraintes génériques avancées

### Bounds multiples

```rust
fn afficher_et_doubler<T>(x: T) -> T
where
    T: std::fmt::Display + Clone + std::ops::Add<Output = T>,
{
    println!("{x}");
    x.clone() + x
}
```

### Bounds conditionnels sur les implémentations

```rust
use std::fmt;

struct Enveloppe<T>(T);

// Display seulement si T est Display
impl<T: fmt::Display> fmt::Display for Enveloppe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Enveloppe({})", self.0)
    }
}
```

## 3. Itérateurs — le cœur fonctionnel de Rust

Les itérateurs sont **lazy** : aucun calcul n'est fait tant qu'on n'a pas consommé l'itérateur.

```rust
let v = vec![1, 2, 3, 4, 5];

// Rien n'est calculé ici !
let iter = v.iter().map(|x| x * 2).filter(|x| x > &4);

// Le calcul se fait ici (collect consomme)
let résultat: Vec<i32> = iter.collect();   // [6, 8, 10]
```

### Méthodes d'adaptation (lazy)

| Méthode | Effet |
|---|---|
| `.map(f)` | transforme chaque élément |
| `.filter(p)` | garde les éléments où p(e) est vrai |
| `.flat_map(f)` | map + aplatissement (comme Python's `itertools.chain.from_iterable(map(...))`) |
| `.take(n)` | les n premiers |
| `.skip(n)` | saute les n premiers |
| `.take_while(p)` | tant que p est vrai |
| `.skip_while(p)` | saute tant que p est vrai |
| `.enumerate()` | ajoute l'index : (i, &val) |
| `.zip(other)` | combine deux itérateurs en paires |
| `.chain(other)` | concatène deux itérateurs |
| `.peekable()` | permet de lire sans consommer (`.peek()`) |
| `.flatten()` | aplatit un itérateur d'itérateurs |

### Méthodes de consommation (terminales)

| Méthode | Retour |
|---|---|
| `.collect::<C>()` | C (Vec, HashSet, HashMap, String...) |
| `.sum::<T>()` | T |
| `.product::<T>()` | T |
| `.count()` | usize |
| `.fold(init, f)` | valeur accumulée |
| `.reduce(f)` | Option<T> |
| `.any(p)` | bool (court-circuit) |
| `.all(p)` | bool (court-circuit) |
| `.find(p)` | Option<&T> |
| `.position(p)` | Option<usize> |
| `.max()` / `.min()` | Option<T> |
| `.max_by_key(f)` | Option<T> |
| `.for_each(f)` | () |
| `.last()` | Option<T> |
| `.nth(n)` | Option<T> |

### Exemples idiomatiques

```rust
// Somme des carrés des impairs
let somme: i32 = (1..=10)
    .filter(|n| n % 2 != 0)
    .map(|n| n * n)
    .sum();                              // 1 + 9 + 25 + 49 + 81 = 165

// Compter les mots
let texte = "hello world hello rust";
let mut freq: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
texte.split_whitespace()
    .for_each(|mot| *freq.entry(mot).or_insert(0) += 1);

// Flatten imbriqué
let matrices = vec![vec![1, 2], vec![3, 4], vec![5]];
let plat: Vec<i32> = matrices.into_iter().flatten().collect();   // [1,2,3,4,5]

// Zip + collect en HashMap
let clés = vec!["a", "b", "c"];
let vals = vec![1, 2, 3];
let map: std::collections::HashMap<_, _> = clés.into_iter().zip(vals).collect();
```

## 4. `collect` vers différents types

```rust
let v: Vec<i32> = (1..=5).collect();
let set: std::collections::HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();
let s: String = vec!['h', 'e', 'l', 'l', 'o'].into_iter().collect();
let résultat: Result<Vec<i32>, _> = vec!["1", "2", "abc"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();                          // Err si l'un échoue — court-circuit !
```

Le dernier exemple est particulièrement puissant : `collect::<Result<Vec<T>, E>>()` propage automatiquement la première erreur.

## 5. `iter()`, `iter_mut()`, `into_iter()`

```rust
let v = vec![1, 2, 3];

// iter() → &T (emprunte, v reste utilisable)
for x in v.iter() { println!("{x}"); }

// iter_mut() → &mut T (emprunte mutablement)
let mut v2 = v.clone();
for x in v2.iter_mut() { *x *= 2; }

// into_iter() → T (consomme v)
for x in v { println!("{x}"); }
// v n'existe plus

// for x in &v   est du sucre pour for x in v.iter()
// for x in &mut v   est du sucre pour for x in v.iter_mut()
```

## 6. Implémenter `Iterator` pour un type custom

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci { Fibonacci { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let prochain = self.a;
        self.a = self.b;
        self.b = prochain + self.b;
        Some(prochain)                   // infini — pas de None
    }
}

let fib: Vec<u64> = Fibonacci::new().take(10).collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

let premier_sup_100 = Fibonacci::new().find(|&n| n > 100);
// Some(144)
```

## 7. `IntoIterator` — rendre un type itérable

```rust
struct Grille { données: Vec<Vec<i32>> }

impl IntoIterator for Grille {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.données.into_iter().flatten().collect::<Vec<_>>().into_iter()
    }
}

let g = Grille { données: vec![vec![1, 2], vec![3, 4]] };
for x in g { println!("{x}"); }    // utilisable dans for directement
```

## 8. Combinateurs d'erreurs avec itérateurs

```rust
fn parser_lignes(lignes: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    lignes.iter()
        .map(|s| s.trim().parse::<i32>())
        .collect()                       // Result<Vec<i32>, _>
}

// Ignorer les erreurs et garder les succès
fn parser_sans_erreurs(lignes: &[&str]) -> Vec<i32> {
    lignes.iter()
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}
```

---

### Performance : itérateurs vs boucles

Les itérateurs Rust sont souvent **aussi rapides ou plus rapides** que les boucles `for` manuelles. Le compilateur inlinine les closures et LLVM vectorise les boucles homogènes.

```rust
// Ces deux versions génèrent (souvent) le même assembleur :
let s1: i32 = v.iter().sum();
let mut s2 = 0i32;
for x in &v { s2 += x; }
```

Préférez les itérateurs pour la clarté — vous n'abandonnez rien en performance.

---

### Piège : `into_iter()` sur `&Vec<T>`

```rust
let v = vec![1, 2, 3];

for x in &v { ... }           // x : &i32
for x in v.iter() { ... }     // x : &i32  ← même chose

for x in v { ... }            // x : i32, v est consommé
for x in v.into_iter() { ... } // idem
```

`&v.into_iter()` → itère sur `&T` (car `IntoIterator for &Vec<T>` yield `&T`).

---

## À retenir

- Génériques avec bounds → code réutilisable, zéro overhead.
- Itérateurs sont **lazy** — la chaîne ne s'exécute qu'au `.collect()`, `.sum()`, etc.
- `collect::<Result<Vec<T>, E>>()` propage la première erreur.
- `filter_map` = `map` + `filter None` en une passe.
- Implémenter `Iterator` (juste `next`) donne 70+ méthodes gratuitement.

---

➡️ [Chapitre 13 — Tests, benchmarks et qualité](../13_tests_qualite/README.md)
