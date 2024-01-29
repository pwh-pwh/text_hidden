pub trait Cipher {
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
}

pub struct XorCipher {
    key: u8,
}

impl XorCipher {
    pub fn new(key: u8) -> Self {
        Self { key }
    }

    pub fn from_str(key: &str) -> Self {
        Self {
            key: key.as_bytes().iter().count() as u8,
        }
    }
}

impl Cipher for XorCipher {
    fn encrypt(&self, text: &str) -> String {
        String::from_utf8(text.bytes().map(|c| c ^ self.key).collect::<Vec<u8>>()).unwrap()
    }

    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
    }
}

impl Default for XorCipher {
    fn default() -> Self {
        Self::new(0)
    }
}

pub struct NoCipher;

impl Cipher for NoCipher {
    fn encrypt(&self, text: &str) -> String {
        String::from(text)
    }

    fn decrypt(&self, text: &str) -> String {
        String::from(text)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_cipher() {
        let cipher = XorCipher::new(4);
        let text = "hello";
        let encrypted = cipher.encrypt(text);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(text, decrypted);
    }
}
