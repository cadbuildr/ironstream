// FILE: cdm_names_directory.rs
// occt: CDM_NamesDirectory

//! Deprecated type alias for backward compatibility.
//! Use HashMap<String, i32> directly instead.

use std::collections::HashMap;

/// Deprecated directory of names mapped to integers.
/// Maps string names (extended) to integer identifiers.
/// Maps to NCollection_DataMap<TCollection_ExtendedString, int>.
pub type CdmNamesDirectory = HashMap<String, i32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_names_directory_creation() {
        let dir: CdmNamesDirectory = HashMap::new();
        assert_eq!(dir.len(), 0);
    }

    #[test]
    fn test_names_directory_insert() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("name1".to_string(), 1);
        dir.insert("name2".to_string(), 2);

        assert_eq!(dir.len(), 2);
        assert_eq!(dir.get("name1"), Some(&1));
        assert_eq!(dir.get("name2"), Some(&2));
    }

    #[test]
    fn test_names_directory_lookup() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("alpha".to_string(), 100);
        dir.insert("beta".to_string(), 200);
        dir.insert("gamma".to_string(), 300);

        assert_eq!(dir.get("alpha"), Some(&100));
        assert_eq!(dir.get("beta"), Some(&200));
        assert_eq!(dir.get("gamma"), Some(&300));
        assert_eq!(dir.get("delta"), None);
    }

    #[test]
    fn test_names_directory_iteration() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("a".to_string(), 1);
        dir.insert("b".to_string(), 2);
        dir.insert("c".to_string(), 3);

        let mut sum = 0;
        for (_, &value) in dir.iter() {
            sum += value;
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_names_directory_remove() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("x".to_string(), 10);
        dir.insert("y".to_string(), 20);

        assert_eq!(dir.len(), 2);

        dir.remove("x");
        assert_eq!(dir.len(), 1);
        assert_eq!(dir.get("x"), None);
        assert_eq!(dir.get("y"), Some(&20));
    }

    #[test]
    fn test_names_directory_contains() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("present".to_string(), 1);

        assert!(dir.contains_key("present"));
        assert!(!dir.contains_key("absent"));
    }

    #[test]
    fn test_names_directory_clear() {
        let mut dir: CdmNamesDirectory = HashMap::new();
        dir.insert("a".to_string(), 1);
        dir.insert("b".to_string(), 2);

        assert_eq!(dir.len(), 2);

        dir.clear();
        assert_eq!(dir.len(), 0);
    }
}
