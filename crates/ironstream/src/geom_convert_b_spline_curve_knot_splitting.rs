// FILE: geom_convert_b_spline_curve_knot_splitting.rs
// occt: GeomConvert_BSplineCurveKnotSplitting

/// An algorithm to determine points at which a BSpline curve should be split
/// in order to obtain arcs of the same continuity.
pub struct GeomConvertBSplineCurveKnotSplitting {
    split_indexes: Vec<usize>,
}

impl GeomConvertBSplineCurveKnotSplitting {
    /// Determines points at which the BSpline curve should be split in order
    /// to obtain arcs with a degree of continuity equal to ContinuityRange.
    pub fn new(knot_multiplicities: &[usize], degree: usize, continuity_range: usize) -> Result<Self, String> {
        if continuity_range > degree {
            return Err("ContinuityRange cannot be greater than degree".to_string());
        }

        let mut split_indexes = vec![0];

        // Add split indices where continuity drops below required level
        // For a B-spline, continuity at a knot = degree - multiplicity
        // If continuity at knot < continuity_range, we must split
        for (idx, &mult) in knot_multiplicities.iter().enumerate() {
            let continuity_at_knot = if degree >= mult {
                degree - mult
            } else {
                0
            };

            if continuity_at_knot < continuity_range {
                split_indexes.push(idx);
            }
        }

        // Ensure last index is included
        if split_indexes.last() != Some(&(knot_multiplicities.len() - 1)) {
            split_indexes.push(knot_multiplicities.len() - 1);
        }

        // Remove duplicates and sort
        split_indexes.sort_unstable();
        split_indexes.dedup();

        Ok(GeomConvertBSplineCurveKnotSplitting { split_indexes })
    }

    /// Returns the number of points at which the analyzed BSpline curve should be split.
    pub fn nb_splits(&self) -> usize {
        self.split_indexes.len()
    }

    /// Fills the split values table with knot indices.
    pub fn splitting(&self) -> Vec<usize> {
        self.split_indexes.clone()
    }

    /// Returns the split knot of index Index.
    pub fn split_value(&self, index: usize) -> Result<usize, String> {
        if index < 1 || index > self.split_indexes.len() {
            return Err(format!(
                "Index {} out of range [1, {}]",
                index,
                self.split_indexes.len()
            ));
        }
        Ok(self.split_indexes[index - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_split() {
        // Cubic B-spline (degree=3) with knot multiplicities
        // Multiplicities: [1, 1, 1, 1, 3, 1, 1, 1] for a clamped curve
        let mults = vec![1, 1, 1, 1, 3, 1, 1, 1];
        let degree = 3;
        let continuity = 1;

        let splitter = GeomConvertBSplineCurveKnotSplitting::new(&mults, degree, continuity)
            .expect("Failed to create splitter");

        // With continuity=1, we split where continuity < 1
        // At index 4, continuity = 3 - 3 = 0 < 1, so we split
        assert!(splitter.nb_splits() > 1);
    }

    #[test]
    fn test_no_splits_needed() {
        // Simple curve with all multiplicities 1, high continuity requirement
        let mults = vec![1, 1, 1];
        let degree = 3;
        let continuity = 0;

        let splitter = GeomConvertBSplineCurveKnotSplitting::new(&mults, degree, continuity)
            .expect("Failed to create splitter");

        // With continuity=0, we split nowhere (all points have continuity >= 0)
        assert_eq!(splitter.nb_splits(), 2); // Only start and end
    }

    #[test]
    fn test_invalid_continuity() {
        let mults = vec![1, 1, 1];
        let degree = 2;
        let continuity = 5; // Greater than degree

        let result = GeomConvertBSplineCurveKnotSplitting::new(&mults, degree, continuity);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_value_access() {
        let mults = vec![1, 2, 1];
        let degree = 3;
        let continuity = 1;

        let splitter =
            GeomConvertBSplineCurveKnotSplitting::new(&mults, degree, continuity)
                .expect("Failed to create splitter");

        // Verify we can access split values
        let first = splitter.split_value(1).expect("Failed to get split value 1");
        assert_eq!(first, splitter.split_indexes[0]);

        // Out of bounds should fail
        let invalid = splitter.split_value(splitter.nb_splits() + 10);
        assert!(invalid.is_err());
    }
}
