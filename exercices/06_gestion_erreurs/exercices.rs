//! Exercices — Chapitre 6 : Gestion des erreurs
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices

fn main() {
    ex_6_1();
    ex_6_2();
    ex_6_3();
    ex_6_4();
    ex_6_5();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 6.1 — Méthodes sur Option
// =============================================================================
// Implémentez les fonctions suivantes en utilisant UNIQUEMENT des méthodes
// sur Option (map, and_then, unwrap_or, filter, etc.) — pas de match ni if let.

fn doubler_si_positif(opt: Option<i32>) -> Option<i32> {
    // TODO : filter + map
    None
}

fn description(opt: Option<&str>) -> String {
    // TODO : map + unwrap_or  (si Some(s) → "valeur: {s}", si None → "absent")
    String::new()
}

fn première_lettre_majuscule(opt: Option<&str>) -> Option<char> {
    // TODO : and_then + chars().next() + to_uppercase().next()
    None
}

fn ex_6_1() {
    assert_eq!(doubler_si_positif(Some(5)), Some(10));
    assert_eq!(doubler_si_positif(Some(-3)), None);
    assert_eq!(doubler_si_positif(None), None);

    assert_eq!(description(Some("bonjour")), "valeur: bonjour");
    assert_eq!(description(None), "absent");

    assert_eq!(première_lettre_majuscule(Some("rust")), Some('R'));
    assert_eq!(première_lettre_majuscule(Some("")), None);
    assert_eq!(première_lettre_majuscule(None), None);
}


// =============================================================================
// 6.2 — Méthodes sur Result
// =============================================================================
// Sans utiliser match ni if let, implémentez :

fn parser_et_valider(s: &str) -> Result<u32, String> {
    // TODO : parse::<i32>().map_err(...), puis vérifier >= 0 avec
    // .and_then(|n| if n >= 0 { Ok(n as u32) } else { Err(...) })
    Err("non implémenté".to_string())
}

fn doubler_resultat(r: Result<i32, String>) -> Result<i32, String> {
    // TODO : map
    Err("non implémenté".to_string())
}

fn ex_6_2() {
    assert_eq!(parser_et_valider("42"), Ok(42));
    assert_eq!(parser_et_valider("0"), Ok(0));
    assert!(parser_et_valider("-1").is_err());
    assert!(parser_et_valider("abc").is_err());

    assert_eq!(doubler_resultat(Ok(21)), Ok(42));
    assert!(doubler_resultat(Err("oups".to_string())).is_err());
}


// =============================================================================
// 6.3 — L'opérateur ?
// =============================================================================
// Implémentez ces fonctions en utilisant ? pour propager les erreurs.
// Ne pas utiliser unwrap().

fn lire_deux_entiers(a: &str, b: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    // TODO : parser a et b avec ?
    todo!()
}

fn somme_fichier_simulé(lignes: &[&str]) -> Result<i32, String> {
    // Simule la lecture d'un fichier : chaque &str est une ligne à parser en i32.
    // Propager les erreurs de parse avec ? (après map_err).
    // Retourner la somme si tout réussit.
    // TODO
    todo!()
}

fn ex_6_3() {
    assert_eq!(lire_deux_entiers("10", "32"), Ok((10, 32)));
    assert!(lire_deux_entiers("10", "abc").is_err());

    assert_eq!(somme_fichier_simulé(&["1", "2", "3"]), Ok(6));
    assert!(somme_fichier_simulé(&["1", "oups", "3"]).is_err());
}


// =============================================================================
// 6.4 — Type d'erreur personnalisé avec thiserror
// =============================================================================
// Définissez AppError avec thiserror pour représenter :
//   - une erreur de parsing (wrappant ParseIntError)
//   - une erreur de validation avec un message
//
// Implémentez parse_age(s: &str) -> Result<u8, AppError> qui :
//   1. Parse s en i32 (ParseIntError → AppError::Parse via #[from])
//   2. Vérifie que la valeur est entre 0 et 150 (sinon AppError::Validation)
//   3. Retourne Ok(valeur as u8)
//
// Note : thiserror nécessite un projet cargo.
// Pour rustc seul, implémentez AppError manuellement (voir cours §5).

// VERSION MANUELLE (sans thiserror) pour compilation avec rustc :

#[derive(Debug, PartialEq)]
enum AppError {
    Parse(std::num::ParseIntError),
    Validation(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "erreur de parsing : {e}"),
            AppError::Validation(s) => write!(f, "validation échouée : {s}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> AppError {
        AppError::Parse(e)
    }
}

fn parse_age(s: &str) -> Result<u8, AppError> {
    // TODO : parser puis valider avec ?
    todo!()
}

fn ex_6_4() {
    assert_eq!(parse_age("25"), Ok(25));
    assert_eq!(parse_age("0"), Ok(0));
    assert_eq!(parse_age("150"), Ok(150));
    assert!(matches!(parse_age("abc"), Err(AppError::Parse(_))));
    assert!(matches!(parse_age("-1"), Err(AppError::Validation(_))));
    assert!(matches!(parse_age("151"), Err(AppError::Validation(_))));
}


// =============================================================================
// 6.5 — Conversion Option ↔ Result
// =============================================================================

fn trouver_et_doubler(v: &[i32], cible: i32) -> Result<i32, String> {
    // TODO :
    // 1. v.iter().find(|&&x| x == cible)  → Option<&i32>
    // 2. .copied()                         → Option<i32>
    // 3. .ok_or_else(|| format!(...))      → Result<i32, String>
    // 4. .map(|n| n * 2)                   → Result<i32, String>
    Err("non implémenté".to_string())
}

fn ex_6_5() {
    let v = vec![1, 5, 10, 42];
    assert_eq!(trouver_et_doubler(&v, 5), Ok(10));
    assert_eq!(trouver_et_doubler(&v, 42), Ok(84));
    assert!(trouver_et_doubler(&v, 99).is_err());
}
