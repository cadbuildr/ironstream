// FILE: interface_bit_map.rs
// occt: Interface_BitMap

/// A bitmap for managing entity flags in a graph.
#[derive(Clone, Debug)]
pub struct InterfaceBitMap {
    bits: Vec<u32>, // each u32 holds 32 bits
}

impl InterfaceBitMap {
    /// Creates a BitMap with a given number of entities
    pub fn new(nb_entities: usize) -> Self {
        let nb_words = (nb_entities + 31) / 32;
        Self {
            bits: vec![0; nb_words],
        }
    }

    /// Sets a bit for an entity (1-indexed)
    pub fn set(&mut self, num: usize) {
        if num > 0 {
            let word_idx = (num - 1) / 32;
            let bit_idx = (num - 1) % 32;
            if word_idx < self.bits.len() {
                self.bits[word_idx] |= 1 << bit_idx;
            }
        }
    }

    /// Clears a bit for an entity (1-indexed)
    pub fn clear(&mut self, num: usize) {
        if num > 0 {
            let word_idx = (num - 1) / 32;
            let bit_idx = (num - 1) % 32;
            if word_idx < self.bits.len() {
                self.bits[word_idx] &= !(1 << bit_idx);
            }
        }
    }

    /// Gets the bit value for an entity (1-indexed)
    pub fn get(&self, num: usize) -> bool {
        if num > 0 {
            let word_idx = (num - 1) / 32;
            let bit_idx = (num - 1) % 32;
            if word_idx < self.bits.len() {
                (self.bits[word_idx] >> bit_idx) & 1 == 1
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let bitmap = InterfaceBitMap::new(100);
        assert!(!bitmap.get(1));
        assert!(!bitmap.get(50));
    }

    #[test]
    fn test_set_and_get() {
        let mut bitmap = InterfaceBitMap::new(100);
        bitmap.set(1);
        assert!(bitmap.get(1));
        bitmap.set(50);
        assert!(bitmap.get(50));
        assert!(!bitmap.get(2));
    }

    #[test]
    fn test_clear() {
        let mut bitmap = InterfaceBitMap::new(100);
        bitmap.set(1);
        assert!(bitmap.get(1));
        bitmap.clear(1);
        assert!(!bitmap.get(1));
    }

    #[test]
    fn test_boundary() {
        let mut bitmap = InterfaceBitMap::new(32);
        bitmap.set(1);
        bitmap.set(32);
        assert!(bitmap.get(1));
        assert!(bitmap.get(32));
    }
}
