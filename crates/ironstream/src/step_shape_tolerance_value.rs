// FILE: step_shape_tolerance_value.rs
// occt: StepShape_ToleranceValue

use std::sync::Arc;

/// Placeholder for a generic transient object
#[derive(Clone, Debug)]
pub struct Transient {
    id: usize,
}

impl Transient {
    pub fn new(id: usize) -> Self {
        Transient { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Represents a tolerance value in STEP format.
pub struct ToleranceValue {
    lower_bound: Option<Arc<Transient>>,
    upper_bound: Option<Arc<Transient>>,
}

impl ToleranceValue {
    /// Create a new ToleranceValue
    pub fn new() -> Self {
        ToleranceValue {
            lower_bound: None,
            upper_bound: None,
        }
    }

    /// Initialize with lower and upper bounds
    pub fn init(&mut self, lower_bound: Arc<Transient>, upper_bound: Arc<Transient>) {
        self.lower_bound = Some(lower_bound);
        self.upper_bound = Some(upper_bound);
    }

    /// Get the lower bound
    pub fn lower_bound(&self) -> Option<&Arc<Transient>> {
        self.lower_bound.as_ref()
    }

    /// Set the lower bound
    pub fn set_lower_bound(&mut self, lower_bound: Arc<Transient>) {
        self.lower_bound = Some(lower_bound);
    }

    /// Get the upper bound
    pub fn upper_bound(&self) -> Option<&Arc<Transient>> {
        self.upper_bound.as_ref()
    }

    /// Set the upper bound
    pub fn set_upper_bound(&mut self, upper_bound: Arc<Transient>) {
        self.upper_bound = Some(upper_bound);
    }
}

impl Default for ToleranceValue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolerance_value_creation() {
        let tv = ToleranceValue::new();
        assert!(tv.lower_bound().is_none());
        assert!(tv.upper_bound().is_none());
    }

    #[test]
    fn test_init_method() {
        let mut tv = ToleranceValue::new();
        let lower = Arc::new(Transient::new(1));
        let upper = Arc::new(Transient::new(2));

        tv.init(lower.clone(), upper.clone());

        assert!(tv.lower_bound().is_some());
        assert_eq!(tv.lower_bound().unwrap().id(), 1);
        assert!(tv.upper_bound().is_some());
        assert_eq!(tv.upper_bound().unwrap().id(), 2);
    }

    #[test]
    fn test_set_lower_bound() {
        let mut tv = ToleranceValue::new();
        let lower = Arc::new(Transient::new(10));

        tv.set_lower_bound(lower);

        assert!(tv.lower_bound().is_some());
        assert_eq!(tv.lower_bound().unwrap().id(), 10);
    }

    #[test]
    fn test_set_upper_bound() {
        let mut tv = ToleranceValue::new();
        let upper = Arc::new(Transient::new(20));

        tv.set_upper_bound(upper);

        assert!(tv.upper_bound().is_some());
        assert_eq!(tv.upper_bound().unwrap().id(), 20);
    }

    #[test]
    fn test_full_initialization() {
        let mut tv = ToleranceValue::new();
        let lower = Arc::new(Transient::new(5));
        let upper = Arc::new(Transient::new(15));

        tv.set_lower_bound(lower);
        tv.set_upper_bound(upper);

        assert_eq!(tv.lower_bound().unwrap().id(), 5);
        assert_eq!(tv.upper_bound().unwrap().id(), 15);
    }
}
