use crate::board::Board;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::log;

use crate::board::{Square, BrdIdx};
use crate::board::enums::{SquareState, Moveable, Team};

use Team::*;
use SquareState::*;

use std::fmt::{Display, Write};

#[cfg(test)] pub mod tests;

/// Root-level structure for managing the game as a collection of board states
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    current: Board,
    previous_boards: Vec<Board>,
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
    /// Current turn's team
    pub fn current_turn(&self) -> Team {
        self.current.current_turn
    }

    /// Attempt to make a move given a source and destination index
    pub fn make_move(&mut self, from: BrdIdx, to: BrdIdx) {
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
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.current)
    }
}