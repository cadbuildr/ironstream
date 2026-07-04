// FILE: step_data_ec_descr.rs
// occt: StepData_ECDescr

use std::collections::HashMap;

//! Describes a Complex Entity (Plex) as a list of Simple ones
pub struct StepDataECDescr {
    members: Vec<String>, // simplified representation of member list
    members_map: HashMap<String, usize>,
}

impl StepDataECDescr {
    //! Creates an ECDescr, empty
    pub fn new() -> Self {
        StepDataECDescr {
            members: Vec::new(),
            members_map: HashMap::new(),
        }
    }

    //! Adds a member in alphabetic order
    pub fn add(&mut self, member_name: &str) {
        if member_name.is_empty() {
            return;
        }

        // Find insertion point to maintain alphabetic order
        let mut insert_at = self.members.len();
        for (i, existing) in self.members.iter().enumerate() {
            if member_name < existing {
                insert_at = i;
                break;
            }
        }

        self.members.insert(insert_at, member_name.to_string());

        // Rebuild map
        self.members_map.clear();
        for (i, name) in self.members.iter().enumerate() {
            self.members_map.insert(name.clone(), i);
        }
    }

    //! Returns the count of members
    pub fn nb_members(&self) -> usize {
        self.members.len()
    }

    //! Returns a Member from its rank (1-based)
    pub fn member(&self, num: usize) -> Option<&str> {
        if num < 1 || num > self.members.len() {
            return None;
        }
        Some(&self.members[num - 1])
    }

    //! Returns the ordered list of types
    pub fn type_list(&self) -> Vec<String> {
        self.members.clone()
    }

    //! Tells if a member matches a step type
    pub fn matches(&self, step_type: &str) -> bool {
        self.members.iter().any(|m| m == step_type)
    }

    //! Returns True (this is complex)
    pub fn is_complex(&self) -> bool {
        true
    }
}

impl Default for StepDataECDescr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ec_descr() {
        let descr = StepDataECDescr::new();
        assert_eq!(descr.nb_members(), 0);
        assert!(descr.is_complex());
    }

    #[test]
    fn test_add_member() {
        let mut descr = StepDataECDescr::new();
        descr.add("TypeA");
        descr.add("TypeB");
        assert_eq!(descr.nb_members(), 2);
    }

    #[test]
    fn test_alphabetic_order() {
        let mut descr = StepDataECDescr::new();
        descr.add("Zebra");
        descr.add("Apple");
        descr.add("Banana");

        assert_eq!(descr.member(1), Some("Apple"));
        assert_eq!(descr.member(2), Some("Banana"));
        assert_eq!(descr.member(3), Some("Zebra"));
    }

    #[test]
    fn test_matches() {
        let mut descr = StepDataECDescr::new();
        descr.add("TypeA");
        descr.add("TypeB");

        assert!(descr.matches("TypeA"));
        assert!(descr.matches("TypeB"));
        assert!(!descr.matches("TypeC"));
    }

    #[test]
    fn test_type_list() {
        let mut descr = StepDataECDescr::new();
        descr.add("Type1");
        descr.add("Type2");

        let list = descr.type_list();
        assert_eq!(list.len(), 2);
    }
}
