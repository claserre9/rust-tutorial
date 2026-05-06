//! Exercices — Chapitre 12 : Concurrence synchrone
//! Ces exercices nécessitent un projet cargo (threads et channels).

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    ex_12_1();
    ex_12_2();
    ex_12_3();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 12.1 — Compteur thread-safe
// =============================================================================
// Implémentez `IncrémenterEnParallèle` : lancez N threads, chacun incrémente
// un compteur partagé M fois. Vérifiez que la valeur finale est N*M.

fn incrémenter_en_parallèle(n_threads: usize, n_fois: usize) -> usize {
    // TODO : Arc<Mutex<usize>> + N threads + join
    0
}

fn ex_12_1() {
    assert_eq!(incrémenter_en_parallèle(10, 100), 1000);
    assert_eq!(incrémenter_en_parallèle(1, 5000), 5000);
}


// =============================================================================
// 12.2 — Pipeline par channels
// =============================================================================
// Implémentez un pipeline à 3 étapes via mpsc::channel :
//   Étape 1 : génère les entiers 1..=N
//   Étape 2 : filtre les pairs
//   Étape 3 : élève au carré
// Retournez le Vec<u64> des résultats dans l'ordre.

fn pipeline(n: u64) -> Vec<u64> {
    // TODO : 3 channels, 3 threads (ou 2 + le main)
    Vec::new()
}

fn ex_12_2() {
    let résultat = pipeline(10);
    // Pairs de 1..=10 : 2,4,6,8,10 → carrés : 4,16,36,64,100
    assert_eq!(résultat, vec![4, 16, 36, 64, 100]);
}


// =============================================================================
// 12.3 — thread::scope
// =============================================================================
// Utilisez thread::scope pour calculer la somme d'un Vec<i64>
// en le divisant en N chunks traités en parallèle.
// Ne pas utiliser Arc ou clone du vecteur.

fn somme_parallèle(données: &[i64], n_threads: usize) -> i64 {
    // TODO : thread::scope + chunks
    0
}

fn ex_12_3() {
    let données: Vec<i64> = (1..=100).collect();
    assert_eq!(somme_parallèle(&données, 4), 5050);
    assert_eq!(somme_parallèle(&données, 1), 5050);

    // Gros vecteur
    let grand: Vec<i64> = (1..=10_000).collect();
    let attendu: i64 = (1..=10_000).sum();
    assert_eq!(somme_parallèle(&grand, 8), attendu);
}
