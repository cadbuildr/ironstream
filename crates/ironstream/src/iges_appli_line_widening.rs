// FILE: iges_appli_line_widening.rs
// occt: IGESAppli_LineWidening

/// Specifies line widening for trace paths.
#[derive(Clone, Debug)]
pub struct IgesAppliLineWidening {
    width: f64,
    left_offset: f64,
    right_offset: f64,
}

impl IgesAppliLineWidening {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            left_offset: 0.0,
            right_offset: 0.0,
        }
    }

    pub fn init(&mut self, w: f64, left: f64, right: f64) {
        self.width = w;
        self.left_offset = left;
        self.right_offset = right;
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn left_offset(&self) -> f64 {
        self.left_offset
    }

    pub fn right_offset(&self) -> f64 {
        self.right_offset
    }
}

impl Default for IgesAppliLineWidening {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut widening = IgesAppliLineWidening::new();
        widening.init(0.5, 0.1, 0.1);
        assert_eq!(widening.width(), 0.5);
        assert_eq!(widening.left_offset(), 0.1);
        assert_eq!(widening.right_offset(), 0.1);
    }
}
