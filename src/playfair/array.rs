use std::fmt::{self, Display, Debug};
use std::cmp::PartialEq;

#[derive(PartialEq, Debug)]
pub enum BoardShape {
    ROW,
    COLUMN,
    RECTANGLE
}

pub struct Board<T> where T: Display + PartialEq {
    pub state: [[T; 5]; 5]
}

impl<T> Board<T> where T: Copy + Display + PartialEq {
    pub fn init(data: &[T]) -> Self {

        let mut state = [[data[0]; 5]; 5];

        for i in 0..state.len() {
            for j in 0..state[0].len() {
                state[i][j] = data[i * 5 + j];
            }
        }

        Board { state }
    }

    /// Returns an array. The array contains the position of
    /// the specified element if it exists, as `[row, column]`.
    pub fn get_position(&self, element: T) -> Option<[usize; 2]> {

        for i in 0..self.state[0].len() {
            for j in 0..self.state[1].len() {
                if element == self.state[i][j] {
                    return Some([i, j]);
                }
            }
        }
        None
    }

    /// Returns `True` if the pair shares the same column.
    fn is_shape_column(&self, pair: [T; 2]) -> bool {
        let c1 = self.get_position(pair[0]).unwrap()[1];
        let c2 = self.get_position(pair[1]).unwrap()[1];

        if c1 == c2 { return true; }
        false
    }

    /// Returns `True` if the pair shares the same row.
    fn is_shape_row(&self, pair: [T; 2]) -> bool {
        let r1 = self.get_position(pair[0]).unwrap()[0];
        let r2 = self.get_position(pair[1]).unwrap()[0];

        if r1 == r2 { return true; }
        false
    }

    /// Returns `True` if the pair shapes a rectangle, i.e. they
    /// are neither in the same row nor column.
    #[allow(dead_code)]
    fn is_shape_rectangle(&self, pair: [T; 2]) -> bool {
        !self.is_shape_column(pair) && !self.is_shape_row(pair)
    }

    /// Return the shape for the given pair, can be either 
    /// `row`, `column` or `rectangle`.
    pub fn get_shape(&self, pair: [T; 2]) -> BoardShape {

        if self.is_shape_column(pair) { BoardShape::COLUMN }
        else if self.is_shape_row(pair) { BoardShape::ROW }
        else { BoardShape::RECTANGLE }
    }
}

impl<T> fmt::Display for Board<T> where T: Display + PartialEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.state[0].len() {
            for j in 0..self.state[1].len() {
                write!(f, "{} ", self.state[i][j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::playfair::array::{Board, BoardShape};
    use crate::playfair::utils::convert_to_char;

    const CHARS: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVXYZ";

    #[test]
    fn is_shape_rectangle() {
        let chars = convert_to_char(CHARS);
        let board = Board::init(&chars);
        assert_eq!(BoardShape::RECTANGLE, board.get_shape(['A', 'R']));
    }

    #[test]
    fn is_shape_column() {
        let chars = convert_to_char(CHARS);
        let board = Board::init(&chars);
        assert_eq!(BoardShape::COLUMN, board.get_shape(['G', 'Q']));
    }

    #[test]
    fn is_shape_row() {
        let chars = convert_to_char(CHARS);
        let board = Board::init(&chars);
        assert_eq!(BoardShape::ROW, board.get_shape(['L','O']));
    }

    #[test]
    fn get_position() {
        let chars = convert_to_char(CHARS);
        let board = Board::init(&chars);
        assert_eq!([3, 2], board.get_position('R').unwrap());
    }
}

