// FILE: interface_category.rs
// occt: Interface_Category

/// Manages entity category classification.
#[derive(Clone, Debug)]
pub struct InterfaceCategory {
    category_id: usize,
    name: String,
}

impl InterfaceCategory {
    /// Creates a Category
    pub fn new(id: usize, name: String) -> Self {
        Self {
            category_id: id,
            name,
        }
    }

    /// Returns the category ID
    pub fn id(&self) -> usize {
        self.category_id
    }

    /// Returns the category name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let cat = InterfaceCategory::new(1, "Type1".to_string());
        assert_eq!(cat.id(), 1);
        assert_eq!(cat.name(), "Type1");
    }
}
