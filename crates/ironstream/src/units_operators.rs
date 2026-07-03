// FILE: units_operators.rs
// occt: Units_Operators

/// Operators for manipulating units and dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitsOperator {
    /// Addition
    Add,
    /// Subtraction
    Subtract,
    /// Multiplication
    Multiply,
    /// Division
    Divide,
    /// Power
    Power,
}

impl UnitsOperator {
    /// Apply the operator to two values
    pub fn apply(self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            UnitsOperator::Add => Ok(left + right),
            UnitsOperator::Subtract => Ok(left - right),
            UnitsOperator::Multiply => Ok(left * right),
            UnitsOperator::Divide => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
            UnitsOperator::Power => Ok(left.powf(right)),
        }
    }

    /// Get the symbol for the operator
    pub fn symbol(self) -> &'static str {
        match self {
            UnitsOperator::Add => "+",
            UnitsOperator::Subtract => "-",
            UnitsOperator::Multiply => "*",
            UnitsOperator::Divide => "/",
            UnitsOperator::Power => "^",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_add() {
        let result = UnitsOperator::Add.apply(3.0, 4.0);
        assert_eq!(result, Ok(7.0));
    }

    #[test]
    fn test_operator_multiply() {
        let result = UnitsOperator::Multiply.apply(3.0, 4.0);
        assert_eq!(result, Ok(12.0));
    }

    #[test]
    fn test_operator_divide_by_zero() {
        let result = UnitsOperator::Divide.apply(3.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_operator_symbol() {
        assert_eq!(UnitsOperator::Add.symbol(), "+");
        assert_eq!(UnitsOperator::Multiply.symbol(), "*");
    }
}
