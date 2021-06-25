//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

extern crate web_sys;
use web_sys::console;

extern crate draught;
use draught::{Board, BrdIdx, SquareState, STD_WIDTH, STD_HEIGHT};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen_test]
fn create() {
    let board = Board::new(STD_WIDTH, STD_HEIGHT);
    assert!(true);
}

#[wasm_bindgen_test]
fn std_num_cells() {
    let board = Board::new(8, 8);
    assert_eq!(64, board.num_cells());
}

//////////////
// INDEXING
//////////////

#[wasm_bindgen_test]
fn cell_index_top_left() {
    let board = Board::new(8, 8);
    assert_eq!(0, board.cell_index(0, 0));
}

#[wasm_bindgen_test]
fn cell_index_central() {
    let board = Board::new(8, 8);
    assert_eq!(9, board.cell_index(1, 1));
}

#[wasm_bindgen_test]
fn cell_index_central_2() {
    let board = Board::new(8, 8);
    assert_eq!(17, board.cell_index(2, 1));
}

#[wasm_bindgen_test]
fn board_index() {
    let board = Board::new(8, 8);

    // first row
    assert_eq!(BrdIdx::from(0, 5), board.board_index(5));
    // second row
    assert_eq!(BrdIdx::from(1, 6), board.board_index(14));
}

///////////////////
// SQUARE STATE
///////////////////

#[wasm_bindgen_test]
fn first_square_unplayable() {
    let board = Board::new(8, 8);
    assert_eq!(SquareState::Unplayable, board.cell_state(board.cell_index(0, 0)));
}

#[wasm_bindgen_test]
fn first_square_row_5_unplayable() {
    let board = Board::new(8, 8);
    assert_eq!(SquareState::Empty, board.cell_state(board.cell_index(5, 0)));
}

#[wasm_bindgen_test]
fn moveable_indices_unplayable() {
    let board = Board::new(8, 8);
    assert_eq!(None, board.diagonal_indices(BrdIdx::from(7, 7)));
    assert_eq!(None, board.diagonal_indices(BrdIdx::from(0, 0)));
    assert_eq!(None, board.diagonal_indices(BrdIdx::from(1, 1)));
}

#[wasm_bindgen_test]
fn moveable_indices_central() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![1, 3, 17, 19]), board.diagonal_indices(BrdIdx::from(1, 2)));
}

#[wasm_bindgen_test]
fn moveable_indices_top_row() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![8, 10]), board.diagonal_indices(BrdIdx::from(0, 1)));
}

#[wasm_bindgen_test]
fn moveable_indices_left_column() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![1, 17]), board.diagonal_indices(BrdIdx::from(1, 0)));
}

#[wasm_bindgen_test]
fn moveable_indices_bottom_row() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![49, 51]), board.diagonal_indices(BrdIdx::from(7, 2)));
}

#[wasm_bindgen_test]
fn moveable_indices_right_column() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![14, 30]), board.diagonal_indices(BrdIdx::from(2, 7)));
}

#[wasm_bindgen_test]
fn moveable_indices_top_right() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![14]), board.diagonal_indices(BrdIdx::from(0, 7)));
}

#[wasm_bindgen_test]
fn moveable_indices_bottom_left() {
    let board = Board::new(8, 8);
    assert_eq!(Some(vec![49]), board.diagonal_indices(BrdIdx::from(7, 0)));
}
