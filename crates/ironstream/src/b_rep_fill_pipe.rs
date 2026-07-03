// FILE: b_rep_fill_pipe.rs
// occt: BRepFill_Pipe

#[derive(Debug, Clone)]
pub struct BRepFillPipe {
    spine_shape: Vec<f64>,
    profile_shape: Vec<f64>,
    result_shape: Vec<f64>,
    error_on_surface: f64,
    force_approx_c1: bool,
}

impl BRepFillPipe {
    pub fn new() -> Self {
        BRepFillPipe {
            spine_shape: Vec::new(),
            profile_shape: Vec::new(),
            result_shape: Vec::new(),
            error_on_surface: 0.0,
            force_approx_c1: false,
        }
    }

    pub fn with_shapes(spine: Vec<f64>, profile: Vec<f64>) -> Self {
        BRepFillPipe {
            spine_shape: spine,
            profile_shape: profile,
            result_shape: Vec::new(),
            error_on_surface: 0.0,
            force_approx_c1: false,
        }
    }

    pub fn perform(&mut self) {
        self.error_on_surface = 0.0;
    }

    pub fn spine(&self) -> &[f64] {
        &self.spine_shape
    }

    pub fn profile(&self) -> &[f64] {
        &self.profile_shape
    }

    pub fn shape(&self) -> &[f64] {
        &self.result_shape
    }

    pub fn error_on_surface(&self) -> f64 {
        self.error_on_surface
    }

    pub fn set_force_approx_c1(&mut self, force: bool) {
        self.force_approx_c1 = force;
    }
}

impl Default for BRepFillPipe {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_creation() {
        let pipe = BRepFillPipe::new();
        assert!(pipe.spine().is_empty());
        assert!(pipe.profile().is_empty());
        assert!(pipe.shape().is_empty());
    }

    #[test]
    fn test_pipe_with_shapes() {
        let spine = vec![0.0, 1.0, 2.0];
        let profile = vec![0.0, 1.0];
        let pipe = BRepFillPipe::with_shapes(spine.clone(), profile.clone());
        assert_eq!(pipe.spine(), &spine[..]);
        assert_eq!(pipe.profile(), &profile[..]);
    }

    #[test]
    fn test_pipe_error_on_surface() {
        let pipe = BRepFillPipe::new();
        assert_eq!(pipe.error_on_surface(), 0.0);
    }

    #[test]
    fn test_pipe_perform() {
        let mut pipe = BRepFillPipe::new();
        pipe.perform();
        assert_eq!(pipe.error_on_surface(), 0.0);
    }

    #[test]
    fn test_pipe_force_approx_c1() {
        let mut pipe = BRepFillPipe::new();
        pipe.set_force_approx_c1(true);
        assert!(pipe.force_approx_c1);
    }
}
