//! Solutions — Chapitre 2

fn main() {
    ex_2_1();
    ex_2_2();
    ex_2_3();
    ex_2_4();
    ex_2_5();
    ex_2_6();
    ex_2_7();
    ex_2_8();
    println!("Toutes les solutions passent ✅");
}

// 2.1
fn ex_2_1() {
    let mut x = 5;
    x = 10;
    assert_eq!(x, 10);
}

// 2.2 — shadowing
fn ex_2_2() {
    let input = "Bonjour";
    let input: String = input.to_uppercase();
    let input: String = input.to_lowercase();
    assert_eq!(input, "bonjour");
}

// 2.3 — overflow géré
fn ex_2_3() {
    let a: u8 = 250;
    let b: u8 = 10;
    let result = a.saturating_add(b);
    assert_eq!(result, 255);
}

// 2.4 — &str en paramètre
fn saluer(nom: &str) -> String {
    format!("Bonjour, {nom} !")
}

fn ex_2_4() {
    let s = String::from("Alice");
    assert_eq!(saluer("Bob"), "Bonjour, Bob !");
    assert_eq!(saluer(&s), "Bonjour, Alice !");
}

// 2.5
fn ex_2_5() {
    let point = (3, 4);
    let (x, y) = point;
    let somme_carres = x * x + y * y;
    assert_eq!(somme_carres, 25);
}

// 2.6 — if expression
fn abs_value(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

fn ex_2_6() {
    assert_eq!(abs_value(-5), 5);
    assert_eq!(abs_value(3), 3);
    assert_eq!(abs_value(0), 0);
}

// 2.7
fn ex_2_7() {
    let res: Result<u8, _> = u8::try_from(300i32);
    assert!(res.is_err());

    let ok: Result<u8, _> = u8::try_from(42i32);
    assert_eq!(ok.unwrap(), 42);
}

// 2.8
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn ex_2_8() {
    assert_eq!(div(10, 2), Some(5));
    assert_eq!(div(10, 0), None);
    assert_eq!(div(10, 0).unwrap_or(-1), -1);
}
