/// Convert fixed `byte` array to a fixed `char` array.
pub fn convert_to_char(data: &[u8; 25]) -> [char; 25] {

    let mut chars = ['A'; 25];

    for i in 0..data.len() {
        chars[i] = data[i] as char;
    }

    chars
}