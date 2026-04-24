//! Exercices — Chapitre 6 : Fonctions, modules et erreurs
//!
//! Pour les exercices 6.4 et 6.5, créez un projet cargo :
//!   cargo new todo_exercice && cd todo_exercice
//!   # Ajoutez les dépendances dans Cargo.toml
//!   # Puis testez avec : cargo run -- lister

fn main() {
    ex_6_1();
    ex_6_2();
    ex_6_3();
    println!("Tests 6.1–6.3 passent ✅");
    println!("6.4 et 6.5 nécessitent un projet cargo séparé.");
}

// =============================================================================
// 6.1 — Closures et fonctions d'ordre supérieur
// =============================================================================
// (a) Implémentez `composer` qui prend deux fonctions f et g et retourne
//     une closure h telle que h(x) = g(f(x)).
//
// (b) Implémentez `appliquer_n_fois` qui applique f n fois à une valeur initiale.

fn composer<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    // TODO : retourner une closure qui applique f puis g
    move |_x| 0
}

fn appliquer_n_fois<F: Fn(i32) -> i32>(f: F, n: u32, debut: i32) -> i32 {
    // TODO : appliquer f n fois à debut
    debut
}

fn ex_6_1() {
    let doubler = |x: i32| x * 2;
    let ajouter_un = |x: i32| x + 1;

    let doubler_puis_ajouter = composer(doubler, ajouter_un);
    assert_eq!(doubler_puis_ajouter(5), 11); // 5*2+1 = 11
    assert_eq!(doubler_puis_ajouter(0), 1);

    let ajouter_puis_doubler = composer(ajouter_un, doubler);
    assert_eq!(ajouter_puis_doubler(5), 12); // (5+1)*2 = 12

    assert_eq!(appliquer_n_fois(|x| x + 3, 4, 0), 12); // 0+3+3+3+3 = 12
    assert_eq!(appliquer_n_fois(|x| x * 2, 3, 1), 8);  // 1*2*2*2 = 8
    assert_eq!(appliquer_n_fois(|x| x + 1, 0, 42), 42); // 0 fois → identité
}


// =============================================================================
// 6.2 — Modules et visibilité
// =============================================================================
// Le module `geometrie` ci-dessous a des erreurs de visibilité.
// Corrigez-le pour que les assertions passent.
// Règle : rendez public SEULEMENT ce qui est utilisé depuis l'extérieur.

mod geometrie {
    struct Cercle {              // TODO : corriger la visibilité
        rayon: f64,
    }

    impl Cercle {
        fn nouveau(rayon: f64) -> Cercle {   // TODO
            Cercle { rayon }
        }

        fn aire(&self) -> f64 {               // TODO
            std::f64::consts::PI * self.rayon * self.rayon
        }

        fn perimetre(&self) -> f64 {          // TODO
            2.0 * std::f64::consts::PI * self.rayon
        }
    }
}

fn ex_6_2() {
    let c = geometrie::Cercle::nouveau(5.0);
    assert!((c.aire() - 78.53981633974483).abs() < 1e-9);
    assert!((c.perimetre() - 31.41592653589793).abs() < 1e-9);
}


// =============================================================================
// 6.3 — Type d'erreur personnalisé
// =============================================================================
// Sans thiserror (pour la compréhension), implémentez manuellement un
// type d'erreur pour une mini-calculatrice qui parse "num op num"
// (ex: "10 / 2", "3 + 4", "5 * 0").
// Opérations supportées : +, -, *, /
// Erreurs possibles : FormatInvalide, NombreInvalide, DivisionParZero, OperateurInconnu

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
    // TODO : parser "a op b" et calculer le résultat
    // Étapes :
    //   1. split_whitespace() → doit donner exactement 3 tokens
    //   2. Parser le premier et troisième token en f64
    //      (utilisez parse::<f64>().map_err(|_| CalcError::NombreInvalide(...)))
    //   3. Le deuxième token est l'opérateur (premier char)
    //   4. Calculer selon l'opérateur
    Err(CalcError::FormatInvalide)
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


// =============================================================================
// 6.4 — Projet : CLI todo (projet cargo séparé)
// =============================================================================
// Créez un projet cargo `todo-cli` avec la structure suivante :
//
//   todo-cli/
//   ├── Cargo.toml        (clap 4 + serde + serde_json + thiserror)
//   └── src/
//       ├── main.rs
//       ├── cli.rs
//       ├── models.rs
//       └── storage.rs
//
// Fonctionnalités :
//   cargo run -- ajouter "Ma tâche"    → ajoute et sauvegarde
//   cargo run -- lister                → affiche [○] ou [✓]
//   cargo run -- terminer <id>         → marque comme fait
//   cargo run -- supprimer <id>        → supprime
//
// Utilisez le code du cours comme point de départ.
// Ajout bonus : commande `vider` qui supprime les tâches terminées.


// =============================================================================
// 6.5 — Bonus : tests unitaires dans le projet todo
// =============================================================================
// Dans storage.rs, ajoutez un module #[cfg(test)] avec :
//   - test_sauvegarder_et_charger : crée des todos, sauvegarde dans un fichier
//     temporaire, recharge et vérifie l'égalité
//   - Utilisez std::env::temp_dir() pour le chemin
//
// Dans models.rs :
//   - test_nouveau : vérifie que Todo::nouveau(1, "test") a fait=false
