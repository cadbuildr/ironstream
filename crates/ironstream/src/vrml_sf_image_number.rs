// FILE: vrml_sf_image_number.rs
// occt: Vrml_SFImageNumber
//
// Faithful port of OCCT Vrml_SFImageNumber (DataExchange/TKDEVRML/Vrml/
// Vrml_SFImageNumber.hxx/.cxx): a single field for image numbers in VRML.

/// Port of Vrml_SFImageNumber.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VrmlSfImageNumber {
    value: u32,
}

impl VrmlSfImageNumber {
    /// Vrml_SFImageNumber with default value 0.
    pub fn new() -> Self {
        VrmlSfImageNumber { value: 0 }
    }

    /// Vrml_SFImageNumber(aValue).
    pub fn with_value(a_value: u32) -> Self {
        VrmlSfImageNumber { value: a_value }
    }

    pub fn set_value(&mut self, a_value: u32) {
        self.value = a_value;
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Default for VrmlSfImageNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_value() {
        let num = VrmlSfImageNumber::new();
        assert_eq!(num.value(), 0);
    }

    #[test]
    fn with_value() {
        let num = VrmlSfImageNumber::with_value(42);
        assert_eq!(num.value(), 42);
    }

    #[test]
    fn set_value() {
        let mut num = VrmlSfImageNumber::new();
        num.set_value(123);
        assert_eq!(num.value(), 123);
    }

    #[test]
    fn equality() {
        let a = VrmlSfImageNumber::with_value(99);
        let b = VrmlSfImageNumber::with_value(99);
        let c = VrmlSfImageNumber::with_value(100);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
