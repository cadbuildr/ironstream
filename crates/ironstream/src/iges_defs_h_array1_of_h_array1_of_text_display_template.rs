// FILE: iges_defs_h_array1_of_h_array1_of_text_display_template.rs
// occt: IGESDefs_HArray1OfHArray1OfTextDisplayTemplate

//! 2D array of text display templates.

#[derive(Clone, Debug)]
pub struct HArray1OfHArray1OfTextDisplayTemplate {
    rows: usize,
    cols: usize,
}

impl HArray1OfHArray1OfTextDisplayTemplate {
    pub fn new(rows: usize, cols: usize) -> Self {
        HArray1OfHArray1OfTextDisplayTemplate { rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Default for HArray1OfHArray1OfTextDisplayTemplate {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let array = HArray1OfHArray1OfTextDisplayTemplate::new(5, 3);
        assert_eq!(array.rows(), 5);
        assert_eq!(array.cols(), 3);
    }

    #[test]
    fn test_default() {
        let array = HArray1OfHArray1OfTextDisplayTemplate::default();
        assert_eq!(array.rows(), 0);
        assert_eq!(array.cols(), 0);
    }
}
