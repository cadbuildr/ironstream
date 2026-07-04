// FILE: step_visual_camera_image.rs
// occt: StepVisual_CameraImage

/// Represents a StepVisual CameraImage
#[derive(Debug, Clone, Default)]
pub struct StepVisual_CameraImage {
    name: Option<String>,
}

impl StepVisual_CameraImage {
    pub fn new() -> Self {
        StepVisual_CameraImage { name: None }
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
        let ci = StepVisual_CameraImage::new();
        assert!(ci.name().is_none());
    }
}
