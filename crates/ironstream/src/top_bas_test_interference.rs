// FILE: top_bas_test_interference.rs
// occt: TopBas_TestInterference

/// Represents a test interference with intersection point, boundary info, and transitions.
#[derive(Clone, Debug)]
pub struct TopBasTestInterference {
    intersection: f64,
    boundary: i32,
    orientation: i32,       // TopAbs_Orientation
    transition: i32,        // TopAbs_Orientation
    boundary_transition: i32, // TopAbs_Orientation
}

impl TopBasTestInterference {
    /// Creates an empty test interference.
    pub fn new() -> Self {
        TopBasTestInterference {
            intersection: 0.0,
            boundary: 0,
            orientation: 0,
            transition: 0,
            boundary_transition: 0,
        }
    }

    /// Creates a test interference with all parameters.
    pub fn with_params(
        inters: f64,
        bound: i32,
        orient: i32,
        trans: i32,
        btrans: i32,
    ) -> Self {
        TopBasTestInterference {
            intersection: inters,
            boundary: bound,
            orientation: orient,
            transition: trans,
            boundary_transition: btrans,
        }
    }

    pub fn set_intersection(&mut self, i: f64) {
        self.intersection = i;
    }

    pub fn set_boundary(&mut self, b: i32) {
        self.boundary = b;
    }

    pub fn set_orientation(&mut self, o: i32) {
        self.orientation = o;
    }

    pub fn set_transition(&mut self, tr: i32) {
        self.transition = tr;
    }

    pub fn set_boundary_transition(&mut self, btr: i32) {
        self.boundary_transition = btr;
    }

    pub fn intersection(&self) -> f64 {
        self.intersection
    }

    pub fn intersection_mut(&mut self) -> &mut f64 {
        &mut self.intersection
    }

    pub fn boundary(&self) -> i32 {
        self.boundary
    }

    pub fn boundary_mut(&mut self) -> &mut i32 {
        &mut self.boundary
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    pub fn transition(&self) -> i32 {
        self.transition
    }

    pub fn boundary_transition(&self) -> i32 {
        self.boundary_transition
    }
}

impl Default for TopBasTestInterference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ti = TopBasTestInterference::new();
        assert_eq!(ti.intersection(), 0.0);
        assert_eq!(ti.boundary(), 0);
    }

    #[test]
    fn test_with_params() {
        let ti = TopBasTestInterference::with_params(1.5, 2, 1, 2, 3);
        assert_eq!(ti.intersection(), 1.5);
        assert_eq!(ti.boundary(), 2);
        assert_eq!(ti.orientation(), 1);
        assert_eq!(ti.transition(), 2);
        assert_eq!(ti.boundary_transition(), 3);
    }

    #[test]
    fn test_setters() {
        let mut ti = TopBasTestInterference::new();
        ti.set_intersection(2.5);
        ti.set_boundary(5);
        ti.set_orientation(1);
        ti.set_transition(2);
        ti.set_boundary_transition(3);
        assert_eq!(ti.intersection(), 2.5);
        assert_eq!(ti.boundary(), 5);
    }

    #[test]
    fn test_mutable_access() {
        let mut ti = TopBasTestInterference::new();
        *ti.intersection_mut() = 3.14;
        *ti.boundary_mut() = 7;
        assert_eq!(ti.intersection(), 3.14);
        assert_eq!(ti.boundary(), 7);
    }
}
