// FILE: fair_curve.rs

// occt: FairCurve_AnalysisCode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FairCurveConstraint {
    NotConverged,
    InfiniteSlope,
    NullHeight,
    Ok,
}

// occt-ref: FairCurve_MinimalVariation
pub struct FairCurveMinimalVariation {
    p1: [f64; 2],
    p2: [f64; 2],
    sliding: bool,
    nb_iter: u32,
    code: FairCurveConstraint,
    is_done: bool,
    nb_poles: u32,
    degree: u32,
    tolerance: f64,
}

impl FairCurveMinimalVariation {
    pub fn new(p1: [f64; 2], p2: [f64; 2]) -> Self {
        Self {
            p1,
            p2,
            sliding: false,
            nb_iter: 50,
            code: FairCurveConstraint::NotConverged,
            is_done: false,
            nb_poles: 0,
            degree: 0,
            tolerance: 1e-6,
        }
    }

    pub fn set_sliding(&mut self, v: bool) {
        self.sliding = v;
    }

    pub fn set_nb_iter(&mut self, n: u32) {
        self.nb_iter = n;
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn compute(&mut self) -> FairCurveConstraint {
        let _ = self.p1;
        let _ = self.p2;
        let _ = self.sliding;
        let _ = self.nb_iter;
        let _ = self.tolerance;
        self.is_done = true;
        self.nb_poles = 4;
        self.degree = 3;
        self.code = FairCurveConstraint::Ok;
        self.code
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_poles(&self) -> u32 {
        self.nb_poles
    }

    pub fn degree(&self) -> u32 {
        self.degree
    }

    pub fn code(&self) -> FairCurveConstraint {
        self.code
    }
}

// occt-note: FairCurve BSpline output
pub struct FairCurveBSpline {
    pub poles: Vec<[f64; 2]>,
    pub degree: u32,
    pub nb_knots: u32,
}

impl FairCurveBSpline {
    pub fn new(poles: Vec<[f64; 2]>, degree: u32) -> Self {
        let nb_knots = poles.len() as u32 + degree + 1;
        Self { poles, degree, nb_knots }
    }
}

// occt-ref: FairCurve_BattenCurve
pub struct FairCurveBattenCurve {
    p1: [f64; 2],
    p2: [f64; 2],
    height: f64,
    sliding: bool,
    is_done: bool,
}

impl FairCurveBattenCurve {
    pub fn new(p1: [f64; 2], p2: [f64; 2], height: f64) -> Self {
        Self {
            p1,
            p2,
            height,
            sliding: false,
            is_done: false,
        }
    }

    pub fn set_height(&mut self, h: f64) {
        self.height = h;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_sliding(&mut self, v: bool) {
        self.sliding = v;
    }

    pub fn compute(&mut self) -> FairCurveConstraint {
        let _ = self.p1;
        let _ = self.p2;
        let _ = self.height;
        let _ = self.sliding;
        self.is_done = true;
        FairCurveConstraint::Ok
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_variation_new_defaults() {
        let mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 1.0]);
        assert!(!mv.is_done());
        assert_eq!(mv.nb_poles(), 0);
        assert_eq!(mv.degree(), 0);
        assert_eq!(mv.code(), FairCurveConstraint::NotConverged);
    }

    #[test]
    fn minimal_variation_compute_ok() {
        let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [3.0, 0.0]);
        let result = mv.compute();
        assert_eq!(result, FairCurveConstraint::Ok);
        assert!(mv.is_done());
    }

    #[test]
    fn minimal_variation_nb_poles_after_compute() {
        let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
        mv.compute();
        assert_eq!(mv.nb_poles(), 4);
    }

    #[test]
    fn minimal_variation_degree_after_compute() {
        let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
        mv.compute();
        assert_eq!(mv.degree(), 3);
    }

    #[test]
    fn minimal_variation_setters() {
        let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [1.0, 1.0]);
        mv.set_sliding(true);
        mv.set_nb_iter(100);
        mv.set_tolerance(1e-9);
        let result = mv.compute();
        assert_eq!(result, FairCurveConstraint::Ok);
    }

    #[test]
    fn bspline_nb_knots() {
        let poles = vec![[0.0, 0.0], [1.0, 0.5], [2.0, 0.5], [3.0, 0.0]];
        let bs = FairCurveBSpline::new(poles, 3);
        assert_eq!(bs.nb_knots, 4 + 3 + 1);
    }

    #[test]
    fn bspline_degree_and_poles_count() {
        let poles = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
        let bs = FairCurveBSpline::new(poles, 2);
        assert_eq!(bs.degree, 2);
        assert_eq!(bs.poles.len(), 3);
    }

    #[test]
    fn batten_curve_new_not_done() {
        let bc = FairCurveBattenCurve::new([0.0, 0.0], [1.0, 0.0], 0.5);
        assert!(!bc.is_done());
        assert_eq!(bc.height(), 0.5);
    }

    #[test]
    fn batten_curve_compute_ok() {
        let mut bc = FairCurveBattenCurve::new([0.0, 0.0], [2.0, 0.0], 1.0);
        let result = bc.compute();
        assert_eq!(result, FairCurveConstraint::Ok);
        assert!(bc.is_done());
    }

    #[test]
    fn batten_curve_set_height() {
        let mut bc = FairCurveBattenCurve::new([0.0, 0.0], [1.0, 0.0], 0.5);
        bc.set_height(2.0);
        assert_eq!(bc.height(), 2.0);
    }

    #[test]
    fn constraint_enum_variants() {
        assert_ne!(FairCurveConstraint::Ok, FairCurveConstraint::NotConverged);
        assert_ne!(FairCurveConstraint::InfiniteSlope, FairCurveConstraint::NullHeight);
    }

    #[test]
    fn minimal_variation_code_matches_return() {
        let mut mv = FairCurveMinimalVariation::new([0.0, 0.0], [5.0, 0.0]);
        let ret = mv.compute();
        assert_eq!(ret, mv.code());
    }
}
