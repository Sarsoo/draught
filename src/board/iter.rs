use crate::board::{Board, Square};
use crate::board::enums::*;

pub struct RowIndexIterator<'a> {
    board: &'a Board,
    row_cursor: usize
}

impl<'a> RowIndexIterator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { 
            board, 
            row_cursor: 0 
        }
    }
}

impl<'a> Iterator for RowIndexIterator<'a> {
    type Item = Vec<usize>;
    
    /// Get next item from the iterator
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.row_cursor >= self.board.height {
            return None
        }
        let mut holder = Vec::with_capacity(self.board.width);

        let start_idx = self.board.cell_index(self.row_cursor, 0);
        for i in start_idx..start_idx + self.board.width {
            holder.push(i);
        }

        self.row_cursor += 1;
        Some(holder)
    }
}

pub struct RowSquareIterator<'a> {
    board: &'a Board,
    index_iter: RowIndexIterator<'a>,
}

impl<'a> RowSquareIterator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { 
            board, 
            index_iter: RowIndexIterator::new(board)
        }
    }
}

impl<'a> Iterator for RowSquareIterator<'a> {
    type Item = Vec<Square>;
    
    /// Get next item from the iterator
    fn next(&mut self) -> Option<Vec<Square>> {

        match self.index_iter.next() {
            Some(x) => {
                let mut holder: Vec<Square> = Vec::with_capacity(self.board.width);
                for i in x {
                    holder.push(self.board.cell(i));
                }
                Some(holder)
            },
            None => None
        }
    }
}

pub struct PieceIterator<'a> {
    board: &'a Board,
    index_cursor: usize,
}

impl<'a> PieceIterator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { 
            board, 
            index_cursor: 0
        }
    }
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = (usize, Square);
    
    /// Get next item from the iterator
    fn next(&mut self) -> Option<(usize, Square)> {

        while self.index_cursor < self.board.num_cells() - 1 {
            self.index_cursor += 1;
            match self.board.cell(self.index_cursor).state {
                SquareState::Empty | SquareState::Unplayable => continue,
                SquareState::Occupied => return Some((self.index_cursor, self.board.cell(self.index_cursor))),
            }
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::board::enums::SquareState;
    use crate::board::Piece;
    use wasm_bindgen_test::*;

    // use crate::log;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn index_iterator() {
        let board = Board::new(2, 2, Team::Black);
        let iter = RowIndexIterator::new(&board);
        let collected: Vec<Vec<usize>> = iter.collect();
        assert_eq!(vec![
            vec![0, 1],
            vec![2, 3]
        ], collected);
    }

    #[wasm_bindgen_test]
    fn square_iterator() {
        let board = Board::new(2, 2, Team::Black);
        let iter = RowSquareIterator::new(&board);
        let collected: Vec<Vec<Square>> = iter.collect();
        assert_eq!(vec![
            vec![
                Square::new(SquareState::Unplayable, Option::None), 
                Square::new(SquareState::Empty, Option::None)
                ],
            vec![
                Square::new(SquareState::Empty, Option::None), 
                Square::new(SquareState::Unplayable, Option::None)
                ]
        ], collected);
    }

    #[wasm_bindgen_test]
    fn piece_iterator_one_piece() {
        let idx = 2;

        let mut board = Board::new(4, 4, Team::Black);
        board.set_cell(idx, Square::new(
            SquareState::Occupied, 
            Some(
                Piece::new(Team::White, Strength::Man)
            )
        ));

        let iter = PieceIterator::new(&board);
        let collected: Vec<(usize, Square)> = iter.collect();

        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0], (idx, board.cell(idx)));
    }

    #[wasm_bindgen_test]
    fn piece_iterator_multiple_pieces() {

        let mut board = Board::new(4, 4, Team::Black);
        board.set_cell(2, Square::new(
            SquareState::Occupied, 
            Some(
                Piece::new(Team::White, Strength::Man)
            )
        ));

        board.set_cell(4, Square::new(
            SquareState::Occupied, 
            Some(
                Piece::new(Team::Black, Strength::Man)
            )
        ));

        board.set_cell(5, Square::new(
            SquareState::Occupied, 
            Some(
                Piece::new(Team::White, Strength::King)
            )
        ));

        let iter = PieceIterator::new(&board);
        let collected: Vec<(usize, Square)> = iter.collect();

        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], (2, board.cell(2)));
        assert_eq!(collected[1], (4, board.cell(4)));
        assert_eq!(collected[2], (5, board.cell(5)));
    }

}