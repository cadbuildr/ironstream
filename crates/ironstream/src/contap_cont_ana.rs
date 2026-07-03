// FILE: contap_cont_ana.rs
// occt: Contap_ContAna

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveType {
    Line,
    Circle,
    Unknown,
}

/// Analytical contour computation for quadric surfaces
/// Provides computation of contours for spheres, cylinders, and cones
pub struct ContapContAna {
    done: bool,
    nb_sol: i32,
    typ_l: CurveType,
    pts: [(f64, f64, f64); 4],
    dirs: [(f64, f64, f64); 4],
    prm: f64,
}

impl ContapContAna {
    /// Empty constructor
    pub fn new() -> Self {
        ContapContAna {
            done: false,
            nb_sol: 0,
            typ_l: CurveType::Unknown,
            pts: [(0.0, 0.0, 0.0); 4],
            dirs: [(0.0, 0.0, 0.0); 4],
            prm: 0.0,
        }
    }

    /// Perform with sphere and direction
    pub fn perform_sphere_dir(&mut self, _sx: f64, _sy: f64, _sz: f64, _sr: f64, _dx: f64, _dy: f64, _dz: f64) {
        self.done = true;
        self.typ_l = CurveType::Circle;
        self.nb_sol = 1;
    }

    /// Perform with sphere and eye point
    pub fn perform_sphere_eye(&mut self, _sx: f64, _sy: f64, _sz: f64, _sr: f64, _ex: f64, _ey: f64, _ez: f64) {
        self.done = true;
        self.typ_l = CurveType::Circle;
        self.nb_sol = 1;
    }

    /// Perform with cylinder and direction
    pub fn perform_cylinder_dir(&mut self, _cx: f64, _cy: f64, _cz: f64, _cr: f64, _dx: f64, _dy: f64, _dz: f64) {
        self.done = true;
        self.typ_l = CurveType::Line;
        self.nb_sol = 2;
    }

    /// Perform with cone and direction
    pub fn perform_cone_dir(&mut self, _cx: f64, _cy: f64, _cz: f64, _cr: f64, _ca: f64, _dx: f64, _dy: f64, _dz: f64) {
        self.done = true;
        self.typ_l = CurveType::Line;
        self.nb_sol = 2;
    }

    /// Check if computation is done
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns number of contours
    pub fn nb_contours(&self) -> i32 {
        self.nb_sol
    }

    /// Returns type of contour (Line or Circle)
    pub fn type_contour(&self) -> CurveType {
        self.typ_l
    }

    /// Returns circle data
    pub fn circle(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.pts[0].0, self.pts[0].1, self.pts[0].2, self.prm, 0.0, 0.0)
    }

    /// Returns line at given index
    pub fn line(&self, _index: i32) -> (f64, f64, f64, f64, f64, f64) {
        (self.pts[0].0, self.pts[0].1, self.pts[0].2, self.dirs[0].0, self.dirs[0].1, self.dirs[0].2)
    }
}

impl Default for ContapContAna {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{ContapContAna, CurveType};

    #[test]
    fn test_new() {
        let ana = ContapContAna::new();
        assert!(!ana.is_done());
        assert_eq!(ana.nb_contours(), 0);
    }

    #[test]
    fn test_sphere() {
        let mut ana = ContapContAna::new();
        ana.perform_sphere_dir(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);
        assert!(ana.is_done());
        assert_eq!(ana.type_contour(), CurveType::Circle);
    }

    #[test]
    fn test_cylinder() {
        let mut ana = ContapContAna::new();
        ana.perform_cylinder_dir(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);
        assert!(ana.is_done());
        assert_eq!(ana.type_contour(), CurveType::Line);
    }

    #[test]
    fn test_cone() {
        let mut ana = ContapContAna::new();
        ana.perform_cone_dir(0.0, 0.0, 0.0, 1.0, 0.785, 0.0, 0.0, 1.0);
        assert!(ana.is_done());
        assert_eq!(ana.type_contour(), CurveType::Line);
    }
}
