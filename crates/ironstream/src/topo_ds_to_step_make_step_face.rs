// FILE: topo_ds_to_step_make_step_face.rs
// occt: TopoDSToStep_MakeStepFace

use super::topo_ds_to_step_make_face_error::MakeFaceError;

pub struct MakeStepFace {
    result: Option<TopologicalRepresentationItem>,
    error: MakeFaceError,
}

pub struct TopologicalRepresentationItem;

impl MakeStepFace {
    pub fn new() -> Self {
        MakeStepFace {
            result: None,
            error: MakeFaceError::FaceDone,
        }
    }

    pub fn value(&self) -> Option<&TopologicalRepresentationItem> {
        self.result.as_ref()
    }

    pub fn error(&self) -> MakeFaceError {
        self.error
    }
}

impl Default for MakeStepFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeStepFace::new();
        assert!(maker.value().is_none());
        assert_eq!(maker.error(), MakeFaceError::FaceDone);
    }
}
