// FILE: standard_ext_character.rs
// occt: Standard_ExtCharacter

/// Extended character type (char16_t)
pub type StandardExtCharacter = u16;

/// Converts a char to ExtCharacter
pub fn to_ext_character(c: u8) -> StandardExtCharacter {
    (c as u16) & 0x00ff
}

/// Converts an ExtCharacter to char
pub fn to_character(c: StandardExtCharacter) -> u8 {
    (c & 0x00ff) as u8
}

/// Checks if an ExtCharacter is in ASCII range
pub fn is_an_ascii(c: StandardExtCharacter) -> bool {
    (c & 0xff00) == 0
}

/// Checks if two characters have the same value
pub fn is_equal(one: StandardExtCharacter, two: StandardExtCharacter) -> bool {
    one == two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ext_character() {
        let ext = to_ext_character(b'A');
        assert_eq!(ext, 65);
    }

    #[test]
    fn test_to_character() {
        let c = to_character(65);
        assert_eq!(c, b'A');
    }

    #[test]
    fn test_is_an_ascii() {
        assert!(is_an_ascii(65));
        assert!(!is_an_ascii(0xff00));
    }

    #[test]
    fn test_is_equal() {
        assert!(is_equal(65, 65));
        assert!(!is_equal(65, 66));
    }
}
