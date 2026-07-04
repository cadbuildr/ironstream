// FILE: moni_tool_sign_shape.rs
// occt: MoniTool_SignShape

/// Signs shapes
pub struct MoniToolSignShape {
    shape_type: String,
}

impl MoniToolSignShape {
    pub fn new(shape_type: &str) -> Self {
        MoniToolSignShape {
            shape_type: shape_type.to_string(),
        }
    }

    pub fn shape_type(&self) -> &str {
        &self.shape_type
    }

    pub fn set_shape_type(&mut self, shape_type: &str) {
        self.shape_type = shape_type.to_string();
    }
}

impl Default for MoniToolSignShape {
    fn default() -> Self {
        MoniToolSignShape {
            shape_type: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sign = MoniToolSignShape::new("Solid");
        assert_eq!(sign.shape_type(), "Solid");
    }
}
