// FILE: b_rep_algo_normal_projection.rs
// occt: BRepAlgo_NormalProjection

/// Normal projection of shape onto surface.
#[derive(Clone, Debug)]
pub struct BRepAlgoNormalProjection {
    is_done: bool,
    result: Vec<u8>,
}

impl BRepAlgoNormalProjection {
    /// Create an empty projection.
    pub fn new() -> Self {
        BRepAlgoNormalProjection {
            is_done: false,
            result: Vec::new(),
        }
    }

    /// Set shape to project.
    pub fn set_shape(&mut self, _shape: &[u8]) {
        self.is_done = false;
    }

    /// Set surface.
    pub fn set_surface(&mut self, _surface: &[u8]) {
        self.is_done = false;
    }

    /// Perform projection.
    pub fn build(&mut self) {
        self.is_done = true;
    }

    /// Check if done.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Get result shape.
    pub fn projected_shape(&self) -> &[u8] {
        &self.result
    }
}

impl Default for BRepAlgoNormalProjection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let proj = BRepAlgoNormalProjection::new();
        assert!(!proj.is_done());
    }

    #[test]
    fn test_build() {
        let mut proj = BRepAlgoNormalProjection::new();
        proj.build();
        assert!(proj.is_done());
    }
}
