# Chapitre 13 — Async/await et Tokio

L'asynchrone en Rust est basé sur des **futures** — des valeurs qui représentent un calcul pas encore terminé. Tokio fournit le runtime pour les exécuter efficacement sur un petit nombre de threads OS.

## 1. `async` / `await` — la syntaxe de base

```rust
use tokio;

async fn dire_bonjour() -> String {
    String::from("bonjour")
}

#[tokio::main]
async fn main() {
    let s = dire_bonjour().await;
    println!("{s}");
}
```

`async fn` retourne un `Future` (implémentation opaque). `.await` suspend l'exécution du coroutine actuel jusqu'à ce que la future soit résolue — **sans bloquer le thread OS**.

### `Cargo.toml`

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## 2. Pourquoi async plutôt que threads ?

| | Threads OS | Async/await |
|---|---|---|
| Coût de création | ~100KB stack, ~50µs | ~quelques centaines d'octets |
| Changement de contexte | kernel space | user space (cheap) |
| Idéal pour | CPU-bound | I/O-bound (réseau, fichiers) |
| Nombre possible | ~10k | ~millions |

Pour un serveur HTTP qui attend des réponses réseau : async. Pour du calcul intensif parallèle : threads (ou rayon).

## 3. Spawner des tâches

```rust
#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async {
        println!("tâche 1");
        42
    });

    let h2 = tokio::spawn(async {
        println!("tâche 2");
        "résultat"
    });

    let (r1, r2) = tokio::join!(h1, h2);
    println!("{} {}", r1.unwrap(), r2.unwrap());
}
```

`tokio::spawn` spawn une tâche sur le thread pool. `tokio::join!` attend plusieurs futures **en parallèle**.

## 4. `tokio::join!` vs `tokio::select!`

```rust
// join! — attend TOUTES les futures
let (r1, r2) = tokio::join!(tâche_a(), tâche_b());

// select! — prend le PREMIER résultat disponible (annule les autres)
tokio::select! {
    val = tâche_a() => println!("a gagne : {val}"),
    val = tâche_b() => println!("b gagne : {val}"),
}
```

`select!` est l'outil clé pour les timeouts et la course entre plusieurs I/O.

## 5. I/O asynchrone avec Tokio

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;

// Lire un fichier
let contenu = fs::read_to_string("fichier.txt").await?;

// Copier un fichier
let mut source = fs::File::open("src.txt").await?;
let mut dest = fs::File::create("dst.txt").await?;
tokio::io::copy(&mut source, &mut dest).await?;
```

### Client HTTP avec `reqwest`

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Todo { id: u32, title: String, completed: bool }

async fn charger_todo(id: u32) -> Result<Todo, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/todos/{id}");
    Client::new().get(&url).send().await?.json::<Todo>().await
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Charger 5 todos en parallèle
    let futures: Vec<_> = (1..=5).map(|i| charger_todo(i)).collect();
    let résultats = futures::future::join_all(futures).await;
    for r in résultats {
        println!("{:?}", r?);
    }
    Ok(())
}
```

## 6. Channels asynchrones

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    while let Some(val) = rx.recv().await {
        println!("{val}");
    }
}
```

`tokio::sync` fournit des variantes async de `Mutex`, `RwLock`, `mpsc`, `oneshot`, `broadcast`, `watch`.

### `oneshot` — un seul message

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    tx.send(42).unwrap();
});

let val = rx.await.unwrap();
println!("{val}");
```

## 7. Timeouts

```rust
use tokio::time::{timeout, Duration};

let résultat = timeout(
    Duration::from_secs(5),
    requete_lente()
).await;

match résultat {
    Ok(val)  => println!("succès : {val:?}"),
    Err(_)   => println!("timeout !"),
}
```

## 8. `async` dans les traits

Depuis Rust 1.75, `async fn` dans les traits est stable (pour les traits non-object-safe) :

```rust
trait Fetcher {
    async fn fetch(&self, url: &str) -> Result<String, reqwest::Error>;
}
```

Pour les trait objects `dyn Trait`, utilisez la crate `async-trait` :

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait Stockage: Send + Sync {
    async fn lire(&self, cle: &str) -> Option<String>;
    async fn ecrire(&self, cle: String, val: String);
}
```

---

### Piège : bloquer le runtime async

```rust
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        std::thread::sleep(std::time::Duration::from_secs(5)); // ❌ bloque le thread !
    });
}
```

`std::thread::sleep` dans un contexte async bloque le thread OS entier, empêchant les autres tâches de s'exécuter. Utilisez toujours `tokio::time::sleep`.

Pour les opérations CPU-bound longues dans un contexte async :

```rust
tokio::task::spawn_blocking(|| {
    // calcul intensif OK ici — s'exécute sur un thread dédié
    calcul_lourd()
}).await.unwrap()
```

---

### Sous le capot : comment fonctionnent les futures

Une `Future` est un state machine généré par le compilateur. `.await` compile en une vérification : "est-ce prêt ?" Si non, la tâche cède la main (`Poll::Pending`). Le waker notifie l'executor quand la future peut progresser. Tout cela se passe en userspace — zéro appel système pour la suspension.

---

## À retenir

- `async fn` retourne une `Future`. `.await` suspend sans bloquer.
- `tokio::spawn` pour les tâches concurrentes, `tokio::join!` pour attendre plusieurs, `tokio::select!` pour la première.
- I/O async : `tokio::fs`, `tokio::io`, `reqwest`.
- Ne jamais bloquer dans un contexte async — utiliser `spawn_blocking` pour le CPU-bound.
- `tokio::sync::mpsc` pour les channels, `oneshot` pour un seul message.

---

➡️ [Chapitre 14 — Unsafe et FFI](../14_unsafe_ffi/README.md)
