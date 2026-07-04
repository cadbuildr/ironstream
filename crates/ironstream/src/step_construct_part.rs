// FILE: step_construct_part.rs
// occt: STEPConstruct_Part

/// Tool for handling STEP parts
pub struct STEPConstruct_Part {
    id: i32,
}

impl STEPConstruct_Part {
    pub fn new() -> Self {
        STEPConstruct_Part { id: 0 }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

impl Default for STEPConstruct_Part {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_part() {
        let part = STEPConstruct_Part::new();
        assert_eq!(part.get_id(), 0);
    }

    #[test]
    fn test_set_id() {
        let mut part = STEPConstruct_Part::new();
        part.set_id(42);
        assert_eq!(part.get_id(), 42);
    }
}
