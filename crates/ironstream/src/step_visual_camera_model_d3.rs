// FILE: step_visual_camera_model_d3.rs
// occt: StepVisual_CameraModelD3

/// Represents a StepVisual CameraModelD3
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModelD3 {
    name: Option<String>,
}

impl StepVisual_CameraModelD3 {
    pub fn new() -> Self {
        StepVisual_CameraModelD3 { name: None }
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
        let cmd3 = StepVisual_CameraModelD3::new();
        assert!(cmd3.name().is_none());
    }
}
