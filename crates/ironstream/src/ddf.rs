// FILE: ddf.rs
// occt: DDF

//! Provides facilities to manipulate data framework in a Draw-Commands environment.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// A placeholder for TDF_Data (framework data).
#[derive(Clone, Debug)]
pub struct TdfData {
    id: u32,
    labels: HashMap<String, TdfLabel>,
}

impl TdfData {
    pub fn new(id: u32) -> Self {
        TdfData {
            id,
            labels: HashMap::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn add_label(&mut self, entry: &str, label: TdfLabel) {
        self.labels.insert(entry.to_string(), label);
    }

    pub fn find_label(&self, entry: &str) -> Option<&TdfLabel> {
        self.labels.get(entry)
    }

    pub fn find_label_mut(&mut self, entry: &str) -> Option<&mut TdfLabel> {
        self.labels.get_mut(entry)
    }
}

/// A label in the data framework.
#[derive(Clone, Debug, PartialEq)]
pub struct TdfLabel {
    entry: String,
}

impl TdfLabel {
    pub fn new(entry: &str) -> Self {
        TdfLabel {
            entry: entry.to_string(),
        }
    }

    pub fn entry(&self) -> &str {
        &self.entry
    }
}

/// A standard GUID (Globally Unique Identifier).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StandardGuid {
    data: [u8; 16],
}

impl StandardGuid {
    pub fn new(data: [u8; 16]) -> Self {
        StandardGuid { data }
    }

    pub fn data(&self) -> &[u8; 16] {
        &self.data
    }
}

/// A TDF attribute.
#[derive(Clone, Debug)]
pub struct TdfAttribute {
    id: StandardGuid,
    value: String,
}

impl TdfAttribute {
    pub fn new(id: StandardGuid, value: &str) -> Self {
        TdfAttribute {
            id,
            value: value.to_string(),
        }
    }

    pub fn id(&self) -> &StandardGuid {
        &self.id
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Global storage for data frameworks.
static FRAMEWORKS_CELL: OnceLock<Mutex<HashMap<String, TdfData>>> = OnceLock::new();
fn frameworks() -> &'static Mutex<HashMap<String, TdfData>> {
    FRAMEWORKS_CELL.get_or_init(|| Mutex::new(HashMap::new()))
}

/// DDF: Draw Data Framework utilities.
pub struct Ddf;

impl Ddf {
    /// Search in draw directory the framework identified by its name.
    pub fn get_df(name: &str, complain: bool) -> Result<TdfData, String> {
        let storage = frameworks().lock().unwrap();

        match storage.get(name) {
            Some(df) => Ok(df.clone()),
            None => {
                if complain {
                    eprintln!("Framework '{}' not found", name);
                }
                Err(format!("Framework '{}' not found", name))
            }
        }
    }

    /// Register a new framework.
    pub fn register_df(name: &str, df: TdfData) {
        let mut storage = frameworks().lock().unwrap();
        storage.insert(name.to_string(), df);
    }

    /// Search in the framework for the label identified by its entry.
    pub fn find_label(df: &TdfData, entry: &str, complain: bool) -> Option<TdfLabel> {
        match df.find_label(entry) {
            Some(label) => Some(label.clone()),
            None => {
                if complain {
                    eprintln!("Label '{}' not found in framework", entry);
                }
                None
            }
        }
    }

    /// Search in the framework for the label by entry; create if missing.
    pub fn add_label(df: &mut TdfData, entry: &str) -> TdfLabel {
        if let Some(label) = df.find_label(entry) {
            label.clone()
        } else {
            let label = TdfLabel::new(entry);
            df.add_label(entry, label.clone());
            label
        }
    }

    /// Search for an attribute by entry and ID.
    pub fn find(
        df: &TdfData,
        entry: &str,
        id: &StandardGuid,
        complain: bool,
    ) -> Option<TdfAttribute> {
        if let Some(label) = df.find_label(entry) {
            // In a real implementation, this would look up attributes attached to the label
            // For now, return a placeholder
            Some(TdfAttribute::new(id.clone(), "placeholder"))
        } else {
            if complain {
                eprintln!("Attribute not found at entry '{}'", entry);
            }
            None
        }
    }

