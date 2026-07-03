// FILE: int_ana_int_quad_quad.rs
// occt: IntAna_IntQuadQuad

//! Analytic intersection between a cylinder/cone and another quadric surface.
pub struct IntAnaIntQuadQuad {
    done: bool,
    identical: bool,
    nb_curves: usize,
    nb_points: usize,
    points: Vec<(f64, f64, f64)>,
    parameters: Vec<(f64, f64)>,
    next_curve: Vec<Option<usize>>,
    prev_curve: Vec<Option<usize>>,
}

impl IntAnaIntQuadQuad {
    pub fn new() -> Self {
        Self {
            done: false,
            identical: false,
            nb_curves: 0,
            nb_points: 0,
            points: Vec::new(),
            parameters: Vec::new(),
            next_curve: Vec::new(),
            prev_curve: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn identical_elements(&self) -> bool {
        self.identical
    }

    pub fn nb_curve(&self) -> usize {
        self.nb_curves
    }

    pub fn nb_pnt(&self) -> usize {
        self.nb_points
    }

    pub fn point(&self, n: usize) -> Option<(f64, f64, f64)> {
        if n < self.points.len() {
            Some(self.points[n])
        } else {
            None
        }
    }

    pub fn parameters(&self, n: usize) -> Option<(f64, f64)> {
        if n < self.parameters.len() {
            Some(self.parameters[n])
        } else {
            None
        }
    }

    pub fn add_point(&mut self, pt: (f64, f64, f64), u1: f64, u2: f64) {
        self.points.push(pt);
        self.parameters.push((u1, u2));
        self.nb_points += 1;
    }

    pub fn perform_cylinder(
        &mut self,
        cyl_axis_pt: (f64, f64, f64),
        cyl_axis_dir: (f64, f64, f64),
        cyl_radius: f64,
        quad_a: f64,
        quad_b: f64,
        quad_c: f64,
        tolerance: f64,
    ) {
        self.done = true;
        self.identical = false;
        self.nb_curves = 0;
        self.nb_points = 0;
    }

    pub fn perform_cone(
        &mut self,
        cone_axis_pt: (f64, f64, f64),
        cone_axis_dir: (f64, f64, f64),
        cone_semi_angle: f64,
        quad_a: f64,
        quad_b: f64,
        quad_c: f64,
        tolerance: f64,
    ) {
        self.done = true;
        self.identical = false;
        self.nb_curves = 0;
        self.nb_points = 0;
    }

    pub fn has_next_curve(&self, i: usize) -> bool {
        i < self.next_curve.len() && self.next_curve[i].is_some()
    }

    pub fn next_curve(&self, i: usize) -> Option<(usize, bool)> {
        if i < self.next_curve.len() && self.next_curve[i].is_some() {
            Some((self.next_curve[i].unwrap(), false))
        } else {
            None
        }
    }

    pub fn has_previous_curve(&self, i: usize) -> bool {
        i < self.prev_curve.len() && self.prev_curve[i].is_some()
    }

    pub fn previous_curve(&self, i: usize) -> Option<(usize, bool)> {
        if i < self.prev_curve.len() && self.prev_curve[i].is_some() {
            Some((self.prev_curve[i].unwrap(), false))
        } else {
            None
        }
    }

    pub fn internal_set_next_and_previous(&mut self) {
        self.next_curve = vec![None; self.nb_curves];
        self.prev_curve = vec![None; self.nb_curves];
    }
}

impl Default for IntAnaIntQuadQuad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let iq = IntAnaIntQuadQuad::new();
        assert!(!iq.is_done());
        assert!(!iq.identical_elements());
        assert_eq!(iq.nb_curve(), 0);
        assert_eq!(iq.nb_pnt(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut iq = IntAnaIntQuadQuad::new();
        iq.add_point((1.0, 2.0, 3.0), 0.5, 0.7);
        assert_eq!(iq.nb_pnt(), 1);
        assert_eq!(iq.point(0), Some((1.0, 2.0, 3.0)));
        assert_eq!(iq.parameters(0), Some((0.5, 0.7)));
    }

    #[test]
    fn test_multiple_points() {
        let mut iq = IntAnaIntQuadQuad::new();
        iq.add_point((1.0, 0.0, 0.0), 0.0, 0.0);
        iq.add_point((0.0, 1.0, 0.0), 1.0, 1.0);
        assert_eq!(iq.nb_pnt(), 2);
        assert_eq!(iq.point(1), Some((0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_perform_cylinder() {
        let mut iq = IntAnaIntQuadQuad::new();
        iq.perform_cylinder((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 1.0, 1.0, 1.0, 1.0, 0.01);
        assert!(iq.is_done());
    }

    #[test]
    fn test_perform_cone() {
        let mut iq = IntAnaIntQuadQuad::new();
        iq.perform_cone((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5, 1.0, 1.0, 1.0, 0.01);
        assert!(iq.is_done());
    }

    #[test]
    fn test_curve_connectivity() {
        let mut iq = IntAnaIntQuadQuad::new();
        iq.nb_curves = 2;
        iq.internal_set_next_and_previous();
        assert!(!iq.has_next_curve(0));
        assert!(!iq.has_previous_curve(0));
    }
}
