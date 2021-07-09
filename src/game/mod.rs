//! Top-level object for managing [`Board`]s, applying and managing turns

use crate::board::Board;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::log;

use crate::board::{Square, BrdIdx};
use crate::board::enums::{Moveable, Team};
use crate::paint::Painter;

// use Team::*;
// use SquareState::*;

use std::fmt::{Display};

#[cfg(test)] pub mod tests;

/// Root-level structure for managing the game as a collection of board states
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    current: Board,
    previous_boards: Vec<Board>,
    painter: Option<Painter>
}

impl Game {
    /// Get a read-only copy of a previous turn's board
    pub fn previous_board(&self, turn: usize) -> &Board {
        &self.previous_boards[turn]
    }

    /// Set current board to given
    pub fn current_board(&self) -> &Board {
        &self.current
    }
}

#[wasm_bindgen]
impl Game {
    /// Get pointer to current board's squares
    pub fn current_board_cells(&self) -> *const Square {
        self.current.cells()
    }

    /// Get pointer to current board's squares
    pub fn current_board_len(&self) -> usize {
        self.current.num_cells()
    }

    /// Current turn's team
    pub fn current_turn(&self) -> Team {
        self.current.current_turn
    }

    /// Attempt to make a move given a source and destination index
    pub fn make_move(&mut self, from: BrdIdx, to: BrdIdx) -> Moveable {
        let able = self.current.can_move(from, to);

        if let Moveable::Allowed = able {
            let (_, col_diff) = Board::idx_diffs(from, to);
            // MOVE
            if col_diff.abs() == 1 {
                self.execute_move(from, to);
            } 
            // JUMP
            else {
                self.execute_jump(from, to);
            }
        } else {
            log!("Unable to make move, {:?}", able);
        }

        // board has been changed, update player turn
        self.current.current_turn = self.current.current_turn.opponent();

        able
    }

    /// Update board state with given move and push new board into current state
    pub fn execute_move(&mut self, from: BrdIdx, to: BrdIdx) {
        // set new board to current and push current to stack
        self.push_new_board(self.current.apply_move(from, to));
    }

    /// Update board state with given jump move and push new board into current state
    pub fn execute_jump(&mut self, from: BrdIdx, to: BrdIdx) {
        // set new board to current and push current to stack
        self.push_new_board(self.current.apply_jump(from, to));
    }

    /// Push current board into the previous turns and set given board to current
    pub fn push_new_board(&mut self, board: Board) {
        self.previous_boards.push(self.current.clone());
        self.set_current(board);
    }

    /// Set current board to given
    pub fn set_current(&mut self, board: Board) {
        self.current = board;
    }

    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, piece_rows: usize, first_turn: Team) -> Game {
        Game {
            current: Board::init_game(
                Board::new(width, height, first_turn), piece_rows,
            ),
            previous_boards: Vec::with_capacity(10),
            painter: None,
        }
    }

    pub fn new_with_canvas(width: usize, height: usize, piece_rows: usize, first_turn: Team, canvas_id: &str, canvas_width: u32, canvas_height: u32) -> Game {
        Game {
            current: Board::init_game(
                Board::new(width, height, first_turn), piece_rows,
            ),
            previous_boards: Vec::with_capacity(10),
            painter: Some(
                Painter::new(canvas_width, canvas_height, canvas_id)
            ),
        }
    }

    pub fn set_painter(&mut self, value: Painter) {
        self.painter = Some(value);
    }

    pub fn draw(&self) {
        match &self.painter {
            Some(p) => p.draw(&self.current),
            None => log!("No painter to draw board with")
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.current)
    }
}
