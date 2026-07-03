// FILE: standard_byte.rs
// occt: Standard_Byte

/// Standard byte type (unsigned char)
pub type StandardByte = u8;

/// Creates a Standard_Byte from an integer
pub fn standard_byte(val: i32) -> StandardByte {
    (val as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_byte() {
        let b: StandardByte = 255;
        assert_eq!(b, 255);
    }

    #[test]
    fn test_standard_byte_from_int() {
        let b = standard_byte(127);
        assert_eq!(b, 127);
    }
}
