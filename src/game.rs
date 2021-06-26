use crate::board::Board;
use indextree::Arena;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    current: Board,
    previous_boards: Vec<Board>,
    tree: Arena<Board>
}