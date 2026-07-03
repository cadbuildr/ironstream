// FILE: b_spl_c_lib_cache_params.rs
// occt: BSplCLib_CacheParams

//! B-spline cache parameters for caching span information during evaluation.
//! Contains parameterization info (degree, periodicity, parameter range)
//! and current span caching data.

/// occt: BSplCLib_CacheParams
///
/// Structure containing parameters describing parameterization of a B-spline curve
/// or surface in one direction (U or V), and data of the current span for caching.
pub struct BSplineCLibCacheParams {
    /// Degree of Bezier/B-spline
    pub degree: i32,

    /// True if the B-spline is periodic
    pub is_periodic: bool,

    /// First valid parameter
    pub first_parameter: f64,

    /// Last valid parameter
    pub last_parameter: f64,

    /// Minimal index of span
    pub span_index_min: i32,

    /// Maximal index of span
    pub span_index_max: i32,

    /// Parameter for the first point of the current span
    pub span_start: f64,

    /// Length of the current span
    pub span_length: f64,

    /// Index of the current span
    pub span_index: i32,
}

impl BSplineCLibCacheParams {
    /// Constructor, prepares data structures for caching.
    ///
    /// # Arguments
    /// * `degree` - degree of the B-spline (or Bezier)
    /// * `is_periodic` - whether the B-spline is periodic
    /// * `flat_knots` - knots of Bezier / B-spline parameterization (1-indexed)
    pub fn new(
        degree: i32,
        is_periodic: bool,
        flat_knots: &[f64],
    ) -> Self {
        let len = flat_knots.len();
        let degree_usize = degree as usize;

        // Convert to 0-indexed for array access
        let first_param = flat_knots[degree_usize];
        let last_param = flat_knots[len - 1 - degree_usize];

        // Span indices: convert from OCCT 1-indexed to 0-indexed
        // OCCT: SpanIndexMin = Lower + degree, SpanIndexMax = Upper - degree - 1
        // With Lower=1, Upper=len (in OCCT), this becomes:
        // 0-indexed: span_index_min = degree, span_index_max = len - degree - 2
        let span_index_min = degree as i32;
        let span_index_max = (len as i32) - degree - 2;

        BSplineCLibCacheParams {
            degree,
            is_periodic,
            first_parameter: first_param,
            last_parameter: last_param,
            span_index_min,
            span_index_max,
            span_start: 0.0,
            span_length: 0.0,
            span_index: 0,
        }
    }

    /// Normalizes the parameter for periodic B-splines.
    /// Wraps the parameter into the valid range [FirstParameter, LastParameter].
    pub fn periodic_normalization(&self, parameter: f64) -> f64 {
        if !self.is_periodic {
            return parameter;
        }

        let period = self.last_parameter - self.first_parameter;

        if parameter < self.first_parameter {
            let scale = ((self.first_parameter - parameter) / period).trunc();
            parameter + period * (scale + 1.0)
        } else if parameter > self.last_parameter {
            let scale = ((parameter - self.last_parameter) / period).trunc();
            parameter - period * (scale + 1.0)
        } else {
            parameter
        }
    }

    /// Verifies validity of the cache using flat parameter of the point.
    /// Returns false if the parameter is outside the current cached span and
    /// we need to relocate.
    pub fn is_cache_valid(&self, parameter: f64) -> bool {
        let new_param = self.periodic_normalization(parameter);
        let delta = new_param - self.span_start;

        // Check if parameter is outside current span boundaries
        if (delta < 0.0 && self.span_index != self.span_index_min)
            || (delta >= self.span_length && self.span_index != self.span_index_max)
        {
            return false;
        }

        if self.span_index == self.span_index_max {
            return true;
        }

        // Check hitting of the next knot within floating point precision
        // This is an approximation of Epsilon logic
        let abs_last = self.last_parameter.abs();
        let abs_param = new_param.abs();
        let min_val = if abs_last < abs_param { abs_last } else { abs_param };
        let eps = min_val * 2.2204460492503131e-16; // machine epsilon approximation

        let delta_to_next = (delta - self.span_length).abs();
        delta_to_next > eps // next knot should be used instead
    }

