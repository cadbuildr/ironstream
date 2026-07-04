// FILE: iges_appli_node.rs
// occt: IGESAppli_Node

/// Represents a geometric node used in finite element analysis.
///
/// IGES Type 134 Form 0
/// Stores nodal coordinates and optional coordinate system reference.
#[derive(Clone, Debug)]
pub struct IgesAppliNode {
    x: f64,
    y: f64,
    z: f64,
    system_handle: Option<String>,
}

impl IgesAppliNode {
    /// Creates a new Node entity.
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            system_handle: None,
        }
    }

    /// Initializes with nodal coordinates and optional coordinate system.
    pub fn init(&mut self, x: f64, y: f64, z: f64, system: Option<String>) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.system_handle = system;
    }

    /// Returns nodal coordinates.
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Returns the coordinate system entity if defined.
    pub fn system(&self) -> Option<&str> {
        self.system_handle.as_deref()
    }

    /// Returns the coordinate system type.
    /// 0=GlobalCartesian, 1=Cartesian, 2=Cylindrical, 3=Spherical
    pub fn system_type(&self) -> i32 {
        if self.system_handle.is_some() {
            1  // Non-global Cartesian by default
        } else {
            0  // Global Cartesian
        }
    }

    /// Returns coordinates after coordinate system transformation.
    /// In a real implementation, this would apply transformation matrices.
    pub fn transformed_nodal_coord(&self) -> (f64, f64, f64) {
        // TODO: Apply coordinate system transformation if system is set
        (self.x, self.y, self.z)
    }
}

impl Default for IgesAppliNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = IgesAppliNode::new();
        assert_eq!(node.coord(), (0.0, 0.0, 0.0));
        assert_eq!(node.system(), None);
        assert_eq!(node.system_type(), 0);
    }

    #[test]
    fn test_init() {
        let mut node = IgesAppliNode::new();
        node.init(1.0, 2.0, 3.0, None);

        assert_eq!(node.coord(), (1.0, 2.0, 3.0));
        assert_eq!(node.system(), None);
    }

    #[test]
    fn test_init_with_system() {
        let mut node = IgesAppliNode::new();
        node.init(5.0, 6.0, 7.0, Some("CART_SYSTEM".to_string()));

        assert_eq!(node.coord(), (5.0, 6.0, 7.0));
        assert_eq!(node.system(), Some("CART_SYSTEM"));
        assert_eq!(node.system_type(), 1);
    }

    #[test]
    fn test_transformed_coord() {
        let mut node = IgesAppliNode::new();
        node.init(1.0, 2.0, 3.0, None);

        let (x, y, z) = node.transformed_nodal_coord();
        assert_eq!((x, y, z), (1.0, 2.0, 3.0));
    }
}
