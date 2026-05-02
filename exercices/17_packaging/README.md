# Exercices — Chapitre 17 : Packaging et CI

## 17.1 — Profil de release optimisé

Ajoutez à votre `todo-cli` un profil release avec LTO et strip.
Mesurez la différence de taille binaire : `ls -lh target/debug/todo-cli target/release/todo-cli`.

## 17.2 — Features conditionnels

Ajoutez à `log-parser` deux features :
- `json` : exporter les statistiques en JSON (serde_json)
- `csv` : exporter en CSV simple (sans crate externe)

Vérifiez que `cargo build --no-default-features` compile sans erreur.

## 17.3 — GitHub Actions CI

Créez `.github/workflows/ci.yml` pour votre dépôt rust-tutorial avec :
- Tests sur ubuntu-latest et macos-latest
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- `cargo test --all-features`
- Cache des dépendances avec `Swatinem/rust-cache`

## 17.4 — Documentation

Documentez toutes les fonctions publiques de `log-parser` avec `///`.
Ajoutez des exemples dans les docstrings.
Vérifiez que `cargo test --doc` et `cargo doc --no-deps` passent sans avertissements.

## 17.5 — `cargo-dist` (bonus)

Configurez `cargo-dist` sur votre `todo-cli` pour générer des binaires
Linux + macOS + Windows automatiquement sur push de tag.
Faites un dry-run : `cargo dist plan`.
