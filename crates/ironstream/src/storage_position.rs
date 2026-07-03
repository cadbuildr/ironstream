// FILE: storage_position.rs
// occt: Storage_Position

//! A position marker in storage (typedef to i64)
//! Maps to `typedef long Storage_Position` in OCCT

pub type StoragePosition = i64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_position_type() {
        let pos: StoragePosition = 12345;
        assert_eq!(pos, 12345);
    }

    #[test]
    fn test_storage_position_range() {
        let max_pos: StoragePosition = i64::MAX;
        let min_pos: StoragePosition = i64::MIN;
        assert!(max_pos > 0);
        assert!(min_pos < 0);
    }

    #[test]
    fn test_storage_position_arithmetic() {
        let pos1: StoragePosition = 100;
        let pos2: StoragePosition = 50;
        assert_eq!(pos1 + pos2, 150);
        assert_eq!(pos1 - pos2, 50);
    }
}
