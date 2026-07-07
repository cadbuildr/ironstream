// FILE: std_persistent_naming.rs
// occt: StdPersistent_Naming

/// Naming attribute persistence
pub struct Naming {
    name: String,
    entry: String,
}

impl Naming {
    /// Create a new naming attribute
    pub fn new(name: &str, entry: &str) -> Self {
        Naming {
            name: name.to_string(),
            entry: entry.to_string(),
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Get the entry
    pub fn entry(&self) -> &str {
        &self.entry
    }

    /// Set the entry
    pub fn set_entry(&mut self, entry: &str) {
        self.entry = entry.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let naming = Naming::new("MyName", "0:1:1");
        assert_eq!(naming.name(), "MyName");
        assert_eq!(naming.entry(), "0:1:1");
    }

    #[test]
    fn test_set_name() {
        let mut naming = Naming::new("MyName", "0:1:1");
        naming.set_name("NewName");
        assert_eq!(naming.name(), "NewName");
    }

    #[test]
    fn test_set_entry() {
        let mut naming = Naming::new("MyName", "0:1:1");
        naming.set_entry("0:1:2");
        assert_eq!(naming.entry(), "0:1:2");
    }
}
