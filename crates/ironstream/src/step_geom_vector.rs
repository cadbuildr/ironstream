// FILE: step_geom_vector.rs
// occt: StepGeom_Vector

pub struct Vector {
    orientation: Option<Box<dyn std::any::Any>>,
    magnitude: f64,
}

impl Vector {
    pub fn new() -> Self {
        Vector {
            orientation: None,
            magnitude: 0.0,
        }
    }

    pub fn init(&mut self, orientation: Option<Box<dyn std::any::Any>>, magnitude: f64) {
        self.orientation = orientation;
        self.magnitude = magnitude;
    }

    pub fn set_orientation(&mut self, orientation: Option<Box<dyn std::any::Any>>) {
        self.orientation = orientation;
    }

    pub fn orientation(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.orientation
    }

    pub fn set_magnitude(&mut self, magnitude: f64) {
        self.magnitude = magnitude;
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vector = Vector::new();
        assert_eq!(vector.magnitude(), 0.0);
        assert!(vector.orientation().is_none());
    }

    #[test]
    fn test_vector_init() {
        let mut vector = Vector::new();
        vector.init(None, 5.0);

        assert_eq!(vector.magnitude(), 5.0);
        assert!(vector.orientation().is_none());
    }

    #[test]
    fn test_vector_setters() {
        let mut vector = Vector::new();
        vector.set_magnitude(3.5);
        assert_eq!(vector.magnitude(), 3.5);
    }
}
