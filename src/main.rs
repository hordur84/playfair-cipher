pub mod hash;
pub mod binary;
pub mod playfair;
pub mod encryption;

use playfair::play::PlayfairCypher;

fn main() {

    /* Playing around with SHA library */
    //hash::main();

    /* Playing around with binary convertion */
    //binary::main();

    /* Playing around with RSA library */
    encryption::main();

    playfair_example();
}

fn playfair_example() {
    let mut playfair_cypher = PlayfairCypher::init("Playfair example").unwrap();

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

