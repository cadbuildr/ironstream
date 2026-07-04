// FILE: step_visual_colour.rs
// occt: StepVisual_Colour

/// Represents a StepVisual Colour
#[derive(Debug, Clone, Default)]
pub struct StepVisual_Colour {
    name: Option<String>,
}

impl StepVisual_Colour {
    pub fn new() -> Self {
        StepVisual_Colour { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = StepVisual_Colour::new();
        assert!(c.name().is_none());
    }
}
