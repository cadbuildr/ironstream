// FILE: iges_draw_rect_array_subfigure.rs
// occt: IGESDraw_RectArraySubfigure

/// Rectangular array subfigure entity
pub struct IgesDrawRectArraySubfigure {
    base_entity: Option<Box<dyn std::any::Any>>,
    nb_rows: i32,
    nb_cols: i32,
    row_spacing: f64,
    col_spacing: f64,
}

impl IgesDrawRectArraySubfigure {
    pub fn new() -> Self {
        IgesDrawRectArraySubfigure {
            base_entity: None,
            nb_rows: 0,
            nb_cols: 0,
            row_spacing: 0.0,
            col_spacing: 0.0,
        }
    }

    pub fn init(&mut self, nb_rows: i32, nb_cols: i32, row_spacing: f64, col_spacing: f64) {
        self.nb_rows = nb_rows;
        self.nb_cols = nb_cols;
        self.row_spacing = row_spacing;
        self.col_spacing = col_spacing;
    }

    pub fn nb_rows(&self) -> i32 {
        self.nb_rows
    }

    pub fn nb_cols(&self) -> i32 {
        self.nb_cols
    }

    pub fn row_spacing(&self) -> f64 {
        self.row_spacing
    }

    pub fn col_spacing(&self) -> f64 {
        self.col_spacing
    }
}

impl Default for IgesDrawRectArraySubfigure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ras = IgesDrawRectArraySubfigure::new();
        assert_eq!(ras.nb_rows(), 0);
        assert_eq!(ras.nb_cols(), 0);
    }

    #[test]
    fn test_init() {
        let mut ras = IgesDrawRectArraySubfigure::new();
        ras.init(3, 4, 1.0, 2.0);
        assert_eq!(ras.nb_rows(), 3);
        assert_eq!(ras.nb_cols(), 4);
        assert_eq!(ras.row_spacing(), 1.0);
        assert_eq!(ras.col_spacing(), 2.0);
    }
}
