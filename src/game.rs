use crate::board::Board;
use indextree::Arena;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Root-level structure for managing the game as a collection of board states
#[wasm_bindgen]
pub struct Game {
    current: Board,
    previous_boards: Vec<Board>,
    tree: Arena<Board>
}