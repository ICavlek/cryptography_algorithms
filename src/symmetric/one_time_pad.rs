pub struct OneTimePad {
    key: Vec<u8>,
    index: usize,
}

impl OneTimePad {
    /// Create a new OneTimePad struct.
    pub fn new(key: &str) -> Self {
        OneTimePad { key: key.as_bytes().to_vec(), index: 0 }
    }

    /// A simple implementation of the One Time Pad encryption algorithm.
    pub fn encrypt(&mut self, str: &str) -> String {
        let mut result = String::new();
        for c in str.chars() {
            let c = c as u8;
            let key = self.key[self.index];
            self.index = (self.index + 1) % self.key.len();
            result.push((c ^ key) as char);
        }
        result
    }

    /// A simple implementation of the One Time Pad decryption algorithm.
    pub fn decrypt(&mut self, str: &str) -> String {
        self.encrypt(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_time_pad() {
        let plain_text: &str = "abc";
        let mut one_time_pad: OneTimePad = OneTimePad::new("222");
        let cipher_text: String = one_time_pad.encrypt(plain_text);
        assert_eq!(cipher_text, "SPQ");
        assert_eq!(one_time_pad.decrypt(cipher_text.as_str()), plain_text);
    }
}