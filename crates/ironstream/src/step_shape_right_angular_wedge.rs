// FILE: step_shape_right_angular_wedge.rs
// occt: StepShape_RightAngularWedge

use std::sync::Arc;

/// Placeholder for StepGeom_Axis2Placement3d
pub struct Axis2Placement3d {
    origin: (f64, f64, f64),
}

impl Axis2Placement3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Axis2Placement3d {
            origin: (x, y, z),
        }
    }

    pub fn origin(&self) -> (f64, f64, f64) {
        self.origin
    }
}

/// Represents a right angular wedge in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct RightAngularWedge {
    name: Arc<str>,
    position: Option<Arc<Axis2Placement3d>>,
    x: f64,
    y: f64,
    z: f64,
    ltx: f64,
}

impl RightAngularWedge {
    /// Create a new RightAngularWedge
    pub fn new() -> Self {
        RightAngularWedge {
            name: Arc::from(""),
            position: None,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            ltx: 0.0,
        }
    }

    /// Initialize with name, position, and dimensions
    pub fn init(
        &mut self,
        name: Arc<str>,
        position: Arc<Axis2Placement3d>,
        x: f64,
        y: f64,
        z: f64,
        ltx: f64,
    ) {
        self.name = name;
        self.position = Some(position);
        self.x = x;
        self.y = y;
        self.z = z;
        self.ltx = ltx;
    }

    /// Set the position (placement)
    pub fn set_position(&mut self, position: Arc<Axis2Placement3d>) {
        self.position = Some(position);
    }

    /// Get the position
    pub fn position(&self) -> Option<&Arc<Axis2Placement3d>> {
        self.position.as_ref()
    }

    /// Set the X dimension
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Get the X dimension
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Set the Y dimension
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Get the Y dimension
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Set the Z dimension
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    /// Get the Z dimension
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Set the Ltx dimension (length in X direction)
    pub fn set_ltx(&mut self, ltx: f64) {
        self.ltx = ltx;
    }

    /// Get the Ltx dimension
    pub fn ltx(&self) -> f64 {
        self.ltx
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for RightAngularWedge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_angular_wedge_creation() {
        let raw = RightAngularWedge::new();
        assert_eq!(raw.name(), "");
        assert_eq!(raw.x(), 0.0);
        assert_eq!(raw.y(), 0.0);
        assert_eq!(raw.z(), 0.0);
        assert_eq!(raw.ltx(), 0.0);
    }

    #[test]
    fn test_init_method() {
        let mut raw = RightAngularWedge::new();
        let position = Arc::new(Axis2Placement3d::new(0.0, 0.0, 0.0));
        let name: Arc<str> = Arc::from("wedge_1");

        raw.init(name.clone(), position.clone(), 10.0, 20.0, 30.0, 5.0);

        assert_eq!(raw.name(), "wedge_1");
        assert_eq!(raw.x(), 10.0);
        assert_eq!(raw.y(), 20.0);
        assert_eq!(raw.z(), 30.0);
        assert_eq!(raw.ltx(), 5.0);
        assert!(raw.position().is_some());
    }

    #[test]
    fn test_set_position() {
        let mut raw = RightAngularWedge::new();
        let position = Arc::new(Axis2Placement3d::new(1.0, 2.0, 3.0));

        raw.set_position(position);

        assert!(raw.position().is_some());
        let pos = raw.position().unwrap();
        let (x, y, z) = pos.origin();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_set_dimensions() {
        let mut raw = RightAngularWedge::new();

        raw.set_x(5.5);
        raw.set_y(6.6);
        raw.set_z(7.7);
        raw.set_ltx(2.2);

        assert_eq!(raw.x(), 5.5);
        assert_eq!(raw.y(), 6.6);
        assert_eq!(raw.z(), 7.7);
        assert_eq!(raw.ltx(), 2.2);
    }
}
