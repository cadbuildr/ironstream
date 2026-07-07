// FILE: iges_data_label_display_entity.rs
// occt: IGESData_LabelDisplayEntity

//! Label display entity for IGES.

#[derive(Clone, Debug)]
pub struct LabelDisplayEntity {
    label: String,
    x: f64,
    y: f64,
}

impl LabelDisplayEntity {
    pub fn new(label: &str, x: f64, y: f64) -> Self {
        LabelDisplayEntity {
            label: label.to_string(),
            x,
            y,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Default for LabelDisplayEntity {
    fn default() -> Self {
        Self::new("", 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = LabelDisplayEntity::new("Label1", 10.5, 20.5);
        assert_eq!(entity.label(), "Label1");
        assert_eq!(entity.position(), (10.5, 20.5));
    }

    #[test]
    fn test_default() {
        let entity = LabelDisplayEntity::default();
        assert_eq!(entity.label(), "");
        assert_eq!(entity.position(), (0.0, 0.0));
    }
}
