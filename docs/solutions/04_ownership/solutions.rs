//! Solutions — Chapitre 4

fn main() {
    ex_4_1();
    ex_4_2();
    ex_4_3();
    ex_4_4();
    ex_4_5();
    ex_4_6();
    ex_4_7();
    println!("Toutes les solutions passent ✅");
}

// 4.1
fn cloner_et_majuscules(s: &String) -> String {
    s.to_uppercase()
}

fn ex_4_1() {
    let x: i32 = 42;
    let _y = x;
    assert_eq!(x, 42);

    let original = String::from("bonjour");
    let majuscules = cloner_et_majuscules(&original);
    assert_eq!(majuscules, "BONJOUR");
    assert_eq!(original, "bonjour");
}

// 4.2
fn compter_voyelles(s: &str) -> usize {
    s.chars().filter(|c| "aeiouyAEIOUY".contains(*c)).count()
}

fn ex_4_2() {
    assert_eq!(compter_voyelles("Bonjour le monde"), 6);
    let owned = String::from("Hello World");
    assert_eq!(compter_voyelles(&owned), 3);
    assert_eq!(owned.len(), 11);
}

// 4.3
fn normaliser(s: &mut String) {
    *s = s.trim().to_lowercase();
}

fn ex_4_3() {
    let mut s = String::from("  Bonjour MONDE  ");
    normaliser(&mut s);
    assert_eq!(s, "bonjour monde");

    let mut s2 = String::from("RUST");
    normaliser(&mut s2);
    assert_eq!(s2, "rust");
}

// 4.4
fn ex_4_4() {
    let mut v = vec![1, 2, 3];
    let r1 = &v;
    let _len = r1.len();
    // r1 n'est plus utilisé après ici — NLL permet de créer r2
    let r2 = &mut v;
    r2.push(4);
    assert_eq!(v, vec![1, 2, 3, 4]);

    fn premier(v: &[i32]) -> Option<&i32> {
        v.first()
    }
    assert_eq!(premier(&[10, 20, 30]), Some(&10));
    assert_eq!(premier(&[]), None);
}

// 4.5
fn dernier_mot(s: &str) -> &str {
    match s.rfind(' ') {
        Some(pos) => &s[pos + 1..],
        None => s,
    }
}

fn ex_4_5() {
    assert_eq!(dernier_mot("hello world"), "world");
    assert_eq!(dernier_mot("un seul"), "seul");
    assert_eq!(dernier_mot("mot"), "mot");
    assert_eq!(dernier_mot(""), "");
    assert_eq!(dernier_mot("  espaces  "), "");
}

// 4.6
fn maximum(nums: &[i32]) -> Option<&i32> {
    nums.iter().reduce(|a, b| if b > a { b } else { a })
}

fn ex_4_6() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    assert_eq!(maximum(&v), Some(&9));
    assert_eq!(maximum(&v[0..3]), Some(&4));
    assert_eq!(maximum(&[]), None);
    assert_eq!(maximum(&[-1, -5, -2]), Some(&-1));
}

// 4.7
struct Cache {
    data: Vec<String>,
}

impl Cache {
    fn nouveau() -> Cache {
        Cache { data: Vec::new() }
    }

    fn ajouter(&mut self, s: &str) {
        self.data.push(s.to_string());
    }

    fn contient(&self, s: &str) -> bool {
        self.data.iter().any(|entry| entry == s)
    }

    fn taille(&self) -> usize {
        self.data.len()
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
