//! AI player logic

use indextree::{Arena, Node, NodeId, NodeEdge};

use rand::prelude::*;
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;

// use draught_web::log;
// use draught_web::log_error;

use crate::board::{Board, BrdIdx};
use crate::board::enums::{MoveType, Moveable, Team};
use crate::board::iter::{PieceIterator};

use Team::*;
// use Strength::*;
// use SquareState::*;

// use std::fmt::{Display, Write};

#[cfg(test)] pub mod tests;

/// Represents a move by source/destination indices and the move type
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

/// For storing boards in the AI tree, stores board with score for comparisons
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardNode {
    pub board: Board,
    pub score: isize,
}

impl BoardNode {
    pub fn new(board: Board, score: isize) -> BoardNode {
        BoardNode {
            board, score
        }
    }

    pub fn brd(board: Board) -> BoardNode {
        BoardNode {
            board, score: 0
        }
    }
}

/// Root-level structure for managing the game as a collection of board states
#[derive(Debug)]
pub struct Computer {
    pub search_depth: usize,
    pub team: Team,
    pub last_node_count: usize,
    pub perfect_chance: f64,
}

impl Computer {
    pub fn new(search_depth: usize, team: Team, perfect_chance: f64) -> Computer {
        Computer {
            search_depth,
            team,
            perfect_chance,
            last_node_count: 0,
        }
    }

