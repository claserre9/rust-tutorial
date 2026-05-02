//! Solutions — Chapitre 15

fn main() {
    ex_15_1();
    ex_15_2();
    ex_15_3();
    println!("Toutes les solutions passent ✅");
}

// 15.1
macro_rules! hashmap {
    () => { std::collections::HashMap::new() };
    ( $($k:expr => $v:expr),+ $(,)? ) => {
        {
            let mut m = std::collections::HashMap::new();
            $(m.insert($k, $v);)+
            m
        }
    };
}

fn ex_15_1() {
    use std::collections::HashMap;
    let m: HashMap<&str, i32> = hashmap!{ "a" => 1, "b" => 2, "c" => 3 };
    assert_eq!(m["a"], 1);
    assert_eq!(m["c"], 3);
    assert_eq!(m.len(), 3);
    let vide: HashMap<&str, i32> = hashmap!{};
    assert!(vide.is_empty());
}

// 15.2
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {
        {
            let a = $a;
            let b = $b;
            let eps = $eps;
            let diff = (a - b).abs();
            if diff > eps {
                panic!(
                    "assert_approx_eq échoué : |{} - {}| = {} > {}",
                    a, b, diff, eps
                );
            }
        }
    };
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-9)
    };
}

fn ex_15_2() {
    assert_approx_eq!(1.0_f64, 1.0000000001, 1e-9);
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 0.001);
}

// 15.3
struct Déconnecté;
struct Connecté { adresse: String }
struct Authentifié { adresse: String, utilisateur: String }

impl Déconnecté {
    fn connecter(self, adresse: &str) -> Connecté {
        Connecté { adresse: adresse.to_string() }
    }
}

impl Connecté {
    fn authentifier(self, utilisateur: &str) -> Authentifié {
        Authentifié { adresse: self.adresse, utilisateur: utilisateur.to_string() }
    }
    fn déconnecter(self) -> Déconnecté { Déconnecté }
}

impl Authentifié {
    fn envoyer(&self, données: &str) -> String {
        format!("{}@{} >> {}", self.utilisateur, self.adresse, données)
    }
    fn déconnecter(self) -> Déconnecté { Déconnecté }
}

fn ex_15_3() {
    let conn = Déconnecté
        .connecter("192.168.1.1")
        .authentifier("admin");
    let données = conn.envoyer("ping");
    assert_eq!(données, "admin@192.168.1.1 >> ping");
    let _ = conn.déconnecter();
}
