# Chapitre 10 — Smart pointers

Rust gère la mémoire sans ramasse-miettes, mais certains cas nécessitent des stratégies d'allocation et de partage plus élaborées que les références simples. Les **smart pointers** sont des types qui *se comportent comme des pointeurs* mais ajoutent de la sémantique : allocation sur le tas, ownership partagé, ou mutabilité intérieure.

## 1. `Box<T>` — allocation sur le tas

La forme la plus simple. Un `Box<T>` place `T` sur le tas (heap) et garde un pointeur sur la pile.

```rust
let b = Box::new(5);       // 5 est sur le tas
println!("{b}");           // déref automatique → 5
```

### Cas d'utilisation

**Types de taille inconnue à la compilation — types récursifs :**

```rust
// ❌ Infini : le compilateur ne peut pas calculer la taille
enum Liste {
    Cons(i32, Liste),
    Nil,
}

// ✅ Box brise la récursion : taille connue (pointeur = 8 octets)
enum Liste {
    Cons(i32, Box<Liste>),
    Nil,
}

let l = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Nil))));
```

**Polymorphisme par trait object (préférer `impl Trait` sinon) :**

```rust
fn créer_animal(nom: &str) -> Box<dyn Animal> {
    if nom == "chat" { Box::new(Chat) } else { Box::new(Chien) }
}
```

### `Deref` — déréférencement automatique

`Box<T>` implémente `Deref<Target = T>`, ce qui permet l'**auto-deref** :

```rust
let b = Box::new(String::from("bonjour"));
println!("{}", b.len());   // comme si c'était une &String directement
```

Le compilateur insère autant de `*` que nécessaire : `*b` → `String`, puis `&String` → `&str` via `Deref`.

### `Drop` — destructeur automatique

Quand un `Box` sort de portée, son contenu est libéré automatiquement via le trait `Drop`. Vous pouvez implémenter `Drop` sur vos propres types :

```rust
struct Ressource(String);

impl Drop for Ressource {
    fn drop(&mut self) {
        println!("libération de {}", self.0);
    }
}

{
    let r = Ressource("connexion DB".to_string());
}  // ← "libération de connexion DB" s'affiche ici
```

Pour libérer *avant* la fin de portée : `drop(r)` (fonction libre de la stdlib).

---

## 2. `Rc<T>` — ownership partagé (monothread)

`Rc<T>` (*Reference Counted*) permet à **plusieurs propriétaires** de partager la même valeur. Un compteur de références est incrémenté à chaque clone et décrémenté à chaque drop. La valeur est libérée quand le compteur atteint zéro.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("partagée"));
let b = Rc::clone(&a);           // incrémente le compteur (ne copie pas la donnée)
let c = Rc::clone(&a);

println!("{}", Rc::strong_count(&a));  // 3

drop(b);
println!("{}", Rc::strong_count(&a));  // 2
```

**`Rc` n'est pas thread-safe.** Pour le multithreading, utilisez `Arc<T>` (chapitre 14).

### Limitation : immutabilité

`Rc<T>` ne donne qu'un accès **immuable**. Pour muter, il faut combiner avec `RefCell<T>`.

---

## 3. `RefCell<T>` — mutabilité intérieure

`RefCell<T>` déplace la vérification des règles d'emprunt du **compile-time** au **runtime**. En échange, on peut avoir plusieurs emprunts immuables *ou* un emprunt mutable, même derrière une référence immuable.

```rust
use std::cell::RefCell;

let données = RefCell::new(vec![1, 2, 3]);

// Emprunt immuable
let lecture = données.borrow();
println!("{:?}", *lecture);
drop(lecture);  // libère l'emprunt

// Emprunt mutable
données.borrow_mut().push(4);
println!("{:?}", données.borrow());  // [1, 2, 3, 4]
```

**Violation des règles → panique au runtime** (pas une erreur de compilation) :

```rust
let r1 = données.borrow();
let r2 = données.borrow_mut();  // 💥 panique : déjà emprunté en lecture
```

Utilisez `try_borrow()` / `try_borrow_mut()` pour éviter les paniques.

### Pattern : `Rc<RefCell<T>>`

Ownership partagé **et** mutable — le pattern le plus courant pour les graphes et les structures de données cycliques en monothread :

```rust
use std::rc::Rc;
use std::cell::RefCell;

let partagé = Rc::new(RefCell::new(0));

let a = Rc::clone(&partagé);
let b = Rc::clone(&partagé);

*a.borrow_mut() += 10;
*b.borrow_mut() += 5;

