// FILE: t_data_std_generic_ext_string.rs
// occt: TDataStd_GenericExtString

/// Base attribute for all attributes with an ExtendedString field.
/// Provides functionality to have multiple attributes with different IDs on the same label.
#[derive(Clone, Debug)]
pub struct TDataStd_GenericExtString {
    string: String,
    id: [u8; 16],
}

impl TDataStd_GenericExtString {
    /// Create a new generic extended string attribute.
    pub fn new() -> Self {
        Self {
            string: String::new(),
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for GenericExtString attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_GenericExtString
        [
            0x52, 0x7F, 0xC3, 0x61, 0xA8, 0xE5, 0x4C, 0x2A, 0xB1, 0x4D, 0x3B, 0x7E, 0x88, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set the string value.
    pub fn set(&mut self, s: String) {
        self.string = s;
    }

    /// Get the string value.
    pub fn get(&self) -> &str {
        &self.string
    }

    /// Set the explicit user-defined GUID.
    pub fn set_id(&mut self, guid: [u8; 16]) {
        self.id = guid;
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Restore from another attribute.
    pub fn restore(&mut self, with: &TDataStd_GenericExtString) {
        self.string = with.string.clone();
        self.id = with.id;
    }

    /// Paste to another attribute.
    pub fn paste(&self) -> TDataStd_GenericExtString {
        TDataStd_GenericExtString {
            string: self.string.clone(),
            id: self.id,
        }
    }
}

impl Default for TDataStd_GenericExtString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_generic_ext_string() {
        let attr = TDataStd_GenericExtString::new();
        assert_eq!(attr.get(), "");
    }

    #[test]
    fn test_set_and_get() {
        let mut attr = TDataStd_GenericExtString::new();
        let test_string = "Hello World".to_string();
        attr.set(test_string.clone());
        assert_eq!(attr.get(), test_string);
    }

    #[test]
    fn test_set_id() {
        let mut attr = TDataStd_GenericExtString::new();
        let new_id = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        attr.set_id(new_id);
        assert_eq!(attr.id(), &new_id);
    }

    #[test]
    fn test_restore() {
        let mut attr1 = TDataStd_GenericExtString::new();
        attr1.set("test".to_string());

        let mut attr2 = TDataStd_GenericExtString::new();
        attr2.restore(&attr1);

        assert_eq!(attr2.get(), "test");
    }

    #[test]
    fn test_paste() {
        let mut attr1 = TDataStd_GenericExtString::new();
        attr1.set("pasted".to_string());

        let attr2 = attr1.paste();
        assert_eq!(attr2.get(), "pasted");
    }

    #[test]
    fn test_default() {
        let attr = TDataStd_GenericExtString::default();
        assert_eq!(attr.get(), "");
    }
}
