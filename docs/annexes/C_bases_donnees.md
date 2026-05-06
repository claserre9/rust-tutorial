# Annexe C — Bases de données : sqlx, Diesel, Redis

## `sqlx` — requêtes async vérifiées à la compilation

```toml
[dependencies]
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "macros", "chrono", "uuid"] }
```

### Connexion et pool

```rust
use sqlx::PgPool;

let pool = PgPool::connect("postgres://user:pass@localhost/madb").await?;

// Avec options
let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(10)
    .connect("postgres://...").await?;
```

### Requêtes typées avec `query_as!`

```rust
#[derive(sqlx::FromRow, Debug)]
struct Utilisateur {
    id: i32,
    email: String,
    actif: bool,
}

// query_as! vérifie la requête SQL à la compilation (nécessite DATABASE_URL en env)
let users = sqlx::query_as!(
    Utilisateur,
    "SELECT id, email, actif FROM utilisateurs WHERE actif = $1",
    true
)
.fetch_all(&pool)
.await?;
```

### Transactions

```rust
let mut tx = pool.begin().await?;

sqlx::query!("INSERT INTO commandes (client_id, total) VALUES ($1, $2)", client_id, total)
    .execute(&mut *tx).await?;

sqlx::query!("UPDATE stocks SET quantite = quantite - $1 WHERE produit_id = $2", qte, produit_id)
    .execute(&mut *tx).await?;

tx.commit().await?;   // rollback automatique si tx est droppé sans commit
```

### Migrations avec `sqlx-cli`

```bash
cargo install sqlx-cli
export DATABASE_URL=postgres://user:pass@localhost/madb

sqlx migrate add creer_utilisateurs      # génère migrations/TIMESTAMP_creer_utilisateurs.sql
sqlx migrate run                          # applique les migrations
sqlx migrate revert                       # annule la dernière
```

```sql
-- migrations/20240315_creer_utilisateurs.sql
CREATE TABLE utilisateurs (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    mot_de_passe TEXT NOT NULL,
    actif BOOLEAN NOT NULL DEFAULT TRUE,
    cree_le TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Diesel — ORM synchrone avec vérification compile-time

```toml
[dependencies]
diesel = { version = "2", features = ["postgres", "r2d2", "chrono"] }
diesel_migrations = "2"
dotenvy = "0.15"
```

### Setup

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup                             # crée la DB et le dossier migrations/
diesel migration generate creer_articles
diesel migration run
```

### Schéma et modèles

```rust
// src/schema.rs — généré par diesel print-schema
diesel::table! {
    articles (id) {
        id -> Int4,
        titre -> Text,
        contenu -> Text,
        publie -> Bool,
    }
}

// src/models.rs
use crate::schema::articles;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub titre: String,
    pub contenu: String,
    pub publie: bool,
}

#[derive(Insertable)]
#[diesel(table_name = articles)]
pub struct NouvelArticle<'a> {
    pub titre: &'a str,
    pub contenu: &'a str,
}
```

### Requêtes

```rust
use diesel::prelude::*;
use crate::schema::articles::dsl::*;

fn lister_publiés(conn: &mut PgConnection) -> Vec<Article> {
    articles
        .filter(publie.eq(true))
        .order(id.desc())
        .limit(10)
        .load::<Article>(conn)
        .expect("erreur DB")
}

fn créer(conn: &mut PgConnection, t: &str, c: &str) -> Article {
    let nouvel = NouvelArticle { titre: t, contenu: c };
    diesel::insert_into(articles)
        .values(&nouvel)
        .get_result(conn)
        .expect("erreur insertion")
}
```

## Redis avec `fred`

```toml
[dependencies]
fred = { version = "9", features = ["tokio-runtime"] }
```

```rust
use fred::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = RedisConfig::from_url("redis://127.0.0.1:6379")?;
    let client = RedisClient::new(config, None, None, None);
    client.connect();
    client.wait_for_connect().await?;

    // String
    client.set("clé", "valeur", Some(Expiration::EX(60)), None, false).await?;
    let val: String = client.get("clé").await?;

    // Hash
    client.hset("user:1", [("nom", "Alice"), ("email", "alice@ex.com")]).await?;
    let nom: String = client.hget("user:1", "nom").await?;

    // Pub/Sub
    let sub = client.clone_new();
    sub.connect();
    sub.subscribe("canal").await?;
    let mut messages = sub.on_message();
    while let Ok(msg) = messages.recv().await {
        println!("message : {:?}", msg.value);
    }

    Ok(())
}
```

## Choisir son outil

| Outil | Type | Forces |
|---|---|---|
| `sqlx` | requêtes brutes async | vérification SQL à la compilation, async natif |
| `Diesel` | ORM sync | type-safe à l'extrême, migrations intégrées |
| `sea-orm` | ORM async | Diesel-like mais async/await |
| `rusqlite` | SQLite sync | simple, embedde SQLite directement |
| `fred` | Redis async | performant, support cluster |
| `mongodb` | MongoDB async | driver officiel |
