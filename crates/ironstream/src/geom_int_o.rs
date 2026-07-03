// FILE: geom_int_o.rs
// occt: GeomInt

/// Static utility class for geometric intersection operations.
/// Provides general purpose tools for surface-surface intersections.
pub struct GeomInt;

impl GeomInt {
    /// Adjusts a parameter to be within a periodic range [theParMin, theParMax]
    /// using the given period.
    ///
    /// # Arguments
    /// * `the_par` - Parameter to adjust
    /// * `the_par_min` - Minimum bound of the range
    /// * `the_par_max` - Maximum bound of the range
    /// * `the_period` - Period value (for periodic parametrization)
    /// * `the_new_par` - Output adjusted parameter
    /// * `the_offset` - Output offset applied
    /// * `the_eps` - Tolerance (default 0.0)
    ///
    /// # Returns
    /// true if adjustment was successful
    pub fn adjust_periodic(
        the_par: f64,
        the_par_min: f64,
        the_par_max: f64,
        the_period: f64,
        the_eps: f64,
    ) -> (bool, f64, f64) {
        let range = the_par_max - the_par_min;
        if the_period <= 0.0 || range <= the_eps {
            return (false, the_par, 0.0);
        }

        let mut new_par = the_par;
        let mut offset = 0.0;

        // Shift parameter to within the range by adding/subtracting the period
        while new_par < the_par_min - the_eps {
            new_par += the_period;
            offset += the_period;
        }
        while new_par > the_par_max + the_eps {
            new_par -= the_period;
            offset -= the_period;
        }

        (true, new_par, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_periodic_in_range() {
        let (success, new_par, offset) =
            GeomInt::adjust_periodic(0.5, 0.0, 1.0, 1.0, 0.0);
        assert!(success);
        assert_eq!(new_par, 0.5);
        assert_eq!(offset, 0.0);
    }

    #[test]
    fn test_adjust_periodic_below_min() {
        let (success, new_par, offset) =
            GeomInt::adjust_periodic(-0.5, 0.0, 1.0, 1.0, 0.0);
        assert!(success);
        assert!(new_par >= 0.0 && new_par <= 1.0);
        assert_eq!(offset, 1.0);
    }

    #[test]
    fn test_adjust_periodic_above_max() {
        let (success, new_par, offset) =
            GeomInt::adjust_periodic(1.5, 0.0, 1.0, 1.0, 0.0);
        assert!(success);
        assert!(new_par >= 0.0 && new_par <= 1.0);
        assert_eq!(offset, -1.0);
    }

    #[test]
    fn test_adjust_periodic_invalid_period() {
        let (success, new_par, offset) =
            GeomInt::adjust_periodic(0.5, 0.0, 1.0, 0.0, 0.0);
        assert!(!success);
        assert_eq!(new_par, 0.5);
        assert_eq!(offset, 0.0);
    }
}
