use super::*;
// use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use crate::board::Square;
use crate::board::enums::Strength::*;
use crate::log;

// use Team::*;

wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// fn initial_tree_size() {
//     let brd = Board::new(3, 2, White);
//     let comp = Computer::new(brd, 3, White);
    
//     assert!(!comp.tree.is_empty());
//     assert_eq!(comp.tree.count(), 1);
// }

#[wasm_bindgen_test]
fn available_moves() {
    // . W . 
    // _ . _

    let mut brd = Board::new(3, 2, White);
    let comp = Computer::new(3, White);

    // log!("{}", brd);

    // can move left and right from central square
    brd.set_cell(brd.cell_index(0, 1), Square::pc(White, Man));

    // log!("{}", brd);

    let moves = comp.available_turns(&brd);

    // log!("{:?}", moves);

    assert_eq!(moves.len(), 2);
    assert!(moves.into_iter().all(|m| m.mv_type == MoveType::Move));
}

#[wasm_bindgen_test]
fn available_moves_jumps() {
    // . W . _ 
    // _ . B . 
    // . _ . _ 
    // _ . _ .
    
    let mut brd = Board::new(4, 4, White);
    let comp = Computer::new(3, White);

    // log!("{}", brd);

    brd.set_cell(brd.cell_index(0, 1), Square::pc(White, Man));
    brd.set_cell(brd.cell_index(1, 2), Square::pc(Black, Man));

    // log!("{}", brd);

    let moves = comp.available_turns(&brd);

    // log!("{:?}", moves);

    assert_eq!(moves.len(), 2);
    assert!(moves[0].mv_type == MoveType::Move);
    assert!(moves[1].mv_type == MoveType::Jump);
}

#[wasm_bindgen_test]
fn available_moves_std_brd() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let comp = Computer::new(3, White);

    // log!("{}", brd);

    let moves = comp.available_turns(&brd);

    // log!("{:?}", moves);

    // 2 diag moves per piece except the last
    assert_eq!(moves.len(), 7);
    assert!(moves.into_iter().all(|m| m.mv_type == MoveType::Move));
}

#[wasm_bindgen_test]
fn expand_node() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let mut comp = Computer::new(3, White);

    // log!("{}", brd);

    let mut tree = Arena::new(); 
    let id = tree.new_node(BoardNode::brd(brd));

    let moves = comp.expand_node(&mut tree, id);

    assert_eq!(moves.len(), 7);
    assert_eq!(tree.count(), 8);
    
    let children: Vec<NodeId> = id.children(&mut tree).collect();
    assert_eq!(children.len(), 7);
}

#[wasm_bindgen_test]
fn expand_layer() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let mut comp = Computer::new(3, White);

    // log!("{}", brd);

    let mut tree = Arena::new(); 
    let id = tree.new_node(BoardNode::brd(brd));

    let moves = comp.expand_layer(&mut tree, vec!(id));
    let moves = comp.expand_layer(&mut tree, moves);

    assert_eq!(moves.len(), 49);
    assert_eq!(tree.count(), 49 + 7 + 1);
}

#[wasm_bindgen_test]
fn leaf_nodes() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let mut comp = Computer::new(3, White);

    let mut tree = Arena::new(); 
    let id = tree.new_node(BoardNode::brd(brd));

    let moves = comp.expand_node(&mut tree, id);    
    let children: Vec<NodeId> = id.children(&mut tree).collect();
    
    let moves2 = comp.expand_node(&mut tree, children[0]);

    assert_eq!(comp.get_leaf_nodes(&mut tree, id).len(), moves.len() + moves2.len() - 1);
}

#[wasm_bindgen_test]
fn best_scores() {
    let brd = Board::new(1, 1, White);
    assert_eq!(Computer::best_score(&brd, vec!(-5, -1, 2, 3, 4)), -5);

    let brd = Board::new(1, 1, Black);
    assert_eq!(Computer::best_score(&brd, vec!(-1, 2, 3, 4)), 4);
}

