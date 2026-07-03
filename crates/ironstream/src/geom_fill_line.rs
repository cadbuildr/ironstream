// FILE: geom_fill_line.rs
// occt: GeomFill_Line

/// Line for surface filling
pub struct Line {
    index: usize,
}

impl Line {
    pub fn new(index: usize) -> Self {
        Line { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let line = Line::new(5);
        assert_eq!(line.index(), 5);
    }

    #[test]
    fn test_zero_index() {
        let line = Line::new(0);
        assert_eq!(line.index(), 0);
    }
}
