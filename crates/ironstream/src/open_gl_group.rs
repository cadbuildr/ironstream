// FILE: open_gl_group.rs
// occt: OpenGl_Group

/// OpenGL group of graphic elements for hierarchical rendering.
#[derive(Debug, Clone)]
pub struct OpenGlGroup {
    is_empty: bool,
    has_elements: bool,
}

impl OpenGlGroup {
    /// Creates a new OpenGL group.
    pub fn new() -> Self {
        OpenGlGroup {
            is_empty: true,
            has_elements: false,
        }
    }

    /// Checks if the group is empty.
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Adds an element to the group.
    pub fn add_element(&mut self) {
        self.is_empty = false;
        self.has_elements = true;
    }

    /// Clears all elements from the group.
    pub fn clear(&mut self) {
        self.is_empty = true;
        self.has_elements = false;
    }

    /// Gets element count indicator.
    pub fn has_elements(&self) -> bool {
        self.has_elements
    }
}

impl Default for OpenGlGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_creation() {
        let group = OpenGlGroup::new();
        assert!(group.is_empty());
        assert!(!group.has_elements());
    }

    #[test]
    fn test_group_add_element() {
        let mut group = OpenGlGroup::new();
        group.add_element();
        assert!(!group.is_empty());
        assert!(group.has_elements());
    }

    #[test]
    fn test_group_clear() {
        let mut group = OpenGlGroup::new();
        group.add_element();
        group.clear();
        assert!(group.is_empty());
        assert!(!group.has_elements());
    }
}
