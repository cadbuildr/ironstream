// FILE: iges_draw_label_display.rs
// occt: IGESDraw_LabelDisplay

/// Label display entity
pub struct IgesDrawLabelDisplay {
    label: String,
    x: f64,
    y: f64,
}

impl IgesDrawLabelDisplay {
    pub fn new() -> Self {
        IgesDrawLabelDisplay {
            label: String::new(),
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn init(&mut self, label: String, x: f64, y: f64) {
        self.label = label;
        self.x = x;
        self.y = y;
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Default for IgesDrawLabelDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ld = IgesDrawLabelDisplay::new();
        assert_eq!(ld.label(), "");
        assert_eq!(ld.x(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut ld = IgesDrawLabelDisplay::new();
        ld.init("LABEL".to_string(), 1.0, 2.0);
        assert_eq!(ld.label(), "LABEL");
        assert_eq!(ld.x(), 1.0);
        assert_eq!(ld.y(), 2.0);
    }
}
