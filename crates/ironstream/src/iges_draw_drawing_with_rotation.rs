// FILE: iges_draw_drawing_with_rotation.rs
// occt: IGESDraw_DrawingWithRotation

/// Drawing with rotation information
pub struct IgesDrawDrawingWithRotation {
    drawing: Box<dyn std::any::Any>,
    rotation_angle: f64,
}

impl IgesDrawDrawingWithRotation {
    pub fn new() -> Self {
        IgesDrawDrawingWithRotation {
            drawing: Box::new(()),
            rotation_angle: 0.0,
        }
    }

    pub fn init(&mut self, rotation_angle: f64) {
        self.rotation_angle = rotation_angle;
    }

    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }
}

impl Default for IgesDrawDrawingWithRotation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dwr = IgesDrawDrawingWithRotation::new();
        assert_eq!(dwr.rotation_angle(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut dwr = IgesDrawDrawingWithRotation::new();
        dwr.init(1.57);
        assert_eq!(dwr.rotation_angle(), 1.57);
    }
}
