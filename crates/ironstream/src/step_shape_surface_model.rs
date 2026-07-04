// FILE: step_shape_surface_model.rs
// occt: StepShape_SurfaceModel

use std::sync::Arc;

/// Placeholder for StepShape_ShellBasedSurfaceModel
pub struct ShellBasedSurfaceModel {
    id: usize,
}

impl ShellBasedSurfaceModel {
    pub fn new(id: usize) -> Self {
        ShellBasedSurfaceModel { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Placeholder for StepShape_FaceBasedSurfaceModel
pub struct FaceBasedSurfaceModel {
    id: usize,
}

impl FaceBasedSurfaceModel {
    pub fn new(id: usize) -> Self {
        FaceBasedSurfaceModel { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// A discriminated union type representing a surface model.
/// Can be ShellBasedSurfaceModel or FaceBasedSurfaceModel.
pub enum SurfaceModel {
    /// Case 1: ShellBasedSurfaceModel
    ShellBasedSurfaceModel(Arc<ShellBasedSurfaceModel>),
    /// Case 2: FaceBasedSurfaceModel
    FaceBasedSurfaceModel(Arc<FaceBasedSurfaceModel>),
}

impl SurfaceModel {
    /// Create a SurfaceModel from a ShellBasedSurfaceModel
    pub fn from_shell_based(model: Arc<ShellBasedSurfaceModel>) -> Self {
        SurfaceModel::ShellBasedSurfaceModel(model)
    }

    /// Create a SurfaceModel from a FaceBasedSurfaceModel
    pub fn from_face_based(model: Arc<FaceBasedSurfaceModel>) -> Self {
        SurfaceModel::FaceBasedSurfaceModel(model)
    }

    /// Get the case number (kind) of this surface model
    /// 1 -> ShellBasedSurfaceModel
    /// 2 -> FaceBasedSurfaceModel
    pub fn case_num(&self) -> usize {
        match self {
            SurfaceModel::ShellBasedSurfaceModel(_) => 1,
            SurfaceModel::FaceBasedSurfaceModel(_) => 2,
        }
    }

    /// Try to get as a ShellBasedSurfaceModel, returns None if this is a FaceBasedSurfaceModel
    pub fn as_shell_based(&self) -> Option<&Arc<ShellBasedSurfaceModel>> {
        match self {
            SurfaceModel::ShellBasedSurfaceModel(model) => Some(model),
            _ => None,
        }
    }

    /// Try to get as a FaceBasedSurfaceModel, returns None if this is a ShellBasedSurfaceModel
    pub fn as_face_based(&self) -> Option<&Arc<FaceBasedSurfaceModel>> {
        match self {
            SurfaceModel::FaceBasedSurfaceModel(model) => Some(model),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_shell_based() {
        let model = Arc::new(ShellBasedSurfaceModel::new(1));
        let sm = SurfaceModel::from_shell_based(model);
        assert_eq!(sm.case_num(), 1);
    }

    #[test]
    fn test_case_num_face_based() {
        let model = Arc::new(FaceBasedSurfaceModel::new(2));
        let sm = SurfaceModel::from_face_based(model);
        assert_eq!(sm.case_num(), 2);
    }

    #[test]
    fn test_as_shell_based() {
        let model = Arc::new(ShellBasedSurfaceModel::new(10));
        let sm = SurfaceModel::from_shell_based(model.clone());
        assert!(sm.as_shell_based().is_some());
        assert_eq!(sm.as_shell_based().unwrap().id(), 10);
        assert!(sm.as_face_based().is_none());
    }

    #[test]
    fn test_as_face_based() {
        let model = Arc::new(FaceBasedSurfaceModel::new(20));
        let sm = SurfaceModel::from_face_based(model.clone());
        assert!(sm.as_face_based().is_some());
        assert_eq!(sm.as_face_based().unwrap().id(), 20);
        assert!(sm.as_shell_based().is_none());
    }

    #[test]
    fn test_multiple_models() {
        let shell = SurfaceModel::from_shell_based(Arc::new(ShellBasedSurfaceModel::new(1)));
        let face = SurfaceModel::from_face_based(Arc::new(FaceBasedSurfaceModel::new(2)));

        assert_eq!(shell.case_num(), 1);
        assert_eq!(face.case_num(), 2);
    }
}
