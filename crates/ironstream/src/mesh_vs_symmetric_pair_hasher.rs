// FILE: mesh_vs_symmetric_pair_hasher.rs
// occt: MeshVS_SymmetricPairHasher

//! Hasher for symmetric pairs of indices (ignores order).

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SymmetricPair {
    pub a: u32,
    pub b: u32,
}

impl SymmetricPair {
    pub fn new(a: u32, b: u32) -> Self {
        if a <= b {
            SymmetricPair { a, b }
        } else {
            SymmetricPair { a: b, b: a }
        }
    }

    pub fn hash(&self) -> u64 {
        let combined = ((self.a as u64) << 32) | (self.b as u64);
        combined.wrapping_mul(0x85ebca6b)
    }

    pub fn are_equal(&self, other: &SymmetricPair) -> bool {
        self.a == other.a && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric_pair_normalization() {
        let p1 = SymmetricPair::new(5, 3);
        let p2 = SymmetricPair::new(3, 5);
        assert_eq!(p1.a, p2.a);
        assert_eq!(p1.b, p2.b);
    }

    #[test]
    fn symmetric_pair_hash_consistency() {
        let p1 = SymmetricPair::new(5, 3);
        let p2 = SymmetricPair::new(3, 5);
        assert_eq!(p1.hash(), p2.hash());
    }

    #[test]
    fn symmetric_pair_equality() {
        let p1 = SymmetricPair::new(5, 3);
        let p2 = SymmetricPair::new(3, 5);
        assert!(p1.are_equal(&p2));
    }
}
