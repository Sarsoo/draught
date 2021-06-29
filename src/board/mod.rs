//! Board module for components related to the checkers board and game structure

#[cfg(test)] pub mod tests;

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

        if to.row > self.height - 1 || to.col > self.width - 1 {
            return Moveable::OutOfBounds;
        }

        let from_square = self.cell(self.cell_idx(from));

        // check source square is occupied
        match from_square.state {
            SquareState::Empty => return Moveable::UnoccupiedSrc,
            SquareState::Unplayable => return Moveable::Unplayable,
            SquareState::Occupied => {

                // if its not the current teams piece then error
                match from_square.occupant {
                    // weird error, shouldn't happen
                    None => panic!("Square is apparently occupied, but no occupant was found from: {}, to: {}, square: {:?}", from, to, from_square),
                    Some(from_square_occupant) => {

                        // piece in the source square is not for the current turn's player
                        if from_square_occupant.team != self.current_turn {
                            return Moveable::WrongTeamSrc;
                        }

                        // cast to signed ints so that -1 will work for black moves
                        let row_diff: i32 = to.row as i32 - from.row as i32;
                        let col_diff: i32 = to.col as i32 - from.col as i32;

                        // depending on whether the piece is a king or not, the piece can make different moves
                        // TODO: refactor to a IsMove()/IsJump() to check whether the move has a legal trajectory
                        match from_square_occupant.strength {
                            Strength::Man => self.validate_man_move(from, to, row_diff, col_diff, from_square_occupant),
                            Strength::King => self.validate_king_move(from, to, row_diff, col_diff, from_square_occupant),
                        };
                    }
                }
            },
        }

        Moveable::Allowed
    }

    pub fn validate_man_move(&self, from: BrdIdx, to: BrdIdx, row_diff: i32, col_diff: i32, from_square_occupant: Piece) -> Moveable {
        // men can only move forwards, below is row difference for each team
        let idx_scale: i32 = match self.current_turn {
            Team::Black => -1,
            Team::White => 1,
        };

        // legal standard move 
        if row_diff == idx_scale {
            // destination is directly to the left or right
            if col_diff.abs() == 1 {
                return Moveable::Allowed;
            } 
            // illegal, not adjacently diagional
            else {
                return Moveable::IllegalTrajectory;
            }
        }
        // legal jump move trajectory
        else if row_diff == 2 * idx_scale {
            // destination is directly to the left or right
            if col_diff.abs() == 2 {

                // piece to be jumped over
                let jumpee = self.get_jumpee(from, row_diff, col_diff);
                match jumpee.state {
                    SquareState::Empty => Moveable::NoJumpablePiece,
                    SquareState::Unplayable => panic!("Found an unplayable piece to try to jump over, from: {}, to: {}, jumpee: {:?}", from, to, jumpee),
                    SquareState::Occupied => {

                        // check whether jumpee is an opponent's piece
                        return Board::validate_jumpee(jumpee, from, to, from_square_occupant);
                    },
                }
            } 
            // illegal, not adjacently diagional
            else {
                return Moveable::IllegalTrajectory;
            }
        }
        // illegal, not adjacently diagonal
        else {
            return Moveable::IllegalTrajectory;
        }
    }

    pub fn validate_king_move(&self, from: BrdIdx, to: BrdIdx, row_diff: i32, col_diff: i32, from_square_occupant: Piece) -> Moveable {
        // legal standard move 
        if row_diff.abs() == 1 {
            // destination is directly to the left or right
            if col_diff.abs() == 1 {
                return Moveable::Allowed;
            } 
            // illegal, not adjacently diagional
            else {
                return Moveable::IllegalTrajectory;
            }
        }
        // legal jump move trajectory
        else if row_diff.abs() == 2 {
            // destination is directly to the left or right
            if col_diff.abs() == 2 {

                // piece to be jumped over
                let jumpee = self.get_jumpee(from, row_diff, col_diff);
                match jumpee.state {
                    SquareState::Empty => Moveable::NoJumpablePiece,
                    SquareState::Unplayable => panic!("Found an unplayable piece to try to jump over, from: {}, to: {}, jumpee: {:?}", from, to, jumpee),
                    SquareState::Occupied => {

                        // check whether jumpee is an opponent's piece
                        return Board::validate_jumpee(jumpee, from, to, from_square_occupant);
                    },
                }
            } 
            // illegal, not adjacently diagional
            else {
                return Moveable::IllegalTrajectory;
            }
        }
        // illegal, not adjacently diagonal
        else {
            return Moveable::IllegalTrajectory;
        }
    }

    pub fn get_jumpee(&self, from: BrdIdx, row_diff: i32, col_diff: i32) -> Square {
        self.cell(
            self.cell_idx(
                BrdIdx::from(
                    ((from.row as i32) + row_diff / 2) as usize, 
                    ((from.col as i32) + col_diff / 2) as usize)
                )
            )
    }

    pub fn validate_jumpee(jumpee: Square, from: BrdIdx, to: BrdIdx, from_occ: Piece) -> Moveable {
        // check whether jumpee is an opponent's piece
        match jumpee.occupant {
            None => panic!("No occupant found when checking the jumpee, from: {}, to: {}, jumpee: {:?}", from, to, jumpee),
            Some(jumpee_occupant_uw) => {
                if Board::check_jumpee_team(from_occ, jumpee_occupant_uw) {
                    return Moveable::Allowed;
                }
                else {
                    return Moveable::JumpingSameTeam;
                }
            },
        }
    }


    pub fn check_jumpee_team(from: Piece, jumpee: Piece) -> bool {
        return from.team.opponent() == jumpee.team
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