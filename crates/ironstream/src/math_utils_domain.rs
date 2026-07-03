// FILE: math_utils_domain.rs
// occt: MathUtils_Domain

//! Parameter domain types for curves and surfaces.
//!
//! Provides lightweight value types for representing 1D and 2D parameter domains
//! with utility methods for bounds checking, clamping, and domain analysis.

/// 1D parameter domain for curves.
///
/// Represents a parameter range [min, max] with utility methods for:
/// - Bounds checking (contains)
/// - Parameter clamping (clamp)
/// - Domain analysis (is_large, is_full_period)
///
/// This is a lightweight value type designed for efficiency.
/// All methods are inline and use simple arithmetic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Domain1D {
    /// Lower bound of the domain
    pub min: f64,
    /// Upper bound of the domain
    pub max: f64,
}

impl Domain1D {
    /// Default constructor - creates empty domain [0, 0].
    #[inline]
    pub const fn new() -> Self {
        Self { min: 0.0, max: 0.0 }
    }

    /// Construct from bounds.
    ///
    /// # Arguments
    /// * `min` - lower bound
    /// * `max` - upper bound
    #[inline]
    pub const fn from_bounds(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Returns the length of the domain.
    #[inline]
    pub const fn length(&self) -> f64 {
        self.max - self.min
    }

    /// Returns the midpoint of the domain.
    #[inline]
    pub const fn mid(&self) -> f64 {
        (self.min + self.max) * 0.5
    }

    /// Check if value is within domain.
    ///
    /// # Arguments
    /// * `u` - parameter value to check
    /// * `tol` - tolerance for boundary check (default 0.0)
    ///
    /// # Returns
    /// true if u is in [min - tol, max + tol]
    #[inline]
    pub fn contains(&self, u: f64, tol: f64) -> bool {
        u >= self.min - tol && u <= self.max + tol
    }

    /// Clamp value to domain bounds.
    ///
    /// # Arguments
    /// * `u` - parameter value to clamp
    ///
    /// # Returns
    /// clamped value in [min, max]
    #[inline]
    pub fn clamp(&self, u: f64) -> f64 {
        if u < self.min {
            self.min
        } else if u > self.max {
            self.max
        } else {
            u
        }
    }

    /// Check if domain is "large" (effectively unbounded for optimization).
    /// Large domains allow skipping bounds checking for performance.
    ///
    /// # Arguments
    /// * `threshold` - size threshold (default 1000)
    ///
    /// # Returns
    /// true if length() > threshold
    #[inline]
    pub fn is_large(&self, threshold: f64) -> bool {
        self.length() > threshold
    }

    /// Check if domain covers a full periodic range.
    ///
    /// # Arguments
    /// * `period` - period of the parameter (e.g., 2*PI for angles)
    /// * `tol` - tolerance (default 1.0e-10)
    ///
    /// # Returns
    /// true if domain covers at least one full period
    #[inline]
    pub fn is_full_period(&self, period: f64, tol: f64) -> bool {
        self.length() >= period - tol
    }

    /// Interpolate within domain.
    ///
    /// # Arguments
    /// * `t` - interpolation parameter in [0, 1]
    ///
    /// # Returns
    /// min + t * length()
    #[inline]
    pub const fn lerp(&self, t: f64) -> f64 {
        self.min + t * (self.max - self.min)
    }

    /// Normalize parameter to [0, 1] range.
    ///
    /// # Arguments
    /// * `u` - parameter value
    ///
    /// # Returns
    /// (u - min) / length(), or 0.5 if length() == 0
    #[inline]
    pub fn normalize(&self, u: f64) -> f64 {
        let len = self.length();
        if len > 0.0 {
            (u - self.min) / len
        } else {
            0.5
        }
    }

    /// Check if domain has finite bounds (not effectively infinite).
    ///
    /// # Arguments
    /// * `inf_limit` - threshold for "infinity" (default 1e100)
    ///
    /// # Returns
    /// true if both min and max are within finite range
    #[inline]
    pub fn is_finite(&self, inf_limit: f64) -> bool {
        self.min > -inf_limit && self.max < inf_limit
    }

    /// Check if this domain equals another within tolerance.
    ///
    /// # Arguments
    /// * `other` - domain to compare with
    /// * `tol` - tolerance for comparison (default 1.0e-10)
    ///
    /// # Returns
    /// true if domains are equal within tolerance
    #[inline]
    pub fn is_equal(&self, other: &Domain1D, tol: f64) -> bool {
        (self.min - other.min).abs() <= tol && (self.max - other.max).abs() <= tol
    }
}

impl Default for Domain1D {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// 2D parameter domain for surfaces.
///
/// Represents a rectangular parameter domain [umin, umax] x [vmin, vmax]
/// with utility methods for:
/// - Bounds checking (contains)
/// - Parameter clamping (clamp)
/// - Domain analysis (is_large, is_full_period)
/// - Access to U and V subdomains
///
/// This is a lightweight value type designed for efficiency.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Domain2D {
    /// Lower U bound
    pub umin: f64,
    /// Upper U bound
    pub umax: f64,
    /// Lower V bound
    pub vmin: f64,
    /// Upper V bound
    pub vmax: f64,
}

impl Domain2D {
    /// Default constructor - creates empty domain.
    #[inline]
    pub const fn new() -> Self {
        Self {
            umin: 0.0,
            umax: 0.0,
            vmin: 0.0,
            vmax: 0.0,
        }
    }

    /// Construct from bounds.
    ///
    /// # Arguments
    /// * `umin` - lower U bound
    /// * `umax` - upper U bound
    /// * `vmin` - lower V bound
    /// * `vmax` - upper V bound
    #[inline]
    pub const fn from_bounds(umin: f64, umax: f64, vmin: f64, vmax: f64) -> Self {
        Self { umin, umax, vmin, vmax }
    }

    /// Construct from two 1D domains.
    ///
    /// # Arguments
    /// * `u_domain` - U parameter domain
    /// * `v_domain` - V parameter domain
    #[inline]
    pub const fn from_domains(u_domain: Domain1D, v_domain: Domain1D) -> Self {
        Self {
            umin: u_domain.min,
            umax: u_domain.max,
            vmin: v_domain.min,
            vmax: v_domain.max,
        }
    }

    /// Returns the U subdomain.
    #[inline]
    pub const fn u(&self) -> Domain1D {
        Domain1D {
            min: self.umin,
            max: self.umax,
        }
    }

    /// Returns the V subdomain.
    #[inline]
    pub const fn v(&self) -> Domain1D {
        Domain1D {
            min: self.vmin,
            max: self.vmax,
        }
    }

    /// Returns U length.
    #[inline]
    pub const fn u_length(&self) -> f64 {
        self.umax - self.umin
    }

    /// Returns V length.
    #[inline]
    pub const fn v_length(&self) -> f64 {
        self.vmax - self.vmin
    }

    /// Returns U midpoint.
    #[inline]
    pub const fn u_mid(&self) -> f64 {
        (self.umin + self.umax) * 0.5
    }

    /// Returns V midpoint.
    #[inline]
    pub const fn v_mid(&self) -> f64 {
        (self.vmin + self.vmax) * 0.5
    }

    /// Check if UV point is within domain.
    ///
    /// # Arguments
    /// * `u` - U parameter
    /// * `v` - V parameter
    /// * `tol` - tolerance for boundary check (default 0.0)
    ///
    /// # Returns
    /// true if (u, v) is in domain
    #[inline]
    pub fn contains(&self, u: f64, v: f64, tol: f64) -> bool {
        u >= self.umin - tol
            && u <= self.umax + tol
            && v >= self.vmin - tol
            && v <= self.vmax + tol
    }

    /// Clamp UV to domain bounds.
    ///
    /// # Arguments
    /// * `u` - U parameter (modified in place)
    /// * `v` - V parameter (modified in place)
    #[inline]
    pub fn clamp(&self, u: &mut f64, v: &mut f64) {
        *u = if *u < self.umin {
            self.umin
        } else if *u > self.umax {
            self.umax
        } else {
            *u
        };

        *v = if *v < self.vmin {
            self.vmin
        } else if *v > self.vmax {
            self.vmax
        } else {
            *v
        };
    }

    /// Check if both U and V domains are "large".
    ///
    /// # Arguments
    /// * `threshold` - size threshold (default 1000)
    ///
    /// # Returns
    /// true if both U and V lengths exceed threshold
    #[inline]
    pub fn is_large(&self, threshold: f64) -> bool {
        self.u_length() > threshold && self.v_length() > threshold
    }

