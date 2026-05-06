//! Exercices — Chapitre 10 : Smart pointers
//!
//! Compilez et exécutez :
//!   rustc --edition 2024 exercices.rs && ./exercices

fn main() {
    ex_10_1();
    ex_10_2();
    ex_10_3();
    ex_10_4();
    ex_10_5();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 10.1 — Box<T> et types récursifs
// =============================================================================
// Définissez un enum `Liste` représentant une liste chaînée d'entiers :
//   - Cons(i32, Box<Liste>)  →  un élément suivi d'une liste
//   - Nil                   →  fin de liste
//
// Implémentez `longueur(l: &Liste) -> usize` qui retourne le nombre d'éléments.

// TODO : déclarez l'enum Liste ici

fn longueur(_l: &Liste) -> usize {
    // TODO : match récursif
    0
}

fn ex_10_1() {
    let l = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Cons(3, Box::new(Liste::Nil))))));
    assert_eq!(longueur(&l), 3);
    assert_eq!(longueur(&Liste::Nil), 0);
}


// =============================================================================
// 10.2 — Rc<T> : ownership partagé
// =============================================================================
// Créez une valeur `Rc<String>` contenant "partagée".
// Faites-en deux clones et vérifiez que Rc::strong_count == 3.
// Après avoir droppé les deux clones, vérifiez que le compteur revient à 1.

use std::rc::Rc;

fn ex_10_2() {
    // TODO
    let base = Rc::new("partagée".to_string());
    // créez deux clones, vérifiez strong_count == 3
    // droppez-les, vérifiez strong_count == 1
    let _ = base;  // à remplacer
}


// =============================================================================
// 10.3 — RefCell<T> : mutabilité intérieure
// =============================================================================
// Implémentez `ajouter(partagé: &RefCell<Vec<i32>>, valeur: i32)`
// qui pousse `valeur` dans le vecteur via borrow_mut().
//
// Puis implémentez `somme(partagé: &RefCell<Vec<i32>>) -> i32`
// qui retourne la somme des éléments via borrow().

use std::cell::RefCell;

fn ajouter(_partagé: &RefCell<Vec<i32>>, _valeur: i32) {
    // TODO
}

fn somme(_partagé: &RefCell<Vec<i32>>) -> i32 {
    // TODO
    0
}

fn ex_10_3() {
    let v = RefCell::new(vec![]);
    ajouter(&v, 10);
    ajouter(&v, 20);
    ajouter(&v, 12);
    assert_eq!(somme(&v), 42);
    assert_eq!(v.borrow().len(), 3);
}


// =============================================================================
// 10.4 — Rc<RefCell<T>> : partagé et mutable
// =============================================================================
// Créez un `Rc<RefCell<i32>>` initialisé à 0.
// Faites-en deux clones a et b.
// Incrémentez via `a` de 7, via `b` de 35.
// Vérifiez que la valeur originale est 42.

fn ex_10_4() {
    // TODO
    let valeur = Rc::new(RefCell::new(0));
    // faites deux clones, mutatez via chacun, vérifiez 42
    let _ = valeur;  // à remplacer
}


// =============================================================================
// 10.5 — Weak<T> et cycles
// =============================================================================
// Vérifiez que Weak::upgrade() retourne Some avant le drop du Rc,
// et None après.

use std::rc::Weak;

fn ex_10_5() {
    let faible: Weak<i32>;
    {
        let fort = Rc::new(99);
        faible = Rc::downgrade(&fort);
        // TODO : assert que faible.upgrade() == Some(Rc::new(99))
        // Astuce : comparez *faible.upgrade().unwrap() avec 99
    }
    // TODO : assert que faible.upgrade().is_none()
}
