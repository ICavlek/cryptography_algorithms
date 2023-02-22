struct CaesarCipher {
    key: i32,
}

impl CaesarCipher {
    fn new(key: i32) -> Self {
        CaesarCipher { key }
    }

    /// A simple implementation of the Caesar cipher encryption algorithm.
    fn encrypt(&self, str: &str) -> String {
        let mut result = String::new();
        for c in str.chars() {
            let mut c = c as u8;
            if c >= 65 && c <= 90 {
                c = (c - 65 + self.key as u8) % 26 + 65;
            } else if c >= 97 && c <= 122 {
                c = (c - 97 + self.key as u8) % 26 + 97;
            }
            result.push(c as char);
        }
        result
    }

    /// A simple implementation of the Caesar cipher decryption algorithm.
    fn decrypt(&self, str: &str) -> String {
        let mut result = String::new();
        for c in str.chars() {
            let mut c = c as u8;
            if c >= 65 && c <= 90 {
                if (c - 65) <= self.key as u8 {
                    c = c - 65  + 26 - self.key as u8;
                } else {
                    c = c - 65 - self.key as u8;
                }
                c = c % 26 + 65;
            } else if c >= 97 && c <= 122 {
                if (c - 97) <= self.key as u8 {
                    c = c - 97  + 26 - self.key as u8;
                } else {
                    c = c - 97 - self.key as u8;
                }
                c = c % 26 + 97;
            }
            result.push(c as char);
        }
        result
    }
}

fn main() {
    let plain_text: &str = "XYZ";
    let caesar_cipher: CaesarCipher = CaesarCipher::new(3);
    let cipher_text: String = caesar_cipher.encrypt(plain_text);
    println!("{} -> {}", plain_text, cipher_text);
    println!("{} -> {}", cipher_text, caesar_cipher.decrypt(&cipher_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let caesar_cipher: CaesarCipher = CaesarCipher::new(3);
        assert_eq!(caesar_cipher.encrypt("abcdefghijklmnopqrstuvwxyz"), "defghijklmnopqrstuvwxyzabc");
    }

    #[test]
    fn test_decrypt() {
        let caesar_cipher: CaesarCipher = CaesarCipher::new(7);
        assert_eq!(caesar_cipher.decrypt("hijklmnopqrstuvwxyzabcdefg"), "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_encrypt_uppercase() {
        let caesar_cipher: CaesarCipher = CaesarCipher::new(5);
        assert_eq!(caesar_cipher.encrypt("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), "FGHIJKLMNOPQRSTUVWXYZABCDE");
    }

    #[test]
    fn test_decrypt_uppercase() {
        let caesar_cipher: CaesarCipher = CaesarCipher::new(2);
        assert_eq!(caesar_cipher.decrypt("CDEFGHIJKLMNOPQRSTUVWXYZAB"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}