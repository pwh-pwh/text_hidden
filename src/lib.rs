use std::fmt::Debug;
use std::io::BufRead;

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

fn binary_to_zero_width<T: AsRef<str>>(binary: T) -> String {
    binary
        .as_ref()
        .chars()
        .map(|c| match c {
            '0' => '\u{200b}'.to_string(),
            '1' => '\u{200c}'.to_string(),
            _ => '\u{200d}'.to_string(),
        })
        .collect::<Vec<String>>()
        .join('\u{feff}'.to_string().as_str())
}

fn encode_text_hidden<T: AsRef<str>>(text: T) -> String {
    binary_to_zero_width(text_to_binary_str(text))
}

fn zero_width_to_binary<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .split('\u{feff}')
        .map(|character| match character {
            "\u{200B}" => '0',
            "\u{200C}" => '1',
            _ => ' ',
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

fn decode_text_hidden<T: AsRef<str>>(text: T) -> String {
    binary_to_text(zero_width_to_binary(text))
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
        let text = "hello";
        let encoded = encode_text_hidden(text);
        let origin_text = decode_text_hidden(encoded);
        assert_eq!(origin_text, text);
    }
}
