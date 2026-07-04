// FILE: step_visual_area_in_set.rs
// occt: StepVisual_AreaInSet

/// Represents a StepVisual AreaInSet
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AreaInSet {
    name: Option<String>,
}

impl StepVisual_AreaInSet {
    pub fn new() -> Self {
        StepVisual_AreaInSet { name: None }
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
        let ais = StepVisual_AreaInSet::new();
        assert!(ais.name().is_none());
    }
}
