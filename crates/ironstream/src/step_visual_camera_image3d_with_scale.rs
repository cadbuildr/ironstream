// FILE: step_visual_camera_image3d_with_scale.rs
// occt: StepVisual_CameraImage3dWithScale

/// Represents a StepVisual CameraImage3dWithScale
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraImage3dWithScale {
    name: Option<String>,
    scale: f64,
}

impl StepVisual_CameraImage3dWithScale {
    pub fn new() -> Self {
        StepVisual_CameraImage3dWithScale {
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
        let ci = StepVisual_CameraImage3dWithScale::new();
        assert_eq!(ci.scale(), 1.0);
    }

    #[test]
    fn test_set_scale() {
        let mut ci = StepVisual_CameraImage3dWithScale::new();
        ci.set_scale(3.5);
        assert_eq!(ci.scale(), 3.5);
    }
}
