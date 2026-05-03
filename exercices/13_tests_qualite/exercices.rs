//! Exercices — Chapitre 11 : Tests et qualité
//!
//! Ces exercices sont conçus pour un projet cargo. Créez :
//!   cargo new test-exercices --lib
//! Mettez le code dans src/lib.rs et les tests dans tests/

// =============================================================================
// 11.1 — Tests unitaires dans le module
// =============================================================================
// Implémentez et testez un struct `Statistiques` qui calcule moyenne,
// médiane et mode à partir d'un Vec<f64>.
//
// Placez les tests dans un module #[cfg(test)] à la fin de src/lib.rs.

pub struct Statistiques {
    données: Vec<f64>,
}

impl Statistiques {
    pub fn new(mut données: Vec<f64>) -> Option<Statistiques> {
        if données.is_empty() {
            return None;
        }
        données.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(Statistiques { données })
    }

    pub fn moyenne(&self) -> f64 {
        // TODO
        0.0
    }

    pub fn mediane(&self) -> f64 {
        // TODO : valeur du milieu (ou moyenne des deux du milieu)
        0.0
    }

    pub fn min(&self) -> f64 { *self.données.first().unwrap() }
    pub fn max(&self) -> f64 { *self.données.last().unwrap() }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO : test_moyenne_simple
    // TODO : test_mediane_impaire (5 éléments)
    // TODO : test_mediane_paire (4 éléments → moyenne des deux du milieu)
    // TODO : test_new_vide_retourne_none
    // TODO : test_min_max
}


// =============================================================================
// 11.2 — #[should_panic]
// =============================================================================
// Implémentez `diviser_positif(a: f64, b: f64) -> f64` qui panique
// si l'un des arguments est négatif ou si b == 0.
// Testez les panics avec #[should_panic(expected = "...")]

pub fn diviser_positif(a: f64, b: f64) -> f64 {
    // TODO : paniquer avec messages explicites
    a / b
}

#[cfg(test)]
mod tests_panic {
    use super::*;

    #[test]
    fn test_division_normale() {
        assert_eq!(diviser_positif(10.0, 2.0), 5.0);
    }

    // TODO : test_diviseur_zero avec #[should_panic(expected = "zéro")]
    // TODO : test_argument_negatif avec #[should_panic(expected = "négatif")]
}


// =============================================================================
// 11.3 — Tests d'intégration (fichier tests/integration.rs)
// =============================================================================
// Dans tests/integration.rs, testez la fonction publique suivante
// avec au moins 3 scénarios : liste vide, un seul élément, plusieurs éléments.

pub fn trouver_max<T: PartialOrd>(liste: &[T]) -> Option<&T> {
    liste.iter().reduce(|a, b| if b > a { b } else { a })
}

// Créez tests/integration.rs avec :
// use test_exercices::trouver_max;
//
// #[test] fn test_max_vide() { ... }
// #[test] fn test_max_un_element() { ... }
// #[test] fn test_max_plusieurs() { ... }
// #[test] fn test_max_strings() { ... }


// =============================================================================
// 11.4 — Projet : parseur de logs (projet cargo séparé)
// =============================================================================
// Créez un projet `log-parser` selon la structure du cours.
// Objectifs :
//   1. Parser correctement les lignes du format Combined Log Format
//   2. Calculer les statistiques de base (total, codes status, top IPs)
//   3. Tests d'intégration avec des données de log réalistes
//   4. Gérer les lignes malformées sans paniquer (retourner Err)
//
// Testez avec ce log de référence (4 lignes valides + 1 invalide) :
//
// 127.0.0.1 - alice [01/Jan/2024:00:00:01 +0000] "GET / HTTP/1.1" 200 1024
// 10.0.0.1 - - [01/Jan/2024:00:00:02 +0000] "POST /api HTTP/1.1" 201 256
// 192.168.1.1 - bob [01/Jan/2024:00:00:03 +0000] "GET /img.png HTTP/1.1" 404 0
// 127.0.0.1 - - [01/Jan/2024:00:00:04 +0000] "GET /style.css HTTP/1.1" 200 512
// LIGNE_INVALIDE
