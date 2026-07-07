// FILE: expr_operators.rs
// occt: Expr_Operators

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl ExprOperator {
    pub fn apply(&self, left: f64, right: f64) -> Option<f64> {
        match self {
            Self::Add => Some(left + right),
            Self::Subtract => Some(left - right),
            Self::Multiply => Some(left * right),
            Self::Divide => if right != 0.0 { Some(left / right) } else { None },
            Self::Power => Some(left.powf(right)),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Power => "^",
            Self::Equal => "=",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply() {
        assert_eq!(ExprOperator::Add.apply(2.0, 3.0), Some(5.0));
        assert_eq!(ExprOperator::Divide.apply(10.0, 0.0), None);
    }
    #[test]
    fn test_str() {
        assert_eq!(ExprOperator::Add.as_str(), "+");
    }
}