    /// Check if U domain covers a full period.
    ///
    /// # Arguments
    /// * `period` - period of U parameter
    /// * `tol` - tolerance (default 1.0e-10)
    #[inline]
    pub fn is_u_full_period(&self, period: f64, tol: f64) -> bool {
        self.u_length() >= period - tol
    }

    /// Check if V domain covers a full period.
    ///
    /// # Arguments
    /// * `period` - period of V parameter
    /// * `tol` - tolerance (default 1.0e-10)
    #[inline]
    pub fn is_v_full_period(&self, period: f64, tol: f64) -> bool {
        self.v_length() >= period - tol
    }

    /// Check if domain has finite bounds (not effectively infinite).
    ///
    /// # Arguments
    /// * `inf_limit` - threshold for "infinity" (default 1e100)
    #[inline]
    pub fn is_finite(&self, inf_limit: f64) -> bool {
        self.umin > -inf_limit
            && self.umax < inf_limit
            && self.vmin > -inf_limit
            && self.vmax < inf_limit
    }
}

impl Default for Domain2D {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain1d_construction() {
        let d = Domain1D::from_bounds(0.0, 10.0);
        assert_eq!(d.min, 0.0);
        assert_eq!(d.max, 10.0);
    }

    #[test]
    fn test_domain1d_length() {
        let d = Domain1D::from_bounds(2.0, 7.0);
        assert_eq!(d.length(), 5.0);
    }

    #[test]
    fn test_domain1d_mid() {
        let d = Domain1D::from_bounds(0.0, 10.0);
        assert_eq!(d.mid(), 5.0);
    }

    #[test]
    fn test_domain1d_contains() {
        let d = Domain1D::from_bounds(0.0, 10.0);
        assert!(d.contains(5.0, 0.0));
        assert!(d.contains(0.0, 0.0));
        assert!(d.contains(10.0, 0.0));
        assert!(!d.contains(11.0, 0.0));
        assert!(!d.contains(-1.0, 0.0));
        assert!(d.contains(10.5, 1.0)); // with tolerance
    }

    #[test]
    fn test_domain1d_clamp() {
        let d = Domain1D::from_bounds(0.0, 10.0);
        assert_eq!(d.clamp(5.0), 5.0);
        assert_eq!(d.clamp(-5.0), 0.0);
        assert_eq!(d.clamp(15.0), 10.0);
    }

    #[test]
    fn test_domain1d_is_large() {
        let small = Domain1D::from_bounds(0.0, 100.0);
        let large = Domain1D::from_bounds(0.0, 2000.0);

        assert!(!small.is_large(1000.0));
        assert!(large.is_large(1000.0));
    }

    #[test]
    fn test_domain1d_is_full_period() {
        let period = std::f64::consts::PI * 2.0; // ~6.283

        let small = Domain1D::from_bounds(0.0, period * 0.9);
        assert!(!small.is_full_period(period, 1e-10));

        let full = Domain1D::from_bounds(0.0, period);
        assert!(full.is_full_period(period, 1e-10));
    }

    #[test]
    fn test_domain1d_lerp() {
        let d = Domain1D::from_bounds(2.0, 12.0);
        assert_eq!(d.lerp(0.0), 2.0);
        assert_eq!(d.lerp(1.0), 12.0);
        assert_eq!(d.lerp(0.5), 7.0);
    }

    #[test]
    fn test_domain1d_normalize() {
        let d = Domain1D::from_bounds(0.0, 10.0);
        assert_eq!(d.normalize(0.0), 0.0);
        assert_eq!(d.normalize(10.0), 1.0);
        assert_eq!(d.normalize(5.0), 0.5);
    }

    #[test]
    fn test_domain1d_normalize_zero_length() {
        let d = Domain1D::from_bounds(5.0, 5.0);
        assert_eq!(d.normalize(5.0), 0.5); // returns default midpoint
    }

    #[test]
    fn test_domain1d_is_finite() {
        let finite = Domain1D::from_bounds(0.0, 10.0);
        assert!(finite.is_finite(1e100));

        let infinite = Domain1D::from_bounds(-1e101, 1e101);
        assert!(!infinite.is_finite(1e100));
    }

