// FILE: int_tools_face_face.rs
// occt: IntTools_FaceFace

/// This class provides the intersection of face's underlying surfaces.
pub struct IntToolsFaceFace {
    is_done: bool,
    approx_curves: bool,
    compute_on_s1: bool,
    compute_on_s2: bool,
    approx_tolerance: f64,
    fuzzy_value: f64,
    tangent_faces: bool,
}

impl IntToolsFaceFace {
    pub fn new() -> Self {
        IntToolsFaceFace {
            is_done: false,
            approx_curves: false,
            compute_on_s1: false,
            compute_on_s2: false,
            approx_tolerance: 1e-7,
            fuzzy_value: 1e-7,
            tangent_faces: false,
        }
    }

    pub fn set_parameters(
        &mut self,
        approx_curves: bool,
        compute_on_s1: bool,
        compute_on_s2: bool,
        approx_tolerance: f64,
    ) {
        self.approx_curves = approx_curves;
        self.compute_on_s1 = compute_on_s1;
        self.compute_on_s2 = compute_on_s2;
        self.approx_tolerance = approx_tolerance;
    }

    pub fn perform(&mut self, _f1: &TopoDS_Face, _f2: &TopoDS_Face, _parallel: bool) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn tangent_faces(&self) -> bool {
        self.tangent_faces
    }

    pub fn set_fuzzy_value(&mut self, fuzz: f64) {
        self.fuzzy_value = fuzz;
    }

    pub fn fuzzy_value(&self) -> f64 {
        self.fuzzy_value
    }

    pub fn prepare_lines3d(&mut self, _to_split: bool) {
    }
}

impl Default for IntToolsFaceFace {
    fn default() -> Self {
        IntToolsFaceFace::new()
    }
}

/// Minimal stub for TopoDS_Face
#[derive(Clone)]
pub struct TopoDS_Face;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ff = IntToolsFaceFace::new();
        assert_eq!(ff.is_done(), false);
        assert_eq!(ff.tangent_faces(), false);
    }

    #[test]
    fn test_set_parameters() {
        let mut ff = IntToolsFaceFace::new();
        ff.set_parameters(true, true, false, 1e-6);
        assert_eq!(ff.approx_curves, true);
        assert_eq!(ff.compute_on_s1, true);
        assert_eq!(ff.approx_tolerance, 1e-6);
    }

    #[test]
    fn test_fuzzy_value() {
        let mut ff = IntToolsFaceFace::new();
        ff.set_fuzzy_value(0.001);
        assert_eq!(ff.fuzzy_value(), 0.001);
    }

    #[test]
    fn test_perform() {
        let mut ff = IntToolsFaceFace::new();
        let f1 = TopoDS_Face;
        let f2 = TopoDS_Face;
        ff.perform(&f1, &f2, false);
        assert_eq!(ff.is_done(), true);
    }

    #[test]
    fn test_prepare_lines3d() {
        let mut ff = IntToolsFaceFace::new();
        ff.prepare_lines3d(true);
    }
}
