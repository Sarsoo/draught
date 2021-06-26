use crate::board::{Board, Square};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::enums::SquareState;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn index_iterator() {
        let board = Board::new(2, 2);
        let iter = RowIndexIterator::new(&board);
        let collected: Vec<Vec<usize>> = iter.collect();
        assert_eq!(vec![
            vec![0, 1],
            vec![2, 3]
        ], collected);
    }

    #[wasm_bindgen_test]
    fn square_iterator() {
        let board = Board::new(2, 2);
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

}