// FILE: iges_dimen_general_note.rs
// occt: IGESDimen_GeneralNote

/// Defines GeneralNote, Type <212> Form <0-1>
/// in package IGESDimen
pub struct IgesDimen_GeneralNote {
    text: String,
    placement: (f64, f64),
    height: f64,
}

impl IgesDimen_GeneralNote {
    pub fn new() -> Self {
        IgesDimen_GeneralNote {
            text: String::new(),
            placement: (0.0, 0.0),
            height: 0.0,
        }
    }

    pub fn init(&mut self, text: String, placement: (f64, f64), height: f64) {
        self.text = text;
        self.placement = placement;
        self.height = height;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn placement(&self) -> (f64, f64) {
        self.placement
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}

impl Default for IgesDimen_GeneralNote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_note_creation() {
        let note = IgesDimen_GeneralNote::new();
        assert_eq!(note.text(), "");
    }

    #[test]
    fn test_general_note_init() {
        let mut note = IgesDimen_GeneralNote::new();
        note.init("TEST".to_string(), (10.0, 20.0), 5.0);

        assert_eq!(note.text(), "TEST");
        assert_eq!(note.placement(), (10.0, 20.0));
        assert_eq!(note.height(), 5.0);
    }
}
