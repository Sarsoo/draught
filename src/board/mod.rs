//! Board module for components related to the checkers board and game structure

pub mod enums;
use enums::*;

pub mod iter;
use iter::*;

use std::fmt::{Display, Write};
use std::option::Option;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Standard width of a checkers board is 8 squares
pub const STD_WIDTH: usize = 8;
/// Standard height of a checkers board is 8 squares
pub const STD_HEIGHT: usize = 8;

/// Model a game piece by its team and strength (normal or kinged)
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

/// Model the standard diagonal movements by north west/east etc
/// 
/// Used as an absolute measure, i.e. not relative to the team making a move
/// 
/// Use options for when movements are blocked/unallowed contextually
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction<T: Clone + Copy> {
    /// North West 
    nw: Option<T>,
    /// North East
    ne: Option<T>,
    /// South East
    se: Option<T>,
    /// South West
    sw: Option<T>,
}

impl<T: Clone + Copy> Direction<T> {
    /// Create an empty direction full of [`Option::None`]
    pub fn empty() -> Direction<T> {
        Direction {
            nw: Option::None,
            ne: Option::None,
            se: Option::None,
            sw: Option::None,
        }
    }
}

/// Model board squares by a state and a possible occupying game piece
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    /// Game piece if square is occupied
    occupant: Option<Piece>,
    /// Description of whether the square is occupied or an unplayable, i.e. off-lattice square
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

/// Model a rank 2 tensor index to identify a board square by row and column
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

impl Display for BrdIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

///////////////
//   BOARD
///////////////

/// Models a single state for a checkers board
#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    /// 1D backing array of board squares for the 2D game board
    cells: Vec<Square>,
    width: usize,
    height: usize,

    current_turn: Team
}

impl Board {
    /// Get a mutable reference to a board square by 1D array index
    pub fn cell_mut(&mut self, idx: usize) -> &mut Square {
        &mut self.cells[idx]
    }

    /// Get the 1D array indices for the diagonally adjacent squares of a given board square
    /// 
    /// # Returns
    /// [`None`]: If the given square is unplayable
    /// 
    /// Some(Vec<usize>): A variable length vector of 1D indices for diagonally adjacent squares. 
    /// Vector may be between 1 and 4 items long depending on the location of the given square
    pub fn diagonal_indices(&self, idx: BrdIdx) -> Option<Vec<usize>> {
        if self.cell_state(self.cell_idx(idx)) == SquareState::Unplayable {
            return None;
        }

        let height_idx = self.height - 1;
        let width_idx = self.width - 1;

        let mut cells = Vec::with_capacity(4);

        if idx.row > 0 {
            if idx.col > 0 {
                cells.push(
                    self.cell_index(idx.row - 1, idx.col - 1)
                );
            }

            if idx.col < width_idx {
                cells.push(
                    self.cell_index(idx.row - 1, idx.col + 1)
                );
            }
        }

        if idx.row < height_idx {
            if idx.col > 0 {
                cells.push(
                    self.cell_index(idx.row + 1, idx.col - 1)
                );
            }

            if idx.col < width_idx {
                cells.push(
                    self.cell_index(idx.row + 1, idx.col + 1)
                );
            }
        }

        cells.shrink_to_fit();
        Some(cells)
    }

    /// Get a [`Direction`] structure for the diagonally adjacent squares of a given board square
    /// 
    /// Similar to the [`Board::diagonal_indices`] function with differently formatted results
    /// 
    /// # Returns
    /// [`None`]: If the given square is unplayable
    /// 
    /// Some(Direction<Square>): A [`Direction`] structure for the diagonally adjacent squares.
    pub fn adjacent_dir(&self, idx: BrdIdx) -> Option<Direction<Square>> {
        if self.cell_state(self.cell_idx(idx)) == SquareState::Unplayable {
            return None;
        }

        let height_idx = self.height - 1;
        let width_idx = self.width - 1;

        let mut dir = Direction::empty();

        if idx.row > 0 {
            if idx.col > 0 {
                dir.nw = Option::Some(self.cell(self.cell_index(idx.row - 1, idx.col - 1)));
            }

            if idx.col < width_idx {
                dir.ne = Option::Some(self.cell(self.cell_index(idx.row - 1, idx.col + 1)));
            }
        }

        if idx.row < height_idx {
            if idx.col > 0 {
                dir.sw = Option::Some(self.cell(self.cell_index(idx.row + 1, idx.col - 1)));
            }

            if idx.col < width_idx {
                dir.se = Option::Some(self.cell(self.cell_index(idx.row + 1, idx.col + 1)));
            }
        }

        Some(dir)
    }

    /// Filter an array of diagonal indices (Like those from [`Board::diagonal_indices`], [`Board::jumpable_indices`]) for those that are legal for a team's man, i.e. solely up or down the board
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

