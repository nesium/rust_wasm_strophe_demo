use wasm_bindgen::prelude::*;

// https://github.com/rustwasm/wasm-bindgen/blob/main/examples/console_log/src/lib.rs

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::log(&format!("[rust] {}", format_args!($($t)*).to_string())))
}
