// FILE: step_visual_camera_model_d2.rs
// occt: StepVisual_CameraModelD2

/// Represents a StepVisual CameraModelD2
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModelD2 {
    name: Option<String>,
}

impl StepVisual_CameraModelD2 {
    pub fn new() -> Self {
        StepVisual_CameraModelD2 { name: None }
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
        let cmd2 = StepVisual_CameraModelD2::new();
        assert!(cmd2.name().is_none());
    }
}
