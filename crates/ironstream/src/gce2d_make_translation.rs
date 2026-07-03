// FILE: gce2d_make_translation.rs
// occt: GCE2d_MakeTranslation

/// Creates translation transformation in 2D.
pub struct Gce2dMakeTranslation {
    done: bool,
    dx: f64,
    dy: f64,
}

impl Gce2dMakeTranslation {
    pub fn new(dx: f64, dy: f64) -> Self {
        Gce2dMakeTranslation {
            done: false,
            dx,
            dy,
        }
    }

    pub fn translate_point(&mut self, x: f64, y: f64) -> (f64, f64) {
        self.done = true;
        (x + self.dx, y + self.dy)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn translation(&self) -> (f64, f64) {
        (self.dx, self.dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let trans = Gce2dMakeTranslation::new(1.0, 2.0);
        assert!(!trans.is_done());
        let (dx, dy) = trans.translation();
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 2.0);
    }

    #[test]
    fn test_translate_point() {
        let mut trans = Gce2dMakeTranslation::new(3.0, 4.0);
        let (x, y) = trans.translate_point(1.0, 1.0);
        assert_eq!(x, 4.0);
        assert_eq!(y, 5.0);
        assert!(trans.is_done());
    }
}
