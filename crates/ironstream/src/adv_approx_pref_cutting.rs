// FILE: adv_approx_pref_cutting.rs
// occt: AdvApprox_PrefCutting

//! Inherits from Cutting; contains a list of preferential points (di)i.
//! If cutting is necessary in [a,b], we cut at the di nearest from (a+b)/2.

/// Cutting strategy using preferential points.
pub struct PrefCutting {
    pnt_of_cutting: Vec<f64>,
}

impl PrefCutting {
    /// Create a new PrefCutting strategy.
    ///
    /// # Arguments
    /// * `cut_pnts` - Array of preferential cutting points
    pub fn new(cut_pnts: Vec<f64>) -> Self {
        PrefCutting {
            pnt_of_cutting: cut_pnts,
        }
    }

    /// Determine the cutting value in interval [a, b].
    /// Returns the point in cut_pnts nearest to (a+b)/2 that lies within ]a,b[.
    /// If no such point exists, returns None.
    pub fn value(&self, a: f64, b: f64) -> Option<f64> {
        if a >= b {
            return None;
        }

        let mid = (a + b) / 2.0;
        let mut best: Option<(f64, f64)> = None; // (distance, value)

        for &pt in &self.pnt_of_cutting {
            // Point must be strictly inside the interval
            if pt > a && pt < b {
                let dist = (pt - mid).abs();
                if let Some((best_dist, _)) = best {
                    if dist < best_dist {
                        best = Some((dist, pt));
                    }
                } else {
                    best = Some((dist, pt));
                }
            }
        }

        best.map(|(_, val)| val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_preferential_point() {
        let pref_cutting = PrefCutting::new(vec![2.0, 3.0, 7.0, 8.0]);
        let result = pref_cutting.value(0.0, 10.0);
        // Midpoint is 5.0, nearest is either 3.0 or 7.0, both at distance 2.0
        // Should pick the first one found: 3.0
        if let Some(val) = result {
            assert!((val - 5.0).abs() <= 2.0);
        }
    }

    #[test]
    fn test_no_points_in_interval() {
        let pref_cutting = PrefCutting::new(vec![0.0, 10.0]);
        // Points at boundaries are not included (must be strictly inside)
        let result = pref_cutting.value(0.0, 10.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_single_point_in_interval() {
        let pref_cutting = PrefCutting::new(vec![5.0]);
        let result = pref_cutting.value(0.0, 10.0);
        assert_eq!(result, Some(5.0));
    }

    #[test]
    fn test_empty_points() {
        let pref_cutting = PrefCutting::new(vec![]);
        let result = pref_cutting.value(0.0, 10.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_invalid_interval() {
        let pref_cutting = PrefCutting::new(vec![5.0]);
        let result = pref_cutting.value(10.0, 5.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_points_equidistant() {
        let pref_cutting = PrefCutting::new(vec![3.0, 7.0]);
        // Both are distance 2.0 from midpoint 5.0
        let result = pref_cutting.value(0.0, 10.0);
        // Should return one of them (deterministically the first in the vec)
        assert!(result == Some(3.0) || result == Some(7.0));
    }
}
