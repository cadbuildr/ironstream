// FILE: step_shape_swept_face_solid.rs
// occt: StepShape_SweptFaceSolid

use std::sync::Arc;

/// Placeholder for StepShape_FaceSurface
pub struct FaceSurface {
    id: usize,
}

impl FaceSurface {
    pub fn new(id: usize) -> Self {
        FaceSurface { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a swept face solid in STEP format.
/// Inherits from StepShape_SolidModel.
pub struct SweptFaceSolid {
    name: Arc<str>,
    swept_area: Option<Arc<FaceSurface>>,
}

impl SweptFaceSolid {
    /// Create a new SweptFaceSolid
    pub fn new() -> Self {
        SweptFaceSolid {
            name: Arc::from(""),
            swept_area: None,
        }
    }

    /// Initialize with name and swept face
    pub fn init(&mut self, name: Arc<str>, swept_area: Arc<FaceSurface>) {
        self.name = name;
        self.swept_area = Some(swept_area);
    }

    /// Set the swept face
    pub fn set_swept_face(&mut self, swept_area: Arc<FaceSurface>) {
        self.swept_area = Some(swept_area);
    }

    /// Get the swept face
    pub fn swept_face(&self) -> Option<&Arc<FaceSurface>> {
        self.swept_area.as_ref()
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

impl Default for SweptFaceSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swept_face_solid_creation() {
        let sfs = SweptFaceSolid::new();
        assert_eq!(sfs.name(), "");
        assert!(sfs.swept_face().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut sfs = SweptFaceSolid::new();
        let swept_face = Arc::new(FaceSurface::new(1));
        let name = Arc::from("swept_face_solid_1");

        sfs.init(name.clone(), swept_face);

        assert_eq!(sfs.name(), "swept_face_solid_1");
        assert!(sfs.swept_face().is_some());
    }

    #[test]
    fn test_set_swept_face() {
        let mut sfs = SweptFaceSolid::new();
        let swept_face = Arc::new(FaceSurface::new(99));

        sfs.set_swept_face(swept_face);

        assert!(sfs.swept_face().is_some());
        assert_eq!(sfs.swept_face().unwrap().id(), 99);
    }

    #[test]
    fn test_set_name() {
        let mut sfs = SweptFaceSolid::new();
        sfs.set_name(Arc::from("face_solid"));

        assert_eq!(sfs.name(), "face_solid");
    }

    #[test]
    fn test_full_initialization() {
        let mut sfs = SweptFaceSolid::new();
        let swept_face = Arc::new(FaceSurface::new(50));
        let name = Arc::from("complete_solid");

        sfs.init(name.clone(), swept_face);

        assert_eq!(sfs.name(), "complete_solid");
        assert!(sfs.swept_face().is_some());
        assert_eq!(sfs.swept_face().unwrap().id(), 50);
    }
}
