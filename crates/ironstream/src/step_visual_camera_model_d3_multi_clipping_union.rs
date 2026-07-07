// FILE: step_visual_camera_model_d3_multi_clipping_union.rs
// occt: StepVisual_CameraModelD3MultiClippingUnion

/// Represents a StepVisual CameraModelD3MultiClippingUnion
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModelD3MultiClippingUnion {
    name: Option<String>,
}

impl StepVisual_CameraModelD3MultiClippingUnion {
    pub fn new() -> Self {
        StepVisual_CameraModelD3MultiClippingUnion { name: None }
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
        let cmdu = StepVisual_CameraModelD3MultiClippingUnion::new();
        assert!(cmdu.name().is_none());
    }
}
