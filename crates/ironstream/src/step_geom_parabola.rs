// FILE: step_geom_parabola.rs
// occt: StepGeom_Parabola

/// Base conic representation
#[derive(Clone, Debug)]
pub struct StepGeomConic {
    name: String,
    position: i32,
}

impl StepGeomConic {
    pub fn new(name: String, position: i32) -> Self {
        StepGeomConic { name, position }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}

/// Parabola representation in STEP format.
/// A parabola is defined by its position and focal distance.
pub struct StepGeomParabola {
    conic: StepGeomConic,
    focal_dist: f64,
}

impl StepGeomParabola {
    /// Create a new parabola
    pub fn new() -> Self {
        StepGeomParabola {
            conic: StepGeomConic::new("".to_string(), 0),
            focal_dist: 0.0,
        }
    }

    /// Initialize parabola with parameters
    pub fn init(&mut self, name: String, position: i32, focal_dist: f64) {
        self.conic = StepGeomConic::new(name, position);
        self.focal_dist = focal_dist;
    }

    /// Set the focal distance
    pub fn set_focal_dist(&mut self, focal_dist: f64) {
        self.focal_dist = focal_dist;
    }

    /// Get the focal distance
    pub fn focal_dist(&self) -> f64 {
        self.focal_dist
    }

    /// Get the conic name
    pub fn name(&self) -> &str {
        self.conic.name()
    }

    /// Get the position
    pub fn position(&self) -> i32 {
        self.conic.position()
    }
}

impl Default for StepGeomParabola {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parabola() {
        let parabola = StepGeomParabola::new();
        assert_eq!(parabola.focal_dist(), 0.0);
    }

    #[test]
    fn test_init_parabola() {
        let mut parabola = StepGeomParabola::new();
        parabola.init("Parabola1".to_string(), 1, 2.5);
        assert_eq!(parabola.name(), "Parabola1");
        assert_eq!(parabola.position(), 1);
        assert_eq!(parabola.focal_dist(), 2.5);
    }

    #[test]
    fn test_set_focal_dist() {
        let mut parabola = StepGeomParabola::new();
        parabola.set_focal_dist(5.0);
        assert_eq!(parabola.focal_dist(), 5.0);
    }

    #[test]
    fn test_default() {
        let parabola = StepGeomParabola::default();
        assert_eq!(parabola.focal_dist(), 0.0);
    }
}
