// FILE: geom_fill_guide_trihedron_plan.rs
// occt: GeomFill_GuideTrihedronPlan

/// Guide trihedron with plan constraint
pub struct GuideTrihedronPlan;

impl GuideTrihedronPlan {
    pub fn new() -> Self {
        GuideTrihedronPlan
    }
}

impl Default for GuideTrihedronPlan {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _plan = GuideTrihedronPlan::new();
    }

    #[test]
    fn test_default() {
        let _plan = GuideTrihedronPlan::default();
    }
}
