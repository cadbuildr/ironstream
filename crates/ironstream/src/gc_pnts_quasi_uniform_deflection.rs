// FILE: gc_pnts_quasi_uniform_deflection.rs
// occt: GCPnts_QuasiUniformDeflection

/// Quasi-uniform deflection-based point sequence on a curve.
pub struct GcPntsQuasiUniformDeflection {
    nb_points: usize,
    deflection: f64,
}

impl GcPntsQuasiUniformDeflection {
    pub fn new() -> Self {
        GcPntsQuasiUniformDeflection {
            nb_points: 0,
            deflection: 0.01,
        }
    }

    pub fn set_deflection(&mut self, defl: f64) {
        if defl > 0.0 {
            self.deflection = defl;
        }
    }

    pub fn perform(&mut self, param_start: f64, param_end: f64) {
        let range = (param_end - param_start).abs();
        self.nb_points = ((range / self.deflection).max(2.0)) as usize;
    }

    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    pub fn parameter(&self, i: usize) -> f64 {
        i as f64 / self.nb_points.max(1) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let defl = GcPntsQuasiUniformDeflection::new();
        assert_eq!(defl.nb_points(), 0);
    }

    #[test]
    fn test_set_deflection() {
        let mut defl = GcPntsQuasiUniformDeflection::new();
        defl.set_deflection(0.05);
        assert!(defl.deflection > 0.0);
    }

    #[test]
    fn test_perform() {
        let mut defl = GcPntsQuasiUniformDeflection::new();
        defl.set_deflection(0.1);
        defl.perform(0.0, 1.0);
        assert!(defl.nb_points() >= 2);
    }
}
