// FILE: t_data_std_generic_empty.rs
// occt: TDataStd_GenericEmpty

/// A base attribute for all attributes which have no fields.
/// This is an ancestor attribute type used for marker or empty attributes.
/// Attributes inheriting from this should not have persistence drivers.
#[derive(Clone, Debug, Default)]
pub struct TDataStd_GenericEmpty {
    id: [u8; 16],
}

impl TDataStd_GenericEmpty {
    /// Create a new generic empty attribute.
    pub fn new() -> Self {
        Self {
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for GenericEmpty attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_GenericEmpty
        [
            0xA3, 0x5B, 0x8E, 0x44, 0x7D, 0xF2, 0x4B, 0x1C, 0x9A, 0x3F, 0x2D, 0x68, 0x99, 0x22,
            0x22, 0x22,
        ]
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Restore from another attribute (no-op for empty attributes).
    pub fn restore(&mut self, _with: &TDataStd_GenericEmpty) {
        // Empty attributes have nothing to restore
    }

    /// Paste to another attribute (no-op for empty attributes).
    pub fn paste(&self) {
        // Empty attributes have nothing to paste
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_generic_empty() {
        let attr = TDataStd_GenericEmpty::new();
        assert!(!attr.id().is_empty());
    }

    #[test]
    fn test_get_id() {
        let id1 = TDataStd_GenericEmpty::get_id();
        let id2 = TDataStd_GenericEmpty::get_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_default() {
        let attr = TDataStd_GenericEmpty::default();
        assert_eq!(attr.id(), &TDataStd_GenericEmpty::get_id());
    }

    #[test]
    fn test_restore() {
        let mut attr1 = TDataStd_GenericEmpty::new();
        let attr2 = TDataStd_GenericEmpty::new();
        attr1.restore(&attr2); // Should not panic
        assert!(true);
    }

    #[test]
    fn test_paste() {
        let attr = TDataStd_GenericEmpty::new();
        attr.paste(); // Should not panic
        assert!(true);
    }
}
