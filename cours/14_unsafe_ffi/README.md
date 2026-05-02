# Chapitre 14 — Unsafe et FFI

`unsafe` est la soupape de sécurité de Rust : un bloc délimité où vous promettez au compilateur que vous avez vérifié manuellement les invariants qu'il ne peut pas vérifier seul. FFI (Foreign Function Interface) permet d'appeler du C depuis Rust et vice versa.

## 1. Qu'est-ce que `unsafe` permet ?

Cinq opérations supplémentaires, uniquement dans un bloc `unsafe {}` :

1. **Déréférencer un pointeur brut** (`*const T` ou `*mut T`)
2. **Appeler une fonction `unsafe`** (y compris les fonctions C via FFI)
3. **Accéder ou modifier une variable `static mut`**
4. **Implémenter un trait `unsafe`** (comme `Send`, `Sync`)
5. **Accéder aux champs d'une `union`**

Tout le reste de Rust reste vérifié, même dans un bloc `unsafe`.

## 2. Pointeurs bruts

```rust
let mut val = 5;
let r1 = &val as *const i32;        // pointeur brut immuable
let r2 = &mut val as *mut i32;      // pointeur brut mutable

// Créer des pointeurs bruts est safe
// Déréférencer est unsafe
unsafe {
    println!("{}", *r1);
    *r2 = 10;
    println!("{}", *r2);
}
```

Les pointeurs bruts peuvent être nuls, peuvent pointer vers de la mémoire libérée, peuvent être aliasés — Rust ne vérifie rien. À vous de garantir la validité.

```rust
// Pointeur nul — légal à créer, dangereux à déréférencer
let p: *const i32 = std::ptr::null();
// unsafe { *p }   // comportement indéfini !
```

## 3. Fonctions `unsafe`

```rust
unsafe fn dangereux() {
    // corps implicitement unsafe
}

unsafe {
    dangereux();
}
```

Une fonction `unsafe` signifie : "l'appelant doit respecter des invariants que je ne peux pas vérifier". Documentez ces invariants avec `# Safety` :

```rust
/// # Safety
/// `ptr` doit pointer vers un `i32` valide et aligné.
/// La mémoire pointée doit rester valide pendant la durée de l'appel.
unsafe fn lire_i32(ptr: *const i32) -> i32 {
    *ptr
}
```

## 4. Abstractions safe sur unsafe

Le pattern idiomatique est d'encapsuler `unsafe` dans une API publique `safe` :

```rust
pub fn split_at(slice: &[i32], index: usize) -> (&[i32], &[i32]) {
    assert!(index <= slice.len());
    // SAFETY : index <= len garanti par l'assertion ci-dessus
    unsafe {
        let ptr = slice.as_ptr();
        (
            std::slice::from_raw_parts(ptr, index),
            std::slice::from_raw_parts(ptr.add(index), slice.len() - index),
        )
    }
}
```

La fonction publique est `safe` car elle vérifie les préconditions avant d'entrer dans `unsafe`.

## 5. FFI — appeler du C depuis Rust

```rust
extern "C" {
    fn abs(x: i32) -> i32;
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

fn main() {
    let n = unsafe { abs(-42) };
    println!("{n}");   // 42
}
```

### Linker le code C

Dans `build.rs` (script de build cargo) :

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/helper.c")
        .compile("helper");
}
```

```toml
[build-dependencies]
cc = "1"
```

### Types C-compatibles

```rust
use std::os::raw::{c_int, c_char, c_double, c_void};
// Ou depuis libc :
// [dependencies] libc = "0.2"
use libc::{c_int, c_char};
```

### `#[repr(C)]` pour les structs

Pour passer des structs à du code C :

```rust
#[repr(C)]
struct Point { x: f64, y: f64 }

extern "C" {
    fn calculer_distance(a: *const Point, b: *const Point) -> f64;
}
```

Sans `#[repr(C)]`, Rust peut réordonner les champs — le code C verrait des données corrompues.

## 6. Exporter du Rust vers C

```rust
// lib.rs
#[no_mangle]                          // désactive le name mangling
pub extern "C" fn additionner(a: i32, b: i32) -> i32 {
    a + b
}
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]   # génère une .so / .dll
```

Le header C correspondant :

```c
// header.h
int32_t additionner(int32_t a, int32_t b);
```

## 7. `cbindgen` — génération automatique de headers

```toml
[build-dependencies]
cbindgen = "0.26"
```

```rust
// build.rs
fn main() {
    cbindgen::Builder::new()
        .with_crate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .generate()
        .expect("cbindgen")
        .write_to_file("include/binding.h");
}
```

## 8. `bindgen` — générer des bindings Rust depuis des headers C

```toml
[build-dependencies]
bindgen = "0.70"
```

```rust
// build.rs
fn main() {
    let bindings = bindgen::Builder::default()
        .header("libcurl/curl.h")
        .generate()
        .expect("bindgen");
    bindings.write_to_file("src/bindings.rs").expect("write");
}
```

Génère automatiquement les `extern "C"`, les structs `#[repr(C)]`, les constantes.

---

### Règle d'or : minimiser unsafe

`unsafe` doit être :
- **Le plus petit possible** — le bloc, pas la fonction entière sauf si nécessaire.
- **Bien commenté** — expliquez pourquoi c'est safe (`// SAFETY: ...`).
- **Testé exhaustivement** — Miri (l'interpréteur Rust) détecte les UB dans les tests.

```bash
cargo +nightly miri test    # détecte les UB dans les tests
```

---

## À retenir

- `unsafe` débloque 5 opérations supplémentaires, rien de plus.
- Encapsulez `unsafe` dans des API publiques `safe` avec préconditions vérifiées.
- `#[repr(C)]` pour les structs partagées avec C. `extern "C"` pour les déclarations.
- `bindgen` automatise la génération des bindings depuis les headers C.
- Miri détecte les undefined behaviors dans les tests.

---

➡️ [Chapitre 15 — Macros procédurales et patterns avancés](../15_macros_patterns/README.md)
