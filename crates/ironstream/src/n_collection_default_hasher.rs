// FILE: n_collection_default_hasher.rs
// occt: NCollection_DefaultHasher

/// Default hasher for integers.
pub struct NCollectionDefaultHasher;

impl NCollectionDefaultHasher {
    pub fn hash_integer(value: i32) -> u32 {
        let mut h = value as u32;
        h = h.wrapping_mul(2654435761);
        h ^ (h >> 16)
    }

    pub fn hash_long(value: i64) -> u32 {
        let h1 = (value >> 32) as u32;
        let h2 = value as u32;
        Self::hash_integer((h1 as i32) ^ (h2 as i32))
    }

    pub fn hash_bytes(data: &[u8]) -> u32 {
        let mut hash = 0u32;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_integer() {
        let h1 = NCollectionDefaultHasher::hash_integer(42);
        let h2 = NCollectionDefaultHasher::hash_integer(42);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_bytes() {
        let h = NCollectionDefaultHasher::hash_bytes(b"test");
        assert!(h > 0);
    }
}
