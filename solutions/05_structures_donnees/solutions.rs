//! Solutions — Chapitre 5

use std::collections::HashMap;

fn main() {
    ex_5_1();
    ex_5_2();
    ex_5_3();
    ex_5_4();
    ex_5_5();
    ex_5_6();
    println!("Toutes les solutions passent ✅");
}

// 5.1
#[derive(Debug, PartialEq)]
struct Vecteur2D {
    x: f64,
    y: f64,
}

impl Vecteur2D {
    fn new(x: f64, y: f64) -> Vecteur2D {
        Vecteur2D { x, y }
    }

    fn norme(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normaliser(&self) -> Vecteur2D {
        let n = self.norme();
        Vecteur2D::new(self.x / n, self.y / n)
    }

    fn addition(&self, autre: &Vecteur2D) -> Vecteur2D {
        Vecteur2D::new(self.x + autre.x, self.y + autre.y)
    }
}

fn ex_5_1() {
    let v1 = Vecteur2D::new(3.0, 4.0);
    assert!((v1.norme() - 5.0).abs() < 1e-9);
    let u = v1.normaliser();
    assert!((u.norme() - 1.0).abs() < 1e-9);
    let v2 = Vecteur2D::new(1.0, 0.0);
    let v3 = v1.addition(&v2);
    assert_eq!(v3, Vecteur2D::new(4.0, 4.0));
}

// 5.2
enum Monnaie {
    Piece(u32),
    Billet(u32),
}

impl Monnaie {
    fn valeur_centimes(&self) -> u32 {
        match self {
            Monnaie::Piece(c) => *c,
            Monnaie::Billet(e) => e * 100,
        }
    }
}

fn ex_5_2() {
    let p1 = Monnaie::Piece(50);
    let p2 = Monnaie::Piece(10);
    let b1 = Monnaie::Billet(20);
    assert_eq!(p1.valeur_centimes(), 50);
    assert_eq!(p2.valeur_centimes(), 10);
    assert_eq!(b1.valeur_centimes(), 2000);
    let total: u32 = [p1, p2, b1].iter().map(|m| m.valeur_centimes()).sum();
    assert_eq!(total, 2060);
}

// 5.3
fn chercher_et_doubler(map: &HashMap<&str, i32>, cle: &str) -> Option<i32> {
    map.get(cle).copied().filter(|&v| v > 0).map(|v| v * 2)
}

fn ex_5_3() {
    let mut map = HashMap::new();
    map.insert("a", 5);
    map.insert("b", -3);
    map.insert("c", 0);
    assert_eq!(chercher_et_doubler(&map, "a"), Some(10));
    assert_eq!(chercher_et_doubler(&map, "b"), None);
    assert_eq!(chercher_et_doubler(&map, "c"), None);
    assert_eq!(chercher_et_doubler(&map, "z"), None);
}

// 5.4
fn lire_et_diviser(s: &str, diviseur: i32) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e| format!("parse error: {e}"))?;
    if diviseur == 0 {
        return Err(String::from("division par zéro"));
    }
    Ok(n / diviseur)
}

fn ex_5_4() {
    assert_eq!(lire_et_diviser("42", 6), Ok(7));
    assert_eq!(lire_et_diviser("10", 3), Ok(3));
    assert!(lire_et_diviser("abc", 2).is_err());
    assert!(lire_et_diviser("10", 0).is_err());
}

// 5.5
#[derive(Debug, PartialEq)]
struct Stats {
    min: f64,
    max: f64,
    moyenne: f64,
}

fn statistiques(data: &[f64]) -> Option<Stats> {
    if data.is_empty() {
        return None;
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let moyenne = data.iter().sum::<f64>() / data.len() as f64;
    Some(Stats { min, max, moyenne })
}

fn ex_5_5() {
    let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let s = statistiques(&data).unwrap();
    assert_eq!(s.min, 1.0);
    assert_eq!(s.max, 9.0);
    assert!((s.moyenne - 3.875).abs() < 1e-9);
    assert!(statistiques(&[]).is_none());
}

// 5.6
fn frequences(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if c != ' ' {
            *map.entry(c).or_insert(0) += 1;
        }
    }
    map
}

fn ex_5_6() {
    let f = frequences("hello world");
    assert_eq!(f[&'l'], 3);
    assert_eq!(f[&'o'], 2);
    assert_eq!(f[&'h'], 1);
    assert!(!f.contains_key(&' '));
    let f2 = frequences("aabbc");
    assert_eq!(f2[&'a'], 2);
    assert_eq!(f2[&'b'], 2);
    assert_eq!(f2[&'c'], 1);
}
