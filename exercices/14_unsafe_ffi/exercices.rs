//! Exercices — Chapitre 14 : Unsafe et FFI

fn main() {
    ex_14_1();
    ex_14_2();
    println!("Tous les tests passent ✅");
}

// =============================================================================
// 14.1 — Pointeurs bruts
// =============================================================================
// Implémentez `swap_brut` qui échange deux valeurs via des pointeurs bruts.
// La signature externe est safe — l'unsafe est encapsulé.

fn swap_brut<T>(a: &mut T, b: &mut T) {
    // TODO : obtenir *mut T pour a et b, utiliser std::ptr::swap dans unsafe
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


// =============================================================================
// 14.2 — Abstraction safe sur slice brute
// =============================================================================
// Implémentez `split_au_milieu` qui divise un &mut [T] en deux moitiés
// mutables simultanées. C'est impossible avec des références Rust normales
// (deux &mut sur le même slice) mais possible avec unsafe.
//
// # Safety de votre implémentation :
//   - les deux moitiés ne se chevauchent pas
//   - les pointeurs sont valides et alignés (garantis par slice d'entrée)

fn split_au_milieu<T>(slice: &mut [T]) -> (&mut [T], &mut [T]) {
    // TODO : unsafe avec std::slice::from_raw_parts_mut
    let milieu = slice.len() / 2;
    slice.split_at_mut(milieu)  // implémentation temporaire — réécrivez avec unsafe
}

fn ex_14_2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (gauche, droite) = split_au_milieu(&mut v);

    assert_eq!(gauche, &[1, 2, 3]);
    assert_eq!(droite, &[4, 5, 6]);

    // Modifier les deux moitiés simultanément
    gauche.iter_mut().for_each(|x| *x *= 10);
    droite.iter_mut().for_each(|x| *x *= 100);

    assert_eq!(v, vec![10, 20, 30, 400, 500, 600]);
}


// =============================================================================
// 14.3 — Appeler une fonction C (projet séparé)
// =============================================================================
// Créez un projet cargo avec un fichier src/helper.c et build.rs.
// La fonction C `int32_t somme_carre(int32_t n)` calcule 1² + 2² + ... + n².
// Appelez-la depuis Rust et vérifiez le résultat.
//
// helper.c :
//   #include <stdint.h>
//   int32_t somme_carre(int32_t n) {
//       int32_t s = 0;
//       for (int32_t i = 1; i <= n; i++) s += i * i;
//       return s;
//   }
//
// build.rs :
//   fn main() {
//       cc::Build::new().file("src/helper.c").compile("helper");
//   }
//
// Cargo.toml [build-dependencies]: cc = "1"
// Rust:
//   extern "C" { fn somme_carre(n: i32) -> i32; }
//   assert_eq!(unsafe { somme_carre(5) }, 55);  // 1+4+9+16+25

fn main_doc() {
    println!("Ex 14.3 : projet cargo séparé avec cc");
}
