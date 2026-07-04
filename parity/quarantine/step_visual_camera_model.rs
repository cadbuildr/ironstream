// FILE: step_visual_camera_model.rs
// occt: StepVisual_CameraModel

/// Represents a StepVisual CameraModel
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModel {
    name: Option<String>,
}

impl StepVisual_CameraModel {
    pub fn new() -> Self {
        StepVisual_CameraModel { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cm = StepVisual_CameraModel::new();
        assert!(cm.name().is_none());
    }
}
