// FILE: contap_point.rs
// occt: Contap_Point

/// Definition of a vertex on the contour line
/// Usually an intersection between contour and surface restriction
pub struct ContapPoint {
    pt: (f64, f64, f64),
    u: f64,
    v: f64,
    para_line: f64,
    on_arc: bool,
    para_arc: f64,
    is_vtx: bool,
    is_mult: bool,
    is_internal: bool,
}

impl ContapPoint {
    /// Empty constructor
    pub fn new() -> Self {
        ContapPoint {
            pt: (0.0, 0.0, 0.0),
            u: 0.0,
            v: 0.0,
            para_line: 0.0,
            on_arc: false,
            para_arc: 0.0,
            is_vtx: false,
            is_mult: false,
            is_internal: false,
        }
    }

    /// Creates a point with coordinates and parameters
    pub fn with_params(pt_x: f64, pt_y: f64, pt_z: f64, u: f64, v: f64) -> Self {
        let mut p = Self::new();
        p.pt = (pt_x, pt_y, pt_z);
        p.u = u;
        p.v = v;
        p
    }

    /// Sets the values for a point
    pub fn set_value(&mut self, pt_x: f64, pt_y: f64, pt_z: f64, u: f64, v: f64) {
        self.pt = (pt_x, pt_y, pt_z);
        self.u = u;
        self.v = v;
    }

    /// Set parameter on the intersection line
    pub fn set_parameter(&mut self, para: f64) {
        self.para_line = para;
    }

    /// Set as vertex on initial restriction facet
    pub fn set_vertex(&mut self) {
        self.is_vtx = true;
    }

    /// Set arc and parameter
    pub fn set_arc(&mut self, _arc: *const u8, param: f64) {
        self.on_arc = true;
        self.para_arc = param;
    }

    /// Mark as multiple intersection
    pub fn set_multiple(&mut self) {
        self.is_mult = true;
    }

    /// Mark as internal point
    pub fn set_internal(&mut self) {
        self.is_internal = true;
    }

    /// Returns intersection point
    pub fn value(&self) -> (f64, f64, f64) {
        self.pt
    }

    /// Returns parameter on the intersection line
    pub fn parameter_on_line(&self) -> f64 {
        self.para_line
    }

    /// Returns surface parameters
    pub fn parameters(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    /// Returns true if point is on arc
    pub fn is_on_arc(&self) -> bool {
        self.on_arc
    }

    /// Returns parameter on arc
    pub fn parameter_on_arc(&self) -> f64 {
        self.para_arc
    }

    /// Returns true if is vertex on restriction
    pub fn is_vertex(&self) -> bool {
        self.is_vtx
    }

    /// Returns true if belongs to multiple lines
    pub fn is_multiple(&self) -> bool {
        self.is_mult
    }

    /// Returns true if is internal point
    pub fn is_internal(&self) -> bool {
        self.is_internal
    }
}

impl Default for ContapPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapPoint;

    #[test]
    fn test_new() {
        let pt = ContapPoint::new();
        assert_eq!(pt.value(), (0.0, 0.0, 0.0));
        assert!(!pt.is_on_arc());
    }

    #[test]
    fn test_with_params() {
        let pt = ContapPoint::with_params(1.0, 2.0, 3.0, 0.5, 0.6);
        assert_eq!(pt.value(), (1.0, 2.0, 3.0));
        assert_eq!(pt.parameters(), (0.5, 0.6));
    }

    #[test]
    fn test_set_value() {
        let mut pt = ContapPoint::new();
        pt.set_value(1.5, 2.5, 3.5, 0.7, 0.8);
        assert_eq!(pt.value(), (1.5, 2.5, 3.5));
    }

    #[test]
    fn test_set_parameter() {
        let mut pt = ContapPoint::new();
        pt.set_parameter(1.5);
        assert_eq!(pt.parameter_on_line(), 1.5);
    }

    #[test]
    fn test_marks() {
        let mut pt = ContapPoint::new();
        pt.set_vertex();
        pt.set_multiple();
        pt.set_internal();

        assert!(pt.is_vertex());
        assert!(pt.is_multiple());
        assert!(pt.is_internal());
    }
}
