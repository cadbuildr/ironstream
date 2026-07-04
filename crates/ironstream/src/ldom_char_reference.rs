// FILE: ldom_char_reference.rs
// occt: LDOM_CharReference

/// Handles character reference and internal entity encoding/decoding
/// for XML input and output streams.
pub struct LDOMCharReference;

impl LDOMCharReference {
    /// Decode character references in a string.
    /// Returns the decoded string and its length.
    pub fn decode(src: &str) -> (String, usize) {
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = src.chars().collect();

        while i < chars.len() {
            if chars[i] == '&' {
                // Try to find entity reference
                if let Some(end) = chars[i..].iter().position(|&c| c == ';') {
                    let entity = chars[i + 1..i + end].iter().collect::<String>();
                    match entity.as_str() {
                        "amp" => result.push('&'),
                        "lt" => result.push('<'),
                        "gt" => result.push('>'),
                        "quot" => result.push('"'),
                        "apos" => result.push('\''),
                        _ => {
                            // Numeric character reference
                            if entity.starts_with('#') {
                                if let Ok(code) = entity[1..].parse::<u32>() {
                                    if let Some(c) = char::from_u32(code) {
                                        result.push(c);
                                    }
                                }
                            }
                        }
                    }
                    i += end + 1;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        let len = result.len();
        (result, len)
    }

    /// Encode a string with character and entity references.
    /// If is_attribute is true, additionally encodes quote characters.
    pub fn encode(src: &str, is_attribute: bool) -> String {
        let mut result = String::new();

        for c in src.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' if is_attribute => result.push_str("&quot;"),
                c if (c as u32) < 0x20 && c != '\t' && c != '\n' && c != '\r' => {
                    // Character reference for control characters
                    result.push_str(&format!("&#x{:x};", c as u32));
                }
                c => result.push(c),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_amp() {
        let (result, _) = LDOMCharReference::decode("&amp;");
        assert_eq!(result, "&");
    }

    #[test]
    fn test_decode_lt() {
        let (result, _) = LDOMCharReference::decode("&lt;");
        assert_eq!(result, "<");
    }

    #[test]
    fn test_decode_gt() {
        let (result, _) = LDOMCharReference::decode("&gt;");
        assert_eq!(result, ">");
    }

    #[test]
    fn test_decode_quot() {
        let (result, _) = LDOMCharReference::decode("&quot;");
        assert_eq!(result, "\"");
    }

    #[test]
    fn test_encode_amp() {
        let result = LDOMCharReference::encode("&", false);
        assert_eq!(result, "&amp;");
    }

    #[test]
    fn test_encode_lt() {
        let result = LDOMCharReference::encode("<", false);
        assert_eq!(result, "&lt;");
    }

    #[test]
    fn test_encode_gt() {
        let result = LDOMCharReference::encode(">", false);
        assert_eq!(result, "&gt;");
    }

    #[test]
    fn test_encode_attribute_quote() {
        let result = LDOMCharReference::encode("\"", true);
        assert_eq!(result, "&quot;");
    }

    #[test]
    fn test_encode_non_attribute_quote() {
        let result = LDOMCharReference::encode("\"", false);
        assert_eq!(result, "\"");
    }

    #[test]
    fn test_decode_length() {
        let (_, len) = LDOMCharReference::decode("hello");
        assert_eq!(len, 5);
    }
}
