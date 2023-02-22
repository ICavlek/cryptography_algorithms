/// A simple implementation of the Caesar cipher encryption algorithm.
fn encrypt(str: &str, shift: i32) -> String {
    let mut result = String::new();
    for c in str.chars() {
        let mut c = c as u8;
        if c >= 65 && c <= 90 {
            c = (c - 65 + shift as u8) % 26 + 65;
        } else if c >= 97 && c <= 122 {
            c = (c - 97 + shift as u8) % 26 + 97;
        }
        result.push(c as char);
    }
    result
}

/// A simple implementation of the Caesar cipher decryption algorithm.
fn decrypt(str: &str, shift: i32) -> String {
    let mut result = String::new();
    for c in str.chars() {
        let mut c = c as u8;
        if c >= 65 && c <= 90 {
            c = (c - 65 - shift as u8) % 26 + 65;
        } else if c >= 97 && c <= 122 {
            c = (c - 97 - shift as u8) % 26 + 97;
        }
        result.push(c as char);
    }
    result
}

fn main() {
    let plain_text: &str = "A";
    let key: i32 = 1;
    let cipher_text: String = encrypt(plain_text, key);
    println!("{} -> {}", plain_text, cipher_text);
    println!("{} -> {}", cipher_text, decrypt(&cipher_text, key));
}
