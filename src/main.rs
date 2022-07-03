pub mod hash;
pub mod binary;
pub mod playfair;

/**
 * TODO:
 * Implement fairplay cypher.&
 * Write down the ruleset from https://en.wikipedia.org/wiki/Playfair_cipher.
 */
fn main() {
    println!("Hello, world!");

    /* Playfair cypher */
    playfair::container::main();

    /* Playing around with hash libraries */
    //hash::main();

    /* Playing around with binary convertion */
    //binary::main();
}

