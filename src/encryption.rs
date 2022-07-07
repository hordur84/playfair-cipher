use rsa::{RsaPrivateKey, RsaPublicKey};
use rand;

pub fn main() {
    
    let mut rng = rand::thread_rng();

    let bits = 16;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    println!("private: {:?}", private_key);
    println!("public: {:?}", public_key);

    println!("primes: {:?}", private_key.primes());
}