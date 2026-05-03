//! Solutions — Chapitre 10

use std::collections::HashMap;
use std::fmt;

fn main() {
    ex_10_1();
    ex_10_2();
    ex_10_3();
    ex_10_4();
    println!("Toutes les solutions passent ✅");
}

// 10.1
struct Pile<T> { données: Vec<T> }

impl<T> Pile<T> {
    fn new() -> Pile<T> { Pile { données: Vec::new() } }
    fn empiler(&mut self, val: T) { self.données.push(val); }
    fn depiler(&mut self) -> Option<T> { self.données.pop() }
    fn sommet(&self) -> Option<&T> { self.données.last() }
    fn est_vide(&self) -> bool { self.données.is_empty() }
    fn taille(&self) -> usize { self.données.len() }
}

impl<T: fmt::Display> fmt::Display for Pile<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[bas → ")?;
        for (i, v) in self.données.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{v}")?;
        }
        write!(f, " → haut]")
    }
}

fn ex_10_1() {
    let mut p: Pile<i32> = Pile::new();
    assert!(p.est_vide());
    p.empiler(1); p.empiler(2); p.empiler(3);
    assert_eq!(p.taille(), 3);
    assert_eq!(p.sommet(), Some(&3));
    assert_eq!(p.depiler(), Some(3));
    assert_eq!(p.depiler(), Some(2));
    assert_eq!(p.taille(), 1);
    assert!(!p.est_vide());
}

// 10.2
fn somme_impairs(n: usize) -> u64 {
    (0u64..).filter(|i| i % 2 != 0).take(n).sum()
}

fn paires_distinctes(n: usize) -> Vec<(usize, usize)> {
    (0..n).flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j))).collect()
}

fn histogramme(s: &str) -> HashMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_lowercase())
        .fold(HashMap::new(), |mut map, c| { *map.entry(c).or_insert(0) += 1; map })
}

fn ex_10_2() {
    assert_eq!(somme_impairs(5), 25);
    assert_eq!(somme_impairs(1), 1);
    let paires = paires_distinctes(3);
    assert_eq!(paires.len(), 6);
    assert!(paires.contains(&(0, 1)));
    assert!(!paires.contains(&(1, 1)));
    let h = histogramme("hello world");
    assert_eq!(h[&'l'], 3);
    assert_eq!(h.get(&' '), None);
}

// 10.3
fn parser_csv_ligne(s: &str) -> Result<Vec<i32>, String> {
    s.split(',')
        .map(|t| t.trim().parse::<i32>().map_err(|_| format!("token invalide: '{}'", t.trim())))
        .collect()
}

fn ex_10_3() {
    assert_eq!(parser_csv_ligne("1, 2, 3, 4"), Ok(vec![1, 2, 3, 4]));
    assert_eq!(parser_csv_ligne("42"), Ok(vec![42]));
    assert!(parser_csv_ligne("1, abc, 3").is_err());
    assert!(parser_csv_ligne("").is_err());
}

// 10.4
struct FenetreGlissante<'a, T> { data: &'a [T], taille: usize, position: usize }

impl<'a, T> FenetreGlissante<'a, T> {
    fn new(data: &'a [T], taille: usize) -> FenetreGlissante<'a, T> {
        FenetreGlissante { data, taille, position: 0 }
    }
}

impl<'a, T> Iterator for FenetreGlissante<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        let fin = self.position + self.taille;
        if fin > self.data.len() { return None; }
        let fenêtre = &self.data[self.position..fin];
        self.position += 1;
        Some(fenêtre)
    }
}

fn ex_10_4() {
    let v = vec![1, 2, 3, 4, 5];
    let fenêtres: Vec<&[i32]> = FenetreGlissante::new(&v, 3).collect();
    assert_eq!(fenêtres, vec![&[1,2,3], &[2,3,4], &[3,4,5]]);
    let vide: Vec<_> = FenetreGlissante::new(&v, 10).collect();
    assert!(vide.is_empty());
    let max_somme = FenetreGlissante::new(&v, 2).map(|w| w.iter().sum::<i32>()).max();
    assert_eq!(max_somme, Some(9));
}
