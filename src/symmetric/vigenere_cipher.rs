#[derive(Debug)]
pub struct VigenereCipher {
    key: String,
    upper_case_start: u8,
    upper_case_end: u8,
    lower_case_start: u8,
    lower_case_end: u8,
    alphabet_length: u8,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        VigenereCipher { key: key.to_string(), upper_case_start: 65, upper_case_end: 90, lower_case_start: 97, lower_case_end: 122, alphabet_length: 26 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_cipher() {
        let plain_text: &str = "XYZ";
        let vigenere_cipher: VigenereCipher = VigenereCipher::new("ABC");
        // let cipher_text: String = vigenere_cipher.encrypt(plain_text);
        // println!("{} -> {}", plain_text, cipher_text);
        // println!("{} -> {}", cipher_text, vigenere_cipher.decrypt(&cipher_text));
    }
}