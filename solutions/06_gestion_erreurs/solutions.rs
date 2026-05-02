//! Solutions — Chapitre 6

fn main() {
    ex_6_1();
    ex_6_2();
    ex_6_3();
    ex_6_4();
    ex_6_5();
    println!("Toutes les solutions passent ✅");
}

// 6.1
fn doubler_si_positif(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&n| n > 0).map(|n| n * 2)
}

fn description(opt: Option<&str>) -> String {
    opt.map(|s| format!("valeur: {s}")).unwrap_or_else(|| "absent".to_string())
}

fn première_lettre_majuscule(opt: Option<&str>) -> Option<char> {
    opt.and_then(|s| s.chars().next())
       .and_then(|c| c.to_uppercase().next())
}

fn ex_6_1() {
    assert_eq!(doubler_si_positif(Some(5)), Some(10));
    assert_eq!(doubler_si_positif(Some(-3)), None);
    assert_eq!(doubler_si_positif(None), None);
    assert_eq!(description(Some("bonjour")), "valeur: bonjour");
    assert_eq!(description(None), "absent");
    assert_eq!(première_lettre_majuscule(Some("rust")), Some('R'));
    assert_eq!(première_lettre_majuscule(Some("")), None);
    assert_eq!(première_lettre_majuscule(None), None);
}

// 6.2
fn parser_et_valider(s: &str) -> Result<u32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("parsing échoué : {e}"))
        .and_then(|n| {
            if n >= 0 { Ok(n as u32) }
            else { Err(format!("{n} est négatif")) }
        })
}

fn doubler_resultat(r: Result<i32, String>) -> Result<i32, String> {
    r.map(|n| n * 2)
}

fn ex_6_2() {
    assert_eq!(parser_et_valider("42"), Ok(42));
    assert_eq!(parser_et_valider("0"), Ok(0));
    assert!(parser_et_valider("-1").is_err());
    assert!(parser_et_valider("abc").is_err());
    assert_eq!(doubler_resultat(Ok(21)), Ok(42));
    assert!(doubler_resultat(Err("oups".to_string())).is_err());
}

// 6.3
fn lire_deux_entiers(a: &str, b: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok((x, y))
}

fn somme_fichier_simulé(lignes: &[&str]) -> Result<i32, String> {
    let mut somme = 0;
    for ligne in lignes {
        let n = ligne.trim().parse::<i32>()
            .map_err(|e| format!("ligne invalide '{}': {}", ligne, e))?;
        somme += n;
    }
    Ok(somme)
}

fn ex_6_3() {
    assert_eq!(lire_deux_entiers("10", "32"), Ok((10, 32)));
    assert!(lire_deux_entiers("10", "abc").is_err());
    assert_eq!(somme_fichier_simulé(&["1", "2", "3"]), Ok(6));
    assert!(somme_fichier_simulé(&["1", "oups", "3"]).is_err());
}

// 6.4
#[derive(Debug, PartialEq)]
enum AppError {
    Parse(std::num::ParseIntError),
    Validation(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "erreur de parsing : {e}"),
            AppError::Validation(s) => write!(f, "validation échouée : {s}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> AppError { AppError::Parse(e) }
}

fn parse_age(s: &str) -> Result<u8, AppError> {
    let n: i32 = s.parse()?;  // ParseIntError → AppError via From
    if n < 0 || n > 150 {
        return Err(AppError::Validation(format!("{n} hors de la plage [0, 150]")));
    }
    Ok(n as u8)
}

fn ex_6_4() {
    assert_eq!(parse_age("25"), Ok(25));
    assert_eq!(parse_age("0"), Ok(0));
    assert_eq!(parse_age("150"), Ok(150));
    assert!(matches!(parse_age("abc"), Err(AppError::Parse(_))));
    assert!(matches!(parse_age("-1"), Err(AppError::Validation(_))));
    assert!(matches!(parse_age("151"), Err(AppError::Validation(_))));
}

// 6.5
fn trouver_et_doubler(v: &[i32], cible: i32) -> Result<i32, String> {
    v.iter()
        .find(|&&x| x == cible)
        .copied()
        .ok_or_else(|| format!("{cible} introuvable dans le vecteur"))
        .map(|n| n * 2)
}

fn ex_6_5() {
    let v = vec![1, 5, 10, 42];
    assert_eq!(trouver_et_doubler(&v, 5), Ok(10));
    assert_eq!(trouver_et_doubler(&v, 42), Ok(84));
    assert!(trouver_et_doubler(&v, 99).is_err());
}
