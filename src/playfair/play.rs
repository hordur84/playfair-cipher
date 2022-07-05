use std::fmt::Display;
use std::cmp::PartialEq;

use super::array::{Board, BoardShape};
use super::utils::convert_to_char;

const CHARS: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVXYZ";

struct PlayfairState<T> where T: Display + PartialEq {
    board: Board<T>
}

impl<T> PlayfairState<T> where T: Display + PartialEq + Copy {

    fn init(data: &[T]) -> Self {
        
        PlayfairState { board: Board::init(data) }
    }

    fn encode_row(&self, pair: [T; 2]) -> [T; 2] {
        let mut data = [pair[0]; 2];

        for i in 0..pair.len() {
            let p = self.board.get_position(pair[i]).unwrap();
            let p_updated = if p[1] == self.board.state[0].len()-1 {
                self.board.state[p[0]][0]
            } else {
                self.board.state[p[0]][p[1]+1]
            };
            data[i] = p_updated;
        }
        data
    }

    fn encode_column(&self, pair: [T; 2]) -> [T; 2] {
        let mut data = [pair[0]; 2];

        for i in 0..pair.len() {
            let p = self.board.get_position(pair[i]).unwrap();
            let p_updated = if p[0] == self.board.state[0].len()-1 {
                self.board.state[0][p[1]]
            } else {
                self.board.state[p[0]+1][p[1]]
            };
            data[i] = p_updated;
        }
        data
    }

    fn encode_rectangle(&self, pair: [T; 2]) -> [T; 2] {
        let mut data = [pair[0]; 2];
        let p1 = self.board.get_position(pair[0]).unwrap();
        let p2 = self.board.get_position(pair[1]).unwrap();

        data[0] = self.board.state[p1[0]][p2[1]];
        data[1] = self.board.state[p2[0]][p1[1]];
        data
    }

    pub fn encode(&self, pair: [T; 2]) -> [T; 2] {

        let shape = self.board.get_shape(pair);

        match shape {
            BoardShape::COLUMN => {
                self.encode_column(pair)
            },
            BoardShape::ROW => {
                self.encode_row(pair)
            },
            BoardShape::RECTANGLE => {
                self.encode_rectangle(pair)
            }
        }
    }
}

pub fn main() {

    let chars = convert_to_char(CHARS);
    let p = PlayfairState::init(&chars);

    let pair = ['G', 'O'];

    println!("{}", p.board);

    println!("encode: {:?}", p.encode(pair));
}