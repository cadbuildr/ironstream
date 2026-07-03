// FILE: app_blend_approx.rs
// occt: AppBlend_Approx

/// Blend surface approximation
#[derive(Clone, Debug)]
pub struct AppBlendApprox {
    degree_u: usize,
    degree_v: usize,
    is_done: bool,
}

impl AppBlendApprox {
    pub fn new() -> Self {
        AppBlendApprox { degree_u: 3, degree_v: 3, is_done: false }
    }

    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn degree_u(&self) -> usize {
        self.degree_u
    }

    pub fn degree_v(&self) -> usize {
        self.degree_v
    }
}

impl Default for AppBlendApprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let approx = AppBlendApprox::new();
        assert!(!approx.is_done());
    }

    #[test]
    fn test_perform() {
        let mut approx = AppBlendApprox::new();
        approx.perform();
        assert!(approx.is_done());
    }
}
