// FILE: standard_c_string_hasher.rs
// occt: Standard_CStringHasher

/// Hash function for C-style strings
pub struct StandardCStringHasher;

impl StandardCStringHasher {
    /// Computes hash of a C-style string
    pub fn hash(s: &str) -> u32 {
        let mut hash: u32 = 5381;
        for c in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(c as u32);
        }
        hash
    }

    /// Computes hash with initial value
    pub fn hash_with_init(s: &str, init: u32) -> u32 {
        let mut hash = init;
        for c in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(c as u32);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_c_string_hasher() {
        let h1 = StandardCStringHasher::hash("test");
        let h2 = StandardCStringHasher::hash("test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_standard_c_string_hasher_different() {
        let h1 = StandardCStringHasher::hash("test1");
        let h2 = StandardCStringHasher::hash("test2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_standard_c_string_hasher_with_init() {
        let h = StandardCStringHasher::hash_with_init("test", 100);
        assert!(h > 100);
    }
}
