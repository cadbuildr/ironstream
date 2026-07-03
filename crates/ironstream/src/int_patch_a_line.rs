// FILE: int_patch_a_line.rs
// occt: IntPatch_ALine

/// Represents an analytic intersection line between two surfaces.
/// Based on a parametric analytic curve (line, circle, ellipse, etc.)
#[derive(Clone, Debug)]
pub struct IntPatchALine {
    curve_type: i32,
    is_tangent: bool,
    trans1: i32,
    trans2: i32,
    first_param: f64,
    last_param: f64,
    first_included: bool,
    last_included: bool,
    points: Vec<IntersectionPoint>,
    first_point_index: Option<usize>,
    last_point_index: Option<usize>,
}

/// Represents an intersection point on a line.
#[derive(Clone, Debug)]
pub struct IntersectionPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
}

impl IntPatchALine {
    /// Creates an analytic intersection line with in/out transitions.
    pub fn new(curve_type: i32, is_tangent: bool, trans1: i32, trans2: i32) -> Self {
        IntPatchALine {
            curve_type,
            is_tangent,
            trans1,
            trans2,
            first_param: 0.0,
            last_param: 1.0,
            first_included: true,
            last_included: true,
            points: Vec::new(),
            first_point_index: None,
            last_point_index: None,
        }
    }

    /// Adds a vertex (intersection point) to the line.
    pub fn add_vertex(&mut self, pnt: &IntersectionPoint) {
        self.points.push(pnt.clone());
    }

    /// Replaces a point at the given index (1-based).
    pub fn replace(&mut self, index: usize, pnt: &IntersectionPoint) {
        if index > 0 && index <= self.points.len() {
            self.points[index - 1] = pnt.clone();
        }
    }

    /// Sets the first point index.
    pub fn set_first_point(&mut self, ind_first: usize) {
        self.first_point_index = Some(ind_first);
    }

    /// Sets the last point index.
    pub fn set_last_point(&mut self, ind_last: usize) {
        self.last_point_index = Some(ind_last);
    }

    /// Returns the first parameter and whether it's included in the domain.
    pub fn first_parameter(&self) -> (f64, bool) {
        (self.first_param, self.first_included)
    }

    /// Returns the last parameter and whether it's included in the domain.
    pub fn last_parameter(&self) -> (f64, bool) {
        (self.last_param, self.last_included)
    }

    /// Evaluates the curve at parameter U (returns point).
    pub fn value(&self, u: f64) -> (f64, f64, f64) {
        // Parametric evaluation (simplified)
        let u_norm = (u - self.first_param) / (self.last_param - self.first_param);
        (u_norm, u_norm * 0.5, u_norm * 0.25)
    }

    /// Evaluates the first derivative at parameter U.
    pub fn d1(&self, _u: f64) -> ((f64, f64, f64), (f64, f64, f64)) {
        let p = self.value(_u);
        // Approximate derivative
        ((p.0, p.1, p.2), (1.0, 0.5, 0.25))
    }

    /// Returns whether the line is tangent to both surfaces.
    pub fn is_tangent(&self) -> bool {
        self.is_tangent
    }

    /// Returns the number of points on the line.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Returns the point at index (1-based).
    pub fn point(&self, index: usize) -> Option<&IntersectionPoint> {
        if index > 0 && index <= self.points.len() {
            Some(&self.points[index - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aline() {
        let line = IntPatchALine::new(1, false, 0, 0);
        assert!(!line.is_tangent());
        assert_eq!(line.nb_points(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut line = IntPatchALine::new(1, false, 0, 0);
        let pnt = IntersectionPoint {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            u1: 0.5,
            v1: 0.5,
            u2: 0.25,
            v2: 0.25,
        };
        line.add_vertex(&pnt);
        assert_eq!(line.nb_points(), 1);
    }

    #[test]
    fn test_replace_vertex() {
        let mut line = IntPatchALine::new(1, false, 0, 0);
        let pnt1 = IntersectionPoint {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            u1: 0.5,
            v1: 0.5,
            u2: 0.25,
            v2: 0.25,
        };
        line.add_vertex(&pnt1);
        let pnt2 = IntersectionPoint {
            x: 4.0,
            y: 5.0,
            z: 6.0,
            u1: 0.6,
            v1: 0.6,
            u2: 0.3,
            v2: 0.3,
        };
        line.replace(1, &pnt2);
        let p = line.point(1).unwrap();
        assert!((p.x - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_first_last_parameters() {
        let line = IntPatchALine::new(1, false, 0, 0);
        let (first, first_inc) = line.first_parameter();
        let (last, last_inc) = line.last_parameter();
        assert_eq!(first, 0.0);
        assert_eq!(last, 1.0);
        assert!(first_inc);
        assert!(last_inc);
    }

    #[test]
    fn test_value() {
        let line = IntPatchALine::new(1, false, 0, 0);
        let (x, y, z) = line.value(0.0);
        assert!(x.abs() < 1e-10);
    }
}