    /// Get vector of available moves for a given board
    fn available_turns(&self, board: &Board) -> Vec<Move> {

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

                        // iterate over jumpable indices
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

    /// Generate tree of boards to given search depth, return root node
    fn gen_tree(&mut self, tree: &mut Arena<BoardNode>, board: Board) -> NodeId {

        // root node of tree
        let root = tree.new_node(BoardNode::brd(board));
        
        let mut nodes = vec!(root);

        for _ in 0..self.search_depth {
            if nodes.len() == 0 {
                break;
            }

            nodes = self.expand_layer(tree, nodes);
        }

        root
    }

    /// Propagate scores up the tree employing MiniMax algorithm
    fn propagate_scores(tree: Arena<BoardNode>, root: NodeId) -> Arena<BoardNode> {

        // need to clone tree because we iterate over it and edit it at the same time
        let mut new_tree = tree.clone();

        for n in root.traverse(&tree) {
            // only check end variant, checks nodes once
            if let NodeEdge::End(node_id) = n {

                // board current being looked at
                let board_node = new_tree
                    .get(node_id) // get Node
                    .expect("No node returned for node id")
                    .get(); // get BoardNode from Node
                
                // get scores of each nodes children
                let children_scores: Vec<isize> = node_id // current node
                    .children(&new_tree)
                    .into_iter()
                    .map(
                        |n| new_tree
                            .get(n) // get Node
                            .expect("No node returned for node id") // unwrap, should always be fine
                            .get() // get BoardNode from Node
                            .score // get score from BoardNode
                    )
                    .collect(); // finalise

                // only nodes which have children, ignores leaves
                if children_scores.len() != 0 {
                    
                    // calculate score to propagate up tree
                    new_tree
                        .get_mut(node_id) // get mutable Node to change score on
                        .expect("No node returned for node id") // should always be fine
                        .get_mut() // get mutable BoardNode from Node
                        .score = Computer::best_score(&board_node.board, children_scores);
                }
            }
        }

        new_tree
    }

    /// Get best of given scores for given team
    fn best_score(board: &Board, children_scores: Vec<isize>) -> isize {
        match board.current_turn { // MiniMax algorithm here
            // whether maximised or minimsed is based on current player
            White => children_scores
                .into_iter()
                .min()
                .expect("Couldn't unwrap min score value"),
            Black => children_scores
                .into_iter()
                .max()
                .expect("Couldn't unwrap max score value"),
        }
    }

    /// For given NodeIDs, insert score of board into tree
    /// Used for leaf nodes ready for propagating up tree
    fn insert_board_scores(&mut self, tree: &mut Arena<BoardNode>, nodes: Vec<NodeId>) {
        for i in nodes.into_iter() {
            match tree.get_mut(i) {
                Some(node) => {
                    let board_node = node.get_mut();
                    board_node.score = board_node.board.score();
                },
                None => {
                    panic!("Failed to get node when inserting board scores");
                }
            }
        }
    }

    /// Traverse tree from given root ID and get NodeIDs of leaf nodes (those without children)
    fn get_leaf_nodes(&mut self, tree: &mut Arena<BoardNode>, root: NodeId) -> Vec<NodeId> {

        let mut leaves = Vec::with_capacity(self.search_depth * 30);

        for n in root.traverse(tree) {

            // only check start variant, checks nodes once 
            if let NodeEdge::Start(node_id) = n {

                let children: Vec<NodeId> = node_id.children(tree).collect();

                if children.len() == 0 {
                    leaves.push(node_id);
                }
            }
        }

        leaves
    }

    /// Expand all given nodes and return newly inserted layer of nodes
    fn expand_layer(&mut self, tree: &mut Arena<BoardNode>, nodes: Vec<NodeId>) -> Vec<NodeId> {

        nodes
            .into_iter().map(
                |n| self.expand_node(tree, n)
            ).flatten()
            .collect()
    }

    /// Insert all possible next boards as children of given board node
    fn expand_node(&mut self, tree: &mut Arena<BoardNode>, node: NodeId) -> Vec<NodeId> {
        let node_board: Option<&Node<BoardNode>> = tree.get(node);
        
        match node_board {
            Some(node_ref) => {

                let board: &BoardNode = &*node_ref.get();

                // possible boards from given
                let boards = self.get_move_boards(board);

                // insert possible boards
                let ids = self.insert_boards(tree, boards);
                // append ids to root node
                ids.iter().for_each( |id| node.append(*id, tree) );
                
                return ids;
            },
            None => {
                panic!("No board retrieved from tree, node: {:?}", node);
            },
        }
    }

    /// Insert given boards into tree
    fn insert_boards(&mut self, tree: &mut Arena<BoardNode>, boards: Vec<BoardNode>) -> Vec<NodeId> {
        
        boards
            .into_iter().map(
                |b| tree.new_node(b)
            ).collect()
    }

    /// Get all possible next boards from given board
    fn get_move_boards(&self, board: &BoardNode) -> Vec<BoardNode> {

        self.available_turns(&board.board)
            .into_iter().map(
                |m| {
                    match m.mv_type {
                        MoveType::Move => BoardNode::brd(
                            board.board.apply_move(m.from, m.to)
                        ),
                        MoveType::Jump => BoardNode::brd(
                            board.board.apply_jump(m.from, m.to)
                        ),
                    }
                }
            ).collect()
    }

    /// Get a new board based on the given using MiniMax to make decisions 
    pub fn get_move(&mut self, brd: Board) -> Option<Board> {

        #[allow(unused_assignments)]
        let mut ret: Option<Board> = None;

        let mut tree = Arena::new();

        // generate a tree to given depth for the given board
        let root_node = self.gen_tree(&mut tree, brd);

        self.last_node_count = tree.count() - 1;

        // get the leaf nodes to insert the board scores with
        let lowest_nodes = self.get_leaf_nodes(&mut tree, root_node);

        // insert the board scores for the leaf nodes
        self.insert_board_scores(&mut tree, lowest_nodes);

        // propagate the scores up the tree, the root node has the best score
        let tree = Computer::propagate_scores(tree, root_node);

        // get root node to compare
        let root_board_node = tree
            .get(root_node) // get Node
            .expect("No node returned for node id")
            .get(); // get BoardNode from Node

        // node ids of available next moves
        let possible_moves: Vec<NodeId> = root_node.children(&tree).collect();

        if possible_moves.len() == 0 {
            return None;
        }

        // DEBUG
        #[cfg(feature = "debug_logs")]
        {
            log!("Current root score: {}", root_board_node.score);
            let scores: Vec<isize> = possible_moves
                .iter()
                .map(|n| tree.get(*n).unwrap().get().score)
                .collect();
            log!("Next boards scores: {:?}", scores);
        }

        let mut rng = rand::thread_rng();
        // random number to compare against threshold
        let perfect_num: f64 = rng.gen();

        // make perfect move
        if perfect_num < self.perfect_chance {
            #[cfg(feature = "debug_logs")]
            log!("Making perfect move");

            // get boards of equal score that are perfect for the given player
            let possible_perfect_moves: Vec<&BoardNode> = possible_moves
                .iter()
                .map(
                    // get immutable references to BoardNodes for possible moves
                    |n| tree
                        .get(*n) // get Node using NodeID
                        .expect("Unable to get perfect move data from tree node")
                        .get() // get *BoardNode from Node
                )
                .filter(
                    // filter for only scores of root node which are perfect moves
                    |b| b.score == root_board_node.score
                )
                .collect();

            // weird error, no child nodes have same score as root node
            // this is odd because the root nodes score is either the max or min of it's children
            if possible_perfect_moves.len() == 0 {
                // log_error!("No next moves matched the score of the root node, picking randomly instead");
                
                ret = Some(Computer::random_choice(&tree, possible_moves, &mut rng));
            }
            // only one possible move, use that
            else if possible_perfect_moves.len() == 1 {
                ret = Some(possible_perfect_moves[0].board.clone());
            }
            // more than one possible perfect move to make, choose one randomly
            else {
                ret = Some(
                    possible_perfect_moves
                        .choose(&mut rng) // random choice
                        .unwrap() // unwrap Option
                        .board
                        .clone()
                );
            }
        } 
        // get random move
        else {
            #[cfg(feature = "debug_logs")]
            log!("Making random move");

            ret = Some(Computer::random_choice(&tree, possible_moves, &mut rng));
        }

        ret
    }

    /// Get a random board from possible node IDs and associated tree
    fn random_choice(tree: &Arena<BoardNode>, possible_moves: Vec<NodeId>, rng: &mut ThreadRng) -> Board {
        let chosen_move = possible_moves.choose(rng).unwrap();
        tree
            .get(*chosen_move)
            .expect("Unable to get random move data from tree node")
            .get()
            .board
            .clone()
    }
}