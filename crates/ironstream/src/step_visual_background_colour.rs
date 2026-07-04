// FILE: step_visual_background_colour.rs
// occt: StepVisual_BackgroundColour

/// Represents a StepVisual BackgroundColour
#[derive(Debug, Clone, Default)]
pub struct StepVisual_BackgroundColour {
    name: Option<String>,
}

impl StepVisual_BackgroundColour {
    pub fn new() -> Self {
        StepVisual_BackgroundColour { name: None }
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
        let bc = StepVisual_BackgroundColour::new();
        assert!(bc.name().is_none());
    }
}
