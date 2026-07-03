// FILE: geom_fill_app_sweep.rs
// occt: GeomFill_AppSweep

/// Approximates a sweep surface
pub struct AppSweep {
    done: bool,
}

impl AppSweep {
    pub fn new() -> Self {
        AppSweep { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for AppSweep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let sweep = AppSweep::new();
        assert!(!sweep.is_done());
    }

    #[test]
    fn test_default() {
        let _sweep = AppSweep::default();
    }
}
