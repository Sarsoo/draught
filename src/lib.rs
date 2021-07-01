//! Draught
//! 
//! An implementation of checkers/draughts in Rust WebAssembly with a minimax AI player

pub mod board;
pub mod utils;
pub mod game;
pub mod player;
pub mod comp;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub use board::Board;
pub use game::Game;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Wrap the [`web_sys`] access to the browser console in a macro for easy logging
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