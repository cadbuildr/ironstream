// FILE: api_header_section_edit_header.rs
// occt: APIHeaderSection_EditHeader

use std::collections::HashMap;

/// Validates if a string is a valid STEP timestamp in format YYYY-MM-DDThh:mm:ss
fn is_timestamp(val: &str) -> bool {
    // Must be exactly 19 characters long
    if val.len() != 19 {
        return false;
    }

    let chars: Vec<char> = val.chars().collect();
    let dizmois = chars[5]; // month tens digit (at position 6, 0-indexed as 5)
    let dizjour = chars[8]; // day tens digit (at position 9, 0-indexed as 8)
    let dizheur = chars[11]; // hour tens digit (at position 12, 0-indexed as 11)

    for i in 0..19 {
        let c = chars[i];
        match i {
            0 => {
                // First digit of year: 1 or 2
                if c != '1' && c != '2' {
                    return false;
                }
            }
            1..=3 => {
                // Year digits: 0-9
                if !c.is_ascii_digit() {
                    return false;
                }
            }
            4 => {
                // Separator: -
                if c != '-' {
                    return false;
                }
            }
            5 => {
                // Month tens: 0 or 1
                if c != '0' && c != '1' {
                    return false;
                }
            }
            6 => {
                // Month ones
                if !c.is_ascii_digit() {
                    return false;
                }
                if dizmois == '1' && (c < '0' || c > '2') {
                    return false;
                }
            }
            7 => {
                // Separator: -
                if c != '-' {
                    return false;
                }
            }
            8 => {
                // Day tens: 0-3
                if c < '0' || c > '3' {
                    return false;
                }
            }
            9 => {
                // Day ones
                if !c.is_ascii_digit() {
                    return false;
                }
                if dizjour == '3' && c != '0' && c != '1' {
                    return false;
                }
            }
            10 => {
                // T separator
                if c != 'T' {
                    return false;
                }
            }
            11 => {
                // Hour tens: 0-2
                if c < '0' || c > '2' {
                    return false;
                }
            }
            12 => {
                // Hour ones
                if !c.is_ascii_digit() {
                    return false;
                }
                if dizheur == '2' && (c < '0' || c > '3') {
                    return false;
                }
            }
            13 => {
                // Colon
                if c != ':' {
                    return false;
                }
            }
            14 => {
                // Minute tens: 0-5
                if c < '0' || c > '5' {
                    return false;
                }
            }
            15 => {
                // Minute ones
                if !c.is_ascii_digit() {
                    return false;
                }
            }
            16 => {
                // Colon
                if c != ':' {
                    return false;
                }
            }
            17 => {
                // Second tens: 0-5
                if c < '0' || c > '5' {
                    return false;
                }
            }
            18 => {
                // Second ones
                if !c.is_ascii_digit() {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

/// Editor for STEP header sections, managing 10 fields:
/// 1. filename
/// 2. timestamp
/// 3. author (first from list)
/// 4. organization (first from list)
/// 5. preprocessor_version
/// 6. originating_system
/// 7. authorization
/// 8. schema_identifiers (first from list)
/// 9. description (first from list)
/// 10. implementation_level
pub struct APIHeaderSection_EditHeader {
    values: HashMap<usize, String>,
}

impl APIHeaderSection_EditHeader {
    /// Creates a new editor with 10 fields
    pub fn new() -> Self {
        APIHeaderSection_EditHeader {
            values: HashMap::new(),
        }
    }

    /// Returns the label for this editor
    pub fn label(&self) -> String {
        "Step Header".to_string()
    }

    /// Always recognizes forms in this implementation
    pub fn recognize(&self, _form: &()) -> bool {
        true
    }

    /// Gets a typed value by field number (1-10)
    pub fn typed_value(&self, num: usize) -> Option<String> {
        self.values.get(&num).cloned()
    }

    /// Gets the HStringValue for a field (returns stored value or default)
    pub fn string_value(&self, _form: &(), num: usize) -> Option<String> {
        self.typed_value(num)
    }

    /// Loads values from a model into the form
    pub fn load(
        &mut self,
        _form: &mut (),
        _ent: &(),
        model: &StepModel,
    ) -> bool {
        self.values.insert(1, model.name.clone());
        self.values.insert(2, model.timestamp.clone());
        self.values.insert(3, model.author.clone());
        self.values.insert(4, model.organization.clone());
        self.values.insert(5, model.preprocessor_version.clone());
        self.values.insert(6, model.originating_system.clone());
        self.values.insert(7, model.authorization.clone());
        self.values.insert(8, model.schema_identifiers.clone());
        self.values.insert(9, model.description.clone());
        self.values.insert(10, model.implementation_level.clone());
        true
    }

    /// Applies edited values back to the model
    pub fn apply(
        &self,
        _form: &(),
        _ent: &(),
        model: &mut StepModel,
    ) -> bool {
        if let Some(val) = self.values.get(&1) {
            model.name = val.clone();
        }
        if let Some(val) = self.values.get(&2) {
            if is_timestamp(val) {
                model.timestamp = val.clone();
            } else {
                return false;
            }
        }
        if let Some(val) = self.values.get(&3) {
            model.author = val.clone();
        }
        if let Some(val) = self.values.get(&4) {
            model.organization = val.clone();
        }
        if let Some(val) = self.values.get(&5) {
            model.preprocessor_version = val.clone();
        }
        if let Some(val) = self.values.get(&6) {
            model.originating_system = val.clone();
        }
        if let Some(val) = self.values.get(&7) {
            model.authorization = val.clone();
        }
        if let Some(val) = self.values.get(&8) {
            model.schema_identifiers = val.clone();
        }
        if let Some(val) = self.values.get(&9) {
            model.description = val.clone();
        }
        if let Some(val) = self.values.get(&10) {
            model.implementation_level = val.clone();
        }
        true
    }

    /// Sets a value for a given field number
    pub fn set_value(&mut self, num: usize, val: String) {
        self.values.insert(num, val);
    }
}

impl Default for APIHeaderSection_EditHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Minimal StepModel representation for testing
struct StepModel {
    name: String,
    timestamp: String,
    author: String,
    organization: String,
    preprocessor_version: String,
    originating_system: String,
    authorization: String,
    schema_identifiers: String,
    description: String,
    implementation_level: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_timestamp_valid() {
        assert!(is_timestamp("2023-05-15T14:30:45"));
        assert!(is_timestamp("2000-01-01T00:00:00"));
        assert!(is_timestamp("2099-12-31T23:59:59"));
    }

    #[test]
    fn test_is_timestamp_invalid_length() {
        assert!(!is_timestamp("2023-05-15T14:30:4"));
        assert!(!is_timestamp("2023-05-15T14:30:456"));
        assert!(!is_timestamp(""));
    }

    #[test]
    fn test_is_timestamp_invalid_format() {
        assert!(!is_timestamp("2023/05/15T14:30:45")); // wrong separator
        assert!(!is_timestamp("2023-13-15T14:30:45")); // invalid month
        assert!(!is_timestamp("2023-05-32T14:30:45")); // invalid day
        assert!(!is_timestamp("2023-05-15T25:30:45")); // invalid hour
        assert!(!is_timestamp("2023-05-15T14:60:45")); // invalid minute
        assert!(!is_timestamp("2023-05-15T14:30:60")); // invalid second
    }

    #[test]
    fn test_editor_creation() {
        let editor = APIHeaderSection_EditHeader::new();
        assert_eq!(editor.label(), "Step Header");
    }

    #[test]
    fn test_editor_recognize() {
        let editor = APIHeaderSection_EditHeader::new();
        assert!(editor.recognize(&()));
    }

    #[test]
    fn test_editor_set_and_get_value() {
        let mut editor = APIHeaderSection_EditHeader::new();
        editor.set_value(1, "myfile.step".to_string());
        assert_eq!(editor.string_value(&(), 1), Some("myfile.step".to_string()));
    }

    #[test]
    fn test_editor_load_and_apply() {
        let mut editor = APIHeaderSection_EditHeader::new();
        let mut model = StepModel {
            name: "original.step".to_string(),
            timestamp: "2023-05-15T14:30:45".to_string(),
            author: "John Doe".to_string(),
            organization: "ACME Corp".to_string(),
            preprocessor_version: "1.0".to_string(),
            originating_system: "CAD System".to_string(),
            authorization: "Public".to_string(),
            schema_identifiers: "AP203".to_string(),
            description: "Test Part".to_string(),
            implementation_level: "2".to_string(),
        };

        // Load from model
        assert!(editor.load(&mut (), &(), &model));
        assert_eq!(editor.string_value(&(), 1), Some("original.step".to_string()));
        assert_eq!(editor.string_value(&(), 3), Some("John Doe".to_string()));

        // Modify and apply
        editor.set_value(1, "modified.step".to_string());
        editor.set_value(3, "Jane Doe".to_string());
        assert!(editor.apply(&(), &(), &mut model));
        assert_eq!(model.name, "modified.step");
        assert_eq!(model.author, "Jane Doe");
    }

    #[test]
    fn test_editor_reject_invalid_timestamp() {
        let mut editor = APIHeaderSection_EditHeader::new();
        let mut model = StepModel {
            name: "test.step".to_string(),
            timestamp: "2023-05-15T14:30:45".to_string(),
            author: "John".to_string(),
            organization: "Org".to_string(),
            preprocessor_version: "1.0".to_string(),
            originating_system: "Sys".to_string(),
            authorization: "Auth".to_string(),
            schema_identifiers: "AP203".to_string(),
            description: "Desc".to_string(),
            implementation_level: "2".to_string(),
        };

        editor.set_value(2, "invalid-timestamp".to_string());
        assert!(!editor.apply(&(), &(), &mut model));
    }

    #[test]
    fn test_default_constructor() {
        let _editor = APIHeaderSection_EditHeader::default();
    }
}
