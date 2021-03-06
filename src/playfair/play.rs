use super::array::{Board, BoardShape};

enum PlayfairMethod {
    DECODE,
    ENCODE
}

pub struct PlayfairCypher {
    board: Board<char>,
    msg_digested: Vec<[char; 2]>,
}

impl PlayfairCypher {

    /// Initialize a Playfair table with a secret phrase.
    pub fn init(phrase: &str) -> Result<Self, String> {
        
        let mut data: Vec<char> = vec![];
        let phrase = phrase.replace(" ", "").to_uppercase();
        let phrase = phrase.as_bytes();
        const ALPHABET: &[u8; 25] = b"ABCDEFGHIKLMNOPQRSTUVWXYZ";
    
        for letter in phrase {
            if !data.contains(&(*letter as char)) {
                data.push(*letter as char);
            }      
        }
    
        for letter in ALPHABET {
            if !data.contains(&(*letter as char)) {
                data.push(*letter as char);
            }
        }

        match data.len() {
            25 => {
                Ok(PlayfairCypher { board: Board::init(&data), msg_digested: vec![] })
            },
            _ => {
                Err(String::from("Choose another phrase!"))
            }
        }
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
    fn process_pair_row(&self, pair: [char; 2], method: &PlayfairMethod) -> [char; 2] {
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
    fn process_pair_column(&self, pair: [char; 2], method: &PlayfairMethod) -> [char; 2] {
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
    fn process_pair_rectangle(&self, pair: [char; 2]) -> [char; 2] {
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
    fn process_pair(&self, pair: [char; 2], method: &PlayfairMethod) -> [char; 2] {

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
    pub fn digest(&mut self, message: &str) {

        let mut data = vec![];
        let mut message = self.filter(message);

        if message.len() % 2 != 0 {
            message.push('X' as u8);
        }

        for i in (0..message.len()).step_by(2) {
            data.push([message[i] as char, message[i+1] as char]);
        }
        self.msg_digested = data;
    }

    /// Show debug information for digested message.
    pub fn show(&self) {
        println!("{}", self.board);
        for pair in &self.msg_digested {
            print!("{} {} ", pair[0], pair[1]);
        }
        print!("\n");
    }

    /// Perform PlayfairCypher encoding/decoding on the digested message.
    /// # Arguments
    /// 
    /// - `method`: specify the method, either `encode` or `decode`.
    fn playfair(&mut self, method: PlayfairMethod) -> String {

        let mut data = String::new();

        for pair in &self.msg_digested {
            let pair_encoded = self.process_pair([pair[0], pair[1]], &method);
            
            data.push(pair_encoded[0] as char);
            data.push(pair_encoded[1] as char);
        }
        data
    }

    /// Encode message using Playfair encoding.
    pub fn playfair_encode(&mut self) -> String {
        self.playfair(PlayfairMethod::ENCODE)
    }

    /// Decode encoded Playfair message.
    pub fn playfair_decode(&mut self) -> String {
        self.playfair(PlayfairMethod::DECODE)
    }

    /// Return a filtered array. Removing everything except the byte representation of the uppercase english
    /// alphabet, except for the letter `J`.
    /// https://en.wikipedia.org/wiki/Playfair_cipher
    fn filter(&self, msg: &str) -> Vec<u8> {
        let msg = msg.to_uppercase().as_bytes().to_vec();
        let msg: Vec<u8> = msg.into_iter().filter(|&x| x >= 65 && x <= 90 && x != 74).collect();
        msg
    }
}