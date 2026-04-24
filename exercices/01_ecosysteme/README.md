# Exercices — Chapitre 1

Ces exercices valident que votre **setup** est correct et que vous maîtrisez les commandes cargo de base.

## 1.1 — Diagnostic

Dans un terminal :

```bash
rustc --version
cargo --version
rustup show
```

1. Quelle est votre version de Rust ?
2. Êtes-vous sur `stable` ?
3. `rustfmt` et `clippy` sont-ils installés ? (Vérifier avec `rustup component list --installed`.)

## 1.2 — Votre premier projet

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

Modifiez `src/main.rs` pour afficher "Bonjour Rust" suivi de la version Rust (utilisez la variable d'env `RUSTC_VERSION` — ou, plus simple, un `const VERSION: &str = "..."`).

## 1.3 — `cargo check` vs `cargo build`

1. Chronométrez `cargo check` et `cargo build` sur `hello_rust` (utilisez `time`).
2. Modifiez `main.rs` en y ajoutant une erreur de type volontaire (ex. `let x: i32 = "hello";`).
3. Relancez `cargo check`. Que voyez-vous ? Corrigez.

## 1.4 — Ajouter une dépendance

```bash
cargo add rand
```

1. Que fait cette commande ? Ouvrez `Cargo.toml`, qu'a-t-elle ajouté ?
2. Dans `main.rs`, tirez un nombre aléatoire entre 1 et 100 et affichez-le.

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=100);
    println!("Nombre tiré : {}", n);
}
```

3. Regardez `Cargo.lock` — que voyez-vous comme nombre de dépendances transitives ?

## 1.5 — `rust-toolchain.toml`

1. Créez un fichier `rust-toolchain.toml` à la racine qui force Rust **1.85.0** avec les composants `rustfmt` et `clippy`.
2. Exécutez `cargo build` à nouveau. Que se passe-t-il ?

## 1.6 — Mode debug vs release

1. Créez `src/main.rs` qui fait un calcul lourd :

```rust
fn main() {
    let mut sum = 0u64;
    for i in 0..100_000_000 {
        sum = sum.wrapping_add(i);
    }
    println!("{sum}");
}
```

2. Chronométrez `cargo run` (debug) vs `cargo run --release`.
3. Expliquez le facteur observé.

## 1.7 — `cargo tree`

1. Installez `cargo tree` si besoin (inclus par défaut).
2. Lancez `cargo tree` sur `hello_rust` (qui dépend de `rand`).
3. Combien de crates transitives sont tirées ? Nommez-en 3.

## 1.8 — `cargo doc`

1. Exécutez `cargo doc --open` dans `hello_rust`. Qu'affiche-t-il ?
2. Ajoutez un commentaire `///` sur une fonction, re-générez, vérifiez qu'il apparaît.

## 1.9 — Formatage et lint

1. Dégradez volontairement le style de `main.rs` (espaces bizarres, tabulation, etc.).
2. Lancez `cargo fmt`. Que s'est-il passé ?
3. Lancez `cargo clippy`. Si vous avez ajouté des warnings, corrigez.

---

Les solutions sont dans `solutions/01_ecosysteme/`.
