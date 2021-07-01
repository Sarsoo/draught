use super::*;
use wasm_bindgen_test::*;
use crate::log;

wasm_bindgen_test_configure!(run_in_browser);

use crate::board::{Piece};
use crate::board::enums::Strength::*;


#[wasm_bindgen_test]
fn make_move() {
    let mut game = Game::new(8, 8, 3, Black);
    // log!("{}", game);
    // log!("{:?}", game);

    let from = BrdIdx::from(5, 2);
    let to = BrdIdx::from(4, 1);

    game.make_move(from, to);
    let board = game.current_board();

    assert_eq!(board.cell(board.cell_index(4, 1)), Square::pc(Black, Man));

    // log!("{}", game);

    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(3, 2);

    game.make_move(from, to);
    let board = game.current_board();
    
    assert_eq!(board.cell(board.cell_index(3, 2)), Square::pc(White, Man));

    // log!("{}", game);
    // log!("{}", game.previous_board(0));
}

#[wasm_bindgen_test]
fn make_jump() {
    let mut game = Game::new(8, 8, 3, Black);
    // log!("{}", game);
    // log!("{:?}", game);

    let square = Square::pc(White, Man);
    game.current.set_cell(
        game.current.cell_idx(BrdIdx::from(4, 1)), 
        square
    );

    let from = BrdIdx::from(5, 2);
    let to = BrdIdx::from(3, 0);

    game.make_move(from, to);

    // log!("{}", game);
    // log!("{}", game.previous_board(0));
}

