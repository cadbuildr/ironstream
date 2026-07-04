// FILE: int_polyh_couple.rs
// occt: IntPolyh_Couple

//! Pair of triangle indices for polyhedron intersection.

/// Couple of triangle indices
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntPolyhCouple {
    pub tri1: i32,
    pub tri2: i32,
}

impl IntPolyhCouple {
    /// Creates new couple
    pub fn new(tri1: i32, tri2: i32) -> Self {
        IntPolyhCouple { tri1, tri2 }
    }

    /// Returns first triangle index
    pub fn first(&self) -> i32 {
        self.tri1
    }

    /// Returns second triangle index
    pub fn second(&self) -> i32 {
        self.tri2
    }

    /// Returns whether couple is valid
    pub fn is_valid(&self) -> bool {
        self.tri1 >= 0 && self.tri2 >= 0
    }
}

impl Default for IntPolyhCouple {
    fn default() -> Self {
        IntPolyhCouple {
            tri1: -1,
            tri2: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_couple_new() {
        let couple = IntPolyhCouple::new(0, 1);
        assert_eq!(couple.first(), 0);
        assert_eq!(couple.second(), 1);
        assert!(couple.is_valid());
    }

    #[test]
    fn test_couple_invalid() {
        let couple = IntPolyhCouple::default();
        assert!(!couple.is_valid());
    }

    #[test]
    fn test_couple_equality() {
        let c1 = IntPolyhCouple::new(0, 1);
        let c2 = IntPolyhCouple::new(0, 1);
        let c3 = IntPolyhCouple::new(1, 2);
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
