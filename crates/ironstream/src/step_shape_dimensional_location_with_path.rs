// FILE: step_shape_dimensional_location_with_path.rs
// occt: StepShape_DimensionalLocationWithPath

//! Representation of STEP entity DimensionalLocationWithPath

#[derive(Clone, Debug)]
pub struct DimensionalLocationWithPath {
    name: String,
    description: Option<String>,
    relating_shape_aspect: Option<String>,
    related_shape_aspect: Option<String>,
    path: Option<String>,
}

impl DimensionalLocationWithPath {
    /// Empty constructor
    pub fn new() -> Self {
        DimensionalLocationWithPath {
            name: String::new(),
            description: None,
            relating_shape_aspect: None,
            related_shape_aspect: None,
            path: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: Option<String>,
        relating_aspect: Option<String>,
        related_aspect: Option<String>,
        path: Option<String>,
    ) {
        self.name = name;
        self.description = description;
        self.relating_shape_aspect = relating_aspect;
        self.related_shape_aspect = related_aspect;
        self.path = path;
    }

    /// Returns field Path
    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    /// Set field Path
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns description field
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Set description field
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}

impl Default for DimensionalLocationWithPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = DimensionalLocationWithPath::new();
        assert_eq!(loc.name(), "");
        assert!(loc.path().is_none());
    }

    #[test]
    fn test_init() {
        let mut loc = DimensionalLocationWithPath::new();
        loc.init(
            "Location1".to_string(),
            Some("desc".to_string()),
            Some("aspect1".to_string()),
            Some("aspect2".to_string()),
            Some("path1".to_string()),
        );
        assert_eq!(loc.name(), "Location1");
        assert_eq!(loc.path(), &Some("path1".to_string()));
    }

    #[test]
    fn test_set_path() {
        let mut loc = DimensionalLocationWithPath::new();
        loc.set_path(Some("new_path".to_string()));
        assert_eq!(loc.path(), &Some("new_path".to_string()));
    }
}