    /// Computes span for the specified parameter.
    /// Updates span_index, span_start, and span_length based on parameter.
    /// Note: This is a simplified version that sets span data.
    /// Full implementation would use BSplCLib::LocateParameter.
    pub fn locate_parameter(&mut self, parameter: &mut f64, flat_knots: &[f64]) {
        let norm_param = self.periodic_normalization(*parameter);

        // Simplified binary search to find span
        let mut index = self.span_index_min;

        for i in (self.span_index_min as usize)..=(self.span_index_max as usize) {
            if i < flat_knots.len() && flat_knots[i + 1] > norm_param {
                index = i as i32;
                break;
            }
        }

        self.span_index = index;

        let i_usize = index as usize;
        if i_usize < flat_knots.len() && i_usize + 1 < flat_knots.len() {
            self.span_start = flat_knots[i_usize];
            self.span_length = flat_knots[i_usize + 1] - self.span_start;
            *parameter = norm_param;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        // Simple flat knots: [0, 0, 0, 1, 2, 3, 3, 3]
        // For degree 2, first parameter should be knots[2] = 0, last should be knots[5] = 3
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        assert_eq!(cache.degree, 2);
        assert!(!cache.is_periodic);
        assert_eq!(cache.first_parameter, 0.0);
        assert_eq!(cache.last_parameter, 3.0);
        assert_eq!(cache.span_index_min, 2);
        assert_eq!(cache.span_index_max, 4);
    }

    #[test]
    fn test_periodic_normalization_non_periodic() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        // For non-periodic, parameter should pass through unchanged
        assert_eq!(cache.periodic_normalization(1.5), 1.5);
        assert_eq!(cache.periodic_normalization(-0.5), -0.5);
        assert_eq!(cache.periodic_normalization(5.0), 5.0);
    }

    #[test]
    fn test_periodic_normalization_below_range() {
        // Periodic with range [0, 4]
        let flat_knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0];
        let cache = BSplineCLibCacheParams::new(3, true, &flat_knots);

        // Below first parameter, should wrap to higher range
        let param = cache.periodic_normalization(-1.0);
        assert!(param >= cache.first_parameter);
        assert!(param <= cache.last_parameter);
    }

    #[test]
    fn test_periodic_normalization_above_range() {
        let flat_knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0];
        let cache = BSplineCLibCacheParams::new(3, true, &flat_knots);

        // Above last parameter, should wrap to lower range
        let param = cache.periodic_normalization(5.5);
        assert!(param >= cache.first_parameter);
        assert!(param <= cache.last_parameter);
    }

    #[test]
    fn test_is_cache_valid_at_span_start() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let mut cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        cache.span_start = 1.0;
        cache.span_length = 1.0;
        cache.span_index = 3;

        // Parameter at span start should be valid
        assert!(cache.is_cache_valid(1.0));
    }

    #[test]
    fn test_is_cache_valid_within_span() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let mut cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        cache.span_start = 1.0;
        cache.span_length = 1.0;
        cache.span_index = 3;

        // Parameter within span should be valid
        assert!(cache.is_cache_valid(1.5));
    }

    #[test]
    fn test_is_cache_valid_outside_span() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let mut cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        cache.span_start = 1.0;
        cache.span_length = 1.0;
        cache.span_index = 3;

        // Parameter below span should be invalid (unless at min span)
        assert!(!cache.is_cache_valid(0.5));

        // Parameter above span should be invalid (unless at max span)
        assert!(!cache.is_cache_valid(2.5));
    }

    #[test]
    fn test_locate_parameter() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let mut cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        let mut param = 1.5;
        cache.locate_parameter(&mut param, &flat_knots);

        assert!(cache.span_index >= cache.span_index_min);
        assert!(cache.span_index <= cache.span_index_max);
        assert!(cache.span_length > 0.0);
    }

    #[test]
    fn test_constructor_different_degrees() {
        let flat_knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0];

        // Degree 3
        let cache = BSplineCLibCacheParams::new(3, false, &flat_knots);
        assert_eq!(cache.degree, 3);
        assert_eq!(cache.first_parameter, flat_knots[3]);
        assert_eq!(cache.last_parameter, flat_knots[7]);
    }

    #[test]
    fn test_periodic_flag() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];

        let cache_non_periodic = BSplineCLibCacheParams::new(2, false, &flat_knots);
        assert!(!cache_non_periodic.is_periodic);

        let cache_periodic = BSplineCLibCacheParams::new(2, true, &flat_knots);
        assert!(cache_periodic.is_periodic);
    }

    #[test]
    fn test_span_index_min_max() {
        let flat_knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let cache = BSplineCLibCacheParams::new(2, false, &flat_knots);

        // For degree 2 with 8 knots:
        // span_index_min = 2
        // span_index_max = 8 - 2 - 1 = 5
        // But max should be clamped to actual spans
        assert_eq!(cache.span_index_min, 2);
        assert!(cache.span_index_max > cache.span_index_min);
    }
}
