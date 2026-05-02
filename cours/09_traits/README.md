# Chapitre 9 — Traits : polymorphisme et abstractions

Les traits sont la réponse de Rust à l'héritage d'interface et aux typeclasses. Plus puissants que les interfaces Java/Go, ils permettent dispatch statique ET dynamique, et intègrent profondément avec le système de types.

## 1. Définir et implémenter un trait

```rust
trait Aire {
    fn aire(&self) -> f64;

    // Méthode par défaut — peut être overridée
    fn est_grande(&self) -> bool {
        self.aire() > 100.0
    }
}

struct Cercle { rayon: f64 }
struct Rectangle { l: f64, h: f64 }

impl Aire for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
}

impl Aire for Rectangle {
    fn aire(&self) -> f64 { self.l * self.h }

    fn est_grande(&self) -> bool {     // override de la méthode par défaut
        self.l > 50.0 || self.h > 50.0
    }
}
```

## 2. Trait bounds — dispatch statique

`impl Trait` et `T: Trait` génèrent du code **monomorphisé** — une version spécialisée par type appelant. Zéro overhead à l'exécution.

```rust
// Syntaxe impl Trait (sucre syntaxique)
fn afficher_aire(forme: &impl Aire) {
    println!("{}", forme.aire());
}

// Syntaxe générique (équivalente)
fn afficher_aire<T: Aire>(forme: &T) {
    println!("{}", forme.aire());
}

// Bounds multiples
fn comparer<T: Aire + std::fmt::Debug>(a: &T, b: &T) {
    println!("{:?} vs {:?}", a.aire(), b.aire());
}

// Where clause (lisibilité pour les signatures complexes)
fn traiter<T, U>(x: T, y: U) -> f64
where
    T: Aire + Clone,
    U: Aire + std::fmt::Display,
{
    x.clone().aire() + y.aire()
}
```

## 3. Trait objects — dispatch dynamique

Quand le type n'est pas connu à la compilation (collection hétérogène, plugin, etc.), utilisez `dyn Trait` :

```rust
fn afficher_dynamique(forme: &dyn Aire) {
    println!("{}", forme.aire());
}

let formes: Vec<Box<dyn Aire>> = vec![
    Box::new(Cercle { rayon: 3.0 }),
    Box::new(Rectangle { l: 4.0, h: 5.0 }),
];

for f in &formes {
    println!("{}", f.aire());          // dispatch via vtable
}
```

`Box<dyn Trait>` est un **fat pointer** : `(pointer_vers_données, pointer_vers_vtable)`. Légère indirection, mais permet le polymorphisme runtime.

### `impl Trait` vs `dyn Trait`

| | `impl Trait` | `dyn Trait` |
|---|---|---|
| Dispatch | statique (monomorphisation) | dynamique (vtable) |
| Overhead | nul | indirection + vtable |
| Taille à la compilation | connue | inconnue (fat ptr) |
| Hétérogénéité | ❌ (même type) | ✅ |
| Object safety | N/A | trait doit être object-safe |

Un trait est **object-safe** si ses méthodes ne retournent pas `Self` (sauf `where Self: Sized`) et n'ont pas de paramètres génériques.

## 4. Traits importants de la stdlib

### `Iterator`

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // + ~70 méthodes par défaut : map, filter, fold, take, skip, ...
}
```

Implémenter `Iterator` donne accès à toute la chaîne de méthodes :

```rust
struct Compteur { valeur: u32, max: u32 }

impl Iterator for Compteur {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.valeur < self.max {
            self.valeur += 1;
            Some(self.valeur)
        } else {
            None
        }
    }
}

let c = Compteur { valeur: 0, max: 5 };
let somme: u32 = c.filter(|n| n % 2 == 0).sum();  // 2 + 4 = 6
```

### `From` / `Into`

```rust
struct Email(String);

impl From<&str> for Email {
    fn from(s: &str) -> Email {
        Email(s.to_string())
    }
}

let e: Email = "user@example.com".into();   // Into gratuit via From
```

### `Default`

```rust
#[derive(Default)]
struct Config {
    debug: bool,          // false
    timeout: u64,         // 0
    nom: String,          // ""
}

let c = Config::default();
let c2 = Config { debug: true, ..Config::default() };
```

### `Drop`

```rust
struct Ressource { nom: String }

impl Drop for Ressource {
    fn drop(&mut self) {
        println!("Libération de {}", self.nom);
    }
}

{
    let _r = Ressource { nom: String::from("fichier") };
}   // "Libération de fichier" — automatique
```

`Drop` est l'équivalent de destructeurs C++ — sans `delete`. Rust garantit qu'il est appelé même en cas de panic (avec quelques exceptions : `std::mem::forget`, abort).

## 5. Traits avec types associés

Les types associés permettent de "lier" un type à un trait sans le répéter partout :

```rust
trait Convertir {
    type Sortie;
    fn convertir(&self) -> Self::Sortie;
}

struct Celsius(f64);

impl Convertir for Celsius {
    type Sortie = f64;
    fn convertir(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0       // → Fahrenheit
    }
}
```

Vs generics `Convertir<T>` : les types associés n'ont qu'une implémentation par type, les generics permettent plusieurs.

## 6. `Send` et `Sync` — traits de concurrence

Ces deux traits marker indiquent si un type est sûr à partager entre threads. Ils sont automatiquement implémentés si tous les champs le sont.

- `Send` : peut être **transféré** d'un thread à l'autre (ownership move cross-thread)
- `Sync` : peut être **référencé** depuis plusieurs threads (`&T: Send` ↔ `T: Sync`)

```rust
// Rc<T> n'est pas Send (compteur de référence non-atomique)
// Arc<T> est Send + Sync (compteur atomique)

use std::sync::Arc;
let data = Arc::new(vec![1, 2, 3]);
let data2 = Arc::clone(&data);
std::thread::spawn(move || {
    println!("{:?}", data2);           // ✅ Arc<Vec<i32>> est Send
});
```

## 7. Supertraits

Un trait peut exiger qu'un autre soit implémenté :

```rust
trait Affichable: std::fmt::Debug + std::fmt::Display {
    fn afficher_complet(&self) {
        println!("Debug: {:?}, Display: {}", self, self);
    }
}

// Pour implémenter Affichable, vous devez implémenter Debug et Display
```

---

### Piège courant : `dyn Trait` dans un `Vec` sans `Box`

```rust
let formes: Vec<dyn Aire> = vec![...]; // ❌ taille inconnue à la compilation
let formes: Vec<Box<dyn Aire>> = vec![...]; // ✅
```

`dyn Aire` seul n'a pas de taille connue — il faut l'envelopper dans `Box<_>` (ou `Arc<_>`, `Rc<_>`).

---

### Sous le capot : monomorphisation

`fn f<T: Trait>(x: T)` génère une version compilée par type T utilisé. Pour 3 types → 3 fonctions en assembleur. Meilleure performance (inlining, etc.) mais binaire plus grand. C'est le même modèle que les templates C++, mais avec des erreurs lisibles.

---

## À retenir

- Traits = interfaces avec méthodes par défaut + dispatch statique ET dynamique.
- `impl Trait` (ou `<T: Trait>`) → monomorphisation, zéro overhead.
- `dyn Trait` → vtable, permet hétérogénéité runtime.
- `Iterator` : implémenter `next()` donne accès à 70+ méthodes gratuitement.
- `Send`/`Sync` : garanties de thread-safety vérifiées à la compilation.

---

➡️ [Chapitre 10 — Génériques et itérateurs avancés](../10_generics_iterators/README.md)
