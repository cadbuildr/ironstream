// FILE: std_persistent_data_xtd_constraint.rs
// occt: StdPersistent_DataXtd_Constraint

/// Constraint persistence for extended attributes
pub struct Constraint {
    constraint_type: i32,
    value: f64,
}

impl Constraint {
    /// Create a new constraint
    pub fn new(constraint_type: i32, value: f64) -> Self {
        Constraint {
            constraint_type,
            value,
        }
    }

    /// Get constraint type
    pub fn constraint_type(&self) -> i32 {
        self.constraint_type
    }

    /// Get constraint value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set constraint value
    pub fn set_value(&mut self, val: f64) {
        self.value = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let constraint = Constraint::new(1, 10.0);
        assert_eq!(constraint.constraint_type(), 1);
        assert_eq!(constraint.value(), 10.0);
    }

    #[test]
    fn test_set_value() {
        let mut constraint = Constraint::new(1, 10.0);
        constraint.set_value(20.0);
        assert_eq!(constraint.value(), 20.0);
    }
}
