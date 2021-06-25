mod utils;

use std::fmt::{Display, Write};
use std::option::Option;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate indextree;
use indextree::Arena;

pub const STD_WIDTH: usize = 8;
pub const STD_HEIGHT: usize = 8;

// use rand::prelude::*;

// use rand_pcg::Pcg64Mcg;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn init_game() {
    log!("initialising wasm");
    utils::set_panic_hook();

    #[cfg(feature = "random_init")]
    log!("random layout enabled");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Black = 0,
    White = 1,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strength {
    Man = 0,
    King = 1
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SquareState {
    Empty = 0,
    Occupied = 1,
    Unplayable = 2
}

impl Display for SquareState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SquareState::Empty => {write!(f, "{}", 'E')},
            SquareState::Occupied => {write!(f, "{}", 'O')},
            SquareState::Unplayable => {write!(f, "{}", 'U')},
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Piece {
    id: u8,
    team: Team,
    strength: Strength
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Square {
    occupant: Option<Piece>,
    state: SquareState
}

impl Square {
    fn new(state: SquareState, occupant: Option<Piece>) -> Square{
        Square {
            occupant,
            state
        }
    }
}


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

///////////////
//   BOARD
///////////////

#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    cells: Vec<Square>,
    width: usize,
    height: usize,

    current_turn: Team
}

impl Board {
    pub fn cell(&self, idx: usize) -> &Square {
        &self.cells[idx]
    }

    pub fn grid_cell(&self, idx: BrdIdx) -> &Square {
        self.cell(self.cell_idx(idx))
    }

    pub fn cell_mut(&mut self, idx: usize) -> &mut Square {
        &mut self.cells[idx]
    }
}

#[wasm_bindgen]
impl Board {
    pub fn cell_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    pub fn cell_idx(&self, idx: BrdIdx) -> usize {
        self.cell_index(idx.row, idx.col)
    }

    pub fn board_index(&self, idx: usize) -> BrdIdx {
        let row = idx / self.width;
        let col = idx - (row * self.width);
        BrdIdx::from(row, col)
    }

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

    // pub fn can_move(&self, from: BrdIdx, to: BrdIdx) -> bool {
    //     let diagonals = self.diagonal_indices(from);
    // }

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

    pub fn current_turn(&self) -> Team {
        self.current_turn
    }

    pub fn cells(&self) -> *const Square {
        self.cells.as_ptr()
    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn cell_state(&self, idx: usize) -> SquareState {
        self.cell(idx).state
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let mut string = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                write!(string, "{}", 
                    self.cell_state( // empty, unocc, unplayable
                        self.cell_index( // 1d vec idx from 2d board idx
                            i, j // 2d board idx
                        )
                    )
                );
            }
            string.push('\n');
        }

        write!(f, "{}", string)
    }
}

///////////////
//   GAME
///////////////

#[wasm_bindgen]
pub struct Game {
    current: Board,
    tree: Arena<Board>
}