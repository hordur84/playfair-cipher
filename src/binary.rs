use hex::encode;

pub fn main() {

    println!("Playing with binary");

    let msg = "Yes please!";
    println!("msg: {}", msg);

    let msg = msg.as_bytes();
    println!("msg as bytes: {:?}", msg);
    
    let msg = binary_from_bytes(msg);
    println!("msg as binary: {:?}", msg);

    let msg = hex_from_string_slice("Yes please!");
    println!("msg as hex: {}", msg);
}

/**
 * Return hex from a string slice.
 */
fn hex_from_string_slice(msg: &str) -> String {

    let msg = encode(msg).to_owned();
    msg
}

/**
 * Return a vector of strings, where each string is an 8 bit binary representation of a u8.
 */
fn binary_from_bytes(bytes: &[u8]) -> Vec<String> {

    let mut result = vec![];
    for byte in bytes {
        let binary = binary_u8(*byte);
        result.push(binary);
    }
    result
}

/**
 * Return the full 8 bit binary representation of a u8.
 */
fn binary_u8(num: u8) -> String {
    let mut cnt: i8 = 7;
    let mut s = String::from("");
    while cnt >= 0 {
        let tmp = num & (1<<cnt);
        match tmp {
            0 => s.push('0'),
            _ => s.push('1')
        }
        cnt = cnt - 1;
    }
    s
}