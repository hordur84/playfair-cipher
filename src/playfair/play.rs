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

    fn encode_row(&self, pair: [u8; 2]) -> [u8; 2] {
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

    fn encode_column(&self, pair: [u8; 2]) -> [u8; 2] {
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

    fn decode_column(&self, pair: [u8; 2]) -> [u8; 2] {
        let mut data = [0; 2];

        for i in 0..pair.len() {
            let p = self.board.get_position(pair[i]).unwrap();
            let p_updated = if p[0] == 0 {
                self.board.state[self.board.state.len()-1][p[1]]
            } else {
                self.board.state[p[0]-1][p[1]]
            };
            data[i] = p_updated;
        }
        data
    }

    fn encode_rectangle(&self, pair: [u8; 2]) -> [u8; 2] {
        let mut data = [pair[0]; 2];
        let p1 = self.board.get_position(pair[0]).unwrap();
        let p2 = self.board.get_position(pair[1]).unwrap();

        data[0] = self.board.state[p1[0]][p2[1]];
        data[1] = self.board.state[p2[0]][p1[1]];
        data
    }

    fn encode_pair(&self, pair: [u8; 2]) -> [u8; 2] {

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

    pub fn decode_pair(&self, pair: [u8; 2]) -> [u8; 2] {
        let shape = self.board.get_shape(pair);

        match shape {
            BoardShape::COLUMN => {
                self.decode_column(pair)
            },
            BoardShape::ROW => {
                !todo!()
            },
            BoardShape::RECTANGLE => {
                !todo!()
            }
        }
    }

    fn process(&self, message: &[u8]) -> Vec<PlayfairPair> {

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

    pub fn encode(&self, message: &[u8]) -> Vec<PlayfairPair> {

        let mut data = vec![];
        let message = self.process(message);

        for pair in message {
            let encoded = self.encode_pair([pair.x, pair.y]);
            let pair_encoded = PlayfairPair {
                x: encoded[0],
                y: encoded[1]
            };
            data.push(pair_encoded);
        }
        data
    }
}

pub fn main() {

    const CHARS: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVXYZ";

    let p = PlayfairState::init(CHARS);

    let pair = ['G' as u8, 'O' as u8];

    println!("{}", p.board);

    println!("encode: {:?}", p.encode_pair(pair));

    let data = p.process(b"HELLOMRSVANSON");
    for item in &data {
        print!("{} {} ", item.x, item.y);
    }
    print!("\n");
    for item in &data {
        print!("{} {} ", item.x as char, item.y as char);
    }
    print!("\n");

    let data = p.encode(b"HELLOMRSVANSON");
    for item in &data {
        print!("{} {} ", item.x, item.y);
    }
    print!("\n");
    for item in &data {
        print!("{} {} ", item.x as char, item.y as char);
    }
    print!("\n");

    /* Array test */
    let s = [[1; 3]; 2];
    println!("s: {:?}", s);
    println!("s row: {}", s.len());
    println!("s col: {}", s[0].len());

    println!("decode: {:?}", p.decode_pair([65, 70]))
}