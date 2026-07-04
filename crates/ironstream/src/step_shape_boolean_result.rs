// FILE: step_shape_boolean_result.rs
// occt: StepShape_BooleanResult

/// Boolean operator enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanOperator {
    Difference,
    Intersection,
    Union,
}

/// Boolean operand type
#[derive(Clone, Debug, PartialEq)]
pub struct BooleanOperand {
    id: String,
}

/// Represents the result of a boolean operation in STEP
pub struct BooleanResult {
    name: Option<String>,
    operator: Option<BooleanOperator>,
    first_operand: Option<BooleanOperand>,
    second_operand: Option<BooleanOperand>,
}

impl BooleanResult {
    /// Create a new BooleanResult
    pub fn new() -> Self {
        BooleanResult {
            name: None,
            operator: None,
            first_operand: None,
            second_operand: None,
        }
    }

    /// Initialize with name, operator, and operands
    pub fn init(
        &mut self,
        name: String,
        operator: BooleanOperator,
        first_operand: BooleanOperand,
        second_operand: BooleanOperand,
    ) {
        self.name = Some(name);
        self.operator = Some(operator);
        self.first_operand = Some(first_operand);
        self.second_operand = Some(second_operand);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Set the operator
    pub fn set_operator(&mut self, operator: BooleanOperator) {
        self.operator = Some(operator);
    }

    /// Get the operator
    pub fn operator(&self) -> Option<BooleanOperator> {
        self.operator
    }

    /// Set the first operand
    pub fn set_first_operand(&mut self, operand: BooleanOperand) {
        self.first_operand = Some(operand);
    }

    /// Get the first operand
    pub fn first_operand(&self) -> Option<&BooleanOperand> {
        self.first_operand.as_ref()
    }

    /// Set the second operand
    pub fn set_second_operand(&mut self, operand: BooleanOperand) {
        self.second_operand = Some(operand);
    }

    /// Get the second operand
    pub fn second_operand(&self) -> Option<&BooleanOperand> {
        self.second_operand.as_ref()
    }
}

impl Default for BooleanResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let result = BooleanResult::new();
        assert_eq!(result.name(), None);
        assert_eq!(result.operator(), None);
        assert_eq!(result.first_operand(), None);
    }

    #[test]
    fn test_init() {
        let mut result = BooleanResult::new();
        let op1 = BooleanOperand { id: "op1".to_string() };
        let op2 = BooleanOperand { id: "op2".to_string() };
        result.init(
            "BoolResult1".to_string(),
            BooleanOperator::Union,
            op1.clone(),
            op2.clone(),
        );
        assert_eq!(result.name(), Some("BoolResult1"));
        assert_eq!(result.operator(), Some(BooleanOperator::Union));
        assert_eq!(result.first_operand(), Some(&op1));
        assert_eq!(result.second_operand(), Some(&op2));
    }

    #[test]
    fn test_set_operator() {
        let mut result = BooleanResult::new();
        result.set_operator(BooleanOperator::Difference);
        assert_eq!(result.operator(), Some(BooleanOperator::Difference));
    }
}
