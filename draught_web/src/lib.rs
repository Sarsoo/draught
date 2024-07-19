//! Draught
//! 
//! An implementation of checkers/draughts in Rust WebAssembly with a minimax AI player

pub mod utils;
pub mod paint;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub use draughtlib::{Board, Game, Computer};
pub use paint::Painter;

/// Wrap the [`web_sys`] access to the browser console in a macro for easy logging
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}
#[macro_export]
macro_rules! log_error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
pub fn init_wasm() {
    log!("Initialising WebAssembly");
    utils::set_panic_hook();
}