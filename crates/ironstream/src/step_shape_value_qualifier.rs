// FILE: step_shape_value_qualifier.rs
// occt: StepShape_ValueQualifier

use std::sync::Arc;

/// Placeholder for StepShape_PrecisionQualifier
pub struct PrecisionQualifier {
    value: i32,
}

impl PrecisionQualifier {
    pub fn new(value: i32) -> Self {
        PrecisionQualifier { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

/// Placeholder for StepShape_TypeQualifier
pub struct TypeQualifier {
    name: Arc<str>,
}

impl TypeQualifier {
    pub fn new(name: Arc<str>) -> Self {
        TypeQualifier { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Placeholder for StepShape_ValueFormatTypeQualifier
pub struct ValueFormatTypeQualifier {
    format: Arc<str>,
}

impl ValueFormatTypeQualifier {
    pub fn new(format: Arc<str>) -> Self {
        ValueFormatTypeQualifier { format }
    }

    pub fn format(&self) -> &str {
        &self.format
    }
}

/// A discriminated union type for value qualifiers.
/// Can be PrecisionQualifier, TypeQualifier, or ValueFormatTypeQualifier.
pub enum ValueQualifier {
    /// Case 1: PrecisionQualifier
    PrecisionQualifier(Arc<PrecisionQualifier>),
    /// Case 2: TypeQualifier
    TypeQualifier(Arc<TypeQualifier>),
    /// Case 4: ValueFormatTypeQualifier
    ValueFormatTypeQualifier(Arc<ValueFormatTypeQualifier>),
}

impl ValueQualifier {
    /// Create from a PrecisionQualifier
    pub fn from_precision(qualifier: Arc<PrecisionQualifier>) -> Self {
        ValueQualifier::PrecisionQualifier(qualifier)
    }

    /// Create from a TypeQualifier
    pub fn from_type(qualifier: Arc<TypeQualifier>) -> Self {
        ValueQualifier::TypeQualifier(qualifier)
    }

    /// Create from a ValueFormatTypeQualifier
    pub fn from_value_format_type(qualifier: Arc<ValueFormatTypeQualifier>) -> Self {
        ValueQualifier::ValueFormatTypeQualifier(qualifier)
    }

    /// Get the case number (kind) of this qualifier
    /// 1 -> PrecisionQualifier
    /// 2 -> TypeQualifier
    /// 4 -> ValueFormatTypeQualifier
    pub fn case_num(&self) -> usize {
        match self {
            ValueQualifier::PrecisionQualifier(_) => 1,
            ValueQualifier::TypeQualifier(_) => 2,
            ValueQualifier::ValueFormatTypeQualifier(_) => 4,
        }
    }

    /// Try to get as PrecisionQualifier
    pub fn as_precision(&self) -> Option<&Arc<PrecisionQualifier>> {
        match self {
            ValueQualifier::PrecisionQualifier(q) => Some(q),
            _ => None,
        }
    }

    /// Try to get as TypeQualifier
    pub fn as_type(&self) -> Option<&Arc<TypeQualifier>> {
        match self {
            ValueQualifier::TypeQualifier(q) => Some(q),
            _ => None,
        }
    }

    /// Try to get as ValueFormatTypeQualifier
    pub fn as_value_format_type(&self) -> Option<&Arc<ValueFormatTypeQualifier>> {
        match self {
            ValueQualifier::ValueFormatTypeQualifier(q) => Some(q),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num_precision() {
        let qualifier = Arc::new(PrecisionQualifier::new(5));
        let vq = ValueQualifier::from_precision(qualifier);
        assert_eq!(vq.case_num(), 1);
    }

    #[test]
    fn test_case_num_type() {
        let qualifier = Arc::new(TypeQualifier::new(Arc::from("type_1")));
        let vq = ValueQualifier::from_type(qualifier);
        assert_eq!(vq.case_num(), 2);
    }

    #[test]
    fn test_case_num_value_format_type() {
        let qualifier = Arc::new(ValueFormatTypeQualifier::new(Arc::from("format")));
        let vq = ValueQualifier::from_value_format_type(qualifier);
        assert_eq!(vq.case_num(), 4);
    }

    #[test]
    fn test_as_precision() {
        let qualifier = Arc::new(PrecisionQualifier::new(10));
        let vq = ValueQualifier::from_precision(qualifier.clone());
        assert!(vq.as_precision().is_some());
        assert_eq!(vq.as_precision().unwrap().value(), 10);
        assert!(vq.as_type().is_none());
    }

    #[test]
    fn test_as_type() {
        let qualifier = Arc::new(TypeQualifier::new(Arc::from("my_type")));
        let vq = ValueQualifier::from_type(qualifier.clone());
        assert!(vq.as_type().is_some());
        assert_eq!(vq.as_type().unwrap().name(), "my_type");
        assert!(vq.as_precision().is_none());
    }

    #[test]
    fn test_as_value_format_type() {
        let qualifier = Arc::new(ValueFormatTypeQualifier::new(Arc::from("INTEGER")));
        let vq = ValueQualifier::from_value_format_type(qualifier.clone());
        assert!(vq.as_value_format_type().is_some());
        assert_eq!(vq.as_value_format_type().unwrap().format(), "INTEGER");
        assert!(vq.as_type().is_none());
    }

    #[test]
    fn test_multiple_qualifiers() {
        let p = ValueQualifier::from_precision(Arc::new(PrecisionQualifier::new(1)));
        let t = ValueQualifier::from_type(Arc::new(TypeQualifier::new(Arc::from("t"))));
        let v = ValueQualifier::from_value_format_type(Arc::new(ValueFormatTypeQualifier::new(Arc::from("v"))));

        assert_eq!(p.case_num(), 1);
        assert_eq!(t.case_num(), 2);
        assert_eq!(v.case_num(), 4);
    }
}
