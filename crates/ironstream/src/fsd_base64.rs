// FILE: fsd_base64.rs
// occt: FSD_Base64

/// Base64 encoding and decoding.
pub struct Base64;

impl Base64 {
    const CHARSET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        for chunk in data.chunks(3) {
            let b1 = chunk[0];
            let b2 = if chunk.len() > 1 { chunk[1] } else { 0 };
            let b3 = if chunk.len() > 2 { chunk[2] } else { 0 };

            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            result.push(Self::CHARSET[((n >> 18) & 0x3F) as usize] as char);
            result.push(Self::CHARSET[((n >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                result.push(Self::CHARSET[((n >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(Self::CHARSET[(n & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }
        result
    }

    pub fn decode(s: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let bytes = s.as_bytes();

        for chunk in bytes.chunks(4) {
            if chunk.len() < 4 {
                break;
            }

            let b1 = Self::decode_char(chunk[0]) as u32;
            let b2 = Self::decode_char(chunk[1]) as u32;
            let b3 = Self::decode_char(chunk[2]) as u32;
            let b4 = Self::decode_char(chunk[3]) as u32;

            if b1 < 64 && b2 < 64 {
                let mut n = (b1 << 18) | (b2 << 12);
                if b3 < 64 {
                    n |= b3 << 6;
                }
                if b4 < 64 {
                    n |= b4;
                }

                result.push(((n >> 16) & 0xFF) as u8);
                if b3 < 64 {
                    result.push(((n >> 8) & 0xFF) as u8);
                    if b4 < 64 {
                        result.push((n & 0xFF) as u8);
                    }
                }
            }
        }
        result
    }

    fn decode_char(c: u8) -> u8 {
        match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_rfc4648_vectors() {
        assert_eq!(Base64::encode(b""), "");
        assert_eq!(Base64::encode(b"f"), "Zg==");
        assert_eq!(Base64::encode(b"fo"), "Zm8=");
        assert_eq!(Base64::encode(b"foo"), "Zm9v");
        assert_eq!(Base64::encode(b"foob"), "Zm9vYg==");
        assert_eq!(Base64::encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(Base64::encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_decode_rfc4648_vectors() {
        assert_eq!(Base64::decode(""), b"");
        assert_eq!(Base64::decode("Zg=="), b"f");
        assert_eq!(Base64::decode("Zm8="), b"fo");
        assert_eq!(Base64::decode("Zm9v"), b"foo");
        assert_eq!(Base64::decode("Zm9vYg=="), b"foob");
        assert_eq!(Base64::decode("Zm9vYmE="), b"fooba");
        assert_eq!(Base64::decode("Zm9vYmFy"), b"foobar");
    }

    #[test]
    fn test_roundtrip_binary() {
        let data: Vec<u8> = (0..=255u8).collect();
        let enc = Base64::encode(&data);
        assert_eq!(Base64::decode(&enc), data);
    }

    #[test]
    fn test_all_charset_chars_used() {
        // "+/" appear for high sextet values
        let enc = Base64::encode(&[0xFB, 0xFF]);
        assert_eq!(enc, "+/8=");
        assert_eq!(Base64::decode("+/8="), vec![0xFB, 0xFF]);
    }
}
