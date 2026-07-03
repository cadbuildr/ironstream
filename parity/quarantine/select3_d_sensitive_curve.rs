// FILE: select3_d_sensitive_curve.rs
// occt: Select3D_SensitiveCurve

/// Stub for Select3D_SensitiveCurve from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_SensitiveCurve {}

impl Select3D_SensitiveCurve {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_SensitiveCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_sensitive_curve_creation() {
        let _obj = Select3D_SensitiveCurve::new();
        let _def = Select3D_SensitiveCurve::default();
    }
}
