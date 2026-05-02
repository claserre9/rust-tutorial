//! Solutions — Chapitre 14

fn main() {
    ex_14_1();
    ex_14_2();
    println!("Toutes les solutions passent ✅");
}

// 14.1
fn swap_brut<T>(a: &mut T, b: &mut T) {
    // SAFETY : a et b sont des références valides non-overlapping (garantie par Rust).
    // std::ptr::swap gère le cas Copy et non-Copy via bitwise swap.
    unsafe { std::ptr::swap(a as *mut T, b as *mut T) }
}

fn ex_14_1() {
    let mut x = 10i32;
    let mut y = 42i32;
    swap_brut(&mut x, &mut y);
    assert_eq!(x, 42);
    assert_eq!(y, 10);

    let mut s1 = String::from("bonjour");
    let mut s2 = String::from("monde");
    swap_brut(&mut s1, &mut s2);
    assert_eq!(s1, "monde");
    assert_eq!(s2, "bonjour");
}

// 14.2
fn split_au_milieu<T>(slice: &mut [T]) -> (&mut [T], &mut [T]) {
    let milieu = slice.len() / 2;
    let ptr = slice.as_mut_ptr();
    let len = slice.len();
    // SAFETY :
    //   - ptr est valide et aligné (provient d'un slice valide)
    //   - les deux slices résultantes ne se chevauchent pas (milieu <= len)
    //   - lifetime lié à l'entrée (pas de dangling)
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, milieu),
            std::slice::from_raw_parts_mut(ptr.add(milieu), len - milieu),
        )
    }
}

fn ex_14_2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (gauche, droite) = split_au_milieu(&mut v);
    assert_eq!(gauche, &[1, 2, 3]);
    assert_eq!(droite, &[4, 5, 6]);
    gauche.iter_mut().for_each(|x| *x *= 10);
    droite.iter_mut().for_each(|x| *x *= 100);
    assert_eq!(v, vec![10, 20, 30, 400, 500, 600]);
}
