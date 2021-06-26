
pub mod enums;
use enums::*;

pub mod iter;
use iter::*;

use std::fmt::{Display, Write};
use std::option::Option;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub const STD_WIDTH: usize = 8;
pub const STD_HEIGHT: usize = 8;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    team: Team,
    strength: Strength
}

impl Piece {
    pub fn new(team: Team, strength: Strength) -> Piece {
        Piece {
            team, strength
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction<T: Clone + Copy> {
    nw: Option<T>,
    ne: Option<T>,
    se: Option<T>,
    sw: Option<T>,
}

impl<T: Clone + Copy> Direction<T> {
    pub fn empty() -> Direction<T> {
        Direction {
            nw: Option::None,
            ne: Option::None,
            se: Option::None,
            sw: Option::None,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    occupant: Option<Piece>,
    state: SquareState
}

impl Square {
    pub fn new(state: SquareState, occupant: Option<Piece>) -> Square{
        Square {
            occupant,
            state
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrdIdx {
    row: usize,
    col: usize
}

#[wasm_bindgen]
impl BrdIdx {
    pub fn from(row: usize, col: usize) -> BrdIdx {
        BrdIdx{
            row, col
        }
    }
}

///////////////
//   BOARD
///////////////

/// Models a single state for a checkers board
#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    cells: Vec<Square>,
    width: usize,
    height: usize,

    current_turn: Team
}

impl Board {
    pub fn cell_mut(&mut self, idx: usize) -> &mut Square {
        &mut self.cells[idx]
    }

    pub fn diagonal_indices(&self, idx: BrdIdx) -> Option<Vec<usize>> {
        if self.cell_state(self.cell_idx(idx)) == SquareState::Unplayable {
            return None;
        }

        let height_idx = self.height - 1;
        let width_idx = self.width - 1;

        let mut cells = Vec::with_capacity(4);
        let mut dir = Direction::empty();

        if idx.row > 0 {
            if idx.col > 0 {
                cells.push(
                    self.cell_index(idx.row - 1, idx.col - 1)
                );
                dir.nw = Option::Some(self.cell(self.cell_index(idx.row - 1, idx.col - 1)));
            }

            if idx.col < width_idx {
                cells.push(
                    self.cell_index(idx.row - 1, idx.col + 1)
                );
                dir.ne = Option::Some(self.cell(self.cell_index(idx.row - 1, idx.col + 1)));
            }
        }

        if idx.row < height_idx {
            if idx.col > 0 {
                cells.push(
                    self.cell_index(idx.row + 1, idx.col - 1)
                );
                dir.sw = Option::Some(self.cell(self.cell_index(idx.row + 1, idx.col - 1)));
            }

            if idx.col < width_idx {
                cells.push(
                    self.cell_index(idx.row + 1, idx.col + 1)
                );
                dir.se = Option::Some(self.cell(self.cell_index(idx.row + 1, idx.col + 1)));
            }
        }

        cells.shrink_to_fit();
        Some(cells)
        // Some(dir)
    }

    pub fn filter_indices(&self, idx: BrdIdx, player: Team, indices: Vec<usize>) -> Vec<usize> {
        indices.into_iter().filter(|i| {
            match player {
                Team::Black => self.board_index(*i).row < idx.row,
                Team::White => self.board_index(*i).row > idx.row,
            }
        }).collect()
    }

    pub fn player_diagonal_indices(&self, idx: BrdIdx, player: Team) -> Option<Vec<usize>> {
        match self.diagonal_indices(idx) {
            Some(x) => Some(self.filter_indices(idx, player, x)),
            None => None
        }
    }

    pub fn jumpable_indices(&self, idx: BrdIdx) -> Option<Vec<usize>> {
        if self.cell_state(self.cell_idx(idx)) == SquareState::Unplayable {
            return None;
        }

        let height_idx = self.height - 1;
        let width_idx = self.width - 1;

        let mut cells = Vec::with_capacity(4);

        if idx.row > 1 {
            if idx.col > 1 {
                cells.push(
                    self.cell_index(idx.row - 2, idx.col - 2)
                );
            }

            if idx.col < width_idx - 1 {
                cells.push(
                    self.cell_index(idx.row - 2, idx.col + 2)
                );
            }
        }

        if idx.row < height_idx - 1 {
            if idx.col > 1 {
                cells.push(
                    self.cell_index(idx.row + 2, idx.col - 2)
                );
            }

            if idx.col < width_idx - 1 {
                cells.push(
                    self.cell_index(idx.row + 2, idx.col + 2)
                );
            }
        }

        cells.shrink_to_fit();
        Some(cells)
    }

    pub fn player_jumpable_indices(&self, idx: BrdIdx, player: Team) -> Option<Vec<usize>> {
        match self.jumpable_indices(idx) {
            Some(x) => Some(self.filter_indices(idx, player, x)),
            None => None
        }
    }
}

#[wasm_bindgen]
impl Board {
    pub fn cell(&self, idx: usize) -> Square {
        self.cells[idx]
    }

    pub fn grid_cell(&self, idx: BrdIdx) -> Square {
        self.cell(self.cell_idx(idx))
    }
    
    pub fn cell_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    pub fn cell_idx(&self, idx: BrdIdx) -> usize {
        self.cell_index(idx.row, idx.col)
    }

    pub fn board_index(&self, idx: usize) -> BrdIdx {
        let row = idx / self.width;
        let col = idx - (row * self.width);
        BrdIdx::from(row, col)
    }

    // pub fn can_move(&self, from: BrdIdx, to: BrdIdx) -> bool {
    //     let diagonals = self.diagonal_indices(from);
    // }

    pub fn new(width: usize, height: usize) -> Board {
        let total_cells = width * height;

        let mut cells: Vec<Square>  = Vec::with_capacity(total_cells);
        let mut playable = false;

        for i in 0..height {
            for _ in 0..width {
                if playable {
                    cells.push(Square::new(SquareState::Empty, None));
                }
                else {
                    cells.push(Square::new(SquareState::Unplayable, None));
                }
                playable = !playable;
            }
            playable = i % 2 == 0;
        }

        Board {
            cells,
            width,
            height,

            current_turn: Team::Black
        }
    }

    pub fn init_game(board: Board) -> Board {
        let mut new_board = board.clone();
        for (idx, row) in RowSquareIterator::new(&board).enumerate() {
            for (jdx, square) in row.iter().enumerate() {

                if square.state == SquareState::Empty || square.state == SquareState::Occupied {
                    if idx < 3 {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::new(
                            SquareState::Occupied, 
                            Some(Piece::new(Team::White, Strength::Man))
                        );
                    } else if idx >= board.height - 3 {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::new(
                            SquareState::Occupied, 
                            Some(Piece::new(Team::Black, Strength::Man))
                        );
                    } else {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::new(
                            SquareState::Empty, 
                            None
                        );
                    }
                }
            }
        }

        new_board
    }

    pub fn current_turn(&self) -> Team {
        self.current_turn
    }

    pub fn cells(&self) -> *const Square {
        self.cells.as_ptr()
    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn cell_state(&self, idx: usize) -> SquareState {
        self.cell(idx).state
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut string = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.cell_index(i, j);

                match self.cell_state(idx) {
                    SquareState::Empty => { write!(string, "{}", SquareState::Empty); },
                    SquareState::Occupied => { write!(string, "{}", self.cell(idx).occupant.unwrap().team); },
                    SquareState::Unplayable => { write!(string, "{}", SquareState::Unplayable); },
                }
            }
            string.push('\n');
        }

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::log;

    wasm_bindgen_test_configure!(run_in_browser);

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
        // third row
        assert_eq!(BrdIdx::from(2, 4), board.board_index(20));
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

    //////////////////////
    // DIAGNOAL INDICES
    //////////////////////

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

    //////////////////////
    // JUMPABLE INDICES
    //////////////////////

    #[wasm_bindgen_test]
    fn jumpable_indices_unplayable() {
        let board = Board::new(8, 8);
        assert_eq!(None, board.jumpable_indices(BrdIdx::from(7, 7)));
        assert_eq!(None, board.jumpable_indices(BrdIdx::from(0, 0)));
        assert_eq!(None, board.jumpable_indices(BrdIdx::from(1, 1)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![24, 28]), board.jumpable_indices(BrdIdx::from(1, 2)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_central() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![10, 14, 42, 46]), board.jumpable_indices(BrdIdx::from(3, 4)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_top_row() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![19]), board.jumpable_indices(BrdIdx::from(0, 1)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_left_column() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![26]), board.jumpable_indices(BrdIdx::from(1, 0)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_bottom_row() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![40, 44]), board.jumpable_indices(BrdIdx::from(7, 2)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_right_column() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![5, 37]), board.jumpable_indices(BrdIdx::from(2, 7)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_top_right() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![21]), board.jumpable_indices(BrdIdx::from(0, 7)));
    }

    #[wasm_bindgen_test]
    fn jumpable_indices_bottom_left() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![42]), board.jumpable_indices(BrdIdx::from(7, 0)));
    }

    #[wasm_bindgen_test]
    fn init_game() {
        let board = Board::init_game(Board::new(8, 8));
        log!("{}", board);
    }

    #[wasm_bindgen_test]
    fn black_diagonal_indices() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![1, 3]), board.player_diagonal_indices(BrdIdx::from(1, 2), Team::Black));
    }

    #[wasm_bindgen_test]
    fn white_diagonal_indices() {
        let board = Board::new(8, 8);
        assert_eq!(Some(vec![17, 19]), board.player_diagonal_indices(BrdIdx::from(1, 2), Team::White));
    }
}