// FILE: interface_undefined_content.rs
// occt: Interface_UndefinedContent

/// Represents undefined content
pub struct InterfaceUndefinedContent {
    content: Vec<u8>,
}

impl InterfaceUndefinedContent {
    pub fn new() -> Self {
        InterfaceUndefinedContent {
            content: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        InterfaceUndefinedContent {
            content: Vec::with_capacity(cap),
        }
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.content.push(byte);
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl Default for InterfaceUndefinedContent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let content = InterfaceUndefinedContent::new();
        assert!(content.is_empty());
    }

    #[test]
    fn test_add_byte() {
        let mut content = InterfaceUndefinedContent::new();
        content.add_byte(42);
        assert_eq!(content.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut content = InterfaceUndefinedContent::new();
        content.add_byte(1);
        content.clear();
        assert!(content.is_empty());
    }
}
