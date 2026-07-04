// FILE: transfer_b_rep_binder_of_shape.rs
// occt: TransferBRep_BinderOfShape

/// A binder that associates a starting object with a unique TopoDS_Shape result.
/// This is the BREP-specific binder for shape transfer operations.
#[derive(Clone, Debug)]
pub struct TransferBRepBinderOfShape {
    /// The bound shape result
    shape: Option<[f64; 3]>, // Placeholder: representing a simple shape identifier
    /// Whether this binder has been set
    is_set: bool,
}

impl TransferBRepBinderOfShape {
    /// Creates a new empty BinderOfShape.
    pub fn new() -> Self {
        Self {
            shape: None,
            is_set: false,
        }
    }

    /// Creates a new BinderOfShape with an initial shape result.
    pub fn with_shape(shape: [f64; 3]) -> Self {
        Self {
            shape: Some(shape),
            is_set: true,
        }
    }

    /// Returns the type name of the result (always "TopoDS_Shape" for this binder).
    pub fn result_type_name(&self) -> &'static str {
        "TopoDS_Shape"
    }

    /// Sets the shape result.
    pub fn set_result(&mut self, shape: [f64; 3]) {
        self.shape = Some(shape);
        self.is_set = true;
    }

    /// Returns the shape result if set.
    pub fn result(&self) -> Option<[f64; 3]> {
        self.shape
    }

    /// Returns whether the result has been set.
    pub fn has_result(&self) -> bool {
        self.is_set
    }

    /// Returns a mutable reference to the shape (creates one if not set).
    pub fn c_result_mut(&mut self) -> &mut [f64; 3] {
        if self.shape.is_none() {
            self.shape = Some([0.0; 3]);
        }
        self.is_set = true;
        self.shape.as_mut().unwrap()
    }

    /// Returns whether this binder can have multiple results (always false for shape binder).
    pub fn multiple(&self) -> bool {
        false
    }
}

impl Default for TransferBRepBinderOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let binder = TransferBRepBinderOfShape::new();
        assert!(!binder.has_result());
        assert_eq!(binder.result(), None);
    }

    #[test]
    fn test_with_shape() {
        let shape = [1.0, 2.0, 3.0];
        let binder = TransferBRepBinderOfShape::with_shape(shape);
        assert!(binder.has_result());
        assert_eq!(binder.result(), Some(shape));
    }

    #[test]
    fn test_set_result() {
        let mut binder = TransferBRepBinderOfShape::new();
        let shape = [4.0, 5.0, 6.0];
        binder.set_result(shape);
        assert!(binder.has_result());
        assert_eq!(binder.result(), Some(shape));
    }

    #[test]
    fn test_result_type_name() {
        let binder = TransferBRepBinderOfShape::new();
        assert_eq!(binder.result_type_name(), "TopoDS_Shape");
    }

    #[test]
    fn test_multiple_always_false() {
        let binder = TransferBRepBinderOfShape::new();
        assert!(!binder.multiple());
    }

    #[test]
    fn test_c_result_mut() {
        let mut binder = TransferBRepBinderOfShape::new();
        {
            let shape_ref = binder.c_result_mut();
            shape_ref[0] = 7.0;
            shape_ref[1] = 8.0;
            shape_ref[2] = 9.0;
        }
        assert!(binder.has_result());
        assert_eq!(binder.result(), Some([7.0, 8.0, 9.0]));
    }

    #[test]
    fn test_default() {
        let binder = TransferBRepBinderOfShape::default();
        assert!(!binder.has_result());
    }
}
