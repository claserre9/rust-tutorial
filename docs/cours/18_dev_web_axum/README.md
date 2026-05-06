# Chapitre 16 — Web avec Axum — Projet API REST

Axum est le framework web de l'écosystème Tokio. Basé sur `tower` (middleware) et `hyper` (HTTP), il compose élégamment avec l'async Rust. Ce chapitre construit une API REST complète avec SQLite via `sqlx`.

## 1. Setup minimal

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(racine));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn racine() -> &'static str {
    "Bonjour depuis Axum !"
}
```

## 2. Extracteurs et réponses

Axum utilise des **extracteurs** — des types qui implémentent `FromRequest` et extraient des données de la requête :

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params { nom: Option<String> }

#[derive(Serialize)]
struct Salut { message: String }

// GET /saluer?nom=Alice
async fn saluer(Query(params): Query<Params>) -> Json<Salut> {
    let nom = params.nom.unwrap_or_else(|| "monde".to_string());
    Json(Salut { message: format!("Bonjour, {nom} !") })
}

// GET /utilisateurs/:id
async fn get_utilisateur(Path(id): Path<u32>) -> String {
    format!("utilisateur #{id}")
}
```

### Types d'extracteurs courants

| Extracteur | Source |
|---|---|
| `Path(x): Path<T>` | segment d'URL |
| `Query(x): Query<T>` | query string |
| `Json(x): Json<T>` | corps JSON (deserialize) |
| `State(x): State<T>` | état partagé de l'app |
| `Extension(x): Extension<T>` | données ajoutées par un middleware |
| `TypedHeader(x)` | header HTTP |

## 3. État partagé avec `State`

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

type Db = Arc<RwLock<HashMap<u32, String>>>;

async fn lister(State(db): State<Db>) -> Json<Vec<String>> {
    let db = db.read().await;
    Json(db.values().cloned().collect())
}

async fn ajouter(
    State(db): State<Db>,
    Json(nom): Json<String>,
) -> (StatusCode, String) {
    let mut db = db.write().await;
    let id = db.len() as u32 + 1;
    db.insert(id, nom);
    (StatusCode::CREATED, format!("créé #{id}"))
}

#[tokio::main]
async fn main() {
    let état: Db = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/items", get(lister).post(ajouter))
        .with_state(état);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    ).await.unwrap();
}
```

## 4. Gestion d'erreurs avec `IntoResponse`

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Debug)]
enum ApiError {
    NonTrouvé,
    BaseDeDonnées(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            ApiError::NonTrouvé => (StatusCode::NOT_FOUND, "ressource introuvable".to_string()),
            ApiError::BaseDeDonnées(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };
        (code, msg).into_response()
    }
}

// Les handlers peuvent retourner Result<T, ApiError>
async fn get_item(Path(id): Path<u32>) -> Result<Json<String>, ApiError> {
    if id == 0 {
        return Err(ApiError::NonTrouvé);
    }
    Ok(Json(format!("item #{id}")))
}
```

## 5. Middleware avec `tower`

```rust
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use axum::middleware;

// Middleware de logging
let app = Router::new()
    .route("/", get(racine))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
```

```toml
[dependencies]
tower-http = { version = "0.6", features = ["cors", "trace"] }
```

## 6. `sqlx` — base de données async

```toml
[dependencies]
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "macros"] }
```

```rust
use sqlx::{SqlitePool, Row};

async fn creer_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tâches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            fait INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(pool).await?;
    Ok(())
}

async fn lister_tâches(pool: &SqlitePool) -> Result<Vec<(i64, String, bool)>, sqlx::Error> {
    sqlx::query("SELECT id, description, fait FROM tâches")
        .map(|row: sqlx::sqlite::SqliteRow| {
            (row.get::<i64, _>("id"),
             row.get::<String, _>("description"),
             row.get::<bool, _>("fait"))
        })
        .fetch_all(pool)
        .await
}
```

---

## Projet fil rouge — API de tâches avec Axum + SQLite

### Structure

```
api-tasks/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── routes/
    │   ├── mod.rs
    │   └── tasks.rs
    ├── models.rs
    ├── db.rs
    └── errors.rs
```

### `Cargo.toml`

```toml
[package]
name = "api-tasks"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "macros"] }
thiserror = "2"
tower-http = { version = "0.6", features = ["cors", "trace"] }
```

