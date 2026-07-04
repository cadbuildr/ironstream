// FILE: step_geom_axis2_placement3d.rs
// occt: StepGeom_Axis2Placement3d

use std::sync::{Arc, Mutex};

/// StepGeom_Axis2Placement3d: A 3D coordinate system defined by location, axis, and reference direction.
#[derive(Clone)]
pub struct Axis2Placement3d {
    name: Arc<String>,
    location: Option<Arc<Mutex<CartesianPoint>>>,
    axis: Option<Arc<Mutex<Direction>>>,
    has_axis: bool,
    ref_direction: Option<Arc<Mutex<Direction>>>,
    has_ref_direction: bool,
}

/// Simplified CartesianPoint reference (placeholder for interop)
#[derive(Clone)]
pub struct CartesianPoint {
    name: String,
}

/// Simplified Direction reference (placeholder for interop)
#[derive(Clone)]
pub struct Direction {
    name: String,
}

impl Axis2Placement3d {
    /// Creates a new Axis2Placement3d.
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            location: None,
            axis: None,
            has_axis: false,
            ref_direction: None,
            has_ref_direction: false,
        }
    }

    /// Initializes the Axis2Placement3d with all parameters.
    pub fn init(
        &mut self,
        name: String,
        location: Option<Arc<Mutex<CartesianPoint>>>,
        has_axis: bool,
        axis: Option<Arc<Mutex<Direction>>>,
        has_ref_direction: bool,
        ref_direction: Option<Arc<Mutex<Direction>>>,
    ) {
        self.name = Arc::new(name);
        self.location = location;
        self.has_axis = has_axis;
        self.axis = axis;
        self.has_ref_direction = has_ref_direction;
        self.ref_direction = ref_direction;
    }

    /// Sets the axis direction.
    pub fn set_axis(&mut self, axis: Arc<Mutex<Direction>>) {
        self.axis = Some(axis);
        self.has_axis = true;
    }

    /// Clears the axis direction.
    pub fn unset_axis(&mut self) {
        self.axis = None;
        self.has_axis = false;
    }

    /// Returns the axis direction.
    pub fn axis(&self) -> Option<Arc<Mutex<Direction>>> {
        self.axis.clone()
    }

    /// Returns true if axis is set.
    pub fn has_axis(&self) -> bool {
        self.has_axis
    }

    /// Sets the reference direction.
    pub fn set_ref_direction(&mut self, ref_direction: Arc<Mutex<Direction>>) {
        self.ref_direction = Some(ref_direction);
        self.has_ref_direction = true;
    }

    /// Clears the reference direction.
    pub fn unset_ref_direction(&mut self) {
        self.ref_direction = None;
        self.has_ref_direction = false;
    }

    /// Returns the reference direction.
    pub fn ref_direction(&self) -> Option<Arc<Mutex<Direction>>> {
        self.ref_direction.clone()
    }

    /// Returns true if reference direction is set.
    pub fn has_ref_direction(&self) -> bool {
        self.has_ref_direction
    }

    /// Returns the name.
    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }

    /// Returns the location.
    pub fn location(&self) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.location.clone()
    }
}

impl Default for Axis2Placement3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis2placement3d_creation() {
        let ap = Axis2Placement3d::new();
        assert!(!ap.has_axis());
        assert!(!ap.has_ref_direction());
    }

    #[test]
    fn test_axis2placement3d_init() {
        let mut ap = Axis2Placement3d::new();
        ap.init("test".to_string(), None, false, None, false, None);
        assert_eq!(ap.name(), "test");
        assert!(!ap.has_axis());
        assert!(!ap.has_ref_direction());
    }

    #[test]
    fn test_axis2placement3d_set_unset_axis() {
        let mut ap = Axis2Placement3d::new();
        let dir = Arc::new(Mutex::new(Direction {
            name: "axis".to_string(),
        }));
        ap.set_axis(dir);
        assert!(ap.has_axis());

        ap.unset_axis();
        assert!(!ap.has_axis());
    }

    #[test]
    fn test_axis2placement3d_set_unset_ref_direction() {
        let mut ap = Axis2Placement3d::new();
        let dir = Arc::new(Mutex::new(Direction {
            name: "ref".to_string(),
        }));
        ap.set_ref_direction(dir);
        assert!(ap.has_ref_direction());

        ap.unset_ref_direction();
        assert!(!ap.has_ref_direction());
    }
}
