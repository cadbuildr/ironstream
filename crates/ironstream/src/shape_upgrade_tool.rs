// FILE: shape_upgrade_tool.rs
// occt: ShapeUpgrade_Tool

/// Tool is a root class for splitting classes.
/// Provides context for recording changes, basic precision value,
/// and limit (minimal and maximal) values for tolerances.
pub struct ShapeUpgradeTool {
    /// Context for reshaping
    context: Option<Box<ShapeBuildReShape>>,
    /// Basic precision value
    precision: f64,
    /// Minimal allowed tolerance
    min_tolerance: f64,
    /// Maximal allowed tolerance
    max_tolerance: f64,
}

/// Context for reshaping operations
#[derive(Clone, Debug)]
pub struct ShapeBuildReShape {
    id: i32,
}

impl ShapeBuildReShape {
    pub fn new(id: i32) -> Self {
        ShapeBuildReShape { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl ShapeUpgradeTool {
    /// Create a new empty tool
    pub fn new() -> Self {
        ShapeUpgradeTool {
            context: None,
            precision: 0.0001,
            min_tolerance: 1.0e-7,
            max_tolerance: 10.0,
        }
    }

    /// Copy all fields from another tool
    pub fn set(&mut self, other: &ShapeUpgradeTool) {
        self.precision = other.precision;
        self.min_tolerance = other.min_tolerance;
        self.max_tolerance = other.max_tolerance;
    }

    /// Set the context for reshaping
    pub fn set_context(&mut self, context: ShapeBuildReShape) {
        self.context = Some(Box::new(context));
    }

    /// Get the context
    pub fn context(&self) -> Option<&ShapeBuildReShape> {
        self.context.as_ref().map(|c| c.as_ref())
    }

    /// Set the basic precision value
    pub fn set_precision(&mut self, precision: f64) {
        self.precision = precision.max(1.0e-10);
    }

    /// Get the basic precision value
    pub fn precision(&self) -> f64 {
        self.precision
    }

    /// Set the minimal allowed tolerance
    pub fn set_min_tolerance(&mut self, min_tol: f64) {
        self.min_tolerance = min_tol.max(1.0e-10);
    }

    /// Get the minimal allowed tolerance
    pub fn min_tolerance(&self) -> f64 {
        self.min_tolerance
    }

    /// Set the maximal allowed tolerance
    pub fn set_max_tolerance(&mut self, max_tol: f64) {
        self.max_tolerance = max_tol;
    }

    /// Get the maximal allowed tolerance
    pub fn max_tolerance(&self) -> f64 {
        self.max_tolerance
    }

    /// Get tolerance limited by [min_tolerance, max_tolerance]
    pub fn limit_tolerance(&self, tolerance: f64) -> f64 {
        tolerance
            .max(self.min_tolerance)
            .min(self.max_tolerance)
    }
}

impl Default for ShapeUpgradeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tool() {
        let tool = ShapeUpgradeTool::new();
        assert!(tool.precision > 0.0);
        assert!(tool.min_tolerance > 0.0);
        assert!(tool.max_tolerance > 0.0);
    }

    #[test]
    fn test_set_precision() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_precision(0.001);
        assert_eq!(tool.precision(), 0.001);
    }

    #[test]
    fn test_set_min_tolerance() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_min_tolerance(0.0001);
        assert_eq!(tool.min_tolerance(), 0.0001);
    }

    #[test]
    fn test_set_max_tolerance() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_max_tolerance(100.0);
        assert_eq!(tool.max_tolerance(), 100.0);
    }

    #[test]
    fn test_limit_tolerance_within_range() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_min_tolerance(0.001);
        tool.set_max_tolerance(0.1);
        let limited = tool.limit_tolerance(0.01);
        assert_eq!(limited, 0.01);
    }

    #[test]
    fn test_limit_tolerance_below_min() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_min_tolerance(0.001);
        let limited = tool.limit_tolerance(0.0001);
        assert_eq!(limited, 0.001);
    }

    #[test]
    fn test_limit_tolerance_above_max() {
        let mut tool = ShapeUpgradeTool::new();
        tool.set_max_tolerance(0.1);
        let limited = tool.limit_tolerance(1.0);
        assert_eq!(limited, 0.1);
    }

    #[test]
    fn test_set_context() {
        let mut tool = ShapeUpgradeTool::new();
        let context = ShapeBuildReShape::new(99);
        tool.set_context(context);
        assert!(tool.context().is_some());
        assert_eq!(tool.context().unwrap().id(), 99);
    }

    #[test]
    fn test_set_tool_from_another() {
        let mut tool1 = ShapeUpgradeTool::new();
        tool1.set_precision(0.01);
        tool1.set_min_tolerance(0.001);

        let mut tool2 = ShapeUpgradeTool::new();
        tool2.set(&tool1);

        assert_eq!(tool2.precision(), 0.01);
        assert_eq!(tool2.min_tolerance(), 0.001);
    }

    #[test]
    fn test_default() {
        let tool = ShapeUpgradeTool::default();
        assert!(tool.precision > 0.0);
    }
}
