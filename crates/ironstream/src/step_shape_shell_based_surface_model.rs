// FILE: step_shape_shell_based_surface_model.rs
// occt: StepShape_ShellBasedSurfaceModel

use std::sync::Arc;

/// Placeholder for StepShape_Shell
#[derive(Clone)]
pub enum ShellType {
    Open(usize),
    Closed(usize),
}

/// Represents a shell-based surface model in STEP format.
/// Inherits from StepGeom_GeometricRepresentationItem.
pub struct ShellBasedSurfaceModel {
    name: Arc<str>,
    sbsm_boundary: Vec<ShellType>,
}

impl ShellBasedSurfaceModel {
    /// Create a new ShellBasedSurfaceModel
    pub fn new() -> Self {
        ShellBasedSurfaceModel {
            name: Arc::from(""),
            sbsm_boundary: Vec::new(),
        }
    }

    /// Initialize with name and boundary shells
    pub fn init(&mut self, name: Arc<str>, boundary: Vec<ShellType>) {
        self.name = name;
        self.sbsm_boundary = boundary;
    }

    /// Set the boundary shells
    pub fn set_sbsm_boundary(&mut self, boundary: Vec<ShellType>) {
        self.sbsm_boundary = boundary;
    }

    /// Get the boundary shells
    pub fn sbsm_boundary(&self) -> &[ShellType] {
        &self.sbsm_boundary
    }

    /// Get a boundary shell by index (1-based as per OCCT convention)
    pub fn sbsm_boundary_value(&self, num: usize) -> Option<ShellType> {
        if num > 0 && num <= self.sbsm_boundary.len() {
            Some(self.sbsm_boundary[num - 1].clone())
        } else {
            None
        }
    }

    /// Get the number of boundary shells
    pub fn nb_sbsm_boundary(&self) -> usize {
        self.sbsm_boundary.len()
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

impl Default for ShellBasedSurfaceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_based_surface_model_creation() {
        let sbsm = ShellBasedSurfaceModel::new();
        assert_eq!(sbsm.name(), "");
        assert_eq!(sbsm.nb_sbsm_boundary(), 0);
    }

    #[test]
    fn test_init_method() {
        let mut sbsm = ShellBasedSurfaceModel::new();
        let boundary = vec![
            ShellType::Open(1),
            ShellType::Closed(2),
            ShellType::Open(3),
        ];
        let name: Arc<str> = Arc::from("surface_model_1");

        sbsm.init(name.clone(), boundary);

        assert_eq!(sbsm.name(), "surface_model_1");
        assert_eq!(sbsm.nb_sbsm_boundary(), 3);
    }

    #[test]
    fn test_set_sbsm_boundary() {
        let mut sbsm = ShellBasedSurfaceModel::new();
        let boundary = vec![
            ShellType::Open(10),
            ShellType::Closed(20),
        ];

        sbsm.set_sbsm_boundary(boundary);
        assert_eq!(sbsm.nb_sbsm_boundary(), 2);
    }

    #[test]
    fn test_sbsm_boundary_value() {
        let mut sbsm = ShellBasedSurfaceModel::new();
        let boundary = vec![
            ShellType::Open(100),
            ShellType::Closed(200),
            ShellType::Open(300),
        ];

        sbsm.set_sbsm_boundary(boundary);

        // 1-based indexing
        let shell1 = sbsm.sbsm_boundary_value(1);
        assert!(shell1.is_some());

        let shell2 = sbsm.sbsm_boundary_value(2);
        assert!(shell2.is_some());

        // Out of bounds
        let shell_out = sbsm.sbsm_boundary_value(4);
        assert!(shell_out.is_none());
    }

    #[test]
    fn test_multiple_operations() {
        let mut sbsm = ShellBasedSurfaceModel::new();
        sbsm.set_name(Arc::from("model_xyz"));

        let boundary = vec![
            ShellType::Open(50),
            ShellType::Closed(60),
        ];
        sbsm.set_sbsm_boundary(boundary);

        assert_eq!(sbsm.name(), "model_xyz");
        assert_eq!(sbsm.nb_sbsm_boundary(), 2);
    }
}
