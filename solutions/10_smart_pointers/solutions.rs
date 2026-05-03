//! Solutions — Chapitre 10

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    ex_10_1();
    ex_10_2();
    ex_10_3();
    ex_10_4();
    ex_10_5();
    println!("Toutes les solutions passent ✅");
}

// 10.1
enum Liste {
    Cons(i32, Box<Liste>),
    Nil,
}

fn longueur(l: &Liste) -> usize {
    match l {
        Liste::Nil => 0,
        Liste::Cons(_, reste) => 1 + longueur(reste),
    }
}

fn ex_10_1() {
    let l = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Cons(3, Box::new(Liste::Nil))))));
    assert_eq!(longueur(&l), 3);
    assert_eq!(longueur(&Liste::Nil), 0);
}

// 10.2
fn ex_10_2() {
    let base = Rc::new("partagée".to_string());
    let clone1 = Rc::clone(&base);
    let clone2 = Rc::clone(&base);
    assert_eq!(Rc::strong_count(&base), 3);
    drop(clone1);
    drop(clone2);
    assert_eq!(Rc::strong_count(&base), 1);
}

// 10.3
fn ajouter(partagé: &RefCell<Vec<i32>>, valeur: i32) {
    partagé.borrow_mut().push(valeur);
}

fn somme(partagé: &RefCell<Vec<i32>>) -> i32 {
    partagé.borrow().iter().sum()
}

fn ex_10_3() {
    let v = RefCell::new(vec![]);
    ajouter(&v, 10);
    ajouter(&v, 20);
    ajouter(&v, 12);
    assert_eq!(somme(&v), 42);
    assert_eq!(v.borrow().len(), 3);
}

// 10.4
fn ex_10_4() {
    let valeur = Rc::new(RefCell::new(0));
    let a = Rc::clone(&valeur);
    let b = Rc::clone(&valeur);
    *a.borrow_mut() += 7;
    *b.borrow_mut() += 35;
    assert_eq!(*valeur.borrow(), 42);
}

// 10.5
fn ex_10_5() {
    let faible: Weak<i32>;
    {
        let fort = Rc::new(99);
        faible = Rc::downgrade(&fort);
        assert_eq!(*faible.upgrade().unwrap(), 99);
    }
    assert!(faible.upgrade().is_none());
}
