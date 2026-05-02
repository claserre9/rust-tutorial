//! Solutions — Chapitre 7

use std::fmt;
use std::ops::Add;

fn main() {
    ex_7_1();
    ex_7_2();
    ex_7_3();
    ex_7_4();
    println!("Toutes les solutions passent ✅");
}

// 7.1 — Fraction
fn pgcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { pgcd(b, a % b) }
}

#[derive(Debug, Clone, Copy)]
struct Fraction { num: i64, den: i64 }

impl Fraction {
    fn new(num: i64, den: i64) -> Fraction {
        assert!(den != 0, "dénominateur nul");
        let signe = if den < 0 { -1 } else { 1 };
        let g = pgcd(num * signe, den * signe);
        Fraction { num: num * signe / g, den: den * signe / g }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 { write!(f, "{}", self.num) }
        else { write!(f, "{}/{}", self.num, self.den) }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl Add for Fraction {
    type Output = Fraction;
    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

fn ex_7_1() {
    let a = Fraction::new(1, 2);
    let b = Fraction::new(1, 3);
    let c = a + b;
    assert_eq!(c, Fraction::new(5, 6));
    assert_eq!(format!("{c}"), "5/6");
    let d = Fraction::new(4, 2);
    assert_eq!(format!("{d}"), "2");
    let e = Fraction::new(-3, -6);
    assert_eq!(format!("{e}"), "1/2");
}

// 7.2 — Newtype Euros/Centimes
#[derive(Debug, Clone, Copy, PartialEq)]
struct Euros(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Centimes(i64);

impl Euros {
    fn from_centimes(c: i64) -> Euros { Euros(c) }
    fn en_centimes(&self) -> Centimes { Centimes(self.0) }
}

impl fmt::Display for Euros {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02} €", self.0 / 100, (self.0 % 100).abs())
    }
}

impl Add for Euros {
    type Output = Euros;
    fn add(self, other: Euros) -> Euros { Euros(self.0 + other.0) }
}

fn ex_7_2() {
    let prix = Euros::from_centimes(1250);
    assert_eq!(format!("{prix}"), "12.50 €");
    let tva = Euros::from_centimes(250);
    let total = prix + tva;
    assert_eq!(total.en_centimes(), Centimes(1500));
    assert_eq!(format!("{total}"), "15.00 €");
}

// 7.3 — Builder
#[derive(Debug)]
struct Requete {
    url: String,
    methode: String,
    timeout_ms: u64,
    headers: Vec<(String, String)>,
}

struct RequeteBuilder {
    url: String,
    methode: String,
    timeout_ms: u64,
    headers: Vec<(String, String)>,
}

impl RequeteBuilder {
    fn new(url: impl Into<String>) -> RequeteBuilder {
        RequeteBuilder {
            url: url.into(),
            methode: String::from("GET"),
            timeout_ms: 5000,
            headers: Vec::new(),
        }
    }
    fn methode(mut self, m: impl Into<String>) -> RequeteBuilder { self.methode = m.into(); self }
    fn timeout_ms(mut self, t: u64) -> RequeteBuilder { self.timeout_ms = t; self }
    fn header(mut self, k: impl Into<String>, v: impl Into<String>) -> RequeteBuilder {
        self.headers.push((k.into(), v.into())); self
    }
    fn build(self) -> Requete {
        Requete { url: self.url, methode: self.methode, timeout_ms: self.timeout_ms, headers: self.headers }
    }
}

fn ex_7_3() {
    let r = RequeteBuilder::new("https://example.com")
        .methode("POST").timeout_ms(10_000)
        .header("Authorization", "Bearer token123")
        .header("Content-Type", "application/json")
        .build();
    assert_eq!(r.url, "https://example.com");
    assert_eq!(r.methode, "POST");
    assert_eq!(r.timeout_ms, 10_000);
    assert_eq!(r.headers.len(), 2);
    let r2 = RequeteBuilder::new("https://api.example.com/v1").build();
    assert_eq!(r2.methode, "GET");
    assert_eq!(r2.timeout_ms, 5000);
    assert!(r2.headers.is_empty());
}

// 7.4 — Ord personnalisé
#[derive(Debug, Clone)]
struct Etudiant { nom: String, note: f64 }

impl Etudiant {
    fn new(nom: &str, note: f64) -> Etudiant { Etudiant { nom: nom.to_string(), note } }
    fn note_entiere(&self) -> i64 { (self.note * 1000.0).round() as i64 }
}

impl PartialEq for Etudiant {
    fn eq(&self, other: &Etudiant) -> bool {
        self.note_entiere() == other.note_entiere() && self.nom == other.nom
    }
}
impl Eq for Etudiant {}

impl PartialOrd for Etudiant {
    fn partial_cmp(&self, other: &Etudiant) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Etudiant {
    fn cmp(&self, other: &Etudiant) -> std::cmp::Ordering {
        other.note_entiere().cmp(&self.note_entiere())   // note décroissante
            .then(self.nom.cmp(&other.nom))               // nom croissant
    }
}

fn ex_7_4() {
    let mut etudiants = vec![
        Etudiant::new("Charlie", 85.5),
        Etudiant::new("Alice", 92.0),
        Etudiant::new("Bob", 85.5),
        Etudiant::new("Diana", 78.0),
    ];
    etudiants.sort();
    assert_eq!(etudiants[0].nom, "Alice");
    assert_eq!(etudiants[1].nom, "Bob");
    assert_eq!(etudiants[2].nom, "Charlie");
    assert_eq!(etudiants[3].nom, "Diana");
}
