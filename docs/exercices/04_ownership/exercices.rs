//! Exercices — Chapitre 4 : Ownership & borrowing
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices

fn main() {
    ex_4_1();
    ex_4_2();
    ex_4_3();
    ex_4_4();
    ex_4_5();
    ex_4_6();
    ex_4_7();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 4.1 — Move vs Copy
// =============================================================================
// Pour chaque bloc, dites s'il compile. Décommentez les lignes une par une
// et observez l'erreur du compilateur, puis recommentez.
//
// (a) i32 est Copy
// (b) String n'est pas Copy
// Ensuite, implémentez `cloner_et_majuscules` sans modifier l'original.

fn cloner_et_majuscules(s: &String) -> String {
    // TODO : retourner une copie en majuscules sans consommer s
    String::new()
}

fn ex_4_1() {
    // (a) — doit compiler
    let x: i32 = 42;
    let _y = x;
    assert_eq!(x, 42); // x est encore valide

    // (b) — décommenter pour voir l'erreur, puis recommenter
    // let s = String::from("hello");
    // let _s2 = s;
    // println!("{s}"); // ❌ moved

    // (c) — cloner_et_majuscules
    let original = String::from("bonjour");
    let majuscules = cloner_et_majuscules(&original);
    assert_eq!(majuscules, "BONJOUR");
    assert_eq!(original, "bonjour"); // original intact
}


// =============================================================================
// 4.2 — Borrowing immuable
// =============================================================================
// Écrivez `compter_voyelles` qui prend un &str et retourne le nombre de voyelles.
// Elle doit fonctionner avec un &str ET avec un &String (coercion).

fn compter_voyelles(s: &str) -> usize {
    // TODO : itérer sur s.chars(), compter les voyelles a,e,i,o,u,y (minuscule/majuscule)
    0
}

fn ex_4_2() {
    let litteral = "Bonjour le monde";
    assert_eq!(compter_voyelles(litteral), 6); // o, u, e, o, e

    let owned = String::from("Hello World");
    assert_eq!(compter_voyelles(&owned), 3); // e, o, o

    // owned est toujours utilisable après l'emprunt
    assert_eq!(owned.len(), 11);
}


// =============================================================================
// 4.3 — Borrowing mutable
// =============================================================================
// Écrivez `normaliser` qui prend un &mut String et:
//   1. Trim les espaces en début/fin
//   2. Met en minuscules
// Modifiez la valeur en place (pas de retour).

fn normaliser(s: &mut String) {
    // TODO : modifier s en place
    // Astuce : les méthodes trim() retournent &str, pas String.
    // Vous aurez besoin de reconstruire le String.
    // Pattern : *s = s.trim().to_lowercase();
}

fn ex_4_3() {
    let mut s = String::from("  Bonjour MONDE  ");
    normaliser(&mut s);
    assert_eq!(s, "bonjour monde");

    let mut s2 = String::from("RUST");
    normaliser(&mut s2);
    assert_eq!(s2, "rust");
}


// =============================================================================
// 4.4 — Règles du borrow checker
// =============================================================================
// Corrigez le code suivant pour qu'il compile sans changer la logique.
// Il y a UN problème de borrow dans chaque sous-exercice.

fn ex_4_4() {
    // (a) Conflit lecteur/éditeur — swap l'ordre pour que r1 soit mort avant r2
    let mut v = vec![1, 2, 3];

    // PROBLÈME : on lit r1 APRÈS avoir créé r2 (éditeur)
    // Réécrivez pour utiliser r1 avant de créer r2.
    let r1 = &v;
    let _len = r1.len(); // TODO : assurez-vous r1 n'est plus utilisé après ici
    let r2 = &mut v;
    r2.push(4);
    assert_eq!(v, vec![1, 2, 3, 4]);

    // (b) Retourner une slice valide d'un vecteur local
    // La fonction ci-dessous ne compile pas. Corrigez-la.
    // fn premier(v: Vec<i32>) -> &i32 { &v[0] }  // ❌ dangling
    // Solution : prenez ownership ou passez une référence
    fn premier(v: &[i32]) -> Option<&i32> {
        // TODO : retourner le premier élément sans dangling
        None
    }
    assert_eq!(premier(&[10, 20, 30]), Some(&10));
    assert_eq!(premier(&[]), None);
}


// =============================================================================
// 4.5 — String slices
// =============================================================================
// Implémentez `dernier_mot` qui retourne une slice vers le dernier mot
// d'une phrase (séparé par espaces). Si la phrase est vide, retourner "".
// N'allouez aucun String.

fn dernier_mot(s: &str) -> &str {
    // TODO : trouver le dernier espace et retourner &s[pos+1..]
    // Astuce : s.rfind(' ')  →  Option<usize>
    ""
}

fn ex_4_5() {
    assert_eq!(dernier_mot("hello world"), "world");
    assert_eq!(dernier_mot("un seul"), "seul");
    assert_eq!(dernier_mot("mot"), "mot");
    assert_eq!(dernier_mot(""), "");
    assert_eq!(dernier_mot("  espaces  "), ""); // trailing space → dernier "mot" est vide
}


// =============================================================================
// 4.6 — Slice de tableau
// =============================================================================
// Écrivez `maximum` qui prend un &[i32] et retourne Some(&i32) sur le max,
// ou None si le slice est vide. Pas d'allocation, pas d'index.

fn maximum(nums: &[i32]) -> Option<&i32> {
    // TODO : itérer pour trouver le max et retourner une référence
    None
}

fn ex_4_6() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    assert_eq!(maximum(&v), Some(&9));
    assert_eq!(maximum(&v[0..3]), Some(&4));
    assert_eq!(maximum(&[]), None);
    assert_eq!(maximum(&[-1, -5, -2]), Some(&-1));
}


// =============================================================================
// 4.7 — Struct et ownership
// =============================================================================
// Définissez un struct `Cache` avec un champ `data: Vec<String>`.
// Implémentez :
//   - `nouveau()` → Cache vide
//   - `ajouter(&mut self, s: &str)` → ajoute une copie de s
//   - `contient(&self, s: &str) -> bool` → cherche s dans data
//   - `taille(&self) -> usize` → nombre d'entrées

struct Cache {
    // TODO
}

impl Cache {
    fn nouveau() -> Cache {
        // TODO
        Cache {}
    }

    fn ajouter(&mut self, s: &str) {
        // TODO
    }

    fn contient(&self, s: &str) -> bool {
        // TODO
        false
    }

    fn taille(&self) -> usize {
        // TODO
        0
    }
}

fn ex_4_7() {
    let mut c = Cache::nouveau();
    assert_eq!(c.taille(), 0);
    assert!(!c.contient("hello"));

    c.ajouter("hello");
    c.ajouter("world");
    assert_eq!(c.taille(), 2);
    assert!(c.contient("hello"));
    assert!(c.contient("world"));
    assert!(!c.contient("rust"));
}
