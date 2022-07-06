pub mod hash;
pub mod binary;
pub mod playfair;

use playfair::play::PlayfairState;

fn main() {

    /* Playing around with hash libraries */
    //hash::main();

    /* Playing around with binary convertion */
    //binary::main();

    let mut playfair_cypher = PlayfairState::init("Playfair example").unwrap();

    let message = "Hello, how are you this evening?";
    println!("message: {}", message);

    playfair_cypher.digest(message);
    playfair_cypher.show();

    let encoded = playfair_cypher.playfair_encode();
    println!("encoded: {}", encoded);

    playfair_cypher.digest(&encoded);

    let decoded = playfair_cypher.playfair_decode();
    println!("decoded: {}", decoded);
}

