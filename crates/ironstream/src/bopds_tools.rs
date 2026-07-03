// FILE: bopds_tools.rs
// occt: BOPDS_Tools

/// Tools for Boolean Operation Data Structure
pub struct BodsTools;

impl BodsTools {
    pub fn new() -> Self {
        BodsTools
    }

    pub fn typeid(&self, id: i32) -> i32 {
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tools = BodsTools::new();
    }

    #[test]
    fn test_typeid() {
        let tools = BodsTools::new();
        assert_eq!(tools.typeid(5), 5);
    }
}
