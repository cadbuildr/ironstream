// FILE: law_interpol.rs
// occt: Law_Interpol
// occt-ref: Law_BSpFunc, Law_Function, Law_Linear

/// Parametric law function: parameter -> value mapping.
// occt-ref: Law_Function
#[derive(Clone, Debug)]
pub struct LawFunction {
    pub param_min: f64,
    pub param_max: f64,
    pub values: Vec<f64>,
}

impl LawFunction {
    pub fn new(param_min: f64, param_max: f64) -> Self {
        Self {
            param_min,
            param_max,
            values: Vec::new(),
        }
    }

    pub fn add_value(&mut self, value: f64) {
        self.values.push(value.max(0.0).min(1.0));
    }

    pub fn value_at(&self, param: f64) -> Option<f64> {
        if param < self.param_min || param > self.param_max || self.values.is_empty() {
            return None;
        }
        let t = (param - self.param_min) / (self.param_max - self.param_min);
        let idx = (t * (self.values.len() - 1) as f64) as usize;
        Some(self.values[idx.min(self.values.len() - 1)])
    }

    pub fn nb_values(&self) -> usize { self.values.len() }
}

impl Default for LawFunction {
    fn default() -> Self {
        Self::new(0.0, 1.0)
    }
}

/// Linear law: slope and intercept.
// occt-ref: Law_Linear
#[derive(Clone, Debug)]
pub struct LawLinear {
    pub slope: f64,
    pub intercept: f64,
}

impl LawLinear {
    pub fn new(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }

    pub fn value_at(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

impl Default for LawLinear {
    fn default() -> Self {
        Self::new(1.0, 0.0)
    }
}

/// B-spline function: parametric curve.
// occt-ref: Law_BSpFunc
#[derive(Clone, Debug)]
pub struct LawBSpFunc {
    pub degree: usize,
    pub knots: Vec<f64>,
    pub coefficients: Vec<f64>,
}

impl LawBSpFunc {
    pub fn new(degree: usize) -> Self {
        Self {
            degree: degree.max(1),
            knots: Vec::new(),
            coefficients: Vec::new(),
        }
    }

    pub fn add_knot(&mut self, knot: f64) {
        self.knots.push(knot);
    }

    pub fn add_coefficient(&mut self, coeff: f64) {
        self.coefficients.push(coeff);
    }

    pub fn is_valid(&self) -> bool {
        self.knots.len() >= 2 * (self.degree + 1)
            && self.coefficients.len() >= self.degree + 1
    }

    pub fn evaluate(&self, param: f64) -> f64 {
        if self.coefficients.is_empty() {
            return 0.0;
        }
        let coeff_idx = ((param * self.coefficients.len() as f64) as usize)
            .min(self.coefficients.len() - 1);
        self.coefficients[coeff_idx]
    }
}

impl Default for LawBSpFunc {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Interpolation law: interpolate through discrete points.
/// occt: Law_Interpol
#[derive(Clone, Debug)]
pub struct LawInterpol {
    pub params: Vec<f64>,
    pub values: Vec<f64>,
}

impl LawInterpol {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_point(&mut self, param: f64, value: f64) {
        self.params.push(param);
        self.values.push(value);
    }

    pub fn interpolate_at(&self, param: f64) -> Option<f64> {
        if self.params.len() < 2 {
            return None;
        }
        for i in 0..self.params.len() - 1 {
            if param >= self.params[i] && param <= self.params[i + 1] {
                let t = (param - self.params[i]) / (self.params[i + 1] - self.params[i]);
                return Some(
                    (1.0 - t) * self.values[i] + t * self.values[i + 1]
                );
            }
        }
        None
    }

    pub fn nb_points(&self) -> usize { self.params.len() }
}

impl Default for LawInterpol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law_function() {
        let mut lf = LawFunction::new(0.0, 1.0);
        lf.add_value(0.0);
        lf.add_value(0.5);
        lf.add_value(1.0);
        assert_eq!(lf.nb_values(), 3);
        assert!(lf.value_at(0.5).is_some());
    }

    #[test]
    fn law_linear() {
        let ll = LawLinear::new(2.0, 1.0);
        assert_eq!(ll.value_at(0.0), 1.0);
        assert_eq!(ll.value_at(1.0), 3.0);
    }

    #[test]
    fn law_bspfunc() {
        let mut bf = LawBSpFunc::new(2);
        for i in 0..6 {
            bf.add_knot(i as f64 / 5.0);
        }
        for i in 0..3 {
            bf.add_coefficient(i as f64 * 0.5);
        }
        assert!(bf.is_valid());
    }

    #[test]
    fn law_interpol() {
        let mut li = LawInterpol::new();
        li.add_point(0.0, 0.0);
        li.add_point(0.5, 1.0);
        li.add_point(1.0, 0.0);
        let val = li.interpolate_at(0.25);
        assert!(val.is_some());
        assert!((val.unwrap() - 0.5).abs() < 0.1);
    }

    #[test]
    fn law_interpol_interpolation() {
        let mut li = LawInterpol::new();
        li.add_point(0.0, 0.0);
        li.add_point(1.0, 1.0);
        let val = li.interpolate_at(0.5);
        assert!(val.is_some());
        assert!((val.unwrap() - 0.5).abs() < 0.01);
    }
}
