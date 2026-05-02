# Annexe B — Regex, parsing et traitement de texte

## Regex avec la crate `regex`

```toml
[dependencies]
regex = "1"
```

```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

let texte = "Aujourd'hui c'est 2024-03-15 et demain 2024-03-16";

// Première correspondance
if let Some(caps) = re.captures(texte) {
    println!("Année: {}, Mois: {}, Jour: {}", &caps[1], &caps[2], &caps[3]);
}

// Toutes les correspondances
for caps in re.captures_iter(texte) {
    println!("Date: {}", &caps[0]);
}

// Remplacement
let résultat = re.replace_all(texte, "YYYY-MM-DD");
```

### Groupes nommés

```rust
let re = Regex::new(r"(?P<annee>\d{4})-(?P<mois>\d{2})-(?P<jour>\d{2})").unwrap();

if let Some(caps) = re.captures("2024-03-15") {
    println!("{} / {} / {}", &caps["annee"], &caps["mois"], &caps["jour"]);
}
```

### Compilation lazy (éviter de recompiler à chaque appel)

```rust
use std::sync::LazyLock;

static RE_EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap()
});

fn est_email(s: &str) -> bool {
    RE_EMAIL.is_match(s)
}
```

## Parsing avec `nom` — parser combinators

`nom` permet de construire des parseurs type-safe composables :

```toml
[dependencies]
nom = "8"
```

```rust
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, space0},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

// Parser une expression "42 + 58"
fn parser_nombre(input: &str) -> IResult<&str, i64> {
    let (reste, chiffres) = digit1(input)?;
    Ok((reste, chiffres.parse().unwrap()))
}

fn parser_expression(input: &str) -> IResult<&str, (i64, char, i64)> {
    tuple((
        parser_nombre,
        preceded(space0, nom::character::complete::one_of("+-*/")),
        preceded(space0, parser_nombre),
    ))(input)
}

let (_, (a, op, b)) = parser_expression("42 + 58").unwrap();
assert_eq!(a + b, 100);
```

### Parser un CSV simple avec nom

```rust
use nom::{
    bytes::complete::take_while,
    character::complete::char,
    multi::separated_list0,
    IResult,
};

fn parser_champ(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c != ',' && c != '\n')(input)
}

fn parser_ligne_csv(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(char(','), parser_champ)(input)
}

let (_, champs) = parser_ligne_csv("alice,30,paris").unwrap();
assert_eq!(champs, vec!["alice", "30", "paris"]);
```

## `serde` — sérialisation générique

### Personnaliser la sérialisation

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]        // snake_case → camelCase en JSON
struct Utilisateur {
    nom_complet: String,                   // → "nomComplet" en JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,                 // omis si None
    #[serde(default)]
    actif: bool,                           // false si absent en déserialisation
    #[serde(rename = "uid")]
    identifiant: u64,                      // "uid" en JSON
}
```

### Formats multiples

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
serde_yaml = "0.9"
```

```rust
// JSON ↔ TOML ↔ YAML : même struct, formats différents
let config: Config = toml::from_str(contenu_toml)?;
let json = serde_json::to_string_pretty(&config)?;
let yaml = serde_yaml::to_string(&config)?;
```

## Traitement de texte avancé

### `unicode-segmentation` — segmentation Unicode correcte

```toml
[dependencies]
unicode-segmentation = "1"
```

```rust
use unicode_segmentation::UnicodeSegmentation;

let s = "Héllo, wörld! 🦀";

// Grapheme clusters (ce que l'utilisateur voit comme "un caractère")
let graphèmes: Vec<&str> = s.graphemes(true).collect();
println!("{}", graphèmes.len());   // 15 (pas 17 bytes !)

// Mots
let mots: Vec<&str> = s.unicode_words().collect();
```

### Fuzzy matching avec `strsim`

```toml
[dependencies]
strsim = "0.11"
```

```rust
use strsim::{jaro_winkler, levenshtein};

let sim = jaro_winkler("Rust", "Ruzt");
println!("{sim:.3}");              // ~0.967

let dist = levenshtein("kitten", "sitting");
println!("{dist}");                // 3
```
