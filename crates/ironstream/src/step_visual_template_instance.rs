// FILE: step_visual_template_instance.rs
// occt: StepVisual_TemplateInstance

pub struct TemplateInstance {
    _data: (),
}

impl TemplateInstance {
    pub fn new() -> Self {
        TemplateInstance { _data: () }
    }
}

impl Default for TemplateInstance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ti = TemplateInstance::new();
        let _ti2 = TemplateInstance::new();
        let _ = ti;
    }

    #[test]
    fn test_default() {
        let ti = TemplateInstance::default();
        let _ti2 = TemplateInstance::new();
        let _ = ti;
    }
}
