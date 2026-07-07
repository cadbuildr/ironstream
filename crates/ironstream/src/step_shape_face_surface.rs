// FILE: step_shape_face_surface.rs
// occt: StepShape_FaceSurface

//! Representation of STEP entity FaceSurface

#[derive(Clone, Debug)]
pub struct FaceSurface {
    name: String,
    bounds: Vec<String>,
    face_geometry: Option<String>, // Placeholder for Surface handle
    same_sense: bool,
}

impl FaceSurface {
    /// Returns a FaceSurface
    pub fn new() -> Self {
        FaceSurface {
            name: String::new(),
            bounds: Vec::new(),
            face_geometry: None,
            same_sense: false,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        bounds: Vec<String>,
        face_geometry: Option<String>,
        same_sense: bool,
    ) {
        self.name = name;
        self.bounds = bounds;
        self.face_geometry = face_geometry;
        self.same_sense = same_sense;
    }

    /// Set FaceGeometry
    pub fn set_face_geometry(&mut self, geometry: Option<String>) {
        self.face_geometry = geometry;
    }

    /// Returns FaceGeometry
    pub fn face_geometry(&self) -> &Option<String> {
        &self.face_geometry
    }

    /// Set SameSense
    pub fn set_same_sense(&mut self, sense: bool) {
        self.same_sense = sense;
    }

    /// Returns SameSense
    pub fn same_sense(&self) -> bool {
        self.same_sense
    }

    /// Returns name field (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns bounds (inherited)
    pub fn bounds(&self) -> &[String] {
        &self.bounds
    }

    /// Set bounds (inherited)
    pub fn set_bounds(&mut self, bounds: Vec<String>) {
        self.bounds = bounds;
    }

    /// Returns number of bounds (inherited)
    pub fn nb_bounds(&self) -> usize {
        self.bounds.len()
    }
}

impl Default for FaceSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fs = FaceSurface::new();
        assert_eq!(fs.name(), "");
        assert!(!fs.same_sense());
        assert!(fs.face_geometry().is_none());
    }

    #[test]
    fn test_init() {
        let mut fs = FaceSurface::new();
        fs.init(
            "FaceSurface1".to_string(),
            vec!["bound1".to_string()],
            Some("surf1".to_string()),
            true,
        );
        assert_eq!(fs.name(), "FaceSurface1");
        assert!(fs.same_sense());
        assert_eq!(fs.face_geometry(), &Some("surf1".to_string()));
    }

    #[test]
    fn test_set_face_geometry() {
        let mut fs = FaceSurface::new();
        fs.set_face_geometry(Some("geometry1".to_string()));
        assert_eq!(fs.face_geometry(), &Some("geometry1".to_string()));
    }

    #[test]
    fn test_inherited_bounds() {
        let mut fs = FaceSurface::new();
        fs.set_bounds(vec!["b1".to_string(), "b2".to_string()]);
        assert_eq!(fs.nb_bounds(), 2);
    }
}
