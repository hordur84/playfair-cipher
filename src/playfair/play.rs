use super::array::{Board, BoardShape};

struct PlayfairPair {
    x: u8,
    y: u8
}

enum PlayfairMethod {
    DECODE,
    ENCODE
}

struct PlayfairState {
    board: Board<u8>
}

impl PlayfairState {

    fn init(data: &[u8]) -> Self {
        
        PlayfairState { board: Board::init(data) }
    }

    /// Returns the processed pairs that are contained within a row of `board`. A pair is encoded
    /// within a row by increasing it's column index by 1. The decoding is the reverse.
    /// # Arguments
    /// 
    /// - `pair`: Array of two values contained within a `board`.
    /// 
    /// # Example (encoding)
    /// 
    /// - `[0,2]` and `[1,4]` -> `[0,3]` and `[1,5]` resepectively. 
    /// Given that `[0,2]` and `[1,4]` are within the same row.
    /// ```
    fn process_pair_row(&self, pair: [u8; 2], method: &PlayfairMethod) -> [u8; 2] {
        let mut data = [pair[0]; 2];

        for i in 0..pair.len() {
            let p = self.board.get_position(pair[i]).unwrap();
            match method {
                PlayfairMethod::ENCODE => {
                    let p_updated = if p[1] == self.board.state[0].len()-1 {
                        self.board.state[p[0]][0]
                    } else {
                        self.board.state[p[0]][p[1]+1]
                    };
                    data[i] = p_updated;
                },
                PlayfairMethod::DECODE => {
                    let p_updated = if p[1] == 0 {
                        self.board.state[p[0]][self.board.state[0].len()-1]
                    } else {
                        self.board.state[p[0]][p[1]-1]
                    };
                    data[i] = p_updated;
                }
            }
        }
        data
    }

    /// Returns the processed pairs that are contained within a column of `board`. A pair is encoded
    /// within a column by increasing it's row index by 1. The decoding is the reverse.
    /// # Arguments
    /// 
    /// - `pair`: Array of two values contained within a `board`.
    /// 
    /// # Example (encoding)
    /// 
    /// - `[0,2]` and `[1,4]` -> `[1,2]` and `[2,4]` resepectively. 
    /// Given that `[0,2]` and `[1,4]` are within the same column.
    /// ```
    fn process_pair_column(&self, pair: [u8; 2], method: &PlayfairMethod) -> [u8; 2] {
        let mut data = [pair[0]; 2];

        for i in 0..pair.len() {
            let p = self.board.get_position(pair[i]).unwrap();
            match method {
                PlayfairMethod::ENCODE => {
                    let p_updated = if p[0] == self.board.state.len()-1 {
                        self.board.state[0][p[1]]
                    } else {
                        self.board.state[p[0]+1][p[1]]
                    };
                    data[i] = p_updated;
                },
                PlayfairMethod::DECODE => {
                    let p_updated = if p[0] == 0 {
                        self.board.state[self.board.state.len()-1][p[1]]
                    } else {
                        self.board.state[p[0]-1][p[1]]
                    };
                    data[i] = p_updated;
                }
            }       
        }
        data
    }

    /// Returns the processed pairs that makes up a rectangle within the context of a `board`. Each pair is encoded
    /// by modifying its column value with the other pair.
    /// # Arguments
    /// 
    /// - `pair`: Array of two values contained within a `board`.
    /// 
    /// # Example (encoding)
    /// 
    /// - `[0,2]` and `[1,4]` -> `[0,4]` and `[1,2]` resepectively. 
    /// Given that `[0,2]` and `[1,4]` make up a rectangle within the `board` context.
    /// ```
    fn process_pair_rectangle(&self, pair: [u8; 2]) -> [u8; 2] {
        let mut data = [pair[0]; 2];
        let p1 = self.board.get_position(pair[0]).unwrap();
        let p2 = self.board.get_position(pair[1]).unwrap();

        data[0] = self.board.state[p1[0]][p2[1]];
        data[1] = self.board.state[p2[0]][p1[1]];
        data
    }

    /// Returns the processed pairs depending on whether they formed a `column`, `row` or a `rectangle`.
    /// 
    /// # Arguments
    /// 
    /// - `pair`: Array of two values contained within a `board`.
    /// - `method`: How to process the pair, can be either `encode` or `decode`.
    /// ```
    fn process_pair(&self, pair: [u8; 2], method: &PlayfairMethod) -> [u8; 2] {

        let shape = self.board.get_shape(pair);

        match shape {
            BoardShape::COLUMN => {
                self.process_pair_column(pair, method)
            },
            BoardShape::ROW => {
                self.process_pair_row(pair, method)
            },
            BoardShape::RECTANGLE => {
                self.process_pair_rectangle(pair)
            }
        }
    }

    /// Converts a string slice into a byte slice, and breaks it down into
    /// pairs of two. If uneven, a byte encoded `X` is added to the last pair.
    /// # Arguments
    /// 
    /// - `message`: byte encoded text message.
    pub fn process(&self, message: &str) -> Vec<PlayfairPair> {

        let message = message.as_bytes();
        let mut data = vec![];
        let mut message = message.to_vec();

        if message.len() % 2 != 0 {
            message.push('X' as u8);
        }

        for i in (0..message.len()).step_by(2) {
            let pair = PlayfairPair {
                x: message[i],
                y: message[i+1]
            };
            data.push(pair);
        }
        data
    }

    // pub fn encode(&self, message: &[u8], method: PlayfairMethod) -> Vec<PlayfairPair> {

    //     let mut data = vec![];
    //     let message = self.process(message);

    //     for pair in message {
    //         let encoded = self.process_pair([pair.x, pair.y], &method);
    //         let pair_encoded = PlayfairPair {
    //             x: encoded[0],
    //             y: encoded[1]
    //         };
    //         data.push(pair_encoded);
    //     }
    //     data
    // }
}

pub fn main() {

    // const CHARS: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVXYZ";

    // let p = PlayfairState::init(CHARS);

    // let pair = ['G' as u8, 'O' as u8];

    // println!("{}", p.board);

    // //println!("encode: {:?}", p.encode_pair(pair));

    // let data = p.process(b"HELLOMRSVANSON");
    // for item in &data {
    //     print!("{} {} ", item.x, item.y);
    // }
    // print!("\n");
    // for item in &data {
    //     print!("{} {} ", item.x as char, item.y as char);
    // }
    // print!("\n");

    // let data = p.encode(b"HELLOMRSVANSON", PlayfairMethod::ENCODE);
    // for item in &data {
    //     print!("{} {} ", item.x, item.y);
    // }
    // print!("\n");
    // for item in &data {
    //     print!("{} {} ", item.x as char, item.y as char);
    // }
    // print!("\n");

    // /* Array test */
    // let s = [[1; 3]; 2];
    // println!("s: {:?}", s);
    // println!("s row: {}", s.len());
    // println!("s col: {}", s[0].len());

    // println!("decode: {:?}", p.process_pair([67, 76], &PlayfairMethod::DECODE));
}