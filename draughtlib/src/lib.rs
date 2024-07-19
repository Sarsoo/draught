pub mod board;
pub mod game;
pub mod comp;

pub use board::{Board, BrdIdx, Piece, Direction, Square};
pub use board::iter::{PieceIterator, RowIndexIterator, RowSquareIterator};
pub use board::enums::{Team, Strength, MoveType, SquareState};
pub use game::{Game};
pub use comp::{Computer, Move, BoardNode};