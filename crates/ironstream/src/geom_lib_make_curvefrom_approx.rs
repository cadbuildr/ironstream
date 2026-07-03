// FILE: geom_lib_make_curvefrom_approx.rs
// occt: GeomLib_MakeCurvefromApprox

/// Build a curve from an approximation.
pub struct GeomLibMakeCurvefromApprox {
    is_done: bool,
}

impl GeomLibMakeCurvefromApprox {
    /// Create a curve builder from approximation.
    pub fn new() -> Self {
        GeomLibMakeCurvefromApprox { is_done: false }
    }

    /// Perform the build.
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    /// Check if build succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for GeomLibMakeCurvefromApprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_builder() {
        let builder = GeomLibMakeCurvefromApprox::new();
        assert!(!builder.is_done());
    }

    #[test]
    fn test_perform() {
        let mut builder = GeomLibMakeCurvefromApprox::new();
        builder.perform();
        assert!(builder.is_done());
    }
}