println!("{}", partagé.borrow());  // 15
```

---

## 4. `Weak<T>` — références faibles

Un problème avec `Rc` : les **références cycliques** empêchent la libération (le compteur ne descend jamais à zéro).

```rust
// ❌ Fuite mémoire : a et b se référencent mutuellement
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Nœud {
    valeur: i32,
    enfant: Option<Rc<RefCell<Nœud>>>,
    parent: Option<Weak<RefCell<Nœud>>>,  // ✅ Weak pour casser le cycle
}
```

`Weak<T>` :
- Ne possède pas la valeur (n'incrémente pas le `strong_count`)
- Peut devenir invalide si la valeur est libérée
- Accès via `.upgrade()` → `Option<Rc<T>>`

```rust
let fort = Rc::new(42);
let faible: Weak<i32> = Rc::downgrade(&fort);

println!("{:?}", faible.upgrade());   // Some(42)
drop(fort);
println!("{:?}", faible.upgrade());   // None
```

**Règle** : les propriétaires utilisent `Rc`, les observateurs utilisent `Weak`.

---

## 5. `Arc<T>` — ownership partagé multithread

`Arc<T>` (*Atomically Reference Counted*) est l'équivalent thread-safe de `Rc<T>`. Le compteur de références utilise des opérations atomiques plutôt que de simples incréments.

```rust
use std::sync::Arc;
use std::thread;

let données = Arc::new(vec![1, 2, 3]);

for _ in 0..3 {
    let données = Arc::clone(&données);
    thread::spawn(move || {
        println!("{:?}", données);
    });
}
```

Pour la mutation partagée entre threads, combinez avec `Mutex<T>` (chapitre 14) :

```rust
use std::sync::{Arc, Mutex};

let compteur = Arc::new(Mutex::new(0));
```

---

## 6. Tableau récapitulatif

| Type | Ownership | Threads | Mutabilité | Quand l'utiliser |
|---|---|---|---|---|
| `Box<T>` | Unique | ✅ | Normale | Heap, types récursifs, trait objects |
| `Rc<T>` | Partagé | ❌ | Immuable | Graphes, arbres (monothread) |
| `Rc<RefCell<T>>` | Partagé | ❌ | Intérieure | Partagé et mutable (monothread) |
| `Arc<T>` | Partagé | ✅ | Immuable | Données partagées entre threads |
| `Arc<Mutex<T>>` | Partagé | ✅ | Intérieure | Mutable entre threads |
| `Weak<T>` | Aucun | ❌ | — | Casser les cycles `Rc` |

---

## 7. `Cell<T>` — mutabilité intérieure pour `Copy`

Pour les types `Copy` (entiers, booléens…), `Cell<T>` est plus léger que `RefCell<T>` car il n'a pas besoin de garder trace des emprunts actifs :

```rust
use std::cell::Cell;

let x = Cell::new(5);
x.set(10);
println!("{}", x.get());   // 10
```

---

### Piège courant : `Rc` vs `Arc`

```rust
// ❌ Rc ne peut pas être envoyé entre threads
use std::rc::Rc;
let r = Rc::new(42);
thread::spawn(move || { println!("{r}"); });  // erreur : Rc<i32> n'implémente pas Send

// ✅ Arc est thread-safe
use std::sync::Arc;
let r = Arc::new(42);
thread::spawn(move || { println!("{r}"); });  // OK
```

### Piège courant : double emprunt `RefCell`

```rust
let v = RefCell::new(vec![1, 2]);
let _r1 = v.borrow();          // emprunt immuable actif
v.borrow_mut().push(3);        // 💥 panique : déjà emprunté
```

Libérez toujours les emprunts `RefCell` avant d'en prendre un mutable — utilisez des blocs `{ }` pour délimiter la portée.

---

## À retenir

- `Box<T>` : allocation heap, types récursifs, trait objects — ownership unique.
- `Rc<T>` : ownership partagé monothread — compteur de références.
- `RefCell<T>` : mutabilité intérieure — vérifications à runtime.
- `Rc<RefCell<T>>` : partagé + mutable (monothread).
- `Arc<T>` : partagé multithread — coût légèrement supérieur à `Rc`.
- `Weak<T>` : référence non-propriétaire pour casser les cycles.
- Préférez l'ownership classique et les références : les smart pointers sont pour les cas où c'est *vraiment* nécessaire.

---

➡️ [Chapitre 11 — Traits : polymorphisme et abstractions](../11_traits/README.md)
