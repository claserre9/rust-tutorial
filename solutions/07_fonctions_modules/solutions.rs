//! Solutions — Chapitre 6 (exercices 6.1–6.3)
//! Les exercices 6.4/6.5 sont dans le dossier todo-cli/

fn main() {
    ex_6_1();
    ex_6_2();
    ex_6_3();
    println!("Toutes les solutions passent ✅");
}

// 6.1
fn composer<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| g(f(x))
}

fn appliquer_n_fois<F: Fn(i32) -> i32>(f: F, n: u32, debut: i32) -> i32 {
    let mut resultat = debut;
    for _ in 0..n {
        resultat = f(resultat);
    }
    resultat
}

fn ex_6_1() {
    let doubler = |x: i32| x * 2;
    let ajouter_un = |x: i32| x + 1;
    let doubler_puis_ajouter = composer(doubler, ajouter_un);
    assert_eq!(doubler_puis_ajouter(5), 11);
    assert_eq!(doubler_puis_ajouter(0), 1);
    let ajouter_puis_doubler = composer(ajouter_un, doubler);
    assert_eq!(ajouter_puis_doubler(5), 12);
    assert_eq!(appliquer_n_fois(|x| x + 3, 4, 0), 12);
    assert_eq!(appliquer_n_fois(|x| x * 2, 3, 1), 8);
    assert_eq!(appliquer_n_fois(|x| x + 1, 0, 42), 42);
}

// 6.2
mod geometrie {
    pub struct Cercle {
        rayon: f64,
    }

    impl Cercle {
        pub fn nouveau(rayon: f64) -> Cercle {
            Cercle { rayon }
        }

        pub fn aire(&self) -> f64 {
            std::f64::consts::PI * self.rayon * self.rayon
        }

        pub fn perimetre(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.rayon
        }
    }
}

fn ex_6_2() {
    let c = geometrie::Cercle::nouveau(5.0);
    assert!((c.aire() - 78.53981633974483).abs() < 1e-9);
    assert!((c.perimetre() - 31.41592653589793).abs() < 1e-9);
}

// 6.3
#[derive(Debug, PartialEq)]
enum CalcError {
    FormatInvalide,
    NombreInvalide(String),
    DivisionParZero,
    OperateurInconnu(char),
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::FormatInvalide => write!(f, "format invalide (attendu: 'num op num')"),
            CalcError::NombreInvalide(s) => write!(f, "nombre invalide : '{s}'"),
            CalcError::DivisionParZero => write!(f, "division par zéro"),
            CalcError::OperateurInconnu(c) => write!(f, "opérateur inconnu : '{c}'"),
        }
    }
}

fn calculer(expr: &str) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(CalcError::FormatInvalide);
    }
    let a = tokens[0].parse::<f64>()
        .map_err(|_| CalcError::NombreInvalide(tokens[0].to_string()))?;
    let b = tokens[2].parse::<f64>()
        .map_err(|_| CalcError::NombreInvalide(tokens[2].to_string()))?;
    let op = tokens[1].chars().next().unwrap_or(' ');
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 { Err(CalcError::DivisionParZero) } else { Ok(a / b) }
        }
        c => Err(CalcError::OperateurInconnu(c)),
    }
}

fn ex_6_3() {
    assert_eq!(calculer("10 + 3"), Ok(13.0));
    assert_eq!(calculer("10 - 3"), Ok(7.0));
    assert_eq!(calculer("10 * 3"), Ok(30.0));
    assert_eq!(calculer("10 / 2"), Ok(5.0));
    assert_eq!(calculer("10 / 0"), Err(CalcError::DivisionParZero));
    assert_eq!(calculer("abc + 1"), Err(CalcError::NombreInvalide(String::from("abc"))));
    assert_eq!(calculer("10 ^ 2"), Err(CalcError::OperateurInconnu('^')));
    assert_eq!(calculer("juste_un_token"), Err(CalcError::FormatInvalide));
}
