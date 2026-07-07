// FILE: hlr_test_projector.rs
// occt: HLRTest_Projector

//! Projector for HLR test visualizations.

#[derive(Clone, Debug)]
pub struct Projector {
    pub id: usize,
    pub focal_length: f64,
    pub perspective: bool,
}

impl Projector {
    pub fn new(id: usize) -> Self {
        Projector {
            id,
            focal_length: 1.0,
            perspective: false,
        }
    }

    pub fn set_focal_length(&mut self, length: f64) {
        self.focal_length = length;
    }

    pub fn set_perspective(&mut self, perspective: bool) {
        self.perspective = perspective;
    }

    pub fn project_point(&self, x: f64, y: f64, z: f64) -> (f64, f64) {
        if self.perspective {
            let scale = self.focal_length / (z + self.focal_length);
            (x * scale, y * scale)
        } else {
            (x, y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let proj = Projector::new(1);
        assert_eq!(proj.id, 1);
        assert_eq!(proj.focal_length, 1.0);
        assert!(!proj.perspective);
    }

    #[test]
    fn test_set_focal_length() {
        let mut proj = Projector::new(1);
        proj.set_focal_length(2.5);
        assert_eq!(proj.focal_length, 2.5);
    }

    #[test]
    fn test_set_perspective() {
        let mut proj = Projector::new(1);
        proj.set_perspective(true);
        assert!(proj.perspective);
    }

    #[test]
    fn test_orthogonal_projection() {
        let proj = Projector::new(1);
        let (x, y) = proj.project_point(10.0, 20.0, 5.0);
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
    }

    #[test]
    fn test_perspective_projection() {
        let mut proj = Projector::new(1);
        proj.set_perspective(true);
        proj.set_focal_length(1.0);

        let (x, y) = proj.project_point(1.0, 1.0, 1.0);
        assert!((x - 0.5).abs() < 0.001);
        assert!((y - 0.5).abs() < 0.001);
    }
}
