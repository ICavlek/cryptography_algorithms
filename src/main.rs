pub mod symmetric;

use crate::symmetric::caesar_cipher::CaesarCipher;
use crate::symmetric::vigenere_cipher::VigenereCipher;

/// An example of the Caesar cipher encryption and decryption algorithms.
fn caesar_cipher_example() {
    println!("--- Caesar Cipher Example (key = 3) ---");
    let plain_text: &str = "XYZ";
    let caesar_cipher: CaesarCipher = CaesarCipher::new(3);
    let cipher_text: String = caesar_cipher.encrypt(plain_text);
    println!("{} -> {}", plain_text, cipher_text);
    println!("{} -> {}", cipher_text, caesar_cipher.decrypt(&cipher_text));
    println!();
}

/// An example of the Vigenere cipher encryption and decryption algorithms.
fn vigenere_cipher_example() {
    println!("--- Vigenere Cipher Example (key = SECRET) ---");
    let plain_text: &str = "THIS IS JUST AN EXAMPLE";
    let vigenere_cipher: VigenereCipher = VigenereCipher::new("SECRET");
    let cipher_text: String = vigenere_cipher.encrypt(plain_text);
    println!("{} -> {}", plain_text, cipher_text);
    println!("{} -> {}", cipher_text, vigenere_cipher.decrypt(&cipher_text));
    println!();
}

fn main() {
    caesar_cipher_example();
    vigenere_cipher_example()
}