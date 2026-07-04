// FILE: step_geom_evaluated_degenerate_pcurve.rs
// occt: StepGeom_EvaluatedDegeneratePcurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint;

#[derive(Clone)]
pub struct EvaluatedDegeneratePcurve {
    name: Arc<String>,
    basis_surface: Option<Arc<Mutex<CartesianPoint>>>,
    degenerate_curve_parameter: f64,
}

impl EvaluatedDegeneratePcurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            basis_surface: None,
            degenerate_curve_parameter: 0.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        basis_surface: Option<Arc<Mutex<CartesianPoint>>>,
        degenerate_curve_parameter: f64,
    ) {
        self.name = Arc::new(name);
        self.basis_surface = basis_surface;
        self.degenerate_curve_parameter = degenerate_curve_parameter;
    }

    pub fn set_basis_surface(&mut self, surf: Arc<Mutex<CartesianPoint>>) {
        self.basis_surface = Some(surf);
    }

    pub fn basis_surface(&self) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.basis_surface.clone()
    }

    pub fn set_degenerate_curve_parameter(&mut self, param: f64) {
        self.degenerate_curve_parameter = param;
    }

    pub fn degenerate_curve_parameter(&self) -> f64 {
        self.degenerate_curve_parameter
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for EvaluatedDegeneratePcurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let edp = EvaluatedDegeneratePcurve::new();
        assert_eq!(edp.degenerate_curve_parameter(), 0.0);
    }
}
