// FILE: standard_o.rs
// occt: Standard

/// Global Standard utilities.
pub struct Standard;

impl Standard {
    pub const PI: f64 = std::f64::consts::PI;
    pub const PI_2: f64 = std::f64::consts::FRAC_PI_2;
    pub const PI_4: f64 = std::f64::consts::FRAC_PI_4;
    pub const E: f64 = std::f64::consts::E;

    pub fn epsilon() -> f64 {
        f64::EPSILON
    }

    pub fn max_value() -> f64 {
        f64::MAX
    }

    pub fn min_value() -> f64 {
        f64::MIN_POSITIVE
    }
}
