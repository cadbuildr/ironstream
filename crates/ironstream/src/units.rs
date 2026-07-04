// FILE: units.rs
// occt: Units

/// Unit system utilities.
pub struct Units;

impl Units {
    // Length units in millimeters
    pub const MM: f64 = 1.0;
    pub const CM: f64 = 10.0;
    pub const M: f64 = 1000.0;
    pub const INCH: f64 = 25.4;
    pub const FOOT: f64 = 304.8;

    // Angle units in radians
    pub const RAD: f64 = 1.0;
    pub const DEG: f64 = std::f64::consts::PI / 180.0;

    pub fn mm_to_m(val: f64) -> f64 {
        val / 1000.0
    }

    pub fn m_to_mm(val: f64) -> f64 {
        val * 1000.0
    }

    pub fn deg_to_rad(val: f64) -> f64 {
        val * std::f64::consts::PI / 180.0
    }

    pub fn rad_to_deg(val: f64) -> f64 {
        val * 180.0 / std::f64::consts::PI
    }
}
