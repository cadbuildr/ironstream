// FILE: step_visual_camera_model_d3_multi_clipping.rs
// occt: StepVisual_CameraModelD3MultiClipping

/// Represents a StepVisual CameraModelD3MultiClipping
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModelD3MultiClipping {
    name: Option<String>,
}

impl StepVisual_CameraModelD3MultiClipping {
    pub fn new() -> Self {
        StepVisual_CameraModelD3MultiClipping { name: None }
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
        let cmd3mc = StepVisual_CameraModelD3MultiClipping::new();
        assert!(cmd3mc.name().is_none());
    }
}
