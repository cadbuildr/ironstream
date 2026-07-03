// FILE: b_rep_fill_shape_law.rs
// occt: BRepFill_ShapeLaw

#[derive(Debug, Clone)]
pub struct BRepFillShapeLaw {
    shape_data: Vec<u8>,
    is_vertex: bool,
    is_constant: bool,
}

impl BRepFillShapeLaw {
    pub fn new() -> Self {
        BRepFillShapeLaw {
            shape_data: Vec::new(),
            is_vertex: false,
            is_constant: true,
        }
    }

    pub fn with_vertex() -> Self {
        BRepFillShapeLaw {
            shape_data: Vec::new(),
            is_vertex: true,
            is_constant: true,
        }
    }

    pub fn with_wire() -> Self {
        BRepFillShapeLaw {
            shape_data: Vec::new(),
            is_vertex: false,
            is_constant: false,
        }
    }

    pub fn is_vertex(&self) -> bool {
        self.is_vertex
    }

    pub fn is_constant(&self) -> bool {
        self.is_constant
    }

    pub fn set_constant(&mut self, constant: bool) {
        self.is_constant = constant;
    }

    pub fn shape_data(&self) -> &[u8] {
        &self.shape_data
    }

    pub fn set_shape_data(&mut self, data: Vec<u8>) {
        self.shape_data = data;
    }
}

impl Default for BRepFillShapeLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_law_new() {
        let sl = BRepFillShapeLaw::new();
        assert!(!sl.is_vertex());
        assert!(sl.is_constant());
    }

    #[test]
    fn test_shape_law_with_vertex() {
        let sl = BRepFillShapeLaw::with_vertex();
        assert!(sl.is_vertex());
        assert!(sl.is_constant());
    }

    #[test]
    fn test_shape_law_with_wire() {
        let sl = BRepFillShapeLaw::with_wire();
        assert!(!sl.is_vertex());
        assert!(!sl.is_constant());
    }

    #[test]
    fn test_shape_law_set_constant() {
        let mut sl = BRepFillShapeLaw::with_wire();
        assert!(!sl.is_constant());
        sl.set_constant(true);
        assert!(sl.is_constant());
    }

    #[test]
    fn test_shape_law_data() {
        let mut sl = BRepFillShapeLaw::new();
        let data = vec![1, 2, 3, 4];
        sl.set_shape_data(data.clone());
        assert_eq!(sl.shape_data(), &data[..]);
    }
}
