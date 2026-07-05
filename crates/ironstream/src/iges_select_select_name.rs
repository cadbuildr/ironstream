// FILE: iges_select_select_name.rs
// occt: IGESSelect_SelectName

/// Selects IGES entities based on their name.
/// Considers Property Name if present, else Short Label, but not the Subscript Number.
pub struct IgesSelectSelectName {
    filter_name: Option<String>,
}

impl IgesSelectSelectName {
    /// Creates an empty SelectName with no filter (all entities are considered good).
    pub fn new() -> Self {
        IgesSelectSelectName {
            filter_name: None,
        }
    }

    /// Sets a name as the selection criterium.
    ///
    /// # Arguments
    /// - `name`: The name to filter by
    pub fn set_name(&mut self, name: Option<String>) {
        self.filter_name = name;
    }

    /// Returns the name used as the filter.
    pub fn name(&self) -> Option<&str> {
        self.filter_name.as_deref()
    }

    /// Determines if an entity matches the name filter.
    ///
    /// # Arguments
    /// - `_rank`: The rank/index of the entity
    /// - `entity_name`: The name of the entity to check
    ///
    /// Returns true if the entity's name matches the filter (or no filter is set)
    pub fn sort(&self, _rank: i32, entity_name: Option<&str>) -> bool {
        match &self.filter_name {
            None => true, // No filter, accept all
            Some(filter) => entity_name.map_or(false, |name| name == filter),
        }
    }

    /// Returns the selection criterium description.
    pub fn extract_label(&self) -> String {
        match &self.filter_name {
            None => "IGES Entity".to_string(),
            Some(name) => format!("IGES Entity, Name : {}", name),
        }
    }
}

impl Default for IgesSelectSelectName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_name_creation() {
        let sn = IgesSelectSelectName::new();
        assert_eq!(sn.name(), None);
    }

    #[test]
    fn test_select_name_set() {
        let mut sn = IgesSelectSelectName::new();
        sn.set_name(Some("TestName".to_string()));
        assert_eq!(sn.name(), Some("TestName"));
    }

    #[test]
    fn test_select_name_sort_no_filter() {
        let sn = IgesSelectSelectName::new();
        assert!(sn.sort(0, Some("AnyName")));
        assert!(sn.sort(0, None));
    }

    #[test]
    fn test_select_name_sort_with_filter_matching() {
        let mut sn = IgesSelectSelectName::new();
        sn.set_name(Some("TestName".to_string()));
        assert!(sn.sort(0, Some("TestName")));
    }

    #[test]
    fn test_select_name_sort_with_filter_not_matching() {
        let mut sn = IgesSelectSelectName::new();
        sn.set_name(Some("TestName".to_string()));
        assert!(!sn.sort(0, Some("OtherName")));
        assert!(!sn.sort(0, None));
    }

    #[test]
    fn test_select_name_extract_label_no_filter() {
        let sn = IgesSelectSelectName::new();
        assert_eq!(sn.extract_label(), "IGES Entity".to_string());
    }

    #[test]
    fn test_select_name_extract_label_with_filter() {
        let mut sn = IgesSelectSelectName::new();
        sn.set_name(Some("Part001".to_string()));
        assert_eq!(
            sn.extract_label(),
            "IGES Entity, Name : Part001".to_string()
        );
    }
}
