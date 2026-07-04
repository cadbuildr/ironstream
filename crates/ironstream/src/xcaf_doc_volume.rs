// FILE: xcaf_doc_volume.rs
// occt: XCAFDoc_Volume

/// Attribute to store volume value.
/// This is a specialized attribute that extends the real value storage concept,
/// providing a convenient way to store volume values within the XDE document structure.
#[derive(Debug, Clone, PartialEq)]
pub struct XCAFDoc_Volume {
    value: f64,
}

impl XCAFDoc_Volume {
    /// Static GUID for XCAFDoc_Volume attribute type
    pub const GUID: &'static str = "efd212f1-6dfd-11d4-b9c8-0060b0ee281b";

    /// Creates a new instance with zero volume
    pub fn new() -> Self {
        XCAFDoc_Volume { value: 0.0 }
    }

    /// Creates a new volume attribute with the given value
    pub fn with_value(value: f64) -> Self {
        XCAFDoc_Volume { value }
    }

    /// Sets the volume value
    pub fn set(&mut self, value: f64) {
        self.value = value;
    }

    /// Gets the volume value
    pub fn get(&self) -> f64 {
        self.value
    }
}

impl Default for XCAFDoc_Volume {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_creation() {
        let volume = XCAFDoc_Volume::new();
        assert_eq!(volume.get(), 0.0);
    }

    #[test]
    fn test_volume_with_value() {
        let volume = XCAFDoc_Volume::with_value(123.456);
        assert_eq!(volume.get(), 123.456);
    }

    #[test]
    fn test_volume_set() {
        let mut volume = XCAFDoc_Volume::new();
        volume.set(200.0);
        assert_eq!(volume.get(), 200.0);
    }

    #[test]
    fn test_volume_default() {
        let volume = XCAFDoc_Volume::default();
        assert_eq!(volume.get(), 0.0);
    }

    #[test]
    fn test_volume_equality() {
        let vol1 = XCAFDoc_Volume::with_value(100.0);
        let vol2 = XCAFDoc_Volume::with_value(100.0);
        let vol3 = XCAFDoc_Volume::with_value(101.0);

        assert_eq!(vol1, vol2);
        assert_ne!(vol1, vol3);
    }
}
