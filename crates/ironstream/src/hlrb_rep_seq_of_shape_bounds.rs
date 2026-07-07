// FILE: hlrb_rep_seq_of_shape_bounds.rs
// occt: HLRBRep_SeqOfShapeBounds

//! Deprecated: Use Vec<ShapeBounds> directly.
//! Sequence of shape bounds for HLR.

#[derive(Clone, Debug)]
pub struct ShapeBounds {
    pub shape_id: usize,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl ShapeBounds {
    pub fn new(shape_id: usize, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        ShapeBounds { shape_id, min_x, max_x, min_y, max_y }
    }

    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
}

pub type HLRBRepSeqOfShapeBounds = Vec<ShapeBounds>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_creation() {
        let mut seq: HLRBRepSeqOfShapeBounds = Vec::new();
        seq.push(ShapeBounds::new(1, 0.0, 10.0, 0.0, 20.0));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].shape_id, 1);
    }

    #[test]
    fn test_shape_bounds_width() {
        let bounds = ShapeBounds::new(1, 0.0, 10.0, 5.0, 15.0);
        assert_eq!(bounds.width(), 10.0);
        assert_eq!(bounds.height(), 10.0);
    }

    #[test]
    fn test_seq_operations() {
        let seq = vec![
            ShapeBounds::new(1, 0.0, 10.0, 0.0, 10.0),
            ShapeBounds::new(2, 20.0, 30.0, 20.0, 30.0),
        ];

        assert_eq!(seq.len(), 2);
        assert_eq!(seq[1].min_x, 20.0);
    }

    #[test]
    fn test_seq_iteration() {
        let seq = vec![
            ShapeBounds::new(1, 0.0, 5.0, 0.0, 5.0),
            ShapeBounds::new(2, 10.0, 15.0, 10.0, 15.0),
        ];

        let widths: Vec<f64> = seq.iter().map(|s| s.width()).collect();
        assert_eq!(widths, vec![5.0, 5.0]);
    }
}
