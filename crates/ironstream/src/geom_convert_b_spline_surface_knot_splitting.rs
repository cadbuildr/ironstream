// FILE: geom_convert_b_spline_surface_knot_splitting.rs
// occt: GeomConvert_BSplineSurfaceKnotSplitting

/// An algorithm to determine isoparametric curves along which a BSpline surface
/// should be split in order to obtain patches of the same continuity.
pub struct GeomConvertBSplineSurfaceKnotSplitting {
    u_split_indexes: Vec<usize>,
    v_split_indexes: Vec<usize>,
}

impl GeomConvertBSplineSurfaceKnotSplitting {
    /// Determines the u- and v-isoparametric curves along which the BSpline surface
    /// should be split in order to obtain patches with the required continuity.
    pub fn new(
        u_knot_multiplicities: &[usize],
        v_knot_multiplicities: &[usize],
        u_degree: usize,
        v_degree: usize,
        u_continuity: usize,
        v_continuity: usize,
    ) -> Result<Self, String> {
        if u_continuity > u_degree {
            return Err("U continuity cannot exceed U degree".to_string());
        }
        if v_continuity > v_degree {
            return Err("V continuity cannot exceed V degree".to_string());
        }

        let u_splits = Self::compute_splits(u_knot_multiplicities, u_degree, u_continuity);
        let v_splits = Self::compute_splits(v_knot_multiplicities, v_degree, v_continuity);

        Ok(GeomConvertBSplineSurfaceKnotSplitting {
            u_split_indexes: u_splits,
            v_split_indexes: v_splits,
        })
    }

    fn compute_splits(
        knot_multiplicities: &[usize],
        degree: usize,
        continuity: usize,
    ) -> Vec<usize> {
        let mut splits = vec![0];

        for (idx, &mult) in knot_multiplicities.iter().enumerate() {
            let continuity_at_knot = if degree >= mult {
                degree - mult
            } else {
                0
            };

            if continuity_at_knot < continuity {
                splits.push(idx);
            }
        }

        if splits.last() != Some(&(knot_multiplicities.len() - 1)) {
            splits.push(knot_multiplicities.len() - 1);
        }

        splits.sort_unstable();
        splits.dedup();
        splits
    }

    /// Returns the number of u-isoparametric curves along which to split.
    pub fn nb_u_splits(&self) -> usize {
        self.u_split_indexes.len()
    }

    /// Returns the number of v-isoparametric curves along which to split.
    pub fn nb_v_splits(&self) -> usize {
        self.v_split_indexes.len()
    }

    /// Fills the U and V split tables with knot indices.
    pub fn splitting(&self) -> (Vec<usize>, Vec<usize>) {
        (self.u_split_indexes.clone(), self.v_split_indexes.clone())
    }

    /// Returns the split knot of index UIndex in the u parametric direction.
    pub fn u_split_value(&self, u_index: usize) -> Result<usize, String> {
        if u_index < 1 || u_index > self.u_split_indexes.len() {
            return Err(format!(
                "UIndex {} out of range [1, {}]",
                u_index,
                self.u_split_indexes.len()
            ));
        }
        Ok(self.u_split_indexes[u_index - 1])
    }

    /// Returns the split knot of index VIndex in the v parametric direction.
    pub fn v_split_value(&self, v_index: usize) -> Result<usize, String> {
        if v_index < 1 || v_index > self.v_split_indexes.len() {
            return Err(format!(
                "VIndex {} out of range [1, {}]",
                v_index,
                self.v_split_indexes.len()
            ));
        }
        Ok(self.v_split_indexes[v_index - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_split_creation() {
        let u_mults = vec![1, 1, 1];
        let v_mults = vec![1, 1, 1];
        let u_degree = 3;
        let v_degree = 3;
        let u_continuity = 1;
        let v_continuity = 1;

        let splitter = GeomConvertBSplineSurfaceKnotSplitting::new(
            &u_mults,
            &v_mults,
            u_degree,
            v_degree,
            u_continuity,
            v_continuity,
        )
        .expect("Failed to create splitter");

        assert!(splitter.nb_u_splits() >= 2);
        assert!(splitter.nb_v_splits() >= 2);
    }

    #[test]
    fn test_invalid_u_continuity() {
        let u_mults = vec![1, 1, 1];
        let v_mults = vec![1, 1, 1];
        let result = GeomConvertBSplineSurfaceKnotSplitting::new(
            &u_mults, &v_mults, 2, 3, 5, 1, // u_continuity > u_degree
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_u_split_value_access() {
        let u_mults = vec![1, 2, 1];
        let v_mults = vec![1, 1];
        let splitter = GeomConvertBSplineSurfaceKnotSplitting::new(&u_mults, &v_mults, 3, 2, 0, 0)
            .expect("Failed to create splitter");

        let first = splitter.u_split_value(1).expect("Failed to get split value");
        assert_eq!(first, 0);

        let invalid = splitter.u_split_value(splitter.nb_u_splits() + 10);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_v_split_value_access() {
        let u_mults = vec![1, 1];
        let v_mults = vec![1, 1, 1];
        let splitter = GeomConvertBSplineSurfaceKnotSplitting::new(&u_mults, &v_mults, 2, 3, 0, 0)
            .expect("Failed to create splitter");

        let first = splitter.v_split_value(1).expect("Failed to get split value");
        assert_eq!(first, 0);
    }
}