### `src/models.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Tâche {
    pub id: i64,
    pub description: String,
    pub fait: bool,
}

#[derive(Debug, Deserialize)]
pub struct NouvellesTâche {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct MajTâche {
    pub description: Option<String>,
    pub fait: Option<bool>,
}
```

### `src/db.rs`

```rust
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connecter(url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tâches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            fait BOOLEAN NOT NULL DEFAULT FALSE
        )"
    ).execute(&pool).await?;

    Ok(pool)
}
```

### `src/routes/tasks.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;
use crate::models::{Tâche, NouvellesTâche, MajTâche};
use crate::errors::ApiError;

pub async fn lister(State(pool): State<SqlitePool>) -> Result<Json<Vec<Tâche>>, ApiError> {
    let tâches = sqlx::query_as!(Tâche, "SELECT id, description, fait FROM tâches ORDER BY id")
        .fetch_all(&pool).await?;
    Ok(Json(tâches))
}

pub async fn créer(
    State(pool): State<SqlitePool>,
    Json(nouvelle): Json<NouvellesTâche>,
) -> Result<(StatusCode, Json<Tâche>), ApiError> {
    let résultat = sqlx::query!(
        "INSERT INTO tâches (description) VALUES (?) RETURNING id, description, fait",
        nouvelle.description
    ).fetch_one(&pool).await?;

    let tâche = Tâche {
        id: résultat.id,
        description: résultat.description,
        fait: résultat.fait,
    };
    Ok((StatusCode::CREATED, Json(tâche)))
}

pub async fn mettre_à_jour(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(maj): Json<MajTâche>,
) -> Result<Json<Tâche>, ApiError> {
    // Récupérer l'existant
    let existant = sqlx::query_as!(Tâche, "SELECT id, description, fait FROM tâches WHERE id = ?", id)
        .fetch_optional(&pool).await?
        .ok_or(ApiError::NonTrouvé(id))?;

    let description = maj.description.unwrap_or(existant.description);
    let fait = maj.fait.unwrap_or(existant.fait);

    sqlx::query!("UPDATE tâches SET description = ?, fait = ? WHERE id = ?", description, fait, id)
        .execute(&pool).await?;

    Ok(Json(Tâche { id, description, fait }))
}

pub async fn supprimer(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, ApiError> {
    let résultat = sqlx::query!("DELETE FROM tâches WHERE id = ?", id)
        .execute(&pool).await?;

    if résultat.rows_affected() == 0 {
        return Err(ApiError::NonTrouvé(id));
    }
    Ok(StatusCode::NO_CONTENT)
}
```

### `src/errors.rs`

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("ressource #{0} introuvable")]
    NonTrouvé(i64),
    #[error("erreur base de données : {0}")]
    Sql(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, msg) = match &self {
            ApiError::NonTrouvé(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Sql(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (code, msg).into_response()
    }
}
```

### `src/main.rs`

```rust
mod db;
mod errors;
mod models;
mod routes;

use axum::{routing::{delete, get, patch, post}, Router};
use tower_http::trace::TraceLayer;
use routes::tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::connecter("sqlite://tasks.db").await?;

    let app = Router::new()
        .route("/tasks",        get(tasks::lister).post(tasks::créer))
        .route("/tasks/:id",    patch(tasks::mettre_à_jour).delete(tasks::supprimer))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("API démarrée sur http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Test rapide

```bash
cargo run &

# Créer
curl -X POST localhost:3000/tasks \
     -H "Content-Type: application/json" \
     -d '{"description": "Apprendre Axum"}'

# Lister
curl localhost:3000/tasks

# Mettre à jour
curl -X PATCH localhost:3000/tasks/1 \
     -H "Content-Type: application/json" \
     -d '{"fait": true}'

# Supprimer
curl -X DELETE localhost:3000/tasks/1
```

---

## À retenir

- Axum = Router + extracteurs + handlers async. Composition via `layer()`.
- `State<T>` pour partager l'état (pool de connexions, config).
- `IntoResponse` pour convertir vos types d'erreur en réponses HTTP.
- `sqlx` : requêtes async vérifiées à la compilation avec `query_as!`.
- Le project template ici est production-ready — ajoutez JWT, validation, migrations selon les besoins.

---

➡️ [Chapitre 19 — Packaging, CI et distribution](../19_packaging/README.md)
