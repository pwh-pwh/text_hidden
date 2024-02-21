use std::fmt::Debug;

fn text_to_binary<T: AsRef<str>>(text: T) -> Vec<u8> {
    text.as_ref().as_bytes().to_vec()
}

fn text_to_binary_str<T: AsRef<str>>(text: T) -> String {
    text_to_binary(text)
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

fn binary_to_zero_width<T: AsRef<str>>(binary: T, ch1: char, ch2: char) -> String {
    binary
        .as_ref()
        .chars()
        .map(|c| match c {
            '0' => ch1.to_string(),
            '1' => ch2.to_string(),
            _ => '\u{200d}'.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn encode_text_hidden<T: AsRef<str>>(text: T, ch1: char, ch2: char) -> String {
    binary_to_zero_width(text_to_binary_str(text), ch1, ch2)
}

fn zero_width_to_binary<T: AsRef<str>>(s: T, ch1: char, ch2: char) -> String {
    s.as_ref()
        .chars()
        .map(|character| {
            if character == ch1 {
                '0'
            } else if character == ch2 {
                '1'
            } else {
                ' '
            }
        })
        .collect::<String>()
}

fn binary_to_text<T: AsRef<str> + Debug>(binary: T) -> String {
    let u8_vec = binary
        .as_ref()
        .split(' ')
        .map(|c| u8::from_str_radix(c, 2).unwrap())
        .collect::<Vec<u8>>();
    String::from_utf8(u8_vec).unwrap()
}

pub fn decode_text_hidden<T: AsRef<str>>(text: T, ch1: char, ch2: char) -> String {
    binary_to_text(zero_width_to_binary(text, ch1, ch2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_binary_str() {
        let text = "hello";
        let binary = text_to_binary_str(text);
        assert_eq!(binary, "01101000 01100101 01101100 01101100 01101111");
    }

    #[test]
    fn test_binary_to_text() {
        let binary_text = "01101000 01100101 01101100 01101100 01101111";
        let text = binary_to_text(binary_text);
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_encode_text_hidden() {
        let text = "key";
        let encoded = encode_text_hidden(text, '\u{200B}', '\u{200C}');
        let origin_text = decode_text_hidden(encoded, '\u{200B}', '\u{200C}');
        assert_eq!(origin_text, text);
    }
}
