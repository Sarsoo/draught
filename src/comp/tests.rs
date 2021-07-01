use super::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn initial_tree_size() {
    let brd = Board::new(3, 2, White);
    let comp = Computer::new(brd, 3, White);
    
    assert!(!comp.tree.is_empty());
    assert_eq!(comp.tree.count(), 1);
}

#[wasm_bindgen_test]
fn available_moves() {
    // . W . 
    // _ . _

    let brd = Board::new(3, 2, White);
    let mut brd2 = brd.clone();
    let comp = Computer::new(brd, 3, White);

    // log!("{}", brd2);

    // can move left and right from central square
    brd2.set_cell(brd2.cell_index(0, 1), Square::pc(White, Man));

    // log!("{}", brd2);

    let moves = comp.available_turns(&brd2);

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
    
    let brd = Board::new(4, 4, White);
    let mut brd2 = brd.clone();
    let comp = Computer::new(brd, 3, White);

    // log!("{}", brd2);

    brd2.set_cell(brd2.cell_index(0, 1), Square::pc(White, Man));
    brd2.set_cell(brd2.cell_index(1, 2), Square::pc(Black, Man));

    // log!("{}", brd2);

    let moves = comp.available_turns(&brd2);

    // log!("{:?}", moves);

    assert_eq!(moves.len(), 2);
    assert!(moves[0].mv_type == MoveType::Move);
    assert!(moves[1].mv_type == MoveType::Jump);
}

#[wasm_bindgen_test]
fn available_moves_std_brd() {
    let brd = Board::init_game(Board::new(8, 8, White), 3);
    let mut brd2 = brd.clone();
    let comp = Computer::new(brd, 3, White);

    // log!("{}", brd2);

    let moves = comp.available_turns(&brd2);

    // log!("{:?}", moves);

    // 2 diag moves per piece except the last
    assert_eq!(moves.len(), 7);
    assert!(moves.into_iter().all(|m| m.mv_type == MoveType::Move));
}
