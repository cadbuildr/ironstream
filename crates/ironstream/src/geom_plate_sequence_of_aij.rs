// FILE: geom_plate_sequence_of_aij.rs
// occt: GeomPlate_SequenceOfAij

//! Deprecated: Use Vec<Aij> directly.
//! Alias for backward compatibility with OCCT.

#[derive(Clone, Debug, PartialEq)]
pub struct Aij {
    pub i: usize,
    pub j: usize,
    pub value: f64,
}

impl Aij {
    pub fn new(i: usize, j: usize, value: f64) -> Self {
        Aij { i, j, value }
    }
}

pub type GeomPlateSequenceOfAij = Vec<Aij>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: GeomPlateSequenceOfAij = Vec::new();
        seq.push(Aij::new(0, 0, 1.5));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].i, 0);
        assert_eq!(seq[0].value, 1.5);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![
            Aij::new(0, 0, 1.0),
            Aij::new(0, 1, 2.0),
            Aij::new(1, 1, 3.0),
        ];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[2].value, 3.0);
    }

    #[test]
    fn test_aij_equality() {
        let a1 = Aij::new(1, 2, 5.0);
        let a2 = Aij::new(1, 2, 5.0);
        let a3 = Aij::new(1, 2, 6.0);

        assert_eq!(a1, a2);
        assert_ne!(a1, a3);
    }
}
