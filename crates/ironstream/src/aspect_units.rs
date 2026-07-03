// FILE: aspect_units.rs
// occt: Aspect_Units

/// Defines unit conversion constants for length measurements in OCCT.
///
/// Since OCCT version 1.5, the default unit is MILLIMETER.
/// This module provides constants and conversion functions for common units:
/// - Meter (1000 mm)
/// - Centimeter (10 mm)
/// - Millimeter (1 mm, the base unit)

/// Conversion factor: 1 meter = 1000 millimeters
pub const METER_TO_MM: f64 = 1000.0;

/// Conversion factor: 1 centimeter = 10 millimeters
pub const CENTIMETER_TO_MM: f64 = 10.0;

/// Conversion factor: 1 millimeter = 1 millimeter (identity)
pub const MILLIMETER_TO_MM: f64 = 1.0;

/// Converts millimeters to centimeters.
pub fn mm_to_centimeter(value: f64) -> f64 {
    value / CENTIMETER_TO_MM
}

/// Converts centimeters to millimeters.
pub fn centimeter_to_mm(value: f64) -> f64 {
    value * CENTIMETER_TO_MM
}

/// Converts millimeters to meters.
pub fn mm_to_meter(value: f64) -> f64 {
    value / METER_TO_MM
}

/// Converts meters to millimeters.
pub fn meter_to_mm(value: f64) -> f64 {
    value * METER_TO_MM
}

/// Converts millimeters to millimeters (identity function for consistency).
pub fn mm_to_millimeter(value: f64) -> f64 {
    value
}

/// Converts millimeters to millimeters (identity function for consistency).
pub fn millimeter_to_mm(value: f64) -> f64 {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(METER_TO_MM, 1000.0);
        assert_eq!(CENTIMETER_TO_MM, 10.0);
        assert_eq!(MILLIMETER_TO_MM, 1.0);
    }

    #[test]
    fn test_mm_to_centimeter() {
        assert_eq!(mm_to_centimeter(10.0), 1.0);
        assert_eq!(mm_to_centimeter(100.0), 10.0);
        assert_eq!(mm_to_centimeter(0.0), 0.0);
    }

    #[test]
    fn test_centimeter_to_mm() {
        assert_eq!(centimeter_to_mm(1.0), 10.0);
        assert_eq!(centimeter_to_mm(10.0), 100.0);
        assert_eq!(centimeter_to_mm(0.0), 0.0);
    }

    #[test]
    fn test_mm_to_meter() {
        assert_eq!(mm_to_meter(1000.0), 1.0);
        assert_eq!(mm_to_meter(2000.0), 2.0);
        assert_eq!(mm_to_meter(0.0), 0.0);
    }

    #[test]
    fn test_meter_to_mm() {
        assert_eq!(meter_to_mm(1.0), 1000.0);
        assert_eq!(meter_to_mm(2.0), 2000.0);
        assert_eq!(meter_to_mm(0.0), 0.0);
    }

    #[test]
    fn test_mm_to_millimeter() {
        assert_eq!(mm_to_millimeter(5.0), 5.0);
        assert_eq!(mm_to_millimeter(100.0), 100.0);
    }

    #[test]
    fn test_millimeter_to_mm() {
        assert_eq!(millimeter_to_mm(5.0), 5.0);
        assert_eq!(millimeter_to_mm(100.0), 100.0);
    }

    #[test]
    fn test_round_trip_conversions() {
        let original_mm = 123.456;

        let to_cm = mm_to_centimeter(original_mm);
        let back_to_mm = centimeter_to_mm(to_cm);
        assert!((back_to_mm - original_mm).abs() < 1e-10);

        let to_m = mm_to_meter(original_mm);
        let back_to_mm_2 = meter_to_mm(to_m);
        assert!((back_to_mm_2 - original_mm).abs() < 1e-10);
    }

    #[test]
    fn test_fractional_conversions() {
        assert_eq!(mm_to_centimeter(5.5), 0.55);
        assert_eq!(centimeter_to_mm(0.5), 5.0);
        assert_eq!(mm_to_meter(500.0), 0.5);
        assert_eq!(meter_to_mm(0.123), 123.0);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(mm_to_centimeter(-10.0), -1.0);
        assert_eq!(centimeter_to_mm(-1.0), -10.0);
        assert_eq!(mm_to_meter(-1000.0), -1.0);
        assert_eq!(meter_to_mm(-1.0), -1000.0);
    }
}
