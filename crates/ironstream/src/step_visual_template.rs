// FILE: step_visual_template.rs
// occt: StepVisual_Template

pub struct Template {
    _data: (),
}

impl Template {
    pub fn new() -> Self {
        Template { _data: () }
    }
}

impl Default for Template {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Template::new();
        let _t2 = Template::new();
        let _ = t;
    }

    #[test]
    fn test_default() {
        let t = Template::default();
        let _t2 = Template::new();
        let _ = t;
    }
}
