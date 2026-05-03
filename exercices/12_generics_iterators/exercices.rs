//! Exercices — Chapitre 10 : Génériques et itérateurs

use std::collections::HashMap;

fn main() {
    ex_10_1();
    ex_10_2();
    ex_10_3();
    ex_10_4();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 10.1 — Struct générique avec bounds
// =============================================================================
// Implémentez `Pile<T>` (stack) avec :
//   - `new() -> Pile<T>`
//   - `empiler(&mut self, val: T)`
//   - `depiler(&mut self) -> Option<T>`
//   - `sommet(&self) -> Option<&T>`
//   - `est_vide(&self) -> bool`
//   - `taille(&self) -> usize`
//
// Bonus : impl Display pour Pile<T> où T: Display
//   Format : "[bas → 1, 2, 3 → haut]"

// TODO

fn ex_10_1() {
    let mut p: Pile<i32> = Pile::new();
    assert!(p.est_vide());

    p.empiler(1);
    p.empiler(2);
    p.empiler(3);
    assert_eq!(p.taille(), 3);
    assert_eq!(p.sommet(), Some(&3));
    assert_eq!(p.depiler(), Some(3));
    assert_eq!(p.depiler(), Some(2));
    assert_eq!(p.taille(), 1);
    assert!(!p.est_vide());
}


// =============================================================================
// 10.2 — Chaîne d'itérateurs
// =============================================================================
// Sans utiliser de boucle for ni collect intermédiaire, résolvez :
//
// (a) Somme des N premiers nombres impairs (1, 3, 5, ...)
// (b) Produit cartésien simplifié : tous les (i, j) avec i in 0..n, j in 0..n, i != j
//     → retourner le vecteur de paires
// (c) Histogramme : compter les occurrences de chaque lettre minuscule dans un &str

fn somme_impairs(n: usize) -> u64 {
    // TODO : (0..).filter().take(n).map().sum()  ou plus direct
    0
}

fn paires_distinctes(n: usize) -> Vec<(usize, usize)> {
    // TODO : flat_map + filter
    Vec::new()
}

fn histogramme(s: &str) -> HashMap<char, usize> {
    // TODO : fold ou for_each sur s.chars()
    HashMap::new()
}

fn ex_10_2() {
    assert_eq!(somme_impairs(5), 25);   // 1+3+5+7+9 = 25
    assert_eq!(somme_impairs(1), 1);

    let paires = paires_distinctes(3);
    assert_eq!(paires.len(), 6);        // (0,1),(0,2),(1,0),(1,2),(2,0),(2,1)
    assert!(paires.contains(&(0, 1)));
    assert!(!paires.contains(&(1, 1)));

    let h = histogramme("hello world");
    assert_eq!(h[&'l'], 3);
    assert_eq!(h.get(&' '), None); // espaces non comptés (pas une lettre minuscule)
}


// =============================================================================
// 10.3 — collect vers Result
// =============================================================================
// Implémentez `parser_csv_ligne` qui parse une ligne CSV de i32 séparés par ','
// Retourne Ok(Vec<i32>) ou Err avec le premier token invalide.

fn parser_csv_ligne(s: &str) -> Result<Vec<i32>, String> {
    // TODO : .split(',').map(|t| t.trim().parse::<i32>().map_err(...)).collect()
    Err("non implémenté".to_string())
}

fn ex_10_3() {
    assert_eq!(parser_csv_ligne("1, 2, 3, 4"), Ok(vec![1, 2, 3, 4]));
    assert_eq!(parser_csv_ligne("42"), Ok(vec![42]));
    assert!(parser_csv_ligne("1, abc, 3").is_err());
    assert!(parser_csv_ligne("").is_err()); // "" .parse() échoue
}


// =============================================================================
// 10.4 — Implémentation Iterator personnalisée
// =============================================================================
// Implémentez l'itérateur `FenetreGlissante<'a, T>` qui yield des &[T]
// (sous-slices de taille `taille`) en avançant d'un pas à chaque fois.
//
// Exemple : [1,2,3,4,5] avec taille=3 → [1,2,3], [2,3,4], [3,4,5]

struct FenetreGlissante<'a, T> {
    data: &'a [T],
    taille: usize,
    position: usize,
}

impl<'a, T> FenetreGlissante<'a, T> {
    fn new(data: &'a [T], taille: usize) -> FenetreGlissante<'a, T> {
        FenetreGlissante { data, taille, position: 0 }
    }
}

impl<'a, T> Iterator for FenetreGlissante<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        // TODO
        None
    }
}

fn ex_10_4() {
    let v = vec![1, 2, 3, 4, 5];
    let fenêtres: Vec<&[i32]> = FenetreGlissante::new(&v, 3).collect();
    assert_eq!(fenêtres, vec![&[1,2,3], &[2,3,4], &[3,4,5]]);

    // Taille > len → aucune fenêtre
    let vide: Vec<_> = FenetreGlissante::new(&v, 10).collect();
    assert!(vide.is_empty());

    // Somme maximale sur une fenêtre de taille 2
    let max_somme = FenetreGlissante::new(&v, 2)
        .map(|w| w.iter().sum::<i32>())
        .max();
    assert_eq!(max_somme, Some(9)); // [4,5]
}
