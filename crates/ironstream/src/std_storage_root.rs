// FILE: std_storage_root.rs
// occt: StdStorage_Root

/// Root data object in storage
pub struct Root {
    name: String,
    ref_num: i32,
}

impl Root {
    /// Create a new root
    pub fn new(name: &str, ref_num: i32) -> Self {
        Root {
            name: name.to_string(),
            ref_num,
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the reference number
    pub fn ref_num(&self) -> i32 {
        self.ref_num
    }

    /// Set the reference number
    pub fn set_ref_num(&mut self, ref_num: i32) {
        self.ref_num = ref_num;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let root = Root::new("MyRoot", 1);
        assert_eq!(root.name(), "MyRoot");
        assert_eq!(root.ref_num(), 1);
    }

    #[test]
    fn test_set_ref_num() {
        let mut root = Root::new("MyRoot", 1);
        root.set_ref_num(42);
        assert_eq!(root.ref_num(), 42);
    }
}
