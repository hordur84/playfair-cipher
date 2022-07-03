use core::fmt;

#[derive(Debug)]
struct Playfair {
    size: usize,
    data: Vec<char>
}

#[derive(Debug)]
struct LetterPair {
    letter1: char,
    letter2: char
}

impl fmt::Display for LetterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.letter1, self.letter2)
    }
}

const CHARS: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVXYZ";

impl Playfair {

    fn init(text: &[u8], size: usize) -> Self {

        let mut data = Vec::new();
        for x in 0..size*size {
            data.push(text[x] as char);
        }
        Self { size, data }
    }

    /// Returns the letter pairs for a given text. If uneven
    /// an `X` will be appended.
    fn convert(text: &str) -> Vec<LetterPair> {

        let mut text = text.replace(" ", "").to_uppercase();
        if text.len() % 2 != 0 {
            text.push('X');
        }

        let text = text.as_bytes();
        let mut pairs = vec![];

        for x in (0..text.len()-1).step_by(2) {
            let coll = LetterPair {
                letter1: text[x] as char,
                letter2: text[x+1] as char
            };
            pairs.push(coll);
        }
        pairs
    }

    /// Return the 1d index for the given letter.
    fn get_index(&self, letter: char) -> Option<usize> {

        for (i, el) in self.data.iter().enumerate() {
            if *el == letter {
                return Some(i);
            }
        }
        None
    }

    /// Return 2d index representation for given letter as `[row, column]`.
    fn get_index_2d(&self, letter: char) -> [usize; 2] {
        let row = self.get_row(letter).unwrap();
        let column = self.get_column(letter).unwrap();
        [row, column]
    }

    /// Return the letter for the given row and column index.
    fn get_letter(&self, row: usize, column: usize) -> char {
        let index = row * self.size + column;
        self.data[index]
    }

    /// Return the column index of the given letter.
    fn get_column(&self, letter: char) -> Option<usize> {
        let index = self.get_index(letter).unwrap();
        let row = self.get_row(letter).unwrap();

        for column in 0..self.size {
            if index == self.size*row + column {
                return Some(column)
            }
        }
        None
    }

    /// Return the row index of the given letter.
    fn get_row(&self, letter: char) -> Option<usize> {
        let index = self.get_index(letter).unwrap();
        for row in 0..=self.size-1 {
            if index <= self.size*row + self.size-1 {
                return Some(row);
            }
        }
        None
    }

    /// Returns `True` if two letters shape up a rectangle, i.e. they are neither
    /// in the same row, or in the same column.
    fn is_shape_rectangle(&self, letters: [char; 2]) -> bool {

        !self.is_shape_column(letters) && !self.is_shape_row(letters)
    }

    /// Returns `True` if two letters are in the same row.
    fn is_shape_row(&self, letters: [char; 2]) -> bool {
        let letter1_row = self.get_row(letters[0]);
        let letter2_row = self.get_row(letters[1]);

        letter1_row == letter2_row
    }

    /// Returns `True` if two letters are in the same column.
    fn is_shape_column(&self, letters: [char; 2]) -> bool {

        let letter1_column = self.get_column(letters[0]);
        let letter2_column = self.get_column(letters[1]);

        letter1_column == letter2_column 
    }

    /// Returns an encoded letter pair for a rectangle.
    fn encode_rectangle(&self, letters: [char; 2]) -> Vec<char> {
        let mut encoded_letters = vec![];

        let letter1 = self.get_index_2d(letters[0]);
        let letter2 = self.get_index_2d(letters[1]);

        encoded_letters.push(self.get_letter(letter1[0], letter2[1]));
        encoded_letters.push(self.get_letter(letter2[0], letter1[1]));
        encoded_letters
    }

    /// Returns an encoded letter pair for a column.
    /// Encoded letters are chosen by 1 step down.
    fn encode_column(&self, letters: [char; 2]) -> Vec<char> {
        let column = self.get_column(letters[0]).unwrap();
        let mut encoded_letters = vec![];

        for letter in letters {
            let row = self.get_row(letter).unwrap();
            let char;
            if row == self.size-1 {
                char = self.get_letter(0, column);
            }
            else {
                char = self.get_letter(row+1, column);
            }
            encoded_letters.push(char);
        }
        encoded_letters
    }

    // Returns an encoded letter pair for a row.
    // Encoded letters are chosen by 1 step to the right.
    fn encode_row(&self, letters: [char; 2]) -> Vec<char> {
        let row = self.get_row(letters[0]).unwrap();
        let mut encoded_letters = vec![];

        for letter in letters {
            let column = self.get_column(letter).unwrap();
            let char;
            if column == self.size-1 {
                char = self.get_letter(row, 0);
            }
            else {
                char = self.get_letter(row, column+1);
            }
            encoded_letters.push(char);
        }
        encoded_letters 
    }


}

impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in (0..self.data.len()-1).step_by(self.size) {
            let mut s = String::new();
            for i in 0..self.size {
                s.push(self.data[x+i]);
                s.push(' ');
            }
            writeln!(f, "{}", s)? 
        }
        Ok(())
    }
}

pub fn main() {

    let p = Playfair::init(CHARS, 3);
    println!("{}", p);

    let letters = ['B', 'I'];

    println!("index: {}", p.get_index(letters[0]).unwrap());
    println!("row: {}", p.get_row(letters[0]).unwrap());
    println!("column: {}", p.get_column(letters[0]).unwrap());
    println!("index: {}", p.get_letter(0, 1));


    println!("same column: {}", p.is_shape_column(letters));
    println!("same row: {}", p.is_shape_row(letters));
    println!("rectangle: {}", p.is_shape_rectangle(letters));

    let encoded = p.encode_row(letters);
    println!("encoded: {:?}", encoded);

    let encoded = p.encode_column(letters);
    println!("encoded: {:?}", encoded);

    let encoded = p.encode_rectangle(letters);
    println!("encoded: {:?}", encoded);

    let msg = "hide the gold in the tree stump";
    let msg = Playfair::convert(msg);

    for m in msg {
        print!("{} ", m);
    }
}