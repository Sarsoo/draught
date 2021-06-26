pub mod board;
pub mod utils;
pub mod game;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn init_game() {
    log!("initialising wasm");
    utils::set_panic_hook();

    #[cfg(feature = "random_init")]
    log!("random layout enabled");
}