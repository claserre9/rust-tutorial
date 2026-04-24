//! Exercices — Chapitre 5 : Structs, enums et collections
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices

use std::collections::HashMap;

fn main() {
    ex_5_1();
    ex_5_2();
    ex_5_3();
    ex_5_4();
    ex_5_5();
    ex_5_6();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 5.1 — Struct avec méthodes
// =============================================================================
// Définissez un struct `Vecteur2D { x: f64, y: f64 }` avec :
//   - `new(x, y)` → constructeur
//   - `norme(&self) -> f64` → sqrt(x²+y²)
//   - `normaliser(&self) -> Vecteur2D` → vecteur unitaire (clone divisé par norme)
//   - `addition(&self, autre: &Vecteur2D) -> Vecteur2D`
// Dérivez Debug et PartialEq.

// TODO : struct Vecteur2D + impl

fn ex_5_1() {
    let v1 = Vecteur2D::new(3.0, 4.0);
    assert!((v1.norme() - 5.0).abs() < 1e-9);

    let u = v1.normaliser();
    assert!((u.norme() - 1.0).abs() < 1e-9);

    let v2 = Vecteur2D::new(1.0, 0.0);
    let v3 = v1.addition(&v2);
    assert_eq!(v3, Vecteur2D::new(4.0, 4.0));
}


// =============================================================================
// 5.2 — Enum avec données
// =============================================================================
// Définissez un enum `Monnaie` avec les variantes :
//   Piece(u32)  → valeur en centimes
//   Billet(u32) → valeur en euros
// Implémentez `valeur_centimes(&self) -> u32` sur l'enum.

// TODO : enum Monnaie + impl

fn ex_5_2() {
    let p1 = Monnaie::Piece(50);            // 50 centimes
    let p2 = Monnaie::Piece(10);            // 10 centimes
    let b1 = Monnaie::Billet(20);           // 20 euros = 2000 centimes

    assert_eq!(p1.valeur_centimes(), 50);
    assert_eq!(p2.valeur_centimes(), 10);
    assert_eq!(b1.valeur_centimes(), 2000);

    let total: u32 = [p1, p2, b1]
        .iter()
        .map(|m| m.valeur_centimes())
        .sum();
    assert_eq!(total, 2060);
}


// =============================================================================
// 5.3 — Option et chaînage
// =============================================================================
// Implémentez `chercher_et_doubler` :
//   - Reçoit un &HashMap<&str, i32> et une clé &str
//   - Retourne Some(valeur * 2) si la clé existe et valeur > 0
//   - Retourne None sinon
// Utilisez .get(), .copied(), .filter(), .map() — PAS de match ni if let.

fn chercher_et_doubler(map: &HashMap<&str, i32>, cle: &str) -> Option<i32> {
    // TODO : chaîne de méthodes sur Option
    None
}

fn ex_5_3() {
    let mut map = HashMap::new();
    map.insert("a", 5);
    map.insert("b", -3);
    map.insert("c", 0);

    assert_eq!(chercher_et_doubler(&map, "a"), Some(10));
    assert_eq!(chercher_et_doubler(&map, "b"), None); // -3 n'est pas > 0
    assert_eq!(chercher_et_doubler(&map, "c"), None); // 0 n'est pas > 0
    assert_eq!(chercher_et_doubler(&map, "z"), None); // absent
}


// =============================================================================
// 5.4 — Result et opérateur ?
// =============================================================================
// Implémentez `lire_entier` qui parse un &str en i32 et divise par un diviseur.
// Retournez une erreur descriptive (String) dans chacun de ces cas :
//   - parse échoue
//   - diviseur == 0
// Utilisez `?` pour propager l'erreur de parse.

fn lire_et_diviser(s: &str, diviseur: i32) -> Result<i32, String> {
    // TODO
    Err("non implémenté".to_string())
}

fn ex_5_4() {
    assert_eq!(lire_et_diviser("42", 6), Ok(7));
    assert_eq!(lire_et_diviser("10", 3), Ok(3)); // division entière
    assert!(lire_et_diviser("abc", 2).is_err());
    assert!(lire_et_diviser("10", 0).is_err());
}


// =============================================================================
// 5.5 — Vec et transformations
// =============================================================================
// Implémentez `statistiques` qui prend un &[f64] et retourne un struct
// `Stats { min: f64, max: f64, moyenne: f64 }`.
// Retourne None si le slice est vide.

#[derive(Debug, PartialEq)]
struct Stats {
    min: f64,
    max: f64,
    moyenne: f64,
}

fn statistiques(data: &[f64]) -> Option<Stats> {
    // TODO
    None
}

fn ex_5_5() {
    let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let s = statistiques(&data).unwrap();
    assert_eq!(s.min, 1.0);
    assert_eq!(s.max, 9.0);
    assert!((s.moyenne - 3.875).abs() < 1e-9);

    assert!(statistiques(&[]).is_none());
}


// =============================================================================
// 5.6 — HashMap : comptage de fréquences
// =============================================================================
// Implémentez `frequences` qui prend un &str et retourne une HashMap<char, usize>
// comptant chaque caractère (ignorez les espaces).

fn frequences(s: &str) -> HashMap<char, usize> {
    // TODO : parcourir s.chars(), sauter ' ', utiliser entry().or_insert()
    HashMap::new()
}

fn ex_5_6() {
    let f = frequences("hello world");
    assert_eq!(f[&'l'], 3);
    assert_eq!(f[&'o'], 2);
    assert_eq!(f[&'h'], 1);
    assert!(!f.contains_key(&' ')); // espaces ignorés

    let f2 = frequences("aabbc");
    assert_eq!(f2[&'a'], 2);
    assert_eq!(f2[&'b'], 2);
    assert_eq!(f2[&'c'], 1);
}
