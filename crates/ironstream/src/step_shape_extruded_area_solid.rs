// FILE: step_shape_extruded_area_solid.rs
// occt: StepShape_ExtrudedAreaSolid

//! Representation of STEP entity ExtrudedAreaSolid

#[derive(Clone, Debug)]
pub struct ExtrudedAreaSolid {
    name: String,
    swept_area: Option<String>, // Placeholder for CurveBoundedSurface handle
    extruded_direction: Option<String>, // Placeholder for Direction handle
    depth: f64,
}

impl ExtrudedAreaSolid {
    /// Returns an ExtrudedAreaSolid
    pub fn new() -> Self {
        ExtrudedAreaSolid {
            name: String::new(),
            swept_area: None,
            extruded_direction: None,
            depth: 0.0,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        swept_area: Option<String>,
        direction: Option<String>,
        depth: f64,
    ) {
        self.name = name;
        self.swept_area = swept_area;
        self.extruded_direction = direction;
        self.depth = depth;
    }

    /// Set ExtrudedDirection
    pub fn set_extruded_direction(&mut self, direction: Option<String>) {
        self.extruded_direction = direction;
    }

    /// Returns ExtrudedDirection
    pub fn extruded_direction(&self) -> &Option<String> {
        &self.extruded_direction
    }

    /// Set Depth
    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }

    /// Returns Depth
    pub fn depth(&self) -> f64 {
        self.depth
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns swept_area field
    pub fn swept_area(&self) -> &Option<String> {
        &self.swept_area
    }

    /// Set swept_area field
    pub fn set_swept_area(&mut self, area: Option<String>) {
        self.swept_area = area;
    }
}

impl Default for ExtrudedAreaSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solid = ExtrudedAreaSolid::new();
        assert_eq!(solid.name(), "");
        assert_eq!(solid.depth(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut solid = ExtrudedAreaSolid::new();
        solid.init(
            "Extrusion1".to_string(),
            Some("area1".to_string()),
            Some("dir1".to_string()),
            10.5,
        );
        assert_eq!(solid.name(), "Extrusion1");
        assert_eq!(solid.depth(), 10.5);
    }

    #[test]
    fn test_set_depth() {
        let mut solid = ExtrudedAreaSolid::new();
        solid.set_depth(5.0);
        assert_eq!(solid.depth(), 5.0);
    }

    #[test]
    fn test_set_extruded_direction() {
        let mut solid = ExtrudedAreaSolid::new();
        solid.set_extruded_direction(Some("dir1".to_string()));
        assert_eq!(solid.extruded_direction(), &Some("dir1".to_string()));
    }
}
