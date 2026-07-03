// FILE: std_prs_deflection_curve.rs
// occt: StdPrs_DeflectionCurve

/// Curve presentation with deflection control.
#[derive(Clone, Debug)]
pub struct DeflectionCurve {
    pub curve_id: u32,
    pub deflection: f64,
    pub point_count: usize,
}

impl DeflectionCurve {
    pub fn new(curve_id: u32, deflection: f64) -> Self {
        Self {
            curve_id,
            deflection: deflection.max(1e-7),
            point_count: 0,
        }
    }

    pub fn set_deflection(&mut self, defl: f64) {
        self.deflection = defl.max(1e-7);
    }

    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    pub fn add_point(&mut self) {
        self.point_count += 1;
    }

    pub fn nb_points(&self) -> usize {
        self.point_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_curve() {
        let curve = DeflectionCurve::new(1, 0.01);
        assert_eq!(curve.curve_id, 1);
        assert_eq!(curve.deflection(), 0.01);
    }

    #[test]
    fn set_deflection() {
        let mut curve = DeflectionCurve::new(1, 0.01);
        curve.set_deflection(0.001);
        assert_eq!(curve.deflection(), 0.001);
    }

    #[test]
    fn add_points() {
        let mut curve = DeflectionCurve::new(1, 0.01);
        for _ in 0..20 {
            curve.add_point();
        }
        assert_eq!(curve.nb_points(), 20);
    }
}