    /// Get the 1D array indices for the diagonally jumpable squares of a given board square
    /// 
    /// # Returns
    /// [`None`]: If the given square is unplayable
    /// 
    /// Some(Vec<usize>): A variable length vector of 1D indices for diagonally jumpable squares. 
    /// Vector may be between 1 and 4 items long depending on the location of the given square
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

    /// Get a [`Direction`] structure for the diagonally jumpable squares of a given board square
    /// 
    /// Similar to the [`Board::jumpable_indices`] function with differently formatted results
    /// 
    /// # Returns
    /// [`None`]: If the given square is unplayable
    /// 
    /// Some(Direction<Square>): A [`Direction`] structure for the diagonally jumpable squares.
    pub fn jumpable_dir(&self, idx: BrdIdx) -> Option<Direction<Square>> {
        if self.cell_state(self.cell_idx(idx)) == SquareState::Unplayable {
            return None;
        }

        let height_idx = self.height - 1;
        let width_idx = self.width - 1;

        let mut dir = Direction::empty();

        if idx.row > 1 {
            if idx.col > 1 {
                dir.nw = Option::Some(self.cell(self.cell_index(idx.row - 2, idx.col - 2)));
            }

            if idx.col < width_idx - 1 {
                dir.ne = Option::Some(self.cell(self.cell_index(idx.row - 2, idx.col + 2)));
            }
        }

        if idx.row < height_idx - 1 {
            if idx.col > 1 {
                dir.sw = Option::Some(self.cell(self.cell_index(idx.row + 2, idx.col - 2)));
            }

            if idx.col < width_idx - 1 {
                dir.se = Option::Some(self.cell(self.cell_index(idx.row + 2, idx.col + 2)));
            }
        }

        Some(dir)
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
    /// Get a copy of a board square by 1D array index
    pub fn cell(&self, idx: usize) -> Square {
        self.cells[idx]
    }

    /// Get a copy of a board square by 2D [`BrdIdx`] index
    pub fn grid_cell(&self, idx: BrdIdx) -> Square {
        self.cell(self.cell_idx(idx))
    }
    
    /// Transform a 2D row/column board index into a single 1D index for use with [`Board::cells`]
    pub fn cell_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    /// Similar to [`Board::cell_index`] but with a [`BrdIdx`] instead of separate indices. Transform a 2D row/column board index into a single 1D index for use with [`Board::cells`]
    pub fn cell_idx(&self, idx: BrdIdx) -> usize {
        self.cell_index(idx.row, idx.col)
    }

    /// Transform a 1D array index (for [`Board::cells`]) into a 2D game board index (by row/col)
    pub fn board_index(&self, idx: usize) -> BrdIdx {
        let row = idx / self.width;
        let col = idx - (row * self.width);
        BrdIdx::from(row, col)
    }

    pub fn can_move(&self, from: BrdIdx, to: BrdIdx) -> Moveable {

        if from.row > self.height - 1 || from.col > self.width - 1 {
            return Moveable::OutOfBounds;
        }

        let from_square = self.cell(self.cell_idx(from));

        match from_square.state {
            Empty => return Moveable::UnoccupiedSrc,
            Unplayable => return Moveable::Unplayable,
            Occupied => {

                // if its not the current teams piece then error
                match from_square.occupant {
                    None => panic!("Square is apparently occupied, but no occupant was found from: {}, to: {}, square: {:?}", from, to, from_square),
                    Some(x) => {

                        // piece in the source square is not for the current turn's player
                        if x.team != self.current_turn {
                            return Moveable::WrongTeamSrc;
                        }

                        // TODO: refactor to a IsMove()/IsJump() to check whether the move has a legal trajectory
                        match x.strength {
                            Man => {
                                match self.current_turn {
                                    Black => {
                                        
                                    },
                                    White => {

                                    },
                                };
                            },
                            King => {
                                match self.current_turn {
                                    Black => {

                                    },
                                    White => {

                                    },
                                };
                            },
                        };

                        // let diagonal = self.adjacent_dir(from);
                        // let allowable_squares = Vec::with_capacity(4);

                        let jumpable = self.jumpable_dir(from);
                    }
                }
            },
        }

        // let is_adjacent = match self.current_turn {
        //     Team::Black => diagonal.nw,
        //     Team::White => {},
        // }

        Moveable::Allowed
    }

    /// Iniitalise a game board without game pieces
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

    /// Reset the given board to a starting layout with 3 rows of opposing pieces
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

    /// Get the [`Board::current_turn`] parameter
    pub fn current_turn(&self) -> Team {
        self.current_turn
    }

    /// Get a pointer to the backing array of board squares, [`Board::cells`]
    pub fn cells(&self) -> *const Square {
        self.cells.as_ptr()
    }

    /// Get the number of board squares
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    /// Get the state of a board square by 1D array index
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

    // #[wasm_bindgen_test]
    // fn init_game() {
    //     let board = Board::init_game(Board::new(8, 8));
    //     log!("{}", board);
    // }

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