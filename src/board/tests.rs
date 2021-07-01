use super::*;
use wasm_bindgen_test::*;
use crate::log;

wasm_bindgen_test_configure!(run_in_browser);


// #[wasm_bindgen_test]
// fn init_game() {
//     let board = Board::init_game(Board::new(8, 8, Black), 3);
//     log!("{}", board);
// }

#[wasm_bindgen_test]
fn create() {
    let board = Board::new(STD_WIDTH, STD_HEIGHT, Black);
    assert!(true);
}

#[wasm_bindgen_test]
fn std_num_cells() {
    let board = Board::new(8, 8, Black);
    assert_eq!(64, board.num_cells());
}

#[wasm_bindgen_test]
fn idx_diffs() {
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(2, 2);
    assert_eq!(Board::idx_diffs(from, to), (1, 1));

    let from = BrdIdx::from(2, 2);
    let to = BrdIdx::from(1, 1);
    assert_eq!(Board::idx_diffs(from, to), (-1, -1));

    let from = BrdIdx::from(5, 0);
    let to = BrdIdx::from(0, 10);
    assert_eq!(Board::idx_diffs(from, to), (-5, 10));
}

#[wasm_bindgen_test]
fn set_cell() {
    let idx = 1;

    let mut board = Board::new(8, 8, Black);
    let square = Square::pc(White, Man);

    board.set_cell(idx, square);
    assert_eq!(square, board.cell(idx));
}

//////////////
// INDEXING
//////////////

#[wasm_bindgen_test]
fn cell_index_top_left() {
    let board = Board::new(8, 8, Black);
    assert_eq!(0, board.cell_index(0, 0));
}

#[wasm_bindgen_test]
fn cell_index_central() {
    let board = Board::new(8, 8, Black);
    assert_eq!(9, board.cell_index(1, 1));
}

#[wasm_bindgen_test]
fn cell_index_central_2() {
    let board = Board::new(8, 8, Black);
    assert_eq!(17, board.cell_index(2, 1));
}

#[wasm_bindgen_test]
fn board_index() {
    let board = Board::new(8, 8, Black);

    // first row
    assert_eq!(BrdIdx::from(0, 5), board.board_index(5));
    // second row
    assert_eq!(BrdIdx::from(1, 6), board.board_index(14));
    // third row
    assert_eq!(BrdIdx::from(2, 4), board.board_index(20));
}

///////////////////
// SQUARE STATE
///////////////////

#[wasm_bindgen_test]
fn first_square_unplayable() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Unplayable, board.cell_state(board.cell_index(0, 0)));
}

#[wasm_bindgen_test]
fn first_square_row_5_unplayable() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Empty, board.cell_state(board.cell_index(5, 0)));
}

//////////////////////
// DIAGNOAL INDICES
//////////////////////

#[wasm_bindgen_test]
fn moveable_indices_unplayable() {
    let board = Board::new(8, 8, Black);
    assert_eq!(None, board.adjacent_indices(BrdIdx::from(7, 7)));
    assert_eq!(None, board.adjacent_indices(BrdIdx::from(0, 0)));
    assert_eq!(None, board.adjacent_indices(BrdIdx::from(1, 1)));
}

#[wasm_bindgen_test]
fn moveable_indices_central() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![1, 3, 17, 19]), board.adjacent_indices(BrdIdx::from(1, 2)));
}

#[wasm_bindgen_test]
fn moveable_indices_top_row() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![8, 10]), board.adjacent_indices(BrdIdx::from(0, 1)));
}

#[wasm_bindgen_test]
fn moveable_indices_left_column() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![1, 17]), board.adjacent_indices(BrdIdx::from(1, 0)));
}

#[wasm_bindgen_test]
fn moveable_indices_bottom_row() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![49, 51]), board.adjacent_indices(BrdIdx::from(7, 2)));
}

#[wasm_bindgen_test]
fn moveable_indices_right_column() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![14, 30]), board.adjacent_indices(BrdIdx::from(2, 7)));
}

#[wasm_bindgen_test]
fn moveable_indices_top_right() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![14]), board.adjacent_indices(BrdIdx::from(0, 7)));
}

#[wasm_bindgen_test]
fn moveable_indices_bottom_left() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![49]), board.adjacent_indices(BrdIdx::from(7, 0)));
}

//////////////////////
// JUMPABLE INDICES
//////////////////////

#[wasm_bindgen_test]
fn jumpable_indices_unplayable() {
    let board = Board::new(8, 8, Black);
    assert_eq!(None, board.jumpable_indices(BrdIdx::from(7, 7)));
    assert_eq!(None, board.jumpable_indices(BrdIdx::from(0, 0)));
    assert_eq!(None, board.jumpable_indices(BrdIdx::from(1, 1)));
}

#[wasm_bindgen_test]
fn jumpable_indices() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![24, 28]), board.jumpable_indices(BrdIdx::from(1, 2)));
}

#[wasm_bindgen_test]
fn jumpable_indices_central() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![10, 14, 42, 46]), board.jumpable_indices(BrdIdx::from(3, 4)));
}

#[wasm_bindgen_test]
fn jumpable_indices_top_row() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![19]), board.jumpable_indices(BrdIdx::from(0, 1)));
}

#[wasm_bindgen_test]
fn jumpable_indices_left_column() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![26]), board.jumpable_indices(BrdIdx::from(1, 0)));
}

#[wasm_bindgen_test]
fn jumpable_indices_bottom_row() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![40, 44]), board.jumpable_indices(BrdIdx::from(7, 2)));
}

#[wasm_bindgen_test]
fn jumpable_indices_right_column() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![5, 37]), board.jumpable_indices(BrdIdx::from(2, 7)));
}

#[wasm_bindgen_test]
fn jumpable_indices_top_right() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![21]), board.jumpable_indices(BrdIdx::from(0, 7)));
}

#[wasm_bindgen_test]
fn jumpable_indices_bottom_left() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![42]), board.jumpable_indices(BrdIdx::from(7, 0)));
}

#[wasm_bindgen_test]
fn black_adjacent_indices() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![1, 3]), board.player_adjacent_indices(BrdIdx::from(1, 2), Black));
}

#[wasm_bindgen_test]
fn white_adjacent_indices() {
    let board = Board::new(8, 8, Black);
    assert_eq!(Some(vec![17, 19]), board.player_adjacent_indices(BrdIdx::from(1, 2), White));
}

////////////////
//   JUMPEE
////////////////
 
#[wasm_bindgen_test]
fn check_jumpee_opposing_teams() {
    let from = Piece::new(Black, Man); 
    let jumpee = Piece::new(White, Man); 
    assert_eq!(Board::check_jumpee_team(from, jumpee), true);

    let from = Piece::new(White, Man); 
    let jumpee = Piece::new(Black, Man); 
    assert_eq!(Board::check_jumpee_team(from, jumpee), true);
}

#[wasm_bindgen_test]
fn check_jumpee_same_teams() {
    let from = Piece::new(Black, Man); 
    let jumpee = Piece::new(Black, Man); 
    assert_eq!(Board::check_jumpee_team(from, jumpee), false);

    let from = Piece::new(White, Man); 
    let jumpee = Piece::new(White, Man); 
    assert_eq!(Board::check_jumpee_team(from, jumpee), false);
}

#[wasm_bindgen_test]
fn check_validate_jumpee_opposing_teams() {
    let jumpee_square = Square::pc(White, Man);
    let from_piece = Piece::new(Black, Man);

    assert_eq!(Board::validate_jumpee(jumpee_square, from_piece), Moveable::Allowed);

    let jumpee_square = Square::pc(Black, Man);
    let from_piece = Piece::new(White, Man);

    assert_eq!(Board::validate_jumpee(jumpee_square, from_piece), Moveable::Allowed);
}

#[wasm_bindgen_test]
fn check_validate_jumpee_same_teams() {
    let jumpee_square = Square::pc(White, Man);
    let from_piece = Piece::new(White, Man);

    assert_eq!(Board::validate_jumpee(jumpee_square, from_piece), Moveable::JumpingSameTeam);

    let jumpee_square = Square::pc(Black, Man);
    let from_piece = Piece::new(Black, Man);

    assert_eq!(Board::validate_jumpee(jumpee_square, from_piece), Moveable::JumpingSameTeam);
}

/////////////////
//    SCORE
/////////////////

#[wasm_bindgen_test]
fn score() {
    //////////////////////////////////
    let board = Board::new(8, 8, Black);    
    assert_eq!(0, board.score());
    //////////////////////////////////

    //////////////////////////////////
    let mut board = Board::new(8, 8, Black);
    let square = Square::pc(Black, Man);
    board.set_cell(1, square);
    assert_eq!(1, board.score());
    //////////////////////////////////

    //////////////////////////////////
    let square = Square::pc(White, Man);
    board.set_cell(5, square);
    let square = Square::pc(Black, Man);
    board.set_cell(7, square);
    let square = Square::pc(Black, Man);
    board.set_cell(8, square);
    assert_eq!(2, board.score());
    //////////////////////////////////
}

///////////////////////
//  MOVE VALIDATION
///////////////////////

#[wasm_bindgen_test]
fn validate_man_move_team_directions() {
    // WHITE NEEDS INCREASING IDX

    // allowed, white moves down board
    let board = Board::new(8, 8, White);
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(2, 2);
    let piece = Piece::new(White, Man);

    assert_eq!(Moveable::Allowed, board.validate_man_move(from, to, piece));

    // unallowed, white moves up board
    let board = Board::new(8, 8, White);
    let from = BrdIdx::from(2, 2);
    let to = BrdIdx::from(1, 1);
    let piece = Piece::new(White, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));

    // allowed, black moves up board
    let board = Board::new(8, 8, Black);
    let from = BrdIdx::from(2, 2);
    let to = BrdIdx::from(1, 1);
    let piece = Piece::new(Black, Man);

    assert_eq!(Moveable::Allowed, board.validate_man_move(from, to, piece));

    // unallowed, black moves down board
    let board = Board::new(8, 8, Black);
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(2, 2);
    let piece = Piece::new(Black, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));
}

#[wasm_bindgen_test]
fn validate_man_move_weird_trajectories() {
    // WHITE NEEDS INCREASING IDX

    // allowed, white moves down board
    let board = Board::new(8, 8, White);
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(3, 2);
    let piece = Piece::new(White, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));

    // unallowed, white moves up board
    let board = Board::new(8, 8, White);
    let from = BrdIdx::from(2, 3);
    let to = BrdIdx::from(1, 1);
    let piece = Piece::new(White, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));

    // allowed, black moves up board
    let board = Board::new(8, 8, Black);
    let from = BrdIdx::from(5, 2);
    let to = BrdIdx::from(1, 1);
    let piece = Piece::new(Black, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));

    // unallowed, black moves down board
    let board = Board::new(8, 8, Black);
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(2, 4);
    let piece = Piece::new(Black, Man);

    assert_eq!(Moveable::IllegalTrajectory, board.validate_man_move(from, to, piece));
}

#[wasm_bindgen_test]
fn can_move() {
    // WHITE NEEDS INCREASING IDX

    // allowed, white moves down board
    let board = Board::new(8, 8, White);
    let mut board = Board::init_game(board, 3);

    // log!("{}", board);

    // white can move down
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(3, 0);
    assert_eq!(board.can_move(from, to), Moveable::Allowed);

    // going straight down
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(3, 1);
    assert_eq!(board.can_move(from, to), Moveable::IllegalTrajectory);

    // going directly right
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(2, 2);
    assert_eq!(board.can_move(from, to), Moveable::IllegalTrajectory);

    // jumping an empty square
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::NoJumpablePiece);

    // empty cell
    let from = BrdIdx::from(3, 0);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::UnoccupiedSrc);

    // out of board
    let from = BrdIdx::from(50, 50);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::OutOfBounds);
    let from = BrdIdx::from(5, 5);
    let to = BrdIdx::from(50, 50);
    assert_eq!(board.can_move(from, to), Moveable::OutOfBounds);

    // unplayable
    let from = BrdIdx::from(0, 0);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::Unplayable);
    let from = BrdIdx::from(1, 1);
    let to = BrdIdx::from(0, 0);
    assert_eq!(board.can_move(from, to), Moveable::Unplayable);

    board.current_turn = Black;

    // wrong teams piece
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::WrongTeamSrc);

}

#[wasm_bindgen_test]
fn can_move_jump() {
    // WHITE NEEDS INCREASING IDX

    // allowed, white moves down board
    let board = Board::new(8, 8, White);
    let mut board = Board::init_game(board, 3);

    board.set_cell(board.cell_index(3, 2), Square::pc(Black, Man));

    // log!("{}", board);
    // log!("{:?}", board.cell(board.cell_index(3, 2)));

    // white can move down
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::Allowed);

    board.set_cell(board.cell_index(3, 2), Square::pc(White, Man));

    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(4, 3);
    assert_eq!(board.can_move(from, to), Moveable::JumpingSameTeam);

    // moving in to full cell
    let from = BrdIdx::from(2, 1);
    let to = BrdIdx::from(3, 2);
    assert_eq!(board.can_move(from, to), Moveable::OccupiedDest);

}