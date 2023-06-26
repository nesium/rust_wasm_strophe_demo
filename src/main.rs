#![feature(extern_types)]
#![recursion_limit = "1024"]

mod client;
mod connector;
mod data_cache;
mod delegate;
mod utils;

use console_error_panic_hook::set_once as set_panic_hook;
pub use utils::log;

fn main() {
    set_panic_hook();
}
