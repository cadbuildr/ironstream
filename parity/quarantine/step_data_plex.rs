// FILE: step_data_plex.rs
// occt: StepData_Plex

use std::collections::HashMap;

//! Represents a Plex (complex entity in STEP)
pub struct StepDataPlex {
    members: Vec<String>,
}

impl StepDataPlex {
    //! Creates a Plex
    pub fn new() -> Self {
        StepDataPlex {
            members: Vec::new(),
        }
    }

    //! Adds a member
    pub fn add(&mut self, member: &str) {
        self.members.push(member.to_string());
    }

    //! Returns the count of members
    pub fn nb_members(&self) -> usize {
        self.members.len()
    }

    //! Returns a member
    pub fn member(&self, num: usize) -> Option<&str> {
        if num < 1 || num > self.members.len() {
            return None;
        }
        Some(&self.members[num - 1])
    }
}

impl Default for StepDataPlex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plex_new() {
        let plex = StepDataPlex::new();
        assert_eq!(plex.nb_members(), 0);
    }

    #[test]
    fn test_add_member() {
        let mut plex = StepDataPlex::new();
        plex.add("member1");
        plex.add("member2");
        assert_eq!(plex.nb_members(), 2);
    }

    #[test]
    fn test_get_member() {
        let mut plex = StepDataPlex::new();
        plex.add("member1");
        assert_eq!(plex.member(1), Some("member1"));
        assert_eq!(plex.member(2), None);
    }
}
