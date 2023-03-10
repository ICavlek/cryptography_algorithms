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
    /// Create a new VigenereCipher struct.
    pub fn new(key: &str) -> Self {
        VigenereCipher { key: key.to_string(), upper_case_start: 65, upper_case_end: 90, lower_case_start: 97, lower_case_end: 122, alphabet_length: 26 }
    }

    /// A simple implementation of the Vigenere cipher encryption algorithm.
    pub fn encrypt(&self, str: &str) -> String {
        let mut result = String::new();
        let mut key_index = 0;
        for c in str.chars() {
            let mut c = c as u8;
            if c >= self.upper_case_start && c <= self.upper_case_end {
                c = (c - self.upper_case_start + self.key.chars().nth(key_index).unwrap() as u8 - self.upper_case_start) % self.alphabet_length + self.upper_case_start;
                key_index = (key_index + 1) % self.key.len();
            } else if c >= self.lower_case_start && c <= self.lower_case_end {
                c = (c - self.lower_case_start + self.key.chars().nth(key_index).unwrap() as u8 - self.lower_case_start) % self.alphabet_length + self.lower_case_start;
                key_index = (key_index + 1) % self.key.len();
            }
            result.push(c as char);
        }
        result
    }

    /// A simple implementation of the Vigenere cipher decryption algorithm.
    pub fn decrypt(&self, str: &str) -> String {
        let mut result = String::new();
        let mut key_index = 0;
        for c in str.chars() {
            let mut c = c as u8;
            if c >= self.upper_case_start && c <= self.upper_case_end {
                if (c - self.upper_case_start) <= self.key.chars().nth(key_index).unwrap() as u8 - self.upper_case_start {
                    c = c + self.alphabet_length;
                }
                c = c - self.upper_case_start;
                let secret_offset = self.key.chars().nth(key_index).unwrap() as u8 - self.upper_case_start;
                c = c - secret_offset;
                c = c % self.alphabet_length + self.upper_case_start;
                key_index = (key_index + 1) % self.key.len();
            } else if c >= self.lower_case_start && c <= self.lower_case_end {
                if (c - self.lower_case_start) <= self.key.chars().nth(key_index).unwrap() as u8 - self.lower_case_start {
                    c = c + self.alphabet_length;
                }
                c = c - self.lower_case_start;
                let secret_offset = self.key.chars().nth(key_index).unwrap() as u8 - self.lower_case_start;
                c = c - secret_offset;
                c = c % self.alphabet_length + self.lower_case_start;
                key_index = (key_index + 1) % self.key.len();
            }
            result.push(c as char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_cipher_encrypt() {
        let plain_text: &str = "this is just an example";
        let vigenere_cipher: VigenereCipher = VigenereCipher::new("secret");
        let cipher_text: String = vigenere_cipher.encrypt(plain_text);
        assert_eq!(cipher_text, "llkj ml byuk eg wbcdtew");
    }

    #[test]
    fn test_vigenere_cipher_decrypt() {
        let plain_text: &str = "this is just an example";
        let vigenere_cipher: VigenereCipher = VigenereCipher::new("secret");
        let cipher_text: String = vigenere_cipher.encrypt(plain_text);
        assert_eq!(vigenere_cipher.decrypt(&cipher_text), plain_text);
    }

    #[test]
    fn test_vigenere_cipher_uppercase_encrypt() {
        let plain_text: &str = "THIS IS JUST AN EXAMPLE";
        let vigenere_cipher: VigenereCipher = VigenereCipher::new("SECRET");
        let cipher_text: String = vigenere_cipher.encrypt(plain_text);
        assert_eq!(cipher_text, "LLKJ ML BYUK EG WBCDTEW");
    }

    #[test]
    fn test_vigenere_cipher_uppercase_decrypt() {
        let plain_text: &str = "THIS IS JUST AN EXAMPLE";
        let vigenere_cipher: VigenereCipher = VigenereCipher::new("SECRET");
        let cipher_text: String = vigenere_cipher.encrypt(plain_text);
        assert_eq!(plain_text, vigenere_cipher.decrypt(&cipher_text));
    }
}