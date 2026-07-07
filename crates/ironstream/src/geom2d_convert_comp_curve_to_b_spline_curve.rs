// FILE: geom2d_convert_comp_curve_to_b_spline_curve.rs
// occt: Geom2dConvert_CompCurveToBSplineCurve

/// Composite curve to B-spline curve converter.
pub struct CompCurveToBSplineCurve {
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    multiplicities: Vec<i32>,
    degree: i32,
    is_periodic: bool,
}

impl CompCurveToBSplineCurve {
    pub fn new() -> Self {
        Self {
            poles: vec![],
            weights: vec![],
            knots: vec![],
            multiplicities: vec![],
            degree: 2,
            is_periodic: false,
        }
    }

    pub fn add_curve(&mut self, poles: Vec<[f64; 2]>, weights: Vec<f64>) {
        self.poles.extend(poles);
        self.weights.extend(weights);
    }

    pub fn degree(&self) -> i32 { self.degree }
    pub fn nb_poles(&self) -> usize { self.poles.len() }
    pub fn nb_knots(&self) -> usize { self.knots.len() }
    pub fn is_periodic(&self) -> bool { self.is_periodic }
    pub fn poles(&self) -> &[[f64; 2]] { &self.poles }
    pub fn weights(&self) -> &[f64] { &self.weights }
    pub fn knots(&self) -> &[f64] { &self.knots }
    pub fn multiplicities(&self) -> &[i32] { &self.multiplicities }
}

impl Default for CompCurveToBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let conv = CompCurveToBSplineCurve::new();
        assert_eq!(conv.degree(), 2);
        assert_eq!(conv.nb_poles(), 0);
        assert!(!conv.is_periodic());
    }

    #[test]
    fn test_add_curve() {
        let mut conv = CompCurveToBSplineCurve::new();
        conv.add_curve(vec![[0.0, 0.0], [1.0, 1.0]], vec![1.0, 1.0]);
        assert_eq!(conv.nb_poles(), 2);
    }
}
