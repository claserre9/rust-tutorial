//! Solutions — Chapitre 8

fn main() {
    ex_8_1();
    ex_8_2();
    ex_8_3();
    ex_8_4();
    println!("Toutes les solutions passent ✅");
}

// 8.1
fn plus_court<'a>(s1: &'a str, s2: &'a str) -> &'a str {
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

// 8.2
struct Titre<'a> {
    contenu: &'a str,
}

impl<'a> Titre<'a> {
    fn afficher(&self) {
        println!("Titre : {}", self.contenu);
    }

    fn tronquer(&self, max: usize) -> &str {
        let fin = max.min(self.contenu.len());
        &self.contenu[..fin]
    }
}

fn ex_8_2() {
    let texte = String::from("Apprendre Rust");
    let titre = Titre { contenu: &texte };
    titre.afficher();
    assert_eq!(titre.tronquer(10), "Apprendre ");
    assert_eq!(titre.tronquer(100), "Apprendre Rust");
}

// 8.3 — les deux &str doivent partager la même lifetime car le retour peut être l'un ou l'autre
fn choisir<'a>(s1: &'a str, s2: &'a str, premier: bool) -> &'a str {
    if premier { s1 } else { s2 }
}

fn ex_8_3() {
    let a = String::from("premier");
    let b = String::from("second");
    assert_eq!(choisir(&a, &b, true), "premier");
    assert_eq!(choisir(&a, &b, false), "second");
}

// 8.4 — élision suffit car un seul paramètre référence → sa lifetime est assignée au retour
fn version_majeur(version: &str) -> &str {
    match version.find('.') {
        Some(pos) => &version[..pos],
        None => version,
    }
}

fn ex_8_4() {
    assert_eq!(version_majeur("1.2.3"), "1");
    assert_eq!(version_majeur("10.0.1"), "10");
    assert_eq!(version_majeur("2024"), "2024");
}
