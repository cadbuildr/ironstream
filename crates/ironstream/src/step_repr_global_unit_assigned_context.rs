// FILE: step_repr_global_unit_assigned_context.rs
// occt: StepRepr_GlobalUnitAssignedContext

/// StepRepr_GlobalUnitAssignedContext: Context with global unit assigned
/// Inherits from StepRepr_RepresentationContext
#[derive(Clone, Debug)]
pub struct StepReprGlobalUnitAssignedContext {
    context_identifier: String,
    context_type: String,
    units: Vec<String>, // Simplified: storing unit identifiers
}

impl StepReprGlobalUnitAssignedContext {
    /// Returns a GlobalUnitAssignedContext
    pub fn new() -> Self {
        StepReprGlobalUnitAssignedContext {
            context_identifier: String::new(),
            context_type: String::new(),
            units: Vec::new(),
        }
    }

    /// Initialize with all fields
    pub fn init(&mut self, context_id: String, context_type: String, units: Vec<String>) {
        self.context_identifier = context_id;
        self.context_type = context_type;
        self.units = units;
    }

    /// Set units
    pub fn set_units(&mut self, units: Vec<String>) {
        self.units = units;
    }

    /// Get units
    pub fn units(&self) -> &[String] {
        &self.units
    }

    /// Get units value by index (1-based)
    pub fn units_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.units.len() {
            Some(&self.units[num - 1])
        } else {
            None
        }
    }

    /// Get number of units
    pub fn nb_units(&self) -> usize {
        self.units.len()
    }

    /// Get context identifier
    pub fn context_identifier(&self) -> &str {
        &self.context_identifier
    }

    /// Get context type
    pub fn context_type(&self) -> &str {
        &self.context_type
    }
}

impl Default for StepReprGlobalUnitAssignedContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ctx = StepReprGlobalUnitAssignedContext::new();
        assert_eq!(ctx.context_identifier(), "");
        assert_eq!(ctx.context_type(), "");
        assert_eq!(ctx.nb_units(), 0);
    }

    #[test]
    fn test_init() {
        let mut ctx = StepReprGlobalUnitAssignedContext::new();
        let units = vec!["meter".to_string(), "second".to_string()];
        ctx.init("ctx1".to_string(), "type1".to_string(), units);
        assert_eq!(ctx.context_identifier(), "ctx1");
        assert_eq!(ctx.context_type(), "type1");
        assert_eq!(ctx.nb_units(), 2);
    }

    #[test]
    fn test_units_value() {
        let mut ctx = StepReprGlobalUnitAssignedContext::new();
        let units = vec!["unit1".to_string(), "unit2".to_string()];
        ctx.set_units(units);
        assert_eq!(ctx.units_value(1), Some(&"unit1".to_string()));
        assert_eq!(ctx.units_value(2), Some(&"unit2".to_string()));
        assert_eq!(ctx.units_value(3), None);
    }

    #[test]
    fn test_set_units() {
        let mut ctx = StepReprGlobalUnitAssignedContext::new();
        assert_eq!(ctx.nb_units(), 0);
        let units = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        ctx.set_units(units);
        assert_eq!(ctx.nb_units(), 3);
    }
}
