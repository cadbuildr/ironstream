// FILE: step_visual_camera_model_d3_multi_clipping_intersection.rs
// occt: StepVisual_CameraModelD3MultiClippingIntersection

/// Represents a StepVisual CameraModelD3MultiClippingIntersection
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraModelD3MultiClippingIntersection {
    name: Option<String>,
}

impl StepVisual_CameraModelD3MultiClippingIntersection {
    pub fn new() -> Self {
        StepVisual_CameraModelD3MultiClippingIntersection { name: None }
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
        let cmdi = StepVisual_CameraModelD3MultiClippingIntersection::new();
        assert!(cmdi.name().is_none());
    }
}