    #[test]
    fn test_domain1d_is_equal() {
        let d1 = Domain1D::from_bounds(0.0, 10.0);
        let d2 = Domain1D::from_bounds(0.0, 10.0);
        let d3 = Domain1D::from_bounds(0.0 + 1e-11, 10.0);

        assert!(d1.is_equal(&d2, 1e-10));
        assert!(d1.is_equal(&d3, 1e-10));
    }

    #[test]
    fn test_domain2d_construction() {
        let d = Domain2D::from_bounds(0.0, 10.0, 1.0, 11.0);
        assert_eq!(d.umin, 0.0);
        assert_eq!(d.umax, 10.0);
        assert_eq!(d.vmin, 1.0);
        assert_eq!(d.vmax, 11.0);
    }

    #[test]
    fn test_domain2d_from_domains() {
        let u = Domain1D::from_bounds(0.0, 10.0);
        let v = Domain1D::from_bounds(2.0, 8.0);
        let d = Domain2D::from_domains(u, v);

        assert_eq!(d.umin, 0.0);
        assert_eq!(d.umax, 10.0);
        assert_eq!(d.vmin, 2.0);
        assert_eq!(d.vmax, 8.0);
    }

    #[test]
    fn test_domain2d_u_v() {
        let d = Domain2D::from_bounds(0.0, 10.0, 2.0, 8.0);
        let u = d.u();
        let v = d.v();

        assert_eq!(u.min, 0.0);
        assert_eq!(u.max, 10.0);
        assert_eq!(v.min, 2.0);
        assert_eq!(v.max, 8.0);
    }

    #[test]
    fn test_domain2d_lengths() {
        let d = Domain2D::from_bounds(0.0, 10.0, 2.0, 8.0);
        assert_eq!(d.u_length(), 10.0);
        assert_eq!(d.v_length(), 6.0);
    }

    #[test]
    fn test_domain2d_mids() {
        let d = Domain2D::from_bounds(0.0, 10.0, 2.0, 8.0);
        assert_eq!(d.u_mid(), 5.0);
        assert_eq!(d.v_mid(), 5.0);
    }

    #[test]
    fn test_domain2d_contains() {
        let d = Domain2D::from_bounds(0.0, 10.0, 0.0, 5.0);
        assert!(d.contains(5.0, 2.5, 0.0));
        assert!(!d.contains(11.0, 2.5, 0.0));
        assert!(!d.contains(5.0, 6.0, 0.0));
    }

    #[test]
    fn test_domain2d_clamp() {
        let d = Domain2D::from_bounds(0.0, 10.0, 2.0, 8.0);
        let mut u = -5.0;
        let mut v = 10.0;

        d.clamp(&mut u, &mut v);

        assert_eq!(u, 0.0);
        assert_eq!(v, 8.0);
    }

    #[test]
    fn test_domain2d_is_large() {
        let small = Domain2D::from_bounds(0.0, 500.0, 0.0, 500.0);
        let large = Domain2D::from_bounds(0.0, 2000.0, 0.0, 2000.0);

        assert!(!small.is_large(1000.0));
        assert!(large.is_large(1000.0));
    }

    #[test]
    fn test_domain2d_is_full_period() {
        let period = std::f64::consts::PI * 2.0;

        let d1 = Domain2D::from_bounds(0.0, period, 0.0, period);
        assert!(d1.is_u_full_period(period, 1e-10));
        assert!(d1.is_v_full_period(period, 1e-10));

        let d2 = Domain2D::from_bounds(0.0, period * 0.5, 0.0, period * 0.5);
        assert!(!d2.is_u_full_period(period, 1e-10));
    }

    #[test]
    fn test_domain2d_is_finite() {
        let finite = Domain2D::from_bounds(0.0, 10.0, 0.0, 10.0);
        assert!(finite.is_finite(1e100));

        let infinite = Domain2D::from_bounds(-1e101, 1e101, 0.0, 10.0);
        assert!(!infinite.is_finite(1e100));
    }

    #[test]
    fn test_domain1d_default() {
        let d = Domain1D::default();
        assert_eq!(d.min, 0.0);
        assert_eq!(d.max, 0.0);
    }

    #[test]
    fn test_domain2d_default() {
        let d = Domain2D::default();
        assert_eq!(d.umin, 0.0);
        assert_eq!(d.umax, 0.0);
        assert_eq!(d.vmin, 0.0);
        assert_eq!(d.vmax, 0.0);
    }
}
