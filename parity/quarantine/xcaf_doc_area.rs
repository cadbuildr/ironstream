// FILE: xcaf_doc_area.rs
// occt: XCAFDoc_Area

/// Attribute to store area value.
/// This is a specialized attribute that extends the real value storage concept,
/// providing a convenient way to store area values within the XDE document structure.
#[derive(Debug, Clone, PartialEq)]
pub struct XCAFDoc_Area {
    value: f64,
}

impl XCAFDoc_Area {
    /// Static GUID for XCAFDoc_Area attribute type
    pub const GUID: &'static str = "efd212f2-6dfd-11d4-b9c8-0060b0ee281b";

    /// Creates a new instance with zero area
    pub fn new() -> Self {
        XCAFDoc_Area { value: 0.0 }
    }

    /// Creates a new area attribute with the given value
    pub fn with_value(value: f64) -> Self {
        XCAFDoc_Area { value }
    }

    /// Sets the area value
    pub fn set(&mut self, value: f64) {
        self.value = value;
    }

    /// Gets the area value
    pub fn get(&self) -> f64 {
        self.value
    }
}

impl Default for XCAFDoc_Area {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_creation() {
        let area = XCAFDoc_Area::new();
        assert_eq!(area.get(), 0.0);
    }

    #[test]
    fn test_area_with_value() {
        let area = XCAFDoc_Area::with_value(42.5);
        assert_eq!(area.get(), 42.5);
    }

    #[test]
    fn test_area_set() {
        let mut area = XCAFDoc_Area::new();
        area.set(100.0);
        assert_eq!(area.get(), 100.0);
    }

    #[test]
    fn test_area_default() {
        let area = XCAFDoc_Area::default();
        assert_eq!(area.get(), 0.0);
    }

    #[test]
    fn test_area_equality() {
        let area1 = XCAFDoc_Area::with_value(42.0);
        let area2 = XCAFDoc_Area::with_value(42.0);
        let area3 = XCAFDoc_Area::with_value(43.0);

        assert_eq!(area1, area2);
        assert_ne!(area1, area3);
    }
}
