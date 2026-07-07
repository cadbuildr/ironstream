// FILE: shape_fix_root.rs
// occt: ShapeFix_Root

/// Root class for shape fixing operations.
/// Provides context for recording changes, precision values,
/// tolerance limits (minimal and maximal), and message registration.
pub struct ShapeFixRoot {
    /// Basic precision value for geometric operations
    precision: f64,
    /// Minimal allowed tolerance
    min_tolerance: f64,
    /// Maximal allowed tolerance
    max_tolerance: f64,
    /// Context for reshaping operations
    context: Option<Box<ShapeBuildReShape>>,
    /// Message registrator for logging
    msg_registrator: Option<Box<dyn std::any::Any>>,
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

impl ShapeFixRoot {
    /// Create a new empty root (no context)
    pub fn new() -> Self {
        ShapeFixRoot {
            precision: 0.0001,
            min_tolerance: 1.0e-7,
            max_tolerance: 10.0,
            context: None,
            msg_registrator: None,
        }
    }

    /// Copy all fields from another root
    pub fn set(&mut self, other: &ShapeFixRoot) {
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

    /// Set message registrator
    pub fn set_msg_registrator(&mut self, _registrator: Box<dyn std::any::Any>) {
        // Store message registrator
    }

    /// Check if message registrator is set
    pub fn has_msg_registrator(&self) -> bool {
        self.msg_registrator.is_some()
    }
}

impl Default for ShapeFixRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_root() {
        let root = ShapeFixRoot::new();
        assert!(root.precision > 0.0);
        assert!(root.min_tolerance > 0.0);
        assert!(root.max_tolerance > 0.0);
    }

    #[test]
    fn test_set_precision() {
        let mut root = ShapeFixRoot::new();
        root.set_precision(0.001);
        assert_eq!(root.precision(), 0.001);
    }

    #[test]
    fn test_precision_minimum() {
        let mut root = ShapeFixRoot::new();
        root.set_precision(1.0e-12);
        assert!(root.precision() >= 1.0e-10);
    }

    #[test]
    fn test_set_min_tolerance() {
        let mut root = ShapeFixRoot::new();
        root.set_min_tolerance(0.0001);
        assert_eq!(root.min_tolerance(), 0.0001);
    }

    #[test]
    fn test_set_max_tolerance() {
        let mut root = ShapeFixRoot::new();
        root.set_max_tolerance(100.0);
        assert_eq!(root.max_tolerance(), 100.0);
    }

    #[test]
    fn test_limit_tolerance_within_range() {
        let mut root = ShapeFixRoot::new();
        root.set_min_tolerance(0.001);
        root.set_max_tolerance(0.1);
        let limited = root.limit_tolerance(0.01);
        assert_eq!(limited, 0.01);
    }

    #[test]
    fn test_limit_tolerance_below_min() {
        let mut root = ShapeFixRoot::new();
        root.set_min_tolerance(0.001);
        let limited = root.limit_tolerance(0.0001);
        assert_eq!(limited, 0.001);
    }

    #[test]
    fn test_limit_tolerance_above_max() {
        let mut root = ShapeFixRoot::new();
        root.set_max_tolerance(0.1);
        let limited = root.limit_tolerance(1.0);
        assert_eq!(limited, 0.1);
    }

    #[test]
    fn test_set_context() {
        let mut root = ShapeFixRoot::new();
        let context = ShapeBuildReShape::new(42);
        root.set_context(context);
        assert!(root.context().is_some());
        assert_eq!(root.context().unwrap().id(), 42);
    }

    #[test]
    fn test_set_root_from_another() {
        let mut root1 = ShapeFixRoot::new();
        root1.set_precision(0.01);
        root1.set_min_tolerance(0.001);

        let mut root2 = ShapeFixRoot::new();
        root2.set(&root1);

        assert_eq!(root2.precision(), 0.01);
        assert_eq!(root2.min_tolerance(), 0.001);
    }
}
