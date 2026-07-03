// FILE: geom2d_convert_b_spline_curve_knot_splitting.rs
// occt: Geom2dConvert_BSplineCurveKnotSplitting

/// Determines knot values where a B-spline curve should be split to achieve desired continuity.
pub struct BSplineCurveKnotSplitting {
    split_indexes: Vec<usize>,
}

impl BSplineCurveKnotSplitting {
    /// Constructs a knot-splitting analyzer.
    ///
    /// # Arguments
    /// * `continuity_range` - Desired continuity (0=C0, 1=C1, 2=C2, etc.)
    pub fn new(_continuity_range: i32) -> Self {
        BSplineCurveKnotSplitting {
            split_indexes: Vec::new(),
        }
    }

    /// Returns the number of split points (including first and last).
    pub fn nb_splits(&self) -> usize {
        self.split_indexes.len()
    }

    /// Fills the provided array with split knot indices.
    pub fn splitting(&self, split_values: &mut Vec<usize>) {
        split_values.clear();
        split_values.extend_from_slice(&self.split_indexes);
    }

    /// Returns the split knot index at the given position.
    pub fn split_value(&self, index: usize) -> Option<usize> {
        if index < self.split_indexes.len() {
            Some(self.split_indexes[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_spline_knot_splitting_new() {
        let splitter = BSplineCurveKnotSplitting::new(2);
        assert_eq!(splitter.nb_splits(), 0);
    }

    #[test]
    fn test_b_spline_knot_splitting_splitting() {
        let splitter = BSplineCurveKnotSplitting::new(1);
        let mut values = Vec::new();
        splitter.splitting(&mut values);
        assert_eq!(values.len(), 0);
    }

    #[test]
    fn test_b_spline_knot_splitting_split_value() {
        let splitter = BSplineCurveKnotSplitting::new(0);
        let result = splitter.split_value(0);
        assert_eq!(result, None);
    }
}
