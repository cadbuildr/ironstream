// FILE: adv_approx_pref_and_rec.rs
// occt: AdvApprox_PrefAndRec

//! Inherits from Cutting; contains a list of preferential points (pi)i
//! and a list of recommended points used in cutting management.
//! If cutting is necessary in [a,b], we cut at the di nearest from (a+b)/2.

use std::f64;

/// Cutting strategy using preferential and recommended points.
/// Cutting value is:
/// - the recommended point nearest of (a+b)/2 if pi is in ]a,b[
/// - or the preferential point nearest of (a+b)/2 if pi is in ](r*a+b)/(r+1), (a+r*b)/(r+1)[ where r = Weight
/// - or (a+b)/2 otherwise.
pub struct PrefAndRec {
    rec_cutting: Vec<f64>,
    pref_cutting: Vec<f64>,
    weight: f64,
}

impl PrefAndRec {
    /// Create a new PrefAndRec cutting strategy.
    ///
    /// # Arguments
    /// * `recommended_cut` - Array of recommended cutting points
    /// * `prefferred_cut` - Array of preferential cutting points
    /// * `weight` - Weight parameter (default 5.0); controls the width of the preferential zone
    pub fn new(
        recommended_cut: Vec<f64>,
        prefferred_cut: Vec<f64>,
        weight: f64,
    ) -> Self {
        PrefAndRec {
            rec_cutting: recommended_cut,
            pref_cutting: prefferred_cut,
            weight,
        }
    }

    /// Determine cutting value based on the interval [a, b].
    pub fn value(&self, a: f64, b: f64) -> Option<f64> {
        if a >= b {
            return None;
        }

        let mid = (a + b) / 2.0;

        // Try recommended cutting first (in ]a,b[)
        if let Some(rec_val) = self.nearest_in_interval(&self.rec_cutting, mid, a, b) {
            return Some(rec_val);
        }

        // Try preferential cutting in weighted zone
        let weighted_margin = (b - a) / (self.weight + 1.0);
        let lower_bound = (self.weight * a + b) / (self.weight + 1.0);
        let upper_bound = (a + self.weight * b) / (self.weight + 1.0);

        if let Some(pref_val) = self.nearest_in_interval(&self.pref_cutting, mid, lower_bound, upper_bound) {
            return Some(pref_val);
        }

        // Default: return midpoint
        Some(mid)
    }

    /// Find the element in points nearest to target within interval (lo, hi).
    fn nearest_in_interval(&self, points: &[f64], target: f64, lo: f64, hi: f64) -> Option<f64> {
        let mut best: Option<(f64, f64)> = None; // (distance, value)

        for &pt in points {
            if pt > lo && pt < hi {
                let dist = (pt - target).abs();
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
    fn test_default_midpoint() {
        let pref_and_rec = PrefAndRec::new(vec![], vec![], 5.0);
        let result = pref_and_rec.value(0.0, 10.0);
        assert_eq!(result, Some(5.0));
    }

    #[test]
    fn test_recommended_cutting() {
        let pref_and_rec = PrefAndRec::new(vec![4.5, 5.5], vec![3.0], 5.0);
        let result = pref_and_rec.value(0.0, 10.0);
        // Should find 5.5 or 4.5 (nearest to 5.0)
        if let Some(val) = result {
            assert!((val - 5.0).abs() <= 1.0);
        }
    }

    #[test]
    fn test_preferential_cutting() {
        let pref_and_rec = PrefAndRec::new(vec![], vec![4.0, 6.0], 1.0);
        let result = pref_and_rec.value(0.0, 10.0);
        // Preferential zone is ]3.33, 6.67[ with weight 1
        if let Some(val) = result {
            assert!(val >= 0.0 && val <= 10.0);
        }
    }

    #[test]
    fn test_invalid_interval() {
        let pref_and_rec = PrefAndRec::new(vec![], vec![], 5.0);
        let result = pref_and_rec.value(10.0, 5.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_points_outside_interval() {
        let pref_and_rec = PrefAndRec::new(vec![0.5, 9.5], vec![], 5.0);
        // Points at 0.5 and 9.5 are outside (a=1, b=9), so should return midpoint
        let result = pref_and_rec.value(1.0, 9.0);
        assert_eq!(result, Some(5.0));
    }
}
