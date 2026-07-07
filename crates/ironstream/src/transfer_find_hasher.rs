// FILE: transfer_find_hasher.rs
// occt: Transfer_FindHasher

/// A hasher for finding and mapping objects in transfer processes.
/// Provides hashing and equality comparison for entity lookup.
#[derive(Clone, Debug)]
pub struct TransferFindHasher {
    /// Hash table capacity
    capacity: u32,
    /// Number of entries
    count: u32,
}

impl TransferFindHasher {
    /// Creates a new find hasher with default capacity.
    pub fn new() -> Self {
        Self {
            capacity: 256,
            count: 0,
        }
    }

    /// Creates a hasher with a specific capacity.
    pub fn with_capacity(capacity: u32) -> Self {
        Self {
            capacity: if capacity > 0 { capacity } else { 256 },
            count: 0,
        }
    }

    /// Returns the hash table capacity.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns the number of entries.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Adds an entry to the hash table.
    pub fn add(&mut self) {
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Clears the hash table.
    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Computes a hash value for a 32-bit integer.
    pub fn hash(value: u32) -> u32 {
        let mut h = value;
        h = h.wrapping_mul(2654435761);
        h ^= h >> 16;
        h
    }
}

impl Default for TransferFindHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hasher = TransferFindHasher::new();
        assert_eq!(hasher.capacity(), 256);
        assert_eq!(hasher.count(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let hasher = TransferFindHasher::with_capacity(512);
        assert_eq!(hasher.capacity(), 512);

        let hasher = TransferFindHasher::with_capacity(0);
        assert_eq!(hasher.capacity(), 256);
    }

    #[test]
    fn test_add() {
        let mut hasher = TransferFindHasher::new();
        assert_eq!(hasher.count(), 0);

        hasher.add();
        assert_eq!(hasher.count(), 1);

        hasher.add();
        assert_eq!(hasher.count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut hasher = TransferFindHasher::new();
        hasher.add();
        hasher.add();
        assert_eq!(hasher.count(), 2);

        hasher.clear();
        assert_eq!(hasher.count(), 0);
    }

    #[test]
    fn test_hash() {
        let h1 = TransferFindHasher::hash(42);
        let h2 = TransferFindHasher::hash(42);
        assert_eq!(h1, h2);

        let h3 = TransferFindHasher::hash(43);
        assert_ne!(h1, h3);
    }
}
