//! Solutions — Chapitre 12

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    ex_12_1();
    ex_12_2();
    ex_12_3();
    println!("Toutes les solutions passent ✅");
}

// 12.1
fn incrémenter_en_parallèle(n_threads: usize, n_fois: usize) -> usize {
    let compteur = Arc::new(Mutex::new(0usize));
    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let c = Arc::clone(&compteur);
            thread::spawn(move || {
                for _ in 0..n_fois {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles { h.join().unwrap(); }
    *compteur.lock().unwrap()
}

fn ex_12_1() {
    assert_eq!(incrémenter_en_parallèle(10, 100), 1000);
    assert_eq!(incrémenter_en_parallèle(1, 5000), 5000);
}

// 12.2
fn pipeline(n: u64) -> Vec<u64> {
    let (tx1, rx1) = mpsc::channel::<u64>();
    let (tx2, rx2) = mpsc::channel::<u64>();
    let (tx3, rx3) = mpsc::channel::<u64>();

    // Étape 1 : génère 1..=n
    thread::spawn(move || {
        for i in 1..=n { tx1.send(i).unwrap(); }
    });

    // Étape 2 : filtre les pairs
    thread::spawn(move || {
        for i in rx1 {
            if i % 2 == 0 { tx2.send(i).unwrap(); }
        }
    });

    // Étape 3 : élève au carré
    thread::spawn(move || {
        for i in rx2 { tx3.send(i * i).unwrap(); }
    });

    rx3.iter().collect()
}

fn ex_12_2() {
    assert_eq!(pipeline(10), vec![4, 16, 36, 64, 100]);
}

// 12.3
fn somme_parallèle(données: &[i64], n_threads: usize) -> i64 {
    let taille_chunk = (données.len() + n_threads - 1) / n_threads;
    let mut total = 0i64;

    thread::scope(|s| {
        let résultats: Vec<_> = données
            .chunks(taille_chunk)
            .map(|chunk| s.spawn(|| chunk.iter().sum::<i64>()))
            .collect();

        for handle in résultats {
            total += handle.join().unwrap();
        }
    });

    total
}

fn ex_12_3() {
    let données: Vec<i64> = (1..=100).collect();
    assert_eq!(somme_parallèle(&données, 4), 5050);
    assert_eq!(somme_parallèle(&données, 1), 5050);
    let grand: Vec<i64> = (1..=10_000).collect();
    let attendu: i64 = (1..=10_000).sum();
    assert_eq!(somme_parallèle(&grand, 8), attendu);
}
