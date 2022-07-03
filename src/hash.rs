use sha2::{self, Digest};
use hex::encode;

struct SomeStruct {}

impl AsRef<[u8]> for SomeStruct {
    fn as_ref(&self) -> &[u8] {
        &[1]
    }
}

pub fn main() {
    println!("Playing with Hash");

    let some_struct1 = SomeStruct {};
    let some_struct2 = SomeStruct {};

    let h1 = create_hash_bytes(some_struct1);
    let h2 = create_hash_bytes(some_struct2);

    println!("h1 bytes: {:?}", h1);
    println!("h2 bytes: {:?}", h2);

    println!("h1 hex: {:?}", encode(h1));
    println!("h2 hex: {:?}", encode(h2));

}

/**
 * Stupid me!
 * Trying to return a slice, &[u8], from a function that created
 * the value. When the function finishes the value is destroyed, so the reference
 * that is returned points to invalid memory. Duh!
 */
fn create_hash_bytes(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    let mut container: [u8; 32] = Default::default();
    hasher.update(data);
    let result = hasher.finalize();
    container.clone_from_slice(&result[..]);
    container
}