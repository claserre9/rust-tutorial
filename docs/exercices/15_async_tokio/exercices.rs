//! Exercices — Chapitre 13 : Async/await et Tokio
//! Créez un projet cargo avec tokio en dépendance.
//! Cargo.toml: tokio = { version = "1", features = ["full"] }

// =============================================================================
// 13.1 — Futures de base
// =============================================================================
// Implémentez `attendre_et_retourner` : attend `ms` millisecondes,
// retourne la valeur doublée. Vérifiez que 3 appels en parallèle
// via tokio::join! prennent ~100ms total (pas 300ms).

// #[tokio::test]
// async fn test_parallèle() {
//     let début = std::time::Instant::now();
//     let (a, b, c) = tokio::join!(
//         attendre_et_retourner(100, 1),
//         attendre_et_retourner(100, 2),
//         attendre_et_retourner(100, 3),
//     );
//     assert!(début.elapsed().as_millis() < 200); // parallèle !
//     assert_eq!((a, b, c), (2, 4, 6));
// }


// =============================================================================
// 13.2 — select! avec timeout
// =============================================================================
// Implémentez `avec_timeout<F>` qui exécute une future avec un timeout.
// Retourne Ok(val) si terminée dans le délai, Err("timeout") sinon.

// async fn avec_timeout<T>(
//     future: impl Future<Output = T>,
//     ms: u64,
// ) -> Result<T, &'static str>
// ...


// =============================================================================
// 13.3 — Channel async et backpressure
// =============================================================================
// Simulez un producteur/consommateur :
//   - Producteur : envoie 20 messages avec un délai de 10ms entre chaque
//   - Consommateur : traite chaque message (délai 30ms) — plus lent
//   - Utilisez un canal borné (sync_channel ou tokio::sync::mpsc avec capacité 5)
//   - Vérifiez que le producteur ralentit (backpressure) grâce au canal borné


// =============================================================================
// 13.4 — Projet mini : scraper async
// =============================================================================
// Dans un projet cargo séparé, écrivez un scraper qui :
//   1. Prend une liste d'URLs (peut-être des endpoints JSONPlaceholder)
//   2. Les requête en parallèle (tokio::spawn ou futures::future::join_all)
//   3. Affiche le code statut et la taille du body de chaque réponse
//   4. Gère les erreurs gracieusement (timeout, erreur réseau)
//
// Dépendances suggérées :
//   reqwest = { version = "0.12", features = ["json"] }
//   tokio = { version = "1", features = ["full"] }
//
// URLs de test (JSONPlaceholder — pas besoin de compte) :
//   https://jsonplaceholder.typicode.com/todos/1
//   https://jsonplaceholder.typicode.com/todos/2
//   ...jusqu'à /todos/10

fn main() {
    println!("Exercices 13.1-13.3 : à implémenter dans un projet cargo avec tokio.");
    println!("Exercice 13.4 : projet cargo séparé avec reqwest.");
}
