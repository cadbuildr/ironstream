// FILE: step_visual_camera_usage.rs
// occt: StepVisual_CameraUsage

/// Represents a StepVisual CameraUsage
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraUsage {
    name: Option<String>,
}

impl StepVisual_CameraUsage {
    pub fn new() -> Self {
        StepVisual_CameraUsage { name: None }
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
        let cu = StepVisual_CameraUsage::new();
        assert!(cu.name().is_none());
    }
}
