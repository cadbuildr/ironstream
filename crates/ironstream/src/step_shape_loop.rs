// FILE: step_shape_loop.rs
// occt: StepShape_Loop

//! Representation of STEP entity Loop

#[derive(Clone, Debug)]
pub struct Loop {
    name: String,
}

impl Loop {
    /// Returns a Loop
    pub fn new() -> Self {
        Loop {
            name: String::new(),
        }
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for Loop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loop_obj = Loop::new();
        assert_eq!(loop_obj.name(), "");
    }

    #[test]
    fn test_set_name() {
        let mut loop_obj = Loop::new();
        loop_obj.set_name("Loop1".to_string());
        assert_eq!(loop_obj.name(), "Loop1");
    }
}
