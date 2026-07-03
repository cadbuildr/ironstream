// FILE: geom_fill_plan_func.rs
// occt: GeomFill_PlanFunc

/// Plan function for surface generation
pub struct PlanFunc;

impl PlanFunc {
    pub fn new() -> Self {
        PlanFunc
    }
}

impl Default for PlanFunc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = PlanFunc::new();
    }

    #[test]
    fn test_default() {
        let _func = PlanFunc::default();
    }
}
