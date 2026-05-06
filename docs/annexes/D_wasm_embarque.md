# Annexe D — WebAssembly et systèmes embarqués

## WebAssembly avec `wasm-bindgen`

Rust compile vers WebAssembly de première classe — performances proches du natif dans le navigateur.

```toml
[package]
name = "mon-wasm"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console", "Window", "Document", "Element"] }
```

### Exporter des fonctions vers JavaScript

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn saluer(nom: &str) -> String {
    format!("Bonjour, {nom} !")
}

// Appeler console.log depuis Rust
#[wasm_bindgen]
pub fn log_depuis_rust(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
```

### Utilisation en JavaScript

```javascript
import init, { additionner, saluer } from './mon_wasm.js';

async function main() {
    await init();                        // charge le .wasm
    console.log(additionner(2, 3));     // 5
    console.log(saluer("Alice"));       // "Bonjour, Alice !"
}
```

### Build et packaging

```bash
cargo install wasm-pack
wasm-pack build --target web             # génère pkg/ avec .wasm + JS bindings
wasm-pack build --target bundler         # pour webpack/vite
wasm-pack build --target nodejs          # pour Node.js
```

### Interopérabilité complexe

```rust
use wasm_bindgen::prelude::*;
use js_sys::Array;

// Retourner un Array JS
#[wasm_bindgen]
pub fn créer_tableau(n: u32) -> Array {
    let arr = Array::new();
    for i in 0..n {
        arr.push(&JsValue::from(i));
    }
    arr
}

// Recevoir un callback JS
#[wasm_bindgen]
pub fn appliquer(f: &js_sys::Function, x: i32) -> JsValue {
    f.call1(&JsValue::NULL, &JsValue::from(x)).unwrap()
}
```

### `wasm-bindgen-futures` — async dans WASM

```rust
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen]
pub async fn fetch_texte(url: String) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap_or_default())
}
```

## Rust pour les systèmes embarqués

### `no_std` — sans bibliothèque standard

```rust
#![no_std]
#![no_main]

use panic_halt as _;   // handler de panic minimal

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
```

`#![no_std]` supprime la dépendance à la stdlib. `core` (types de base) et `alloc` (heap optionnel) restent disponibles.

### Cible STM32 avec `embassy` (Rust async embarqué)

```toml
[dependencies]
embassy-stm32 = { version = "0.1", features = ["stm32f103c8", "time-driver-any"] }
embassy-executor = { version = "0.5", features = ["arch-cortex-m"] }
embassy-time = "0.3"
cortex-m-rt = "0.7"
defmt = "0.3"
```

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use defmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        info!("LED on");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;

        info!("LED off");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
    }
}
```

### Outils embarqués

```bash
# Installer la toolchain ARM
rustup target add thumbv7m-none-eabi

# Compiler pour ARM Cortex-M3
cargo build --release --target thumbv7m-none-eabi

# Flasher et déboguer
cargo install probe-rs-tools
cargo flash --chip STM32F103C8 --release
cargo run    # avec runner = "probe-rs run" dans .cargo/config.toml
```

### Ressources de l'écosystème embedded

| Crate | Rôle |
|---|---|
| `embassy-*` | framework async pour embedded |
| `defmt` | logging ultra-léger pour microcontroleurs |
| `probe-rs` | flashing/debugging via JTAG/SWD |
| `embedded-hal` | traits d'abstraction matériel |
| `heapless` | Vec, HashMap, etc. sans allocation |
| `cortex-m` | support bas niveau Cortex-M |

### `heapless` — collections sans heap

```rust
use heapless::Vec;

let mut v: Vec<u8, 16> = Vec::new();  // capacité max 16, sur la stack
v.push(1).unwrap();
v.push(2).unwrap();
```

Toutes les opérations qui pourraient allouer retournent `Result` plutôt que d'allouer dynamiquement.
