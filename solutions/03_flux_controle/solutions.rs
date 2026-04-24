//! Solutions — Chapitre 3

fn main() {
    ex_3_1();
    ex_3_2();
    ex_3_3();
    ex_3_4();
    ex_3_5();
    ex_3_6();
    ex_3_7();
    ex_3_8();
    println!("Toutes les solutions passent ✅");
}

// 3.1
fn signe(n: i32) -> &'static str {
    if n > 0 { "positif" } else if n < 0 { "négatif" } else { "zéro" }
}

fn ex_3_1() {
    assert_eq!(signe(5), "positif");
    assert_eq!(signe(-3), "négatif");
    assert_eq!(signe(0), "zéro");
}

// 3.2
fn premier_carre_superieur(start: u32, threshold: u32) -> u32 {
    let mut n = start;
    loop {
        if n * n > threshold {
            break n;
        }
        n += 1;
    }
}

fn ex_3_2() {
    assert_eq!(premier_carre_superieur(0, 20), 5);
    assert_eq!(premier_carre_superieur(0, 99), 10);
    assert_eq!(premier_carre_superieur(8, 50), 8);
}

// 3.3
fn pairs_jusqu_a(n: u32) -> Vec<u32> {
    let mut evens = Vec::new();
    for i in 0..=n {
        if i % 2 == 0 {
            evens.push(i);
        }
    }
    evens
}

fn ex_3_3() {
    assert_eq!(pairs_jusqu_a(6), vec![0, 2, 4, 6]);
    assert_eq!(pairs_jusqu_a(7), vec![0, 2, 4, 6]);
    assert_eq!(pairs_jusqu_a(0), vec![0]);
}

// 3.4
fn premiere_paire_produit() -> (u32, u32) {
    'outer: for i in 1..=5u32 {
        for j in 1..=5u32 {
            if i * j > 12 {
                break 'outer (i, j);
            }
        }
    }
}

fn ex_3_4() {
    let (i, j) = premiere_paire_produit();
    assert!(i * j > 12);
    'check: for a in 1..=5u32 {
        for b in 1..=5u32 {
            if (a, b) == (i, j) { break 'check; }
            assert!(a * b <= 12);
        }
    }
}

// 3.5
fn note(score: u32) -> &'static str {
    match score {
        100 => "A+",
        90..=99 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn ex_3_5() {
    assert_eq!(note(100), "A+");
    assert_eq!(note(95), "A");
    assert_eq!(note(85), "B");
    assert_eq!(note(72), "C");
    assert_eq!(note(65), "D");
    assert_eq!(note(50), "F");
}

// 3.6
#[derive(Debug)]
enum Commande {
    Avancer(u32),
    Tourner { angle: i32 },
    Stop,
}

fn executer(cmd: &Commande) -> String {
    match cmd {
        Commande::Avancer(d) => format!("avance de {d}m"),
        Commande::Tourner { angle: 0 } => String::from("tout droit"),
        Commande::Tourner { angle } => format!("tourne de {angle}°"),
        Commande::Stop => String::from("arrêt"),
    }
}

fn ex_3_6() {
    assert_eq!(executer(&Commande::Avancer(10)), "avance de 10m");
    assert_eq!(executer(&Commande::Tourner { angle: 90 }), "tourne de 90°");
    assert_eq!(executer(&Commande::Tourner { angle: -45 }), "tourne de -45°");
    assert_eq!(executer(&Commande::Tourner { angle: 0 }), "tout droit");
    assert_eq!(executer(&Commande::Stop), "arrêt");
}

// 3.7
fn double_ou_zero(opt: Option<u32>) -> u32 {
    if let Some(n) = opt { n * 2 } else { 0 }
}

fn somme_pile(mut pile: Vec<i32>) -> i32 {
    let mut total = 0;
    while let Some(x) = pile.pop() {
        total += x;
    }
    total
}

fn ex_3_7() {
    assert_eq!(double_ou_zero(Some(21)), 42);
    assert_eq!(double_ou_zero(None), 0);
    assert_eq!(somme_pile(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(somme_pile(vec![]), 0);
    assert_eq!(somme_pile(vec![-1, 1]), 0);
}

// 3.8
fn parser_personne(input: &str) -> Result<(String, u32), String> {
    let Some((prenom, age_str)) = input.split_once(':') else {
        return Err(format!("format invalide, ':' manquant dans '{input}'"));
    };
    let Ok(age) = age_str.parse::<u32>() else {
        return Err(format!("âge invalide : '{age_str}'"));
    };
    Ok((prenom.to_string(), age))
}

fn ex_3_8() {
    assert_eq!(parser_personne("Alice:30"), Ok((String::from("Alice"), 30)));
    assert_eq!(parser_personne("Bob:0"), Ok((String::from("Bob"), 0)));
    assert!(parser_personne("pas-de-deux-points").is_err());
    assert!(parser_personne("Alice:abc").is_err());
    assert!(parser_personne("Alice:-1").is_err());
}
