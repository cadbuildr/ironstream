// FILE: step_visual_camera_image2d_with_scale.rs
// occt: StepVisual_CameraImage2dWithScale

/// Represents a StepVisual CameraImage2dWithScale
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraImage2dWithScale {
    name: Option<String>,
    scale: f64,
}

impl StepVisual_CameraImage2dWithScale {
    pub fn new() -> Self {
        StepVisual_CameraImage2dWithScale {
            name: None,
            scale: 1.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ci = StepVisual_CameraImage2dWithScale::new();
        assert_eq!(ci.scale(), 1.0);
    }

    #[test]
    fn test_set_scale() {
        let mut ci = StepVisual_CameraImage2dWithScale::new();
        ci.set_scale(2.0);
        assert_eq!(ci.scale(), 2.0);
    }
}
