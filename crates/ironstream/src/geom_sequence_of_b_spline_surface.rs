// FILE: geom_sequence_of_b_spline_surface.rs
// occt: Geom_SequenceOfBSplineSurface

//! Deprecated: Use Vec<BSplineSurface> directly.
//! Alias for backward compatibility with OCCT.

#[derive(Clone, Debug)]
pub struct BSplineSurface {
    pub id: usize,
}

impl BSplineSurface {
    pub fn new(id: usize) -> Self {
        BSplineSurface { id }
    }
}

pub type GeomSequenceOfBSplineSurface = Vec<BSplineSurface>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: GeomSequenceOfBSplineSurface = Vec::new();
        seq.push(BSplineSurface::new(1));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].id, 1);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![
            BSplineSurface::new(1),
            BSplineSurface::new(2),
            BSplineSurface::new(3),
        ];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[2].id, 3);
    }

    #[test]
    fn test_sequence_iteration() {
        let seq = vec![
            BSplineSurface::new(10),
            BSplineSurface::new(20),
        ];

        let ids: Vec<usize> = seq.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![10, 20]);
    }
}
