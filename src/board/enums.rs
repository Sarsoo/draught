extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::fmt::{Display, Write};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Black = 0,
    White = 1,
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Team::White => write!(f, "{}", 'W'),
            Team::Black => write!(f, "{}", 'B'),
        }
    }
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
            SquareState::Empty => write!(f, "{}", 'E'),
            SquareState::Occupied => write!(f, "{}", 'O'),
            SquareState::Unplayable => write!(f, "{}", 'U'),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moveable {
    Allowed = 0,
    Occupied = 1,
    OutOfBounds = 2,
}