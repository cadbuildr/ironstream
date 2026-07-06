// FILE: step_shape_box_domain.rs
// occt: StepShape_BoxDomain

/// Placeholder for CartesianPoint
#[derive(Clone, Debug, PartialEq)]
pub struct CartesianPoint {
    x: f64,
    y: f64,
    z: f64,
}

/// Represents a box domain in STEP
pub struct BoxDomain {
    corner: Option<CartesianPoint>,
    xlength: f64,
    ylength: f64,
    zlength: f64,
}

impl BoxDomain {
    /// Create a new BoxDomain
    pub fn new() -> Self {
        BoxDomain {
            corner: None,
            xlength: 0.0,
            ylength: 0.0,
            zlength: 0.0,
        }
    }

    /// Initialize with corner point and dimensions
    pub fn init(&mut self, corner: CartesianPoint, xlength: f64, ylength: f64, zlength: f64) {
        self.corner = Some(corner);
        self.xlength = xlength;
        self.ylength = ylength;
        self.zlength = zlength;
    }

    /// Set the corner point
    pub fn set_corner(&mut self, corner: CartesianPoint) {
        self.corner = Some(corner);
    }

    /// Get the corner point
    pub fn corner(&self) -> Option<&CartesianPoint> {
        self.corner.as_ref()
    }

    /// Set the X length
    pub fn set_xlength(&mut self, xlength: f64) {
        self.xlength = xlength;
    }

    /// Get the X length
    pub fn xlength(&self) -> f64 {
        self.xlength
    }

    /// Set the Y length
    pub fn set_ylength(&mut self, ylength: f64) {
        self.ylength = ylength;
    }

    /// Get the Y length
    pub fn ylength(&self) -> f64 {
        self.ylength
    }

    /// Set the Z length
    pub fn set_zlength(&mut self, zlength: f64) {
        self.zlength = zlength;
    }

    /// Get the Z length
    pub fn zlength(&self) -> f64 {
        self.zlength
    }
}

impl Default for BoxDomain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let bd = BoxDomain::new();
        assert_eq!(bd.corner(), None);
        assert_eq!(bd.xlength(), 0.0);
        assert_eq!(bd.ylength(), 0.0);
        assert_eq!(bd.zlength(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut bd = BoxDomain::new();
        let corner = CartesianPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        bd.init(corner.clone(), 10.0, 20.0, 30.0);
        assert_eq!(bd.corner(), Some(&corner));
        assert_eq!(bd.xlength(), 10.0);
        assert_eq!(bd.ylength(), 20.0);
        assert_eq!(bd.zlength(), 30.0);
    }

    #[test]
    fn test_set_dimensions() {
        let mut bd = BoxDomain::new();
        bd.set_xlength(5.0);
        bd.set_ylength(6.0);
        bd.set_zlength(7.0);
        assert_eq!(bd.xlength(), 5.0);
        assert_eq!(bd.ylength(), 6.0);
        assert_eq!(bd.zlength(), 7.0);
    }
}
