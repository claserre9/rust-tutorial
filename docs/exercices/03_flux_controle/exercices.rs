//! Exercices — Chapitre 3 : Flux de contrôle
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices
//!
//! Ou créez un projet cargo et mettez ce contenu dans src/main.rs.

fn main() {
    ex_3_1();
    ex_3_2();
    ex_3_3();
    ex_3_4();
    ex_3_5();
    ex_3_6();
    ex_3_7();
    ex_3_8();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 3.1 — `if` comme expression
// =============================================================================
// Complétez `signe` pour qu'elle retourne "positif", "négatif" ou "zéro"
// en utilisant `if` comme expression (pas de `return`, pas de `match`).

fn signe(n: i32) -> &'static str {
    // TODO : une seule expression if/else if/else
    ""
}

fn ex_3_1() {
    assert_eq!(signe(5), "positif");
    assert_eq!(signe(-3), "négatif");
    assert_eq!(signe(0), "zéro");
}


// =============================================================================
// 3.2 — `loop` avec valeur de retour
// =============================================================================
// Utilisez `loop` pour trouver le premier entier >= start dont le carré
// dépasse threshold. Retournez cet entier via `break valeur`.

fn premier_carre_superieur(start: u32, threshold: u32) -> u32 {
    // TODO : loop avec break valeur
    0
}

fn ex_3_2() {
    // 5² = 25 > 20, mais 4² = 16 <= 20
    assert_eq!(premier_carre_superieur(0, 20), 5);
    // 10² = 100 > 99
    assert_eq!(premier_carre_superieur(0, 99), 10);
    // start = 8 : 8² = 64 > 50
    assert_eq!(premier_carre_superieur(8, 50), 8);
}


// =============================================================================
// 3.3 — `for` avec range et enumerate
// =============================================================================
// Remplissez le vecteur `evens` avec les nombres pairs de 0 à n (inclus)
// en utilisant `for` sur un range. N'utilisez PAS filter().

fn pairs_jusqu_a(n: u32) -> Vec<u32> {
    let mut evens = Vec::new();
    // TODO : for i in 0..=n, ajouter si pair
    evens
}

fn ex_3_3() {
    assert_eq!(pairs_jusqu_a(6), vec![0, 2, 4, 6]);
    assert_eq!(pairs_jusqu_a(7), vec![0, 2, 4, 6]);
    assert_eq!(pairs_jusqu_a(0), vec![0]);
}


// =============================================================================
// 3.4 — Étiquettes de boucle
// =============================================================================
// Trouvez la première paire (i, j) avec i in 1..=5, j in 1..=5, i*j > 12.
// Retournez-la via break 'outer (i, j).

fn premiere_paire_produit() -> (u32, u32) {
    // TODO : boucle étiquetée 'outer
    (0, 0)
}

fn ex_3_4() {
    let (i, j) = premiere_paire_produit();
    assert!(i * j > 12, "attendu i*j > 12, obtenu {}*{} = {}", i, j, i * j);
    // Vérifier que c'est bien la PREMIÈRE : aucune paire antérieure ne satisfait
    'check: for a in 1..=5u32 {
        for b in 1..=5u32 {
            if (a, b) == (i, j) {
                break 'check;
            }
            assert!(a * b <= 12, "paire ({},{}) avec produit {} vient avant ({},{})", a, b, a*b, i, j);
        }
    }
}


// =============================================================================
// 3.5 — `match` sur entier avec garde
// =============================================================================
// Classifiez un score (0–100) :
//   90..=100 → "A"
//   80..=89  → "B"
//   70..=79  → "C"
//   60..=69  → "D"
//   autre    → "F"
// Bonus : si le score est exactement 100, retournez "A+" (garde `if`).

fn note(score: u32) -> &'static str {
    // TODO : match avec ranges et guard
    ""
}

fn ex_3_5() {
    assert_eq!(note(100), "A+");
    assert_eq!(note(95), "A");
    assert_eq!(note(85), "B");
    assert_eq!(note(72), "C");
    assert_eq!(note(65), "D");
    assert_eq!(note(50), "F");
}


// =============================================================================
// 3.6 — `match` sur enum
// =============================================================================
// L'enum `Commande` est défini ci-dessous. Implémentez `executer` qui retourne
// une description string de la commande executée.

#[derive(Debug)]
enum Commande {
    Avancer(u32),           // distance en mètres
    Tourner { angle: i32 }, // angle en degrés (négatif = gauche)
    Stop,
}

fn executer(cmd: &Commande) -> String {
    // TODO : match cmd { ... }
    // Avancer(d)        → "avance de {d}m"
    // Tourner { angle } → "tourne de {angle}°"  (négatif = gauche, positif = droite, si angle == 0 → "tout droit")
    // Stop              → "arrêt"
    String::new()
}

fn ex_3_6() {
    assert_eq!(executer(&Commande::Avancer(10)), "avance de 10m");
    assert_eq!(executer(&Commande::Tourner { angle: 90 }), "tourne de 90°");
    assert_eq!(executer(&Commande::Tourner { angle: -45 }), "tourne de -45°");
    assert_eq!(executer(&Commande::Tourner { angle: 0 }), "tout droit");
    assert_eq!(executer(&Commande::Stop), "arrêt");
}


// =============================================================================
// 3.7 — `if let` et `while let`
// =============================================================================
// 3.7a : Utilisez `if let` pour extraire la valeur d'une Option<u32>.
//        Retournez la valeur doublée si Some, 0 si None.
//
// 3.7b : Utilisez `while let` pour vider une pile (Vec) et calculer la somme.

fn double_ou_zero(opt: Option<u32>) -> u32 {
    // TODO : if let
    0
}

fn somme_pile(mut pile: Vec<i32>) -> i32 {
    let mut total = 0;
    // TODO : while let Some(x) = pile.pop()
    total
}

fn ex_3_7() {
    assert_eq!(double_ou_zero(Some(21)), 42);
    assert_eq!(double_ou_zero(None), 0);

    assert_eq!(somme_pile(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(somme_pile(vec![]), 0);
    assert_eq!(somme_pile(vec![-1, 1]), 0);
}


// =============================================================================
// 3.8 — `let else` (early return)
// =============================================================================
// Parsez une chaîne de la forme "prenom:age" (ex: "Alice:30").
// Retournez une erreur descriptive si le format est invalide ou si l'âge
// n'est pas un u32 valide. Utilisez `let else` pour les early returns.

fn parser_personne(input: &str) -> Result<(String, u32), String> {
    // TODO : let else pour séparer le ':' et parser l'âge
    // Indices :
    //   input.split_once(':')  →  Option<(&str, &str)>
    //   age_str.parse::<u32>() →  Result<u32, _>
    Err("non implémenté".to_string())
}

fn ex_3_8() {
    assert_eq!(parser_personne("Alice:30"), Ok((String::from("Alice"), 30)));
    assert_eq!(parser_personne("Bob:0"), Ok((String::from("Bob"), 0)));
    assert!(parser_personne("pas-de-deux-points").is_err());
    assert!(parser_personne("Alice:abc").is_err());
    assert!(parser_personne("Alice:-1").is_err()); // u32 refuse les négatifs
}
