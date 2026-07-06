// FILE: step_shape_qualified_representation_item.rs
// occt: StepShape_QualifiedRepresentationItem

use std::sync::Arc;

/// Placeholder for StepShape_ValueQualifier
#[derive(Clone, Debug)]
pub struct ValueQualifier {
    value: String,
}

impl ValueQualifier {
    pub fn new(value: String) -> Self {
        ValueQualifier { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Placeholder for StepRepr_RepresentationItem base class
pub struct RepresentationItem {
    name: Arc<str>,
}

impl RepresentationItem {
    pub fn new(name: Arc<str>) -> Self {
        RepresentationItem { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a qualified representation item in STEP format.
/// Used for dimensional tolerances.
pub struct QualifiedRepresentationItem {
    name: Arc<str>,
    qualifiers: Vec<ValueQualifier>,
}

impl QualifiedRepresentationItem {
    /// Create a new QualifiedRepresentationItem
    pub fn new() -> Self {
        QualifiedRepresentationItem {
            name: Arc::from(""),
            qualifiers: Vec::new(),
        }
    }

    /// Initialize with name and qualifiers
    pub fn init(&mut self, name: Arc<str>, qualifiers: Vec<ValueQualifier>) {
        self.name = name;
        self.qualifiers = qualifiers;
    }

    /// Get all qualifiers
    pub fn qualifiers(&self) -> &[ValueQualifier] {
        &self.qualifiers
    }

    /// Get the number of qualifiers
    pub fn nb_qualifiers(&self) -> usize {
        self.qualifiers.len()
    }

    /// Set all qualifiers
    pub fn set_qualifiers(&mut self, qualifiers: Vec<ValueQualifier>) {
        self.qualifiers = qualifiers;
    }

    /// Get a qualifier by index (1-based as per OCCT convention)
    pub fn qualifiers_value(&self, num: usize) -> Option<ValueQualifier> {
        if num > 0 && num <= self.qualifiers.len() {
            Some(self.qualifiers[num - 1].clone())
        } else {
            None
        }
    }

    /// Set a qualifier at a specific index (1-based as per OCCT convention)
    pub fn set_qualifiers_value(&mut self, num: usize, qualifier: ValueQualifier) {
        if num > 0 && num <= self.qualifiers.len() {
            self.qualifiers[num - 1] = qualifier;
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }
}

impl Default for QualifiedRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualified_representation_item_creation() {
        let qri = QualifiedRepresentationItem::new();
        assert_eq!(qri.name(), "");
        assert_eq!(qri.nb_qualifiers(), 0);
    }

    #[test]
    fn test_init_method() {
        let mut qri = QualifiedRepresentationItem::new();
        let qualifiers = vec![
            ValueQualifier::new("qualifier1".to_string()),
            ValueQualifier::new("qualifier2".to_string()),
        ];
        let name: Arc<str> = Arc::from("item1");

        qri.init(name.clone(), qualifiers);

        assert_eq!(qri.name(), "item1");
        assert_eq!(qri.nb_qualifiers(), 2);
    }

    #[test]
    fn test_set_qualifiers() {
        let mut qri = QualifiedRepresentationItem::new();
        let qualifiers = vec![
            ValueQualifier::new("q1".to_string()),
            ValueQualifier::new("q2".to_string()),
            ValueQualifier::new("q3".to_string()),
        ];

        qri.set_qualifiers(qualifiers);
        assert_eq!(qri.nb_qualifiers(), 3);
    }

    #[test]
    fn test_qualifiers_value() {
        let mut qri = QualifiedRepresentationItem::new();
        let qualifiers = vec![
            ValueQualifier::new("first".to_string()),
            ValueQualifier::new("second".to_string()),
        ];

        qri.set_qualifiers(qualifiers);

        // 1-based indexing
        let q1 = qri.qualifiers_value(1);
        assert!(q1.is_some());
        assert_eq!(q1.unwrap().value(), "first");

        let q2 = qri.qualifiers_value(2);
        assert!(q2.is_some());
        assert_eq!(q2.unwrap().value(), "second");

        // Out of bounds
        let q_out = qri.qualifiers_value(3);
        assert!(q_out.is_none());
    }

    #[test]
    fn test_set_qualifiers_value() {
        let mut qri = QualifiedRepresentationItem::new();
        let qualifiers = vec![
            ValueQualifier::new("old".to_string()),
            ValueQualifier::new("value".to_string()),
        ];

        qri.set_qualifiers(qualifiers);
        qri.set_qualifiers_value(1, ValueQualifier::new("new".to_string()));

        let q1 = qri.qualifiers_value(1);
        assert!(q1.is_some());
        assert_eq!(q1.unwrap().value(), "new");
    }
}
