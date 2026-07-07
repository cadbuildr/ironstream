// FILE: step_geom_rational_b_spline_curve.rs
// occt: StepGeom_RationalBSplineCurve

/// Represents a rational B-spline curve
pub struct StepGeomRationalBSplineCurve {
    name: String,
    degree: i32,
    nb_control_points: i32,
    /// Weights for rational B-spline control points
    weights: Vec<f64>,
}

impl StepGeomRationalBSplineCurve {
    pub fn new(name: String, degree: i32, nb_control_points: i32) -> Self {
        StepGeomRationalBSplineCurve {
            name,
            degree,
            nb_control_points,
            weights: vec![1.0; nb_control_points as usize],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn nb_control_points(&self) -> i32 {
        self.nb_control_points
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn set_weight(&mut self, index: usize, weight: f64) {
        if index < self.weights.len() {
            self.weights[index] = weight;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_curve() {
        let curve = StepGeomRationalBSplineCurve::new("Curve1".to_string(), 3, 10);
        assert_eq!(curve.name(), "Curve1");
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.nb_control_points(), 10);
    }

    #[test]
    fn test_weights() {
        let curve = StepGeomRationalBSplineCurve::new("Curve1".to_string(), 3, 5);
        assert_eq!(curve.weights().len(), 5);
        assert_eq!(curve.weights()[0], 1.0);
    }

    #[test]
    fn test_set_weight() {
        let mut curve = StepGeomRationalBSplineCurve::new("Curve1".to_string(), 3, 5);
        curve.set_weight(2, 2.5);
        assert_eq!(curve.weights()[2], 2.5);
    }
}
