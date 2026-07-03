// FILE: standard_character.rs
// occt: Standard_Character

/// Character utilities and constants (single byte).
pub type StandardCharacter = u8;

/// Check if character is ASCII.
pub fn is_ascii(c: StandardCharacter) -> bool {
    c < 128
}

/// Check if character is alphanumeric.
pub fn is_alphanumeric(c: StandardCharacter) -> bool {
    (c >= b'0' && c <= b'9')
        || (c >= b'a' && c <= b'z')
        || (c >= b'A' && c <= b'Z')
}

/// Check if character is alphabetic.
pub fn is_alphabetic(c: StandardCharacter) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}

/// Check if character is numeric.
pub fn is_numeric(c: StandardCharacter) -> bool {
    c >= b'0' && c <= b'9'
}

/// Check if character is a space.
pub fn is_space(c: StandardCharacter) -> bool {
    c == b' '
}

/// Check if character is whitespace.
pub fn is_whitespace(c: StandardCharacter) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

/// Convert character to uppercase.
pub fn to_upper(c: StandardCharacter) -> StandardCharacter {
    if c >= b'a' && c <= b'z' {
        c - 32
    } else {
        c
    }
}

/// Convert character to lowercase.
pub fn to_lower(c: StandardCharacter) -> StandardCharacter {
    if c >= b'A' && c <= b'Z' {
        c + 32
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii(b'a'));
        assert!(is_ascii(b'A'));
        assert!(is_ascii(b'0'));
        assert!(is_ascii(b' '));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric(b'a'));
        assert!(is_alphanumeric(b'A'));
        assert!(is_alphanumeric(b'5'));
        assert!(!is_alphanumeric(b' '));
        assert!(!is_alphanumeric(b'!'));
    }

    #[test]
    fn test_is_alphabetic() {
        assert!(is_alphabetic(b'a'));
        assert!(is_alphabetic(b'Z'));
        assert!(!is_alphabetic(b'5'));
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric(b'0'));
        assert!(is_numeric(b'9'));
        assert!(!is_numeric(b'a'));
    }

    #[test]
    fn test_is_space() {
        assert!(is_space(b' '));
        assert!(!is_space(b'a'));
        assert!(!is_space(b'\t'));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(b' '));
        assert!(is_whitespace(b'\t'));
        assert!(is_whitespace(b'\n'));
        assert!(!is_whitespace(b'a'));
    }

    #[test]
    fn test_to_upper() {
        assert_eq!(to_upper(b'a'), b'A');
        assert_eq!(to_upper(b'z'), b'Z');
        assert_eq!(to_upper(b'A'), b'A');
        assert_eq!(to_upper(b'5'), b'5');
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(to_lower(b'A'), b'a');
        assert_eq!(to_lower(b'Z'), b'z');
        assert_eq!(to_lower(b'a'), b'a');
        assert_eq!(to_lower(b'5'), b'5');
    }
}
