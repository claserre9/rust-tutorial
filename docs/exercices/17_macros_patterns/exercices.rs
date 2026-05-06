//! Exercices — Chapitre 15 : Macros et patterns

fn main() {
    ex_15_1();
    ex_15_2();
    ex_15_3();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 15.1 — macro_rules! : map littéral
// =============================================================================
// Implémentez la macro `hashmap!` qui crée un HashMap depuis des paires clé => val.
// Usage : hashmap!{ "a" => 1, "b" => 2 }

// TODO : macro_rules! hashmap

fn ex_15_1() {
    use std::collections::HashMap;

    let m: HashMap<&str, i32> = hashmap!{ "a" => 1, "b" => 2, "c" => 3 };
    assert_eq!(m["a"], 1);
    assert_eq!(m["c"], 3);
    assert_eq!(m.len(), 3);

    // HashMap vide
    let vide: HashMap<&str, i32> = hashmap!{};
    assert!(vide.is_empty());
}


// =============================================================================
// 15.2 — macro_rules! : assert_approx_eq!
// =============================================================================
// Implémentez `assert_approx_eq!(a, b, epsilon)` :
// panique si |a - b| > epsilon, avec un message informatif.
// Si epsilon est omis, utilise 1e-9 par défaut.

// TODO : macro_rules! assert_approx_eq

fn ex_15_2() {
    assert_approx_eq!(1.0_f64, 1.0000000001, 1e-9);
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 0.001);
}


// =============================================================================
// 15.3 — State machine par types
// =============================================================================
// Modélisez une connexion réseau avec les états :
//   Déconnecté → Connecté → Authentifié → Déconnecté
//
// Règles :
//   - Seul Déconnecté peut se connecter (→ Connecté)
//   - Seul Connecté peut s'authentifier (→ Authentifié)
//   - Seul Authentifié peut envoyer des données
//   - Tout état peut se déconnecter
//
// Utiliser des types distincts pour rendre les transitions illégales
// à la compilation (pas de match, pas d'enum avec état runtime).

struct Déconnecté;
struct Connecté { adresse: String }
struct Authentifié { adresse: String, utilisateur: String }

// TODO : implémenter les transitions comme méthodes

fn ex_15_3() {
    let conn = Déconnecté
        .connecter("192.168.1.1")
        .authentifier("admin");

    let données = conn.envoyer("ping");
    assert_eq!(données, "admin@192.168.1.1 >> ping");

    let _ = conn.déconnecter();
    // Impossible d'appeler .envoyer() sur un Déconnecté → erreur de compilation
}
