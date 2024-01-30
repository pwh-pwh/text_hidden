use crate::cipher::{Cipher, XorCipher};
use crate::pose::{Pose, SimplePose};
use crate::utils::{decode_text_hidden, encode_text_hidden};

pub struct TextHidden<C: Cipher, P: Pose> {
    cipher: C,
    pose: P,
    ch1: char,
    ch2: char,
}

impl<C: Cipher, P: Pose> TextHidden<C, P> {
    pub fn new(cipher: C, pose: P, ch1: char, ch2: char) -> Self {
        Self {
            cipher,
            pose,
            ch1,
            ch2,
        }
    }

    /// 隐藏文本 text显示文本 和 key隐写文字，返回隐藏后的文本
    pub fn text_hidden(&self, text: &str, key: &str) -> String {
        let key = self.cipher.encrypt(key);
        self.pose
            .pose(text, encode_text_hidden(key, self.ch1, self.ch2).as_str())
    }

    pub fn text_recover(&self, text: &str) -> Result<String, &'static str> {
        let mut idx_left = None;
        let mut idx_right = None;
        for (i, c) in text.chars().enumerate() {
            if c == self.ch1 || c == self.ch2 {
                if idx_left.is_none() {
                    idx_left = Some(i);
                }
                idx_right = Some(i);
            }
        }
        if idx_right.is_none() || idx_left.is_none() {
            Err("can not recover")
        } else {
            let idx_left = idx_left.unwrap();
            let idx_right = idx_right.unwrap();
            let key = text
                .chars()
                .skip(idx_left)
                .take(idx_right - idx_left + 1)
                .collect::<String>();
            Ok(self
                .cipher
                .decrypt(decode_text_hidden(key, self.ch1, self.ch2).as_str()))
        }
    }
}

impl Default for TextHidden<XorCipher, SimplePose> {
    fn default() -> Self {
        Self::new(
            XorCipher::default(),
            SimplePose::default(),
            '\u{200B}',
            '\u{200C}',
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cipher::NoCipher;

    #[test]
    fn test_text_hidden() {
        let cipher = NoCipher;
        let pose = SimplePose::default();
        let text_hidden = TextHidden::new(cipher, pose, '\u{200B}', '\u{200C}');
        let th = text_hidden.text_hidden("hello", "key");
        let result = text_hidden.text_recover(th.as_str());
        assert_eq!(result.unwrap(), "key");
    }
}
