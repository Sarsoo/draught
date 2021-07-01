//! AI player logic

use indextree::{Arena, NodeId, Node};

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::log;

use crate::board::{Square, Board, BrdIdx};
use crate::board::enums::{SquareState, MoveType, Moveable, Team, Strength};
use crate::board::iter::{PieceIterator};

use Team::*;
use Strength::*;
use SquareState::*;

use std::fmt::{Display, Write};

#[cfg(test)] pub mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    from: BrdIdx,
    to: BrdIdx,
    mv_type: MoveType,
}

impl Move {
    pub fn new(from: BrdIdx, to: BrdIdx, mv_type: MoveType) -> Move {
        Move {
            from, to, mv_type
        }
    }
}

/// Root-level structure for managing the game as a collection of board states
#[derive(Debug)]
pub struct Computer {
    pub tree: Arena<Board>,
    pub root_node_id: NodeId,
    pub search_depth: usize,
    pub team: Team,
}

impl Computer {
    pub fn new(initial_board: Board, search_depth: usize, team: Team) -> Computer {
        let mut tree = Arena::new();
        let root_node_id = tree.new_node(initial_board);
        Computer {
            tree,
            root_node_id,
            search_depth,
            team
        }
    }

    pub fn update_board(&mut self, new_board: Board) {
        let mut tree = Arena::new();
        tree.new_node(new_board);
        self.tree = tree;
    }

    pub fn available_turns(&self, board: &Board) -> Vec<Move> {

        // allocate capacity for 2 moves per piece, likely too much but will be shrunk
        // to reduce memory re-allocations
        let mut moves = Vec::with_capacity(board.num_player(board.current_turn) * 2);

        // get all pieces in the board
        for (idx, square) in PieceIterator::new(board) {

            match square.occupant {
                None => continue,
                Some(piece) => {

                    // filter for current team's pieces
                    if piece.team == board.current_turn {
                        let from_brd_idx = board.board_index(idx);
                        let adj_op = board.adjacent_indices(from_brd_idx);
                        let jump_op = board.jumpable_indices(from_brd_idx);

                        // iterate over adjacent indices
                        if let Some(adj) = adj_op {
                            for i in adj {
                                let to_brd_idx = board.board_index(i);

                                // check if can move
                                if board.can_move(from_brd_idx, to_brd_idx) == Moveable::Allowed {
                                    moves.push(Move::new(from_brd_idx, to_brd_idx, MoveType::Move));
                                }
                            }
                        } else {
                            panic!("Unable to unwrap adjacent indices, from: {}, brd: {}", from_brd_idx, board);
                        }

                        // iterate over adjacent indices
                        if let Some(jump) = jump_op {
                            for i in jump {
                                let to_brd_idx = board.board_index(i);

                                // check if can move
                                if board.can_move(from_brd_idx, to_brd_idx) == Moveable::Allowed {
                                    moves.push(Move::new(from_brd_idx, to_brd_idx, MoveType::Jump));
                                }
                            }
                        } else {
                            panic!("Unable to unwrap adjacent indices, from: {}, brd: {}", from_brd_idx, board);
                        }
                    }
                },
            }
        }

        moves.shrink_to_fit();
        moves
    }

    pub fn gen_tree(&mut self, board: Board) {

    }

    pub fn insert_boards(&mut self, boards: Vec<Board>) -> Vec<NodeId> {
        
        boards
        .into_iter().map(
            |b| self.tree.new_node(b)
        ).collect()
    }

    pub fn get_move_boards(&self, board: &Board) -> Vec<Board> {

        self.available_turns(board)
        .into_iter().map(
            |m| {
                match m.mv_type {
                    MoveType::Move => board.apply_move(m.from, m.to),
                    MoveType::Jump => board.apply_jump(m.from, m.to),
                }
            }
        ).collect()
    }

    pub fn get_move(&self) {
        
    }
}