#[derive(Debug)]
pub struct CaesarCipher {
    key: i32,
    upper_case_start: u8,
    upper_case_end: u8,
    lower_case_start: u8,
    lower_case_end: u8,
    alphabet_length: u8,
}

impl CaesarCipher {
    pub fn new(key: i32) -> Self {
        CaesarCipher { key, upper_case_start: 65, upper_case_end: 90, lower_case_start: 97, lower_case_end: 122, alphabet_length: 26 }
    }

    /// A simple implementation of the Caesar cipher encryption algorithm.
    pub fn encrypt(&self, str: &str) -> String {
        let mut result = String::new();
        for c in str.chars() {
            let mut c = c as u8;
            if c >= self.upper_case_start && c <= self.upper_case_end {
                c = (c - self.upper_case_start + self.key as u8) % self.alphabet_length + self.upper_case_start;
            } else if c >= self.lower_case_start && c <= self.lower_case_end {
                c = (c - self.lower_case_start + self.key as u8) % self.alphabet_length + self.lower_case_start;
            }
            result.push(c as char);
        }
        result
    }

    /// A simple implementation of the Caesar cipher decryption algorithm.
    pub fn decrypt(&self, str: &str) -> String {
        let mut result = String::new();
        for c in str.chars() {
            let mut c = c as u8;
            if c >= self.upper_case_start && c <= self.upper_case_end {
                if (c - self.upper_case_start) <= self.key as u8 {
                    c = c - self.upper_case_start  + self.alphabet_length - self.key as u8;
                } else {
                    c = c - self.upper_case_start - self.key as u8;
                }
                c = c % self.alphabet_length + self.upper_case_start;
            } else if c >= self.lower_case_start && c <= self.lower_case_end {
                if (c - self.lower_case_start) <= self.key as u8 {
                    c = c - self.lower_case_start  + self.alphabet_length - self.key as u8;
                } else {
                    c = c - self.lower_case_start - self.key as u8;
                }
                c = c % self.alphabet_length + self.lower_case_start;
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