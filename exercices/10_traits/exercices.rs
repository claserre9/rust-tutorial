//! Exercices — Chapitre 9 : Traits

fn main() {
    ex_9_1();
    ex_9_2();
    ex_9_3();
    ex_9_4();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 9.1 — Définir et implémenter un trait
// =============================================================================
// Définissez le trait `Resumable` avec :
//   - `resumer(&self) -> String` — résumé court (obligatoire)
//   - `longueur(&self) -> usize` — par défaut : resumer().len()
//
// Implémentez-le pour :
//   - `Article { titre: String, contenu: String }` → "titre: contenu_tronqué_à_50"
//   - `Tweet { auteur: String, texte: String }` → "@auteur: texte"

// TODO

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
    assert_eq!(t.longueur(), t.resumer().len()); // méthode par défaut
}


// =============================================================================
// 9.2 — Dispatch dynamique
// =============================================================================
// Créez une fonction `resumer_tous` qui prend un `&[Box<dyn Resumable>]`
// et retourne un Vec<String> avec les résumés.
//
// Créez aussi `plus_long_resume` qui retourne le résumé le plus long.

fn resumer_tous(items: &[Box<dyn Resumable>]) -> Vec<String> {
    // TODO
    Vec::new()
}

fn plus_long_resume(items: &[Box<dyn Resumable>]) -> Option<String> {
    // TODO : utiliser Iterator sur les résumés
    None
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
    assert!(plus_long.contains("tweet") || plus_long.contains("Tweet") || plus_long.contains("user"));
}


// =============================================================================
// 9.3 — Implémenter Iterator
// =============================================================================
// Implémentez un itérateur `Plage { debut: i32, fin: i32, pas: i32 }`
// qui génère debut, debut+pas, debut+2*pas, ... tant que la valeur < fin.

struct Plage {
    courant: i32,
    fin: i32,
    pas: i32,
}

impl Plage {
    fn new(debut: i32, fin: i32, pas: i32) -> Plage {
        Plage { courant: debut, fin, pas }
    }
}

impl Iterator for Plage {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        // TODO
        None
    }
}

fn ex_9_3() {
    let p: Vec<i32> = Plage::new(0, 10, 2).collect();
    assert_eq!(p, vec![0, 2, 4, 6, 8]);

    let somme: i32 = Plage::new(1, 6, 1).sum();
    assert_eq!(somme, 15);

    // Itérateur vide si debut >= fin
    assert_eq!(Plage::new(5, 5, 1).count(), 0);
}


// =============================================================================
// 9.4 — Drop et RAII
// =============================================================================
// Implémentez un struct `Chronometre` qui, à sa création, enregistre
// l'instant de départ, et à son drop, imprime la durée écoulée.
// Utilisez `std::time::Instant`.
//
// Bonus : ajoutez `Chronometre::lire(&self) -> std::time::Duration`

struct Chronometre {
    // TODO
}

impl Chronometre {
    fn nouveau(nom: &str) -> Chronometre {
        println!("⏱ Démarrage de '{nom}'");
        // TODO
        Chronometre {}
    }

    fn lire(&self) -> std::time::Duration {
        // TODO
        std::time::Duration::ZERO
    }
}

impl Drop for Chronometre {
    fn drop(&mut self) {
        // TODO : afficher "⏱ 'nom' : X.XXXms"
    }
}

fn ex_9_4() {
    {
        let _c = Chronometre::nouveau("ex_9_4");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let d = _c.lire();
        assert!(d.as_millis() >= 10);
    }  // drop ici → affiche la durée
}
