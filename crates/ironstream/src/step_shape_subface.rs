// FILE: step_shape_subface.rs
// occt: StepShape_Subface

use std::sync::Arc;

/// Placeholder for StepShape_FaceBound
pub struct FaceBound {
    id: usize,
}

impl FaceBound {
    pub fn new(id: usize) -> Self {
        FaceBound { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_Face (base class)
pub struct Face {
    name: Arc<str>,
    bounds: Vec<Arc<FaceBound>>,
}

impl Face {
    pub fn new(name: Arc<str>) -> Self {
        Face {
            name,
            bounds: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bounds(&self) -> &[Arc<FaceBound>] {
        &self.bounds
    }
}

/// Represents a subface in STEP format.
/// Inherits from StepShape_Face.
pub struct Subface {
    name: Arc<str>,
    bounds: Vec<Arc<FaceBound>>,
    parent_face: Option<Arc<Face>>,
}

impl Subface {
    /// Create a new Subface
    pub fn new() -> Self {
        Subface {
            name: Arc::from(""),
            bounds: Vec::new(),
            parent_face: None,
        }
    }

    /// Initialize with all fields (inherited and own)
    pub fn init(
        &mut self,
        name: Arc<str>,
        bounds: Vec<Arc<FaceBound>>,
        parent_face: Arc<Face>,
    ) {
        self.name = name;
        self.bounds = bounds;
        self.parent_face = Some(parent_face);
    }

    /// Get the parent face
    pub fn parent_face(&self) -> Option<&Arc<Face>> {
        self.parent_face.as_ref()
    }

    /// Set the parent face
    pub fn set_parent_face(&mut self, parent_face: Arc<Face>) {
        self.parent_face = Some(parent_face);
    }

    /// Get the name (from inherited fields)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Get the bounds (from inherited fields)
    pub fn bounds(&self) -> &[Arc<FaceBound>] {
        &self.bounds
    }

    /// Set the bounds
    pub fn set_bounds(&mut self, bounds: Vec<Arc<FaceBound>>) {
        self.bounds = bounds;
    }

    /// Get a bound by index (1-based as per OCCT convention)
    pub fn bounds_value(&self, num: usize) -> Option<Arc<FaceBound>> {
        if num > 0 && num <= self.bounds.len() {
            Some(self.bounds[num - 1].clone())
        } else {
            None
        }
    }

    /// Get the number of bounds
    pub fn nb_bounds(&self) -> usize {
        self.bounds.len()
    }
}

impl Default for Subface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subface_creation() {
        let sf = Subface::new();
        assert_eq!(sf.name(), "");
        assert_eq!(sf.nb_bounds(), 0);
        assert!(sf.parent_face().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut sf = Subface::new();
        let bounds = vec![
            Arc::new(FaceBound::new(1)),
            Arc::new(FaceBound::new(2)),
        ];
        let parent = Arc::new(Face::new(Arc::from("parent_face")));
        let name: Arc<str> = Arc::from("subface_1");

        sf.init(name.clone(), bounds, parent.clone());

        assert_eq!(sf.name(), "subface_1");
        assert_eq!(sf.nb_bounds(), 2);
        assert!(sf.parent_face().is_some());
    }

    #[test]
    fn test_set_parent_face() {
        let mut sf = Subface::new();
        let parent = Arc::new(Face::new(Arc::from("parent")));

        sf.set_parent_face(parent);

        assert!(sf.parent_face().is_some());
        assert_eq!(sf.parent_face().unwrap().name(), "parent");
    }

    #[test]
    fn test_set_bounds() {
        let mut sf = Subface::new();
        let bounds = vec![
            Arc::new(FaceBound::new(10)),
            Arc::new(FaceBound::new(20)),
            Arc::new(FaceBound::new(30)),
        ];

        sf.set_bounds(bounds);

        assert_eq!(sf.nb_bounds(), 3);
    }

    #[test]
    fn test_bounds_value() {
        let mut sf = Subface::new();
        let bounds = vec![
            Arc::new(FaceBound::new(100)),
            Arc::new(FaceBound::new(200)),
        ];

        sf.set_bounds(bounds);

        // 1-based indexing
        let b1 = sf.bounds_value(1);
        assert!(b1.is_some());
        assert_eq!(b1.unwrap().id(), 100);

        let b2 = sf.bounds_value(2);
        assert!(b2.is_some());
        assert_eq!(b2.unwrap().id(), 200);

        // Out of bounds
        let b_out = sf.bounds_value(3);
        assert!(b_out.is_none());
    }

    #[test]
    fn test_full_initialization() {
        let mut sf = Subface::new();
        sf.set_name(Arc::from("full_subface"));

        let bounds = vec![Arc::new(FaceBound::new(5))];
        let parent = Arc::new(Face::new(Arc::from("full_parent")));

        sf.set_bounds(bounds);
        sf.set_parent_face(parent);

        assert_eq!(sf.name(), "full_subface");
        assert_eq!(sf.nb_bounds(), 1);
        assert!(sf.parent_face().is_some());
    }
}
