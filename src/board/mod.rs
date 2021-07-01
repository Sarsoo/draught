//! Board module for components related to the checkers board and game structure

#[cfg(test)] pub mod tests;

pub mod enums;
use enums::*;
use enums::Team::*;
use enums::Strength::*;
use enums::SquareState::*;

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

/// Game piece given by its team and strength (normal or kinged)
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub team: Team,
    pub strength: Strength
}

#[wasm_bindgen]
impl Piece {
    #[wasm_bindgen(constructor)]
    pub fn new(team: Team, strength: Strength) -> Piece {
        Piece {
            team, strength
        }
    }
}

/// Standard diagonal movements given by north west/east etc
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

/// Board squares given by a state and a possible occupying game piece
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    /// Game piece if square is occupied
    pub occupant: Option<Piece>,
    /// Description of whether the square is occupied or an unplayable, i.e. off-lattice square
    pub state: SquareState
}

#[wasm_bindgen]
impl Square {
    /// Standard constructor function to form square from state and occupant
    #[wasm_bindgen(constructor)]
    pub fn new(state: SquareState, occupant: Option<Piece>) -> Square{
        Square {
            occupant,
            state
        }
    }

    /// Helper function for a well-formed piece square by team and strength
    pub fn pc(team: Team, strength: Strength) -> Square {
        Square {
            occupant: Some(Piece::new(team, strength)),
            state: Occupied,
        }
    }

    /// Helper function for a well-formed empty square
    pub fn empty() -> Square {
        Square {
            occupant: None,
            state: Empty,
        }
    }

    /// Helper function for a well-formed unplayable square
    pub fn unplay() -> Square {
        Square {
            occupant: None,
            state: Unplayable,
        }
    }
}

/// Rank 2 tensor index to identify a board square by row and column
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

/// Single state of a checkers board
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    /// 1D backing array of board squares for the 2D game board
    cells: Vec<Square>,
    pub width: usize,
    pub height: usize,

    pub current_turn: Team
}

