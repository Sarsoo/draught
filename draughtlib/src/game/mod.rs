//! Top-level object for managing [`Board`]s, applying and managing turns

use crate::board::Board;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// use draught_web::log;

use crate::board::{Square, BrdIdx};
use crate::board::enums::{SquareState, Moveable, Team};
// use draught_web::paint::Painter;
use crate::comp::Computer;

use Team::*;
use SquareState::*;

use std::fmt::{Display};

#[cfg(test)] pub mod tests;

/// Root-level structure for managing the game as a collection of board states
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug)]
pub struct Game {
    current: Board,
    selected_piece: Option<BrdIdx>,
    previous_boards: Vec<Board>,
    search_depth: usize,
    pub last_node_count: usize,
    pub perfect_chance: f64,
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

    /// Current board's score
    pub fn score(&self) -> isize {
        self.current.score()
    }

    /// Get currently winning player
    pub fn winning(&self) -> Option<Team> {
        let current_score = self.score();

        if current_score < 0 {
            Some(White)
        } else if current_score == 0 {
            None
        } else {
            Some(Black)
        }
    }

    /// Check if a player has won
    pub fn has_won(&self) -> Option<Team> {

        if self.current.num_player(White) == 0 {
            Some(Black)
        } else if self.current.num_player(Black) == 0 {
            Some(White)
        } else {
            None
        }
    }

    /// Get square on current board for given index
    pub fn current_cell_state(&self, idx: &BrdIdx) -> Square {
        self.current.cell(self.current.cell_idx(*idx))
    }

    /// Set tree depth for AI to search to
    pub fn set_search_depth(&mut self, search_depth: usize) {
        self.search_depth = search_depth;
    }

    /// Set given index as selected piece
    /// TODO: Check whether valid square?
    pub fn set_selected(&mut self, idx: &BrdIdx) {
        
        if self.current.cell(self.current.cell_idx(*idx)).state != Occupied {
            panic!("Tried to select an unoccupied or empty square");
        } 

        self.selected_piece = Some(*idx);
    }

    /// Set proportion of perfect moves from AI
    pub fn set_perfect_chance(&mut self, new_chance: f64) {
        self.perfect_chance = new_chance;
    }

    /// Clear currently selected piece
    pub fn clear_selected(&mut self) {
        self.selected_piece = None;
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
            // log!("Unable to make move, {:?}", able);
        }

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

    /// Get new game without board renderer
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(width: usize, height: usize, piece_rows: usize, first_turn: Team, search_depth: usize) -> Game {
        Game {
            current: Board::init_game(
                Board::new(width, height, first_turn), piece_rows,
            ),
            selected_piece: None,
            previous_boards: Vec::with_capacity(10),
            search_depth,
            last_node_count: 0,
            perfect_chance: 0.5,
        }
    }

    /// Create computer, get move from current board and update current board
    pub fn ai_move(&mut self) {
        
        let mut comp = Computer::new(self.search_depth, self.current.current_turn, self.perfect_chance);

        let new_brd = comp.get_move(self.current.clone());

        self.last_node_count = comp.last_node_count;

        match new_brd {
            Some(brd) => self.push_new_board(brd),
            None => {
                // log!("No possible moves, re-pushing current board");

                let mut new_brd = self.current.clone();
                new_brd.current_turn = new_brd.current_turn.opponent();

                self.push_new_board(new_brd);
            },
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.current)
    }
}
