// FILE: standard_hash_utils.rs
// occt: Standard_HashUtils

/// Utilities for hashing
pub struct StandardHashUtils;

impl StandardHashUtils {
    /// Combines two hash values
    pub fn combine_hash(h1: u32, h2: u32) -> u32 {
        h1.wrapping_mul(31).wrapping_add(h2)
    }

    /// Computes hash for a slice of bytes
    pub fn hash_bytes(bytes: &[u8]) -> u32 {
        let mut hash: u32 = 5381;
        for b in bytes {
            hash = hash.wrapping_mul(33).wrapping_add(*b as u32);
        }
        hash
    }

    /// Computes hash for an integer
    pub fn hash_int(value: i32) -> u32 {
        let bytes = value.to_le_bytes();
        Self::hash_bytes(&bytes)
    }

    /// Rotates hash value
    pub fn rotate_hash(hash: u32, amount: u32) -> u32 {
        hash.rotate_left(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_hash() {
        let combined = StandardHashUtils::combine_hash(100, 200);
        assert!(combined > 0);
    }

    #[test]
    fn test_hash_bytes() {
        let h1 = StandardHashUtils::hash_bytes(b"test");
        let h2 = StandardHashUtils::hash_bytes(b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_int() {
        let h = StandardHashUtils::hash_int(42);
        assert!(h > 0);
    }

    #[test]
    fn test_rotate_hash() {
        let rotated = StandardHashUtils::rotate_hash(0x12345678, 8);
        assert_ne!(rotated, 0x12345678);
    }
}
