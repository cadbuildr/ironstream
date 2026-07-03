// FILE: std_prs_hlr_poly_shape.rs
// occt: StdPrs_HLRPolyShape

/// Hidden Line Removal polyshape presenter.
#[derive(Clone, Debug)]
pub struct HlrPolyShape {
    pub shape_id: u32,
    pub polygon_count: usize,
    pub is_computed: bool,
}

impl HlrPolyShape {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            polygon_count: 0,
            is_computed: false,
        }
    }

    pub fn add_polygon(&mut self) {
        self.polygon_count += 1;
    }

    pub fn nb_polygons(&self) -> usize {
        self.polygon_count
    }

    pub fn compute(&mut self) {
        self.is_computed = true;
    }

    pub fn is_computed(&self) -> bool {
        self.is_computed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_shape() {
        let shape = HlrPolyShape::new(1);
        assert_eq!(shape.shape_id, 1);
        assert!(!shape.is_computed());
    }

    #[test]
    fn add_polygons() {
        let mut shape = HlrPolyShape::new(1);
        shape.add_polygon();
        shape.add_polygon();
        assert_eq!(shape.nb_polygons(), 2);
    }

    #[test]
    fn compute() {
        let mut shape = HlrPolyShape::new(1);
        shape.compute();
        assert!(shape.is_computed());
    }
}