////////////////////
//  UNBOUND FUNCS
////////////////////

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
    pub fn adjacent_indices(&self, idx: BrdIdx) -> Option<Vec<usize>> {
        if self.cell_state(self.cell_idx(idx)) == Unplayable {
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
    /// Similar to the [`Board::adjacent_indices`] function with differently formatted results
    /// 
    /// # Returns
    /// [`None`]: If the given square is unplayable
    /// 
    /// Some(Direction<Square>): A [`Direction`] structure for the diagonally adjacent squares.
    pub fn adjacent_dir(&self, idx: BrdIdx) -> Option<Direction<Square>> {
        if self.cell_state(self.cell_idx(idx)) == Unplayable {
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

    /// Filter an array of diagonal indices (Like those from [`Board::adjacent_indices`], [`Board::jumpable_indices`]) for those that are legal for a team's man, i.e. solely up or down the board
    pub fn filter_indices(&self, idx: BrdIdx, player: Team, indices: Vec<usize>) -> Vec<usize> {
        indices.into_iter().filter(|i| {
            match player {
                Black => self.board_index(*i).row < idx.row,
                White => self.board_index(*i).row > idx.row,
            }
        }).collect()
    }

    pub fn player_adjacent_indices(&self, idx: BrdIdx, player: Team) -> Option<Vec<usize>> {
        match self.adjacent_indices(idx) {
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
        if self.cell_state(self.cell_idx(idx)) == Unplayable {
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
        if self.cell_state(self.cell_idx(idx)) == Unplayable {
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

    /// Get the difference between two [`BrdIdx`] objects
    pub fn idx_diffs(from: BrdIdx, to: BrdIdx) -> (isize, isize) {
        // cast to signed ints so that -1 will work for black moves
        (to.row as isize - from.row as isize, to.col as isize - from.col as isize)
    }
}

///////////////////
//  BOUND FUNCS
///////////////////

#[wasm_bindgen]
impl Board {
    /// Get a copy of a board square by 1D array index
    pub fn cell(&self, idx: usize) -> Square {
        self.cells[idx]
    }

    /// Get a copy of a board square by 1D array index
    pub fn set_cell(&mut self, idx: usize, square: Square) {
        // TODO: handle this error better?
        if idx >= self.num_cells() {
            panic!("Given index is too large, idx: {}, square: {:?}", idx, square);
        }
        self.cells[idx] = square;
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

    /// Check whether a move given by source and destination indices is legal 
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
            Empty => return Moveable::UnoccupiedSrc,
            Unplayable => return Moveable::Unplayable,
            Occupied => {

                // if its not the current teams piece then error
                match from_square.occupant {
                    // weird error, shouldn't happen
                    None => panic!("Square is apparently occupied, but no occupant was found from: {}, to: {}, square: {:?}", from, to, from_square),
                    Some(from_square_occupant) => {

                        // piece in the source square is not for the current turn's player
                        if from_square_occupant.team != self.current_turn {
                            return Moveable::WrongTeamSrc;
                        }

                        // depending on whether the piece is a king or not, the piece can make different moves
                        // below validate_*_move() functions just check whether the trajectory is valid i.e a single ajacacent diagonal move for moves and 2 squares for a jump
                        // this includes validating the jumpee when jumping
                        // 
                        // we use the Allowed type to indicate that the trajectory check passed
                        // but we catch it instead of returning to allow further checks on 
                        // the destination square
                        // TODO: refactor to a IsMove()/IsJump() to check whether the move has a legal trajectory
                        match from_square_occupant.strength {
                            Man => {
                                let strength_check = self.validate_man_move(from, to, from_square_occupant);
                                if strength_check != Moveable::Allowed {
                                    return strength_check;
                                }
                            },
                            King => {
                                let strength_check = self.validate_king_move(from, to, from_square_occupant);
                                if strength_check != Moveable::Allowed {
                                    return strength_check;
                                }
                            },
                        };

                        let to_square = self.cell(self.cell_idx(to));
                        match to_square.state {
                            Empty => {
                                return Moveable::Allowed;
                            },
                            Unplayable => return Moveable::Unplayable,
                            Occupied => return Moveable::OccupiedDest,
                        }
                    }
                }
            },
        }
    }

    pub fn validate_man_move(&self, from: BrdIdx, to: BrdIdx, from_square_occupant: Piece) -> Moveable {
        let (row_diff, col_diff) = Board::idx_diffs(from, to);

        // men can only move forwards, below is row difference for each team
        let idx_scale: isize = match self.current_turn {
            Black => -1,
            White => 1,
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
                let jumpee = self.cell(self.jumpee_idx(from, to));
                match jumpee.state {
                    Empty => Moveable::NoJumpablePiece,
                    Unplayable => panic!("Found an unplayable piece to try to jump over, from: {}, to: {}, jumpee: {:?}", from, to, jumpee),
                    Occupied => {

                        // check whether jumpee is an opponent's piece
                        return Board::validate_jumpee(jumpee, from_square_occupant);
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

    pub fn validate_king_move(&self, from: BrdIdx, to: BrdIdx, from_square_occupant: Piece) -> Moveable {
        let (row_diff, col_diff) = Board::idx_diffs(from, to);

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
                let jumpee = self.cell(self.jumpee_idx(from, to));
                match jumpee.state {
                    Empty => Moveable::NoJumpablePiece,
                    Unplayable => panic!("Found an unplayable piece to try to jump over, from: {}, to: {}, jumpee: {:?}", from, to, jumpee),
                    Occupied => {

                        // check whether jumpee is an opponent's piece
                        return Board::validate_jumpee(jumpee, from_square_occupant);
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

    pub fn jumpee_idx(&self, from: BrdIdx, to: BrdIdx) -> usize {
        let (row_diff, col_diff) = Board::idx_diffs(from, to);
        self.cell_idx(
            BrdIdx::from(
                ((from.row as isize) + row_diff / 2) as usize, 
                ((from.col as isize) + col_diff / 2) as usize)
            )
    }

    /// Get a pointer to the backing array of board squares, [`Board::cells`]
    pub fn cells(&self) -> *const Square {
        self.cells.as_ptr()
    }

    /// Get the number of board squares
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    /// Get the number of remaining pieces
    pub fn num_pieces(&self) -> usize {
        let pieces: Vec<_> = PieceIterator::new(self).collect();
        pieces.len()
    }

    /// Get the number of remaining pieces for a player
    pub fn num_player(&self, team: Team) -> usize {
        let mut total = 0;
        for (_, square) in PieceIterator::new(self) {
            match square.occupant {
                None => {},
                Some(x) => {
                    if x.team == team {
                        total += 1;
                    }
                },
            }
        }
        total
    }

    /// Get the score value, Black - White pieces
    pub fn score(&self) -> isize {
        let mut black: isize = 0;
        let mut white: isize = 0;

        for (_, square) in PieceIterator::new(self) {
            if let Some(x) = square.occupant {
                match x.team {
                    Black => {
                        black += 1;
                    },
                    White => {
                        white += 1;
                    },
                }
            }
        }

        black - white
    }

    /// Get the state of a board square by 1D array index
    pub fn cell_state(&self, idx: usize) -> SquareState {
        self.cell(idx).state
    }

    pub fn apply_move(&self, from: BrdIdx, to: BrdIdx) -> Board {
        let mut new = self.clone();

        let from_idx = self.cell_idx(from);
        let to_idx = self.cell_idx(to);

        // make move update
        new.set_cell(
            to_idx, // destination square
            self.cell(from_idx) // source piece
        );

        // remove old piece
        new.set_cell(
            from_idx, // destination square
            Square::empty() // empty piece
        );

        new
    }

    pub fn apply_jump(&self, from: BrdIdx, to: BrdIdx) -> Board {
        let mut new = self.clone();

        let from_idx = self.cell_idx(from);
        let to_idx = self.cell_idx(to);

        // make move update
        new.set_cell(
            to_idx, // destination square
            self.cell(from_idx) // source piece
        );

        // remove old piece
        new.set_cell(
            from_idx, // destination square
            Square::empty() // empty piece
        );

        // remove jumpee
        new.set_cell(
            self.jumpee_idx(from, to), // destination square
            Square::empty() // empty piece
        );


        new
    }
}

/////////////////////////
//   BOUND TYPE FUNCS
/////////////////////////

#[wasm_bindgen]
impl Board {
    /// Unwrap the jumpee piece from the square and [`Board::check_jumpee_team`] with [`Moveable`] response
    pub fn validate_jumpee(jumpee: Square, from_occ: Piece) -> Moveable {
        // check whether jumpee is an opponent's piece
        match jumpee.occupant {
            None => panic!("No occupant found when checking the from: {:?} , jumpee: {:?}", from_occ, jumpee),
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

    /// Check that the source piece and the jumpee are of opposing teams
    pub fn check_jumpee_team(from: Piece, jumpee: Piece) -> bool {
        return from.team.opponent() == jumpee.team
    }

    /// Initialise a game board without game pieces
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, current_turn: Team) -> Board {
        let total_cells = width * height;

        let mut cells: Vec<Square>  = Vec::with_capacity(total_cells);
        let mut playable = false;

        for i in 0..height {
            for _ in 0..width {
                if playable {
                    cells.push(Square::empty());
                }
                else {
                    cells.push(Square::unplay());
                }
                playable = !playable;
            }
            playable = i % 2 == 0;
        }

        Board {
            cells,
            width,
            height,

            current_turn
        }
    }

    /// Reset the given board to a starting layout with 3 rows of opposing pieces
    pub fn init_game(board: Board, piece_rows: usize) -> Board {
        let mut new_board = board.clone();
        for (idx, row) in RowSquareIterator::new(&board).enumerate() {
            for (jdx, square) in row.iter().enumerate() {

                if square.state == Empty || square.state == Occupied {
                    if idx < piece_rows {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::pc(White, Man);
                    } else if idx >= board.height - piece_rows {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::pc(Black, Man);
                    } else {
                        let cell_idx = new_board.cell_index(idx, jdx);
                        new_board.cells[cell_idx] = Square::empty();
                    }
                }
            }
        }

        new_board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut string = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.cell_index(i, j);

                match self.cell_state(idx) {
                    Empty => { write!(string, "_ "); },
                    Occupied => { write!(string, "{} ", self.cell(idx).occupant.unwrap().team); },
                    Unplayable => { write!(string, ". "); },
                }
            }
            string.push('\n');
        }

        write!(f, "{}", string)
    }
}