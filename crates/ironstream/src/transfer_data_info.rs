// FILE: transfer_data_info.rs
// occt: Transfer_DataInfo

/// Information structure for transfer data objects.
/// Provides metadata about transferred entities.
#[derive(Clone, Debug)]
pub struct TransferDataInfo {
    /// Entity type name
    type_name: String,
    /// Entity identifier
    entity_id: u32,
    /// Number of sub-entities
    nb_sub_entities: u32,
}

impl TransferDataInfo {
    /// Creates a new data info structure.
    pub fn new(type_name: &str, entity_id: u32) -> Self {
        Self {
            type_name: String::from(type_name),
            entity_id,
            nb_sub_entities: 0,
        }
    }

    /// Returns the type name.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the entity ID.
    pub fn entity_id(&self) -> u32 {
        self.entity_id
    }

    /// Returns the number of sub-entities.
    pub fn nb_sub_entities(&self) -> u32 {
        self.nb_sub_entities
    }

    /// Sets the number of sub-entities.
    pub fn set_nb_sub_entities(&mut self, count: u32) {
        self.nb_sub_entities = count;
    }
}

impl Default for TransferDataInfo {
    fn default() -> Self {
        Self::new("Unknown", 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = TransferDataInfo::new("Shape", 42);
        assert_eq!(info.type_name(), "Shape");
        assert_eq!(info.entity_id(), 42);
        assert_eq!(info.nb_sub_entities(), 0);
    }

    #[test]
    fn test_set_nb_sub_entities() {
        let mut info = TransferDataInfo::new("Compound", 10);
        info.set_nb_sub_entities(5);
        assert_eq!(info.nb_sub_entities(), 5);
    }

    #[test]
    fn test_default() {
        let info = TransferDataInfo::default();
        assert_eq!(info.type_name(), "Unknown");
        assert_eq!(info.entity_id(), 0);
    }
}
