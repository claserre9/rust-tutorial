//! Exercices — Chapitre 2
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices
//!
//! Ou créez un projet cargo et mettez ce contenu dans src/main.rs.

fn main() {
    ex_2_1();
    ex_2_2();
    ex_2_3();
    ex_2_4();
    ex_2_5();
    ex_2_6();
    ex_2_7();
    ex_2_8();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 2.1 — Immuabilité
// =============================================================================
// Corrigez le code suivant pour qu'il compile et affiche 10.

fn ex_2_1() {
    // TODO : modifier la déclaration
    let x = 5;
    // x = 10;         // décommenter et corriger
    assert_eq!(x, 10);
}


// =============================================================================
// 2.2 — Shadowing vs mut
// =============================================================================
// Utilisez le SHADOWING (pas mut) pour transformer `input` en majuscules,
// puis en lowercase. Le type peut changer (String).

fn ex_2_2() {
    let input = "Bonjour";
    // TODO : par shadowing, transformer successivement
    // assert_eq!(input, "bonjour");
}


// =============================================================================
// 2.3 — Overflow checks
// =============================================================================
// Démontrez l'overflow. En debug, l'opération ci-dessous panique.
// Remplacez par une variante qui gère l'overflow (wrap, checked, saturating).

fn ex_2_3() {
    let a: u8 = 250;
    let b: u8 = 10;

    // TODO : remplacer `a + b` par une variante sûre qui retourne 255
    // let result = a + b;  // panic en debug
    let result: u8 = 0;  // TODO
    assert_eq!(result, 255);
}


// =============================================================================
// 2.4 — &str vs String
// =============================================================================
// Écrivez une fonction `saluer` qui prend un &str et retourne un String
// "Bonjour, {nom} !". Puis appelez-la avec un littéral ET avec un String.

fn saluer(/* TODO */) -> String {
    // TODO
    String::new()
}

fn ex_2_4() {
    let s = String::from("Alice");
    // assert_eq!(saluer("Bob"), "Bonjour, Bob !");
    // assert_eq!(saluer(&s), "Bonjour, Alice !");
}


// =============================================================================
// 2.5 — Déstructuration de tuple
// =============================================================================

fn ex_2_5() {
    let point = (3, 4);
    // TODO : déstructurer en (x, y) et calculer x² + y² = 25
    let somme_carres: i32 = 0; // TODO
    assert_eq!(somme_carres, 25);
}


// =============================================================================
// 2.6 — Expression vs statement
// =============================================================================
// Complétez la fonction pour qu'elle retourne la valeur absolue.
// UTILISEZ `if` comme expression (pas de `return`).

fn abs_value(x: i32) -> i32 {
    // TODO : une seule expression avec if
    0
}

fn ex_2_6() {
    assert_eq!(abs_value(-5), 5);
    assert_eq!(abs_value(3), 3);
    assert_eq!(abs_value(0), 0);
}


// =============================================================================
// 2.7 — Conversion From / TryFrom
// =============================================================================

fn ex_2_7() {
    let x: i64 = i64::from(42i32);
    assert_eq!(x, 42);

    // TODO : convertir 300i32 en u8 avec TryFrom, gérer le Result
    // Doit échouer (300 > 255).
    let res: Result<u8, _> = u8::try_from(300i32);
    assert!(res.is_err());

    // TODO : convertir 42i32 en u8, doit réussir
    let ok: Result<u8, _> = u8::try_from(42i32);
    assert_eq!(ok.unwrap(), 42);
}


// =============================================================================
// 2.8 — Option<T>
// =============================================================================
// Écrivez `div(a, b)` qui retourne Some(a/b) si b != 0, None sinon.

fn div(a: i32, b: i32) -> Option<i32> {
    // TODO
    None
}

fn ex_2_8() {
    assert_eq!(div(10, 2), Some(5));
    assert_eq!(div(10, 0), None);
    assert_eq!(div(10, 0).unwrap_or(-1), -1);
}