#[wasm_bindgen_test]
fn propagate_scores() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let mut comp = Computer::new(3, White);

    // log!("{}", brd);

    let mut tree = Arena::new();
    let root = tree.new_node(BoardNode::brd(brd));

    comp.expand_layer(&mut tree, vec!(root));
    
    let moves = comp.propagate_scores(tree, root);
    // log!("{}", moves.len());

    // log!("{}", tree.count());
}

// #[wasm_bindgen_test]
// fn tree_2_depth() {
//     // log!("{}", performance.timing().request_start());

//     let iter = 3;
//     let mut times = Vec::with_capacity(iter);

//     for _ in 0..iter {
//         times.push(time_tree_gen(6));
//     }

//     log!("{:?}", times);
// }

// fn time_tree_gen(depth: usize) {
//     web_sys::console::time_with_label("tree_timer");

//     let mut comp = Computer::new(depth, White);

//     let mut tree = Arena::new();  
//     let brd = Board::init_game(Board::new(8, 8, White), 3);

//     comp.gen_tree(&mut tree, brd);

//     web_sys::console::time_end_with_label("tree_timer");
//     log!("{}", tree.count());
// }

// #[wasm_bindgen_test]
// fn tree_last_nodes() {
//     let mut brd = Board::new(2, 2, White);
//     brd.set_cell(1, Square::pc(White, King));
//     let mut comp = Computer::new(3, White);

//     // log!("{}", brd);
//     // log!("{}", brd.score());

//     // let moves = comp.available_turns(&brd);
//     // log!("{}", moves.len());

//     let mut tree = Arena::new();    
//     let root_node = comp.gen_tree(&mut tree, brd);

//     let lowest_nodes = comp.get_leaf_nodes(&mut tree, root_node);

//     // log!("{:#?}", lowest_nodes);

//     comp.insert_board_scores(&mut tree, lowest_nodes);

//     // log!("{:#?}", tree);
//     // log!("{}", tree.count());
// }

// #[wasm_bindgen_test]
// fn tree_score_propagation() {
//     let mut brd = Board::new(4, 4, White);
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 2)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 0)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(2, 1)), Square::pc(Black, Man));
//     let mut comp = Computer::new(1, White);

//     // log!("{}", brd);
//     // log!("{}", brd.score());

//     // let moves = comp.available_turns(&brd);
//     // log!("{}", moves.len());

//     let mut tree = Arena::new();    
//     let root_node = comp.gen_tree(&mut tree, brd);
//     // log!("{}", root_node);

//     let lowest_nodes = comp.get_leaf_nodes(&mut tree, root_node);

//     // log!("{:#?}", lowest_nodes);

//     comp.insert_board_scores(&mut tree, lowest_nodes);

//     let tree = comp.propagate_scores(tree, root_node);

//     let scores: Vec<NodeId> = root_node
//             .children(&tree)
//             .collect();
//     let scores: Vec<isize> = scores.into_iter().map(|n| tree.get(n).unwrap().get().score).collect();
//     // log!("SCORES: {:?}", scores);

//     // log!("{:#?}", tree);
//     // log!("{}", tree.count());
// }

// #[wasm_bindgen_test]
// fn tree_get_move() {
//     let mut brd = Board::new(5, 4, White);
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 2)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 0)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 4)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(2, 1)), Square::pc(Black, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(2, 3)), Square::pc(Black, Man));
//     let mut comp = Computer::new(2, White);

//     // log!("{}", brd);

//     // log!("{:?}", comp.get_move(brd).unwrap());
// }

// #[wasm_bindgen_test]
// fn tree_get_move() {
//     let mut brd = Board::new(5, 5, White);
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 2)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(1, 0)), Square::pc(White, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(2, 1)), Square::pc(Black, Man));
//     brd.set_cell(brd.cell_idx(BrdIdx::from(4, 1)), Square::pc(Black, Man));
//     let mut comp = Computer::new(2, White);

//     log!("{}", brd);

//     let next = comp.get_move(brd).unwrap();

//     log!("{}", next);
//     let mut comp = Computer::new(2, White);

//     let next = comp.get_move(next);
//     // log!("{}", next);
// }