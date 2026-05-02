//! Exercices — Chapitre 8 : Lifetimes

fn main() {
    ex_8_1();
    ex_8_2();
    ex_8_3();
    ex_8_4();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 8.1 — Annoter une fonction
// =============================================================================
// Corrigez la signature de `plus_court` pour qu'elle compile.
// La fonction retourne la référence vers la chaîne la plus courte.

fn plus_court(/* TODO : ajouter lifetime */(s1: &str, s2: &str) -> &str {
    if s1.len() <= s2.len() { s1 } else { s2 }
}

fn ex_8_1() {
    let s1 = String::from("long string");
    let résultat;
    {
        let s2 = String::from("xy");
        résultat = plus_court(&s1, &s2);
        assert_eq!(résultat, "xy");
    }
}


// =============================================================================
// 8.2 — Struct avec référence
// =============================================================================
// Définissez un struct `Titre<'a>` qui emprunte une &str.
// Il doit avoir une méthode `afficher(&self)` qui imprime le titre.
// Ajoutez aussi `tronquer(&self, max: usize) -> &str` qui retourne
// les `max` premiers bytes de la chaîne (sans allouer).

// TODO : struct Titre<'a> + impl

fn ex_8_2() {
    let texte = String::from("Apprendre Rust");
    let titre = Titre { contenu: &texte };
    // afficher ne retourne rien — juste compilation suffisante
    titre.afficher();
    assert_eq!(titre.tronquer(10), "Apprendre ");
    assert_eq!(titre.tronquer(100), "Apprendre Rust"); // ne dépasse pas la longueur
}


// =============================================================================
// 8.3 — Choisir la bonne lifetime
// =============================================================================
// La fonction `choisir` prend deux &str et un bool.
// Si le bool est vrai, retourne s1 ; sinon retourne s2.
// Quelle est la bonne annotation ?
// Indice : le retour peut venir de l'un ou de l'autre.

fn choisir(s1: &str, s2: &str, premier: bool) -> &str {
    // TODO : corriger la signature (ajouter la bonne annotation)
    if premier { s1 } else { s2 }
}

fn ex_8_3() {
    let a = String::from("premier");
    let b = String::from("second");
    assert_eq!(choisir(&a, &b, true), "premier");
    assert_eq!(choisir(&a, &b, false), "second");
}


// =============================================================================
// 8.4 — Éviter une lifetime inutile
// =============================================================================
// La fonction `version_majeur` extrait le numéro majeur d'une string "X.Y.Z".
// Elle DOIT retourner &str (pas String).
// Version 1 : annotez correctement.
// Version 2 : pouvez-vous utiliser les règles d'élision (sans annotation) ?

fn version_majeur(version: &str) -> &str {
    // TODO : retourner le premier segment avant le '.'
    // ex: "1.2.3" → "1"
    version
}

fn ex_8_4() {
    assert_eq!(version_majeur("1.2.3"), "1");
    assert_eq!(version_majeur("10.0.1"), "10");
    assert_eq!(version_majeur("2024"), "2024"); // pas de '.' → tout retourner
}
