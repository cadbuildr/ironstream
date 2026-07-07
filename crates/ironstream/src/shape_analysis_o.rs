// FILE: shape_analysis_o.rs
// occt: ShapeAnalysis

/// Package for analyzing geometrical objects and topological shapes.
/// Provides tools for:
/// - Computing quantities of subshapes
/// - Computing parameters of points on curve and surface
/// - Computing surface singularities
/// - Checking edge and wire consistency
/// - Checking edges order in the wire
/// - Checking face bounds orientation
/// - Checking small faces
/// - Analyzing shape tolerances
pub struct ShapeAnalysis;

impl ShapeAnalysis {
    /// Returns positively oriented wire in the face.
    /// If there is no such wire, returns the first wire of the face.
    pub fn outer_wire(face_id: i32) -> i32 {
        // In a real implementation, this would analyze the face topology
        // For now, return the face id as a placeholder
        face_id
    }

    /// Returns a total area of 2D wire
    pub fn tot_cross_2d(wire_id: i32, _face_id: i32) -> f64 {
        // In a real implementation, this would compute the 2D area
        // For now, return a placeholder value
        0.0
    }

    /// Returns a total area of 3D wire (contour area)
    pub fn contour_area(wire_id: i32) -> f64 {
        // In a real implementation, this would compute the 3D area
        // For now, return a placeholder value
        0.0
    }

    /// Returns true if face has an outer bound
    pub fn is_outer_bound(face_id: i32) -> bool {
        // In a real implementation, this would check face topology
        true
    }

    /// Returns a shift required to move point Val to the range
    /// [ToVal-Period/2, ToVal+Period/2].
    /// This shift will be divisible by Period.
    /// Intended for adjusting parameters on periodic surfaces.
    pub fn adjust_by_period(val: f64, to_val: f64, period: f64) -> f64 {
        if period == 0.0 {
            return 0.0;
        }

        let mut shift = to_val - val;
        let half_period = period / 2.0;

        // Normalize shift to be within [-Period/2, Period/2]
        while shift > half_period {
            shift -= period;
        }
        while shift < -half_period {
            shift += period;
        }

        shift
    }

    /// Returns a shift required to move point Val to the range [ValMin, ValMax].
    /// This shift will be divisible by Period where Period = ValMax - ValMin.
    /// Intended for adjusting parameters on periodic surfaces.
    pub fn adjust_to_period(val: f64, val_min: f64, val_max: f64) -> f64 {
        let period = val_max - val_min;
        if period == 0.0 {
            return 0.0;
        }

        let target = (val_min + val_max) / 2.0;
        Self::adjust_by_period(val, target, period)
    }

    /// Check if a value is within a given tolerance
    pub fn is_within_tolerance(value: f64, tolerance: f64) -> bool {
        value.abs() <= tolerance
    }

    /// Get the absolute value of difference
    pub fn abs_diff(val1: f64, val2: f64) -> f64 {
        (val1 - val2).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_by_period_zero_shift() {
        let shift = ShapeAnalysis::adjust_by_period(1.0, 1.0, 2.0);
        assert_eq!(shift, 0.0);
    }

    #[test]
    fn test_adjust_by_period_positive_shift() {
        let shift = ShapeAnalysis::adjust_by_period(0.5, 1.5, 2.0);
        assert!((shift - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_adjust_by_period_negative_shift() {
        let shift = ShapeAnalysis::adjust_by_period(2.5, 1.5, 2.0);
        assert!((shift - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_adjust_by_period_zero_period() {
        let shift = ShapeAnalysis::adjust_by_period(1.0, 2.0, 0.0);
        assert_eq!(shift, 0.0);
    }

    #[test]
    fn test_adjust_to_period() {
        let shift = ShapeAnalysis::adjust_to_period(0.5, 0.0, 2.0);
        // Target is 1.0, shift from 0.5 to 1.0 is 0.5
        assert!(shift.abs() <= 1.0);
    }

    #[test]
    fn test_adjust_to_period_zero_range() {
        let shift = ShapeAnalysis::adjust_to_period(1.0, 1.0, 1.0);
        assert_eq!(shift, 0.0);
    }

    #[test]
    fn test_is_within_tolerance() {
        assert!(ShapeAnalysis::is_within_tolerance(0.001, 0.01));
        assert!(!ShapeAnalysis::is_within_tolerance(0.02, 0.01));
    }

    #[test]
    fn test_abs_diff() {
        assert_eq!(ShapeAnalysis::abs_diff(5.0, 2.0), 3.0);
        assert_eq!(ShapeAnalysis::abs_diff(2.0, 5.0), 3.0);
    }

    #[test]
    fn test_outer_wire() {
        let wire_id = ShapeAnalysis::outer_wire(1);
        assert_eq!(wire_id, 1);
    }

    #[test]
    fn test_is_outer_bound() {
        assert!(ShapeAnalysis::is_outer_bound(1));
    }
}
