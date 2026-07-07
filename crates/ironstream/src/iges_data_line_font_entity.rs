// FILE: iges_data_line_font_entity.rs
// occt: IGESData_LineFontEntity

//! Line font entity for IGES.

#[derive(Clone, Debug)]
pub struct LineFontEntity {
    font_id: i32,
    pattern: Vec<f64>,
}

impl LineFontEntity {
    pub fn new(font_id: i32) -> Self {
        LineFontEntity {
            font_id,
            pattern: Vec::new(),
        }
    }

    pub fn font_id(&self) -> i32 {
        self.font_id
    }

    pub fn set_font_id(&mut self, id: i32) {
        self.font_id = id;
    }

    pub fn add_pattern_element(&mut self, element: f64) {
        self.pattern.push(element);
    }

    pub fn pattern(&self) -> &[f64] {
        &self.pattern
    }
}

impl Default for LineFontEntity {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = LineFontEntity::new(1);
        assert_eq!(entity.font_id(), 1);
    }

    #[test]
    fn test_set_font_id() {
        let mut entity = LineFontEntity::new(1);
        entity.set_font_id(5);
        assert_eq!(entity.font_id(), 5);
    }

    #[test]
    fn test_pattern() {
        let mut entity = LineFontEntity::new(1);
        entity.add_pattern_element(0.1);
        entity.add_pattern_element(0.2);
        assert_eq!(entity.pattern().len(), 2);
    }
}
