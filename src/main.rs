#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// The JavaScript XMPPClient which wraps StropheJS
#[wasm_bindgen(module = "/web/xmpp_client.js")]
extern "C" {
    type JSXMPPClient;

    #[wasm_bindgen(constructor)]
    fn new() -> JSXMPPClient;

    #[wasm_bindgen(method)]
    fn connect(this: &JSXMPPClient, jid: String, password: String);
}

// The Rust XMPPClient which interacts with the server through JSXMPPClient
#[wasm_bindgen]
pub struct RustXMPPClient {
    client: JSXMPPClient
}

#[wasm_bindgen]
impl RustXMPPClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RustXMPPClient {
            client: JSXMPPClient::new()
        }
    }

    pub fn connect(&self, jid: String, password: String) {
        self.client.connect(jid, password)
    }
}

fn main() {
    set_panic_hook();
}