// FILE: standard_uuid.rs
// occt: Standard_UUID

//! Universally Unique Identifier (UUID) structure.

use std::fmt;

/// Universally Unique Identifier structure.
/// Matches the Windows GUID format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardUuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl StandardUuid {
    /// Create a new UUID with all zeros.
    pub fn new() -> Self {
        StandardUuid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }
    }

    /// Create a UUID from components.
    pub fn from_parts(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Self {
        StandardUuid {
            data1: d1,
            data2: d2,
            data3: d3,
            data4: d4,
        }
    }

    /// Generate a random UUID.
    pub fn random() -> Self {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let hasher = RandomState::new();
        let mut s = hasher.build_hasher();
        s.write_usize(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as usize);

        let hash = s.finish();
        StandardUuid {
            data1: (hash >> 32) as u32,
            data2: (hash >> 16) as u16,
            data3: hash as u16,
            data4: [
                ((hash >> 56) & 0xFF) as u8,
                ((hash >> 48) & 0xFF) as u8,
                ((hash >> 40) & 0xFF) as u8,
                ((hash >> 32) & 0xFF) as u8,
                ((hash >> 24) & 0xFF) as u8,
                ((hash >> 16) & 0xFF) as u8,
                ((hash >> 8) & 0xFF) as u8,
                (hash & 0xFF) as u8,
            ],
        }
    }

    /// Check if this UUID is nil (all zeros).
    pub fn is_nil(&self) -> bool {
        self.data1 == 0 && self.data2 == 0 && self.data3 == 0 && self.data4.iter().all(|&b| b == 0)
    }
}

impl Default for StandardUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StandardUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_creation() {
        let uuid = StandardUuid::new();
        assert!(uuid.is_nil());
    }

    #[test]
    fn test_uuid_from_parts() {
        let uuid = StandardUuid::from_parts(0x12345678, 0xABCD, 0xEF00, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(uuid.data1, 0x12345678);
        assert_eq!(uuid.data2, 0xABCD);
        assert!(!uuid.is_nil());
    }

    #[test]
    fn test_uuid_random() {
        let uuid1 = StandardUuid::random();
        let uuid2 = StandardUuid::random();
        assert!(!uuid1.is_nil());
        assert!(!uuid2.is_nil());
        // Random UUIDs may be equal by chance, but very unlikely
    }

    #[test]
    fn test_uuid_is_nil() {
        let nil_uuid = StandardUuid::new();
        assert!(nil_uuid.is_nil());

        let uuid = StandardUuid::from_parts(1, 0, 0, [0; 8]);
        assert!(!uuid.is_nil());
    }

    #[test]
    fn test_uuid_display() {
        let uuid = StandardUuid::from_parts(0x12345678, 0xABCD, 0xEF00, [1, 2, 3, 4, 5, 6, 7, 8]);
        let s = uuid.to_string();
        assert!(s.contains("12345678"));
        assert!(s.contains("ABCD"));
    }

    #[test]
    fn test_uuid_equality() {
        let uuid1 = StandardUuid::from_parts(0x12345678, 0xABCD, 0xEF00, [1, 2, 3, 4, 5, 6, 7, 8]);
        let uuid2 = StandardUuid::from_parts(0x12345678, 0xABCD, 0xEF00, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_uuid_clone() {
        let uuid1 = StandardUuid::random();
        let uuid2 = uuid1;
        assert_eq!(uuid1, uuid2);
    }
}
