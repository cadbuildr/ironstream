// FILE: transfer_b_rep_shape_list_binder.rs
// occt: TransferBRep_ShapeListBinder

/// A binder that associates a starting entity with a list of Shape results.
/// Each shape in the list is treated as an item of the list, not as a separate result.
#[derive(Clone, Debug)]
pub struct TransferBRepShapeListBinder {
    /// List of shape results (represented as simple 3D points)
    shapes: Vec<[f64; 3]>,
}

impl TransferBRepShapeListBinder {
    /// Creates a new empty shape list binder.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    /// Creates a shape list binder with initial shapes.
    pub fn with_shapes(shapes: Vec<[f64; 3]>) -> Self {
        Self { shapes }
    }

    /// Returns whether this binder contains multiple results.
    pub fn is_multiple(&self) -> bool {
        self.shapes.len() > 1
    }

    /// Returns the type name for results (always "TopoDS_Shape").
    pub fn result_type_name(&self) -> &'static str {
        "TopoDS_Shape"
    }

    /// Adds a shape to the result list.
    pub fn add_result(&mut self, shape: [f64; 3]) {
        self.shapes.push(shape);
    }

    /// Sets a shape at a specific index (1-based to match OCCT).
    pub fn set_result(&mut self, index: usize, shape: [f64; 3]) {
        if index > 0 && index <= self.shapes.len() {
            self.shapes[index - 1] = shape;
        }
    }

    /// Returns the number of shapes in the result list.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    /// Returns a shape at the given index (1-based).
    pub fn shape(&self, index: usize) -> Option<[f64; 3]> {
        if index > 0 && index <= self.shapes.len() {
            Some(self.shapes[index - 1])
        } else {
            None
        }
    }

    /// Returns all shapes.
    pub fn shapes(&self) -> &[[f64; 3]] {
        &self.shapes
    }

    /// Returns a mutable reference to all shapes.
    pub fn shapes_mut(&mut self) -> &mut Vec<[f64; 3]> {
        &mut self.shapes
    }
}

impl Default for TransferBRepShapeListBinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let binder = TransferBRepShapeListBinder::new();
        assert_eq!(binder.nb_shapes(), 0);
        assert!(!binder.is_multiple());
    }

    #[test]
    fn test_add_result() {
        let mut binder = TransferBRepShapeListBinder::new();
        binder.add_result([1.0, 2.0, 3.0]);
        assert_eq!(binder.nb_shapes(), 1);
        assert!(!binder.is_multiple());

        binder.add_result([4.0, 5.0, 6.0]);
        assert_eq!(binder.nb_shapes(), 2);
        assert!(binder.is_multiple());
    }

    #[test]
    fn test_shape_access() {
        let mut binder = TransferBRepShapeListBinder::new();
        binder.add_result([1.0, 2.0, 3.0]);
        binder.add_result([4.0, 5.0, 6.0]);

        assert_eq!(binder.shape(1), Some([1.0, 2.0, 3.0]));
        assert_eq!(binder.shape(2), Some([4.0, 5.0, 6.0]));
        assert_eq!(binder.shape(3), None);
        assert_eq!(binder.shape(0), None);
    }

    #[test]
    fn test_set_result() {
        let mut binder = TransferBRepShapeListBinder::new();
        binder.add_result([1.0, 2.0, 3.0]);
        binder.add_result([4.0, 5.0, 6.0]);

        binder.set_result(2, [7.0, 8.0, 9.0]);
        assert_eq!(binder.shape(2), Some([7.0, 8.0, 9.0]));
        assert_eq!(binder.shape(1), Some([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_result_type_name() {
        let binder = TransferBRepShapeListBinder::new();
        assert_eq!(binder.result_type_name(), "TopoDS_Shape");
    }

    #[test]
    fn test_with_shapes() {
        let shapes = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let binder = TransferBRepShapeListBinder::with_shapes(shapes);
        assert_eq!(binder.nb_shapes(), 2);
        assert!(binder.is_multiple());
    }
}
