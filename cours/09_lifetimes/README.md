# Chapitre 8 — Lifetimes : durées de vie explicites

Les lifetimes sont la réponse de Rust à la question : "combien de temps cette référence reste-t-elle valide ?" Le compilateur les infère la plupart du temps — vous n'avez besoin de les annoter qu'au niveau des signatures quand l'inférence ne suffit pas.

## 1. Le problème des lifetimes

```rust
fn plus_long(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

Ce code ne compile pas. Le compilateur ne peut pas savoir si le retour vit aussi longtemps que `s1` ou `s2`. Il faut l'annoter :

```rust
fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

`'a` est un **paramètre de lifetime**. La signature dit : "le retour vit au moins aussi longtemps que `'a`" — et `'a` est la durée de vie **commune** à `s1` et `s2` (leur intersection).

```rust
let s1 = String::from("long string");
let resultat;
{
    let s2 = String::from("xy");
    resultat = plus_long(&s1, &s2);
    println!("{resultat}");             // ✅ s2 vit ici
}
// println!("{resultat}");              // ❌ s2 est morte
```

## 2. Lifetime élision — quand annoter n'est pas nécessaire

Le compilateur applique des **règles d'élision** pour inférer les lifetimes dans la plupart des cas. Vous n'annotez que quand il n'y arrive pas.

### Règles d'élision

1. Chaque paramètre de référence reçoit sa propre lifetime (`fn f(x: &T)` → `fn f<'a>(x: &'a T)`)
2. S'il y a exactement un paramètre référence, sa lifetime est assignée au retour.
3. S'il y a `&self` ou `&mut self`, la lifetime de `self` est assignée au retour.

```rust
fn premier_mot(s: &str) -> &str {           // élision OK (règle 2)
    &s[..s.find(' ').unwrap_or(s.len())]
}

// équivalent à :
fn premier_mot<'a>(s: &'a str) -> &'a str { ... }
```

```rust
struct Parseur<'a> {
    contenu: &'a str,                        // référence dans un struct → lifetime obligatoire
}

impl<'a> Parseur<'a> {
    fn token(&self) -> &str {                // règle 3 : lifetime de &self
        &self.contenu[..4]
    }
}
```

## 3. Lifetimes dans les structs

Un struct ne peut contenir de référence que si sa lifetime est annotée. Le struct ne peut pas **survivre** à la donnée référencée.

```rust
struct ExtraitTexte<'a> {
    partie: &'a str,
}

impl<'a> ExtraitTexte<'a> {
    fn annonce_et_retourne(&self, annonce: &str) -> &str {
        println!("{annonce}");
        self.partie                           // lifetime de self, règle 3
    }
}

let roman = String::from("Appel de la forêt. Il était une fois...");
let extrait;
{
    let premier_phrase = roman.split('.').next().expect(".");
    let e = ExtraitTexte { partie: premier_phrase };
    extrait = e.annonce_et_retourne("Attention !");
    println!("{extrait}");
}
```

## 4. `'static` — la lifetime la plus longue

`'static` signifie que la référence vit pour toute la durée du programme :

```rust
let s: &'static str = "je suis dans le binaire";  // littéral → 'static
```

`'static` apparaît souvent dans les bounds de traits pour les threads :

```rust
fn lancer<F: Fn() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}
```

`'static` ici signifie : "la closure ne capture pas de références éphémères". Pas forcément que la closure vit pour toujours.

## 5. Bornes de lifetime sur les types génériques

```rust
fn afficher_plus_long<'a, T>(s1: &'a str, s2: &'a str, extra: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("extra: {extra}");
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

## 6. Lifetime variance — covariance vs contravariance

Concept avancé, utile pour comprendre pourquoi certains code compilent :

- `&'long T` peut être utilisé là où `&'short T` est attendu (covariance sur `'a` dans `&'a T`)
- `&mut 'a T` est **invariant** sur `'a` — on ne peut pas substituer une durée de vie plus courte ou plus longue

En pratique : une référence vers une longue durée de vie est toujours acceptable là où une courte est attendue. Le contraire est faux.

```rust
fn prend_court<'a>(s: &'a str) { println!("{s}"); }

let long = String::from("longue vie");
let s: &'static str = "statique";

prend_court(s);           // &'static str accepté où &'a str attendu — ✅ (covariance)
prend_court(&long);       // &'x str avec 'x < 'static — ✅
```

## 7. `Cow<'a, B>` — Clone On Write

Un type utilitaire qui évite des copies inutiles :

```rust
use std::borrow::Cow;

fn normaliser<'a>(s: &'a str) -> Cow<'a, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))     // alloue seulement si nécessaire
    } else {
        Cow::Borrowed(s)                    // zéro allocation si pas de modification
    }
}

let a = normaliser("hello_world");          // Borrowed — pas d'allocation
let b = normaliser("hello world");          // Owned — alloue une fois
```

---

### Piège courant : lifetime trop restrictive

```rust
// Trop restrictif : lie artificiellement les lifetimes de x et du retour
fn mystere<'a>(x: &'a i32, _y: &'a i32) -> &'a i32 { x }

// Correct : le retour vit aussi longtemps que x (pas besoin de 'a pour y)
fn mystere2<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 { x }
```

Trop de lifetimes partagées peuvent rejeter du code valide en forçant des durées de vie plus courtes.

---

### Comment lire un message d'erreur de lifetime

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:33
  |
2 | fn plus_long(s1: &str, s2: &str) -> &str {
  |                  ----      ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
    signature does not say whether it is borrowed from `s1` or `s2`
```

Le compilateur dit : "je ne sais pas si le retour vit aussi longtemps que s1 ou s2 — aide-moi". C'est la situation décrite en §1.

---

## À retenir

- Une lifetime `'a` dit : "cette référence est valide au moins pendant `'a`".
- Le compilateur infère la plupart des lifetimes (élision). Annotez seulement quand nécessaire.
- `'static` : vit toute la durée du programme (littéraux de strings, ou closures sans captures éphémères).
- Structs avec références → lifetime obligatoire sur le struct.
- Trop de lifetimes partagées = code trop restrictif. Utilisez des lifetimes distinctes si les paramètres sont indépendants.

---

➡️ [Chapitre 10 — Smart pointers](../10_smart_pointers/README.md)
