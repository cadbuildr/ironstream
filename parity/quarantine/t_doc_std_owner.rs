// FILE: t_doc_std_owner.rs
// occt: TDocStd_Owner

/// An Owner attribute for tracking ownership of document elements.
#[derive(Clone, Debug)]
pub struct TDocStd_Owner {
    owner_name: String,
    id: [u8; 16],
}

impl TDocStd_Owner {
    /// Create a new Owner attribute.
    pub fn new() -> Self {
        Self {
            owner_name: String::new(),
            id: Self::get_id(),
        }
    }

    /// Create an Owner with a specific name.
    pub fn with_name(name: String) -> Self {
        Self {
            owner_name: name,
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Owner attributes.
    pub fn get_id() -> [u8; 16] {
        [
            0x2F, 0x3C, 0x4D, 0x5E, 0x6F, 0x70, 0x81, 0x92, 0xA3, 0xB4, 0xC5, 0xD6, 0x88, 0x22,
            0x22, 0x22,
        ]
    }

    /// Set the owner name.
    pub fn set_owner(&mut self, name: String) {
        self.owner_name = name;
    }

    /// Get the owner name.
    pub fn owner(&self) -> &str {
        &self.owner_name
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }
}

impl Default for TDocStd_Owner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_owner() {
        let owner = TDocStd_Owner::new();
        assert_eq!(owner.owner(), "");
    }

    #[test]
    fn test_create_with_name() {
        let owner = TDocStd_Owner::with_name("John".to_string());
        assert_eq!(owner.owner(), "John");
    }

    #[test]
    fn test_set_owner() {
        let mut owner = TDocStd_Owner::new();
        owner.set_owner("Alice".to_string());
        assert_eq!(owner.owner(), "Alice");
    }

    #[test]
    fn test_get_id() {
        let id = TDocStd_Owner::get_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_default() {
        let owner = TDocStd_Owner::default();
        assert_eq!(owner.owner(), "");
    }
}
