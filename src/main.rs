pub mod symmetric;

use crate::symmetric::caesar_cipher::CaesarCipher;

/// An example of the Caesar cipher encryption and decryption algorithms.
fn caesar_cipher_example() {
    let plain_text: &str = "XYZ";
    let caesar_cipher: CaesarCipher = CaesarCipher::new(3);
    let cipher_text: String = caesar_cipher.encrypt(plain_text);
    println!("{} -> {}", plain_text, cipher_text);
    println!("{} -> {}", cipher_text, caesar_cipher.decrypt(&cipher_text));
}

fn main() {
    caesar_cipher_example();
}