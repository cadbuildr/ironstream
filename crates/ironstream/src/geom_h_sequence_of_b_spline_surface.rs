// FILE: geom_h_sequence_of_b_spline_surface.rs
// occt: Geom_HSequenceOfBSplineSurface

//! Deprecated: Use Arc<Vec<BSplineSurface>> directly.
//! Alias for backward compatibility with OCCT.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BSplineSurface {
    pub id: usize,
}

impl BSplineSurface {
    pub fn new(id: usize) -> Self {
        BSplineSurface { id }
    }
}

pub type GeomHSequenceOfBSplineSurface = Arc<Vec<BSplineSurface>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_sequence_creation() {
        let vec = vec![BSplineSurface::new(1), BSplineSurface::new(2)];
        let h_seq: GeomHSequenceOfBSplineSurface = Arc::new(vec);

        assert_eq!(h_seq.len(), 2);
        assert_eq!(h_seq[0].id, 1);
        assert_eq!(h_seq[1].id, 2);
    }

    #[test]
    fn test_h_sequence_shared() {
        let vec = vec![BSplineSurface::new(10)];
        let h_seq1 = Arc::new(vec);
        let h_seq2 = Arc::clone(&h_seq1);

        assert_eq!(Arc::strong_count(&h_seq1), 2);
        assert_eq!(h_seq2[0].id, 10);
    }

    #[test]
    fn test_sequence_access() {
        let vec = vec![
            BSplineSurface::new(5),
            BSplineSurface::new(6),
            BSplineSurface::new(7),
        ];
        let h_seq: GeomHSequenceOfBSplineSurface = Arc::new(vec);

        assert_eq!(h_seq.len(), 3);
        assert_eq!(h_seq[2].id, 7);
    }
}
