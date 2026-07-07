// FILE: tdf_attribute_iterator.rs
// occt: TDF_AttributeIterator

/// Iterates over the current valid attributes of a label.
/// Supports iteration including/excluding forgotten attributes.
/// TODO: In OCCT, works with TDF_Label and TDF_Attribute
pub struct TdfAttributeIterator {
    value: Option<()>, // TODO: occ::handle<TDF_Attribute>
    without_forgotten: bool,
}

impl TdfAttributeIterator {
    /// Creates an empty iterator.
    pub fn new() -> Self {
        TdfAttributeIterator {
            value: None,
            without_forgotten: true,
        }
    }

    /// Initializes the iterator on a label.
    /// TODO: Accept TDF_Label
    pub fn initialize(&mut self, without_forgotten: bool) {
        self.without_forgotten = without_forgotten;
        self.value = None;
    }

    /// Returns true if there is a current attribute.
    pub fn more(&self) -> bool {
        self.value.is_some()
    }

    /// Moves to the next attribute.
    pub fn next(&mut self) {
        // TODO: Implement proper traversal
        self.value = None;
    }

    /// Returns the current attribute value.
    /// TODO: Return occ::handle<TDF_Attribute>
    pub fn value(&self) -> Option<()> {
        self.value
    }

    /// Returns a pointer to the current attribute.
    /// TODO: Return *const TDF_Attribute
    pub fn ptr_value(&self) -> Option<()> {
        self.value
    }
}

impl Default for TdfAttributeIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_iterator_new() {
        let it = TdfAttributeIterator::new();
        assert!(!it.more());
    }

    #[test]
    fn test_attribute_iterator_initialize() {
        let mut it = TdfAttributeIterator::new();
        it.initialize(true);
        assert!(!it.more());
    }

    #[test]
    fn test_attribute_iterator_next() {
        let mut it = TdfAttributeIterator::new();
        it.next();
        assert!(!it.more());
    }

    #[test]
    fn test_attribute_iterator_value() {
        let it = TdfAttributeIterator::new();
        assert!(it.value().is_none());
    }

    #[test]
    fn test_attribute_iterator_ptr_value() {
        let it = TdfAttributeIterator::new();
        assert!(it.ptr_value().is_none());
    }

    #[test]
    fn test_attribute_iterator_without_forgotten() {
        let mut it = TdfAttributeIterator::new();
        it.initialize(false);
        assert!(!it.without_forgotten);
    }

    #[test]
    fn test_attribute_iterator_default() {
        let it = TdfAttributeIterator::default();
        assert!(!it.more());
    }
}
