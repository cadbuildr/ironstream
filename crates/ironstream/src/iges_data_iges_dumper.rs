// FILE: iges_data_iges_dumper.rs
// occt: IGESData_IGESDumper

//! Dumper for IGES entities, outputs formatted representations.

#[derive(Clone, Debug)]
pub struct IGESDumper {
    level: i32,
}

impl IGESDumper {
    pub fn new(level: i32) -> Self {
        IGESDumper { level }
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn set_level(&mut self, level: i32) {
        self.level = level;
    }

    pub fn dump_entity(&self, id: usize) -> String {
        format!("Entity #{}", id)
    }

    pub fn dump_header(&self, text: &str) -> String {
        format!("  {}", text)
    }
}

impl Default for IGESDumper {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dumper = IGESDumper::new(5);
        assert_eq!(dumper.level(), 5);
    }

    #[test]
    fn test_set_level() {
        let mut dumper = IGESDumper::new(0);
        dumper.set_level(10);
        assert_eq!(dumper.level(), 10);
    }

    #[test]
    fn test_dump_entity() {
        let dumper = IGESDumper::new(0);
        let result = dumper.dump_entity(42);
        assert!(result.contains("42"));
    }

    #[test]
    fn test_default() {
        let dumper = IGESDumper::default();
        assert_eq!(dumper.level(), 0);
    }
}