    /// Register all available commands with the interpreter.
    pub fn all_commands(_interpreter: &str) {
        // In real implementation: register all DDF commands
        // - BasicCommands
        // - DataCommands
        // - TransactionCommands
        // - BrowserCommands
    }

    /// Register basic data framework commands.
    pub fn basic_commands(_interpreter: &str) {
        // newdf, save, restore, clear
    }

    /// Register data framework manipulation commands.
    pub fn data_commands(_interpreter: &str) {
        // create, clear, copy
    }

    /// Register transaction commands.
    pub fn transaction_commands(_interpreter: &str) {
        // open, commit, abort, undo
    }

    /// Register browser commands.
    pub fn browser_commands(_interpreter: &str) {
        // browse, show, explore
    }

    /// Clear all registered frameworks.
    pub fn clear_all() {
        frameworks().lock().unwrap().clear();
    }

    /// Get the number of registered frameworks.
    pub fn framework_count() -> usize {
        frameworks().lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdf_data_creation() {
        let df = TdfData::new(1);
        assert_eq!(df.id(), 1);
    }

    #[test]
    fn test_tdf_label_creation() {
        let label = TdfLabel::new("0:1");
        assert_eq!(label.entry(), "0:1");
    }

    #[test]
    fn test_standard_guid_creation() {
        let data = [0u8; 16];
        let guid = StandardGuid::new(data);
        assert_eq!(guid.data(), &data);
    }

    #[test]
    fn test_tdf_attribute_creation() {
        let guid = StandardGuid::new([1u8; 16]);
        let attr = TdfAttribute::new(guid.clone(), "test_value");
        assert_eq!(attr.id(), &guid);
        assert_eq!(attr.value(), "test_value");
    }

    #[test]
    fn test_register_and_get_df() {
        Ddf::clear_all();
        let df = TdfData::new(1);
        Ddf::register_df("mydf", df.clone());

        let retrieved = Ddf::get_df("mydf", false);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().id(), 1);
    }

    #[test]
    fn test_get_nonexistent_df() {
        Ddf::clear_all();
        let result = Ddf::get_df("nonexistent", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_label_to_df() {
        Ddf::clear_all();
        let mut df = TdfData::new(1);
        let label = Ddf::add_label(&mut df, "0:1:1");

        assert_eq!(label.entry(), "0:1:1");
        assert!(df.find_label("0:1:1").is_some());
    }

    #[test]
    fn test_find_label_in_df() {
        Ddf::clear_all();
        let mut df = TdfData::new(1);
        let label1 = TdfLabel::new("0:1");
        df.add_label("0:1", label1);

        let found = Ddf::find_label(&df, "0:1", false);
        assert!(found.is_some());
        assert_eq!(found.unwrap().entry(), "0:1");
    }

    #[test]
    fn test_find_nonexistent_label() {
        let df = TdfData::new(1);
        let found = Ddf::find_label(&df, "nonexistent", false);
        assert!(found.is_none());
    }

    #[test]
    fn test_find_attribute() {
        let mut df = TdfData::new(1);
        let label = TdfLabel::new("0:1");
        df.add_label("0:1", label);

        let guid = StandardGuid::new([2u8; 16]);
        let found = Ddf::find(&df, "0:1", &guid, false);
        assert!(found.is_some());
    }

    #[test]
    fn test_framework_count() {
        Ddf::clear_all();
        assert_eq!(Ddf::framework_count(), 0);

        Ddf::register_df("df1", TdfData::new(1));
        assert_eq!(Ddf::framework_count(), 1);

        Ddf::register_df("df2", TdfData::new(2));
        assert_eq!(Ddf::framework_count(), 2);

        Ddf::clear_all();
        assert_eq!(Ddf::framework_count(), 0);
    }

    #[test]
    fn test_multiple_labels_in_df() {
        let mut df = TdfData::new(1);
        Ddf::add_label(&mut df, "0:1");
        Ddf::add_label(&mut df, "0:2");
        Ddf::add_label(&mut df, "0:3");

        assert!(df.find_label("0:1").is_some());
        assert!(df.find_label("0:2").is_some());
        assert!(df.find_label("0:3").is_some());
    }

    #[test]
    fn test_guid_equality() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let guid1 = StandardGuid::new(data);
        let guid2 = StandardGuid::new(data);

        assert_eq!(guid1, guid2);
    }
}
