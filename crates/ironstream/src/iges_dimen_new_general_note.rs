// FILE: iges_dimen_new_general_note.rs
// occt: IGESDimen_NewGeneralNote

/// Defines NewGeneralNote, Type <223> Form <0>
/// in package IGESDimen
pub struct IgesDimen_NewGeneralNote {
    text: String,
    placement: (f64, f64),
    height: f64,
    geometry: Vec<IgesData_IgesEntity>,
}

impl IgesDimen_NewGeneralNote {
    pub fn new() -> Self {
        IgesDimen_NewGeneralNote {
            text: String::new(),
            placement: (0.0, 0.0),
            height: 0.0,
            geometry: Vec::new(),
        }
    }

    pub fn init(&mut self, text: String, placement: (f64, f64), height: f64, geom: Vec<IgesData_IgesEntity>) {
        self.text = text;
        self.placement = placement;
        self.height = height;
        self.geometry = geom;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn nb_geometry(&self) -> usize {
        self.geometry.len()
    }
}

impl Default for IgesDimen_NewGeneralNote {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesData_IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_general_note_creation() {
        let note = IgesDimen_NewGeneralNote::new();
        assert_eq!(note.text(), "");
    }
}
