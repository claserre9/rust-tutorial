//! Exercices — Chapitre 7 : Structs avancés, Display, newtype

use std::fmt;

fn main() {
    ex_7_1();
    ex_7_2();
    ex_7_3();
    ex_7_4();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 7.1 — Display et opérateurs
// =============================================================================
// Définissez un struct `Fraction { num: i64, den: i64 }` avec :
//   - `new(num, den)` — réduit automatiquement (pgcd), den toujours positif
//   - Display : "3/4", "-1/2", "2" (si den == 1)
//   - impl Add pour Fraction + Fraction
//   - impl PartialEq

fn pgcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { pgcd(b, a % b) }
}

// TODO : struct Fraction + impl

fn ex_7_1() {
    let a = Fraction::new(1, 2);
    let b = Fraction::new(1, 3);
    let c = a + b;
    assert_eq!(c, Fraction::new(5, 6));
    assert_eq!(format!("{c}"), "5/6");

    let d = Fraction::new(4, 2);
    assert_eq!(format!("{d}"), "2");

    let e = Fraction::new(-3, -6);
    assert_eq!(format!("{e}"), "1/2");  // réduit + den positif
}


// =============================================================================
// 7.2 — Pattern newtype
// =============================================================================
// Créez des newtypes `Euros(i64)` et `Centimes(i64)` (en centimes).
// Implémentez :
//   - Euros::from_centimes(c: i64) -> Euros
//   - Euros::en_centimes(&self) -> Centimes
//   - Display pour Euros : "12.50 €" (toujours 2 décimales)
//   - impl Add<Euros> pour Euros

// TODO

fn ex_7_2() {
    let prix = Euros::from_centimes(1250);
    assert_eq!(format!("{prix}"), "12.50 €");

    let tva = Euros::from_centimes(250);
    let total = prix + tva;
    assert_eq!(total.en_centimes(), Centimes(1500));
    assert_eq!(format!("{total}"), "15.00 €");
}


// =============================================================================
// 7.3 — Builder pattern
// =============================================================================
// Implémentez un builder pour `Requete` :
//   url: String (obligatoire)
//   methode: String (défaut "GET")
//   timeout_ms: u64 (défaut 5000)
//   headers: Vec<(String, String)> (défaut vide)

// TODO : struct Requete, struct RequeteBuilder + méthodes chaînées

fn ex_7_3() {
    let r = RequeteBuilder::new("https://example.com")
        .methode("POST")
        .timeout_ms(10_000)
        .header("Authorization", "Bearer token123")
        .header("Content-Type", "application/json")
        .build();

    assert_eq!(r.url, "https://example.com");
    assert_eq!(r.methode, "POST");
    assert_eq!(r.timeout_ms, 10_000);
    assert_eq!(r.headers.len(), 2);

    // Défauts
    let r2 = RequeteBuilder::new("https://api.example.com/v1").build();
    assert_eq!(r2.methode, "GET");
    assert_eq!(r2.timeout_ms, 5000);
    assert!(r2.headers.is_empty());
}


// =============================================================================
// 7.4 — Implémenter Ord personnalisé
// =============================================================================
// Définissez un struct `Etudiant { nom: String, note: f64 }`.
// Implémentez PartialEq, Eq, PartialOrd, Ord pour trier par NOTE décroissante,
// puis par NOM croissant si égalité.
// (Note : f64 n'est pas Ord — astuce: multiplier par 1000 et arrondir en i64)

// TODO

fn ex_7_4() {
    let mut etudiants = vec![
        Etudiant::new("Charlie", 85.5),
        Etudiant::new("Alice", 92.0),
        Etudiant::new("Bob", 85.5),
        Etudiant::new("Diana", 78.0),
    ];

    etudiants.sort();

    // Alice (92), puis Bob et Charlie (85.5, ordre alpha), puis Diana (78)
    assert_eq!(etudiants[0].nom, "Alice");
    assert_eq!(etudiants[1].nom, "Bob");
    assert_eq!(etudiants[2].nom, "Charlie");
    assert_eq!(etudiants[3].nom, "Diana");
}
