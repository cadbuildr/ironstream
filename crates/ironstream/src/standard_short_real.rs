// FILE: standard_short_real.rs
// occt: Standard_ShortReal

/// Short real (32-bit float) utilities.
pub struct ShortReal;

impl ShortReal {
    pub const MIN: f32 = f32::MIN;
    pub const MAX: f32 = f32::MAX;
    pub const EPSILON: f32 = f32::EPSILON;
    pub const PI: f32 = std::f32::consts::PI;

    pub fn epsilon() -> f32 {
        f32::EPSILON
    }

    pub fn max_value() -> f32 {
        f32::MAX
    }

    pub fn min_value() -> f32 {
        f32::MIN_POSITIVE
    }
}
