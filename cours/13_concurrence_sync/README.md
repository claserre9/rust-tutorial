# Chapitre 12 — Concurrence synchrone : threads, Arc, Mutex

Rust garantit la **data-race freedom** à la compilation grâce à `Send` et `Sync`. Ce chapitre couvre les threads OS, le partage de données thread-safe, et les primitives de synchronisation.

## 1. Spawner un thread

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("depuis un thread !");
});

handle.join().unwrap();   // attend la fin
```

`spawn` retourne un `JoinHandle<T>` où `T` est le type de retour de la closure.

### Capturer des données avec `move`

```rust
let données = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("{données:?}");    // move : données appartient au thread
});

// println!("{données:?}");     // ❌ moved
handle.join().unwrap();
```

La closure doit être `move` pour prendre ownership des données capturées — le thread peut vivre plus longtemps que la portée de création.

## 2. Passer des données entre threads

### `Arc<T>` — comptage de références atomique

`Rc<T>` n'est pas `Send`. Pour partager entre threads, utilisez `Arc<T>` (Atomic Reference Counting) :

```rust
use std::sync::Arc;

let données = Arc::new(vec![1, 2, 3]);

let mut handles = Vec::new();
for _ in 0..5 {
    let données = Arc::clone(&données);
    handles.push(thread::spawn(move || {
        println!("{:?}", données);
    }));
}

for h in handles { h.join().unwrap(); }
```

`Arc::clone` incrémente le compteur de façon atomique (CAS, pas de mutex).

### `Mutex<T>` — exclusion mutuelle

Pour **modifier** des données partagées :

```rust
use std::sync::{Arc, Mutex};

let compteur = Arc::new(Mutex::new(0));

let mut handles = Vec::new();
for _ in 0..10 {
    let compteur = Arc::clone(&compteur);
    handles.push(thread::spawn(move || {
        let mut val = compteur.lock().unwrap();
        *val += 1;
    }));                    // MutexGuard droppé ici → unlock automatique
}

for h in handles { h.join().unwrap(); }
println!("{}", *compteur.lock().unwrap()); // 10
```

`lock()` bloque jusqu'à l'obtention du verrou et retourne un `MutexGuard<T>`. Quand le guard est droppé, le verrou est relâché. **RAII appliqué à la synchronisation**.

### `RwLock<T>` — multiple lecteurs OU un éditeur

```rust
use std::sync::RwLock;

let cache = Arc::new(RwLock::new(std::collections::HashMap::<String, i32>::new()));

// Écriture
cache.write().unwrap().insert("clé".to_string(), 42);

// Lectures concurrentes (plusieurs lecteurs simultanés)
let c1 = Arc::clone(&cache);
let c2 = Arc::clone(&cache);
let h1 = thread::spawn(move || { println!("{:?}", c1.read().unwrap().get("clé")); });
let h2 = thread::spawn(move || { println!("{:?}", c2.read().unwrap().get("clé")); });
h1.join().unwrap(); h2.join().unwrap();
```

## 3. Channels — communication par message

```rust
use std::sync::mpsc;   // multiple producer, single consumer

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("bonjour")).unwrap();
    tx.send(String::from("monde")).unwrap();
    // tx est droppé ici → rx.recv() retournera Err
});

for message in rx {    // rx implémente IntoIterator → termine quand tx est droppé
    println!("{message}");
}
```

### Multiple producteurs

```rust
let (tx, rx) = mpsc::channel();

for i in 0..5 {
    let tx = tx.clone();    // cloner le sender pour chaque thread
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}
drop(tx);                   // important : droper le sender original !

for val in rx { println!("{val}"); }
```

### `mpsc::sync_channel` — channel borné (backpressure)

```rust
let (tx, rx) = mpsc::sync_channel(10);   // buffer de 10 messages
// tx.send() bloque si le buffer est plein → backpressure automatique
```

## 4. `std::sync::Condvar` — variable de condition

Pour attendre qu'une condition soit remplie :

```rust
use std::sync::{Arc, Mutex, Condvar};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut prêt = lock.lock().unwrap();
    *prêt = true;
    cvar.notify_one();    // réveille un thread en attente
});

let (lock, cvar) = &*pair;
let mut prêt = lock.lock().unwrap();
while !*prêt {
    prêt = cvar.wait(prêt).unwrap();   // libère le mutex et attend
}
println!("thread principal notifié !");
```

## 5. `std::thread::scope` — threads avec borrows (1.63+)

`thread::spawn` exige `'static`. `thread::scope` permet de borrower des données locales :

```rust
let données = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        println!("{données:?}");    // borrow valide : scope garantit la fin avant retour
    });
    s.spawn(|| {
        println!("{données:?}");    // deux lecteurs OK
    });
});                                 // garantie : tous les threads sont terminés ici
println!("{données:?}");            // ✅ données est toujours valide
```

`thread::scope` est la façon moderne de spawner des threads courts sans `Arc`.

## 6. Rayon — parallélisme de données

Pour les algorithmes data-parallel, `rayon` transforme les itérateurs séquentiels en parallèles :

```toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

let v: Vec<i32> = (0..1_000_000).collect();

// Séquentiel
let somme_seq: i64 = v.iter().map(|&x| x as i64 * x as i64).sum();

// Parallèle (utilise tous les cœurs)
let somme_par: i64 = v.par_iter().map(|&x| x as i64 * x as i64).sum();

assert_eq!(somme_seq, somme_par);
```

`par_iter()` divise automatiquement le travail en chunks et les distribue sur un thread pool. Zéro changement de logique — seul `iter()` → `par_iter()`.

---

### Piège : deadlock

```rust
let m1 = Arc::new(Mutex::new(0));
let m2 = Arc::new(Mutex::new(0));

let m1a = Arc::clone(&m1); let m2a = Arc::clone(&m2);
let h1 = thread::spawn(move || {
    let _g1 = m1a.lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(10));
    let _g2 = m2a.lock().unwrap();   // attend m2 pendant que h2 détient m2 et attend m1
});

let m1b = Arc::clone(&m1); let m2b = Arc::clone(&m2);
let h2 = thread::spawn(move || {
    let _g2 = m2b.lock().unwrap();
    let _g1 = m1b.lock().unwrap();   // DEADLOCK
});
```

**Solution** : toujours acquérir les verrous dans le même ordre, ou utiliser `try_lock()`.

---

### Sous le capot : `Send` et `Sync`

- `Arc<T>: Send` si `T: Send + Sync`
- `Mutex<T>: Send` si `T: Send`
- `Mutex<T>: Sync` si `T: Send`
- `MutexGuard<T>: !Send` — on ne peut pas transférer un verrou à un autre thread

Si vous essayez d'envoyer un `Rc<T>` dans un thread, erreur de compilation : `Rc<_>: !Send`. Le compilateur prévient les races avant que vous n'exécutiez une seule ligne.

---

## À retenir

- `Arc<T>` pour partager, `Mutex<T>` pour modifier.
- `MutexGuard` déverrouille automatiquement à son drop (RAII).
- `mpsc::channel` pour communication unidirectionnelle par messages.
- `thread::scope` pour borrower des données locales dans des threads courts.
- `rayon::par_iter()` pour parallélisme data-parallel transparent.

---

➡️ [Chapitre 14 — Concurrence asynchrone avec Tokio](../14_async_tokio/README.md)
