//! Solutions — Chapitre 9

fn main() {
    ex_9_1();
    ex_9_2();
    ex_9_3();
    ex_9_4();
    println!("Toutes les solutions passent ✅");
}

// 9.1
trait Resumable {
    fn resumer(&self) -> String;
    fn longueur(&self) -> usize { self.resumer().len() }
}

struct Article { titre: String, contenu: String }
struct Tweet { auteur: String, texte: String }

impl Resumable for Article {
    fn resumer(&self) -> String {
        let extrait: String = self.contenu.chars().take(50).collect();
        format!("{}: {}", self.titre, extrait)
    }
}

impl Resumable for Tweet {
    fn resumer(&self) -> String {
        format!("@{}: {}", self.auteur, self.texte)
    }
}

fn ex_9_1() {
    let a = Article {
        titre: String::from("Rust 2024"),
        contenu: String::from("La nouvelle édition apporte beaucoup de nouveautés intéressantes pour les développeurs."),
    };
    let t = Tweet {
        auteur: String::from("rustlang"),
        texte: String::from("Rust 1.80 released!"),
    };
    assert!(a.resumer().starts_with("Rust 2024: "));
    assert_eq!(t.resumer(), "@rustlang: Rust 1.80 released!");
    assert_eq!(t.longueur(), t.resumer().len());
}

// 9.2
fn resumer_tous(items: &[Box<dyn Resumable>]) -> Vec<String> {
    items.iter().map(|i| i.resumer()).collect()
}

fn plus_long_resume(items: &[Box<dyn Resumable>]) -> Option<String> {
    items.iter().map(|i| i.resumer()).max_by_key(|s| s.len())
}

fn ex_9_2() {
    let items: Vec<Box<dyn Resumable>> = vec![
        Box::new(Article {
            titre: String::from("Titre"),
            contenu: String::from("Contenu court."),
        }),
        Box::new(Tweet {
            auteur: String::from("user"),
            texte: String::from("Un tweet assez long pour être le plus long."),
        }),
    ];
    let résumés = resumer_tous(&items);
    assert_eq!(résumés.len(), 2);
    let plus_long = plus_long_resume(&items).unwrap();
    assert!(plus_long.contains("user"));
}

// 9.3
struct Plage { courant: i32, fin: i32, pas: i32 }

impl Plage {
    fn new(debut: i32, fin: i32, pas: i32) -> Plage { Plage { courant: debut, fin, pas } }
}

impl Iterator for Plage {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.courant >= self.fin { return None; }
        let val = self.courant;
        self.courant += self.pas;
        Some(val)
    }
}

fn ex_9_3() {
    let p: Vec<i32> = Plage::new(0, 10, 2).collect();
    assert_eq!(p, vec![0, 2, 4, 6, 8]);
    let somme: i32 = Plage::new(1, 6, 1).sum();
    assert_eq!(somme, 15);
    assert_eq!(Plage::new(5, 5, 1).count(), 0);
}

// 9.4
struct Chronometre {
    nom: String,
    debut: std::time::Instant,
}

impl Chronometre {
    fn nouveau(nom: &str) -> Chronometre {
        println!("⏱ Démarrage de '{nom}'");
        Chronometre { nom: nom.to_string(), debut: std::time::Instant::now() }
    }

    fn lire(&self) -> std::time::Duration {
        self.debut.elapsed()
    }
}

impl Drop for Chronometre {
    fn drop(&mut self) {
        let d = self.debut.elapsed();
        println!("⏱ '{}' : {:.3}ms", self.nom, d.as_secs_f64() * 1000.0);
    }
}

fn ex_9_4() {
    {
        let _c = Chronometre::nouveau("ex_9_4");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let d = _c.lire();
        assert!(d.as_millis() >= 10);
    }
}
