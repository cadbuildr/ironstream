// FILE: step_geom_uniform_curve_and_rational_b_spline_curve.rs
// occt: StepGeom_UniformCurveAndRationalBSplineCurve

pub struct UniformCurveAndRationalBSplineCurve {
    uniform_curve: Option<Box<dyn std::any::Any>>,
    rational_b_spline_curve: Option<Box<dyn std::any::Any>>,
    weights_data: Vec<f64>,
}

impl UniformCurveAndRationalBSplineCurve {
    pub fn new() -> Self {
        UniformCurveAndRationalBSplineCurve {
            uniform_curve: None,
            rational_b_spline_curve: None,
            weights_data: vec![],
        }
    }

    pub fn set_uniform_curve(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.uniform_curve = curve;
    }

    pub fn uniform_curve(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.uniform_curve
    }

    pub fn set_rational_b_spline_curve(&mut self, curve: Option<Box<dyn std::any::Any>>) {
        self.rational_b_spline_curve = curve;
    }

    pub fn rational_b_spline_curve(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.rational_b_spline_curve
    }

    pub fn set_weights_data(&mut self, weights: Vec<f64>) {
        self.weights_data = weights;
    }

    pub fn weights_data(&self) -> &[f64] {
        &self.weights_data
    }

    pub fn weights_data_value(&self, index: usize) -> Option<f64> {
        self.weights_data.get(index).copied()
    }

    pub fn nb_weights_data(&self) -> usize {
        self.weights_data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_curve_and_rational_b_spline_curve_creation() {
        let curve = UniformCurveAndRationalBSplineCurve::new();
        assert_eq!(curve.nb_weights_data(), 0);
    }

    #[test]
    fn test_set_weights_data() {
        let mut curve = UniformCurveAndRationalBSplineCurve::new();
        let weights = vec![1.0, 2.0, 3.0];
        curve.set_weights_data(weights);

        assert_eq!(curve.nb_weights_data(), 3);
        assert_eq!(curve.weights_data_value(0), Some(1.0));
        assert_eq!(curve.weights_data_value(1), Some(2.0));
        assert_eq!(curve.weights_data_value(2), Some(3.0));
        assert_eq!(curve.weights_data_value(3), None);
    }
}
