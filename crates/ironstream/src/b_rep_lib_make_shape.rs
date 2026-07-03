// FILE: b_rep_lib_make_shape.rs
// occt: BRepLib_MakeShape

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeModification {
    Unchanged,
    Modified,
    Degenerated,
}

/// Root class for all shape construction commands
pub struct MakeShape {
    done: bool,
}

impl MakeShape {
    pub fn new() -> Self {
        MakeShape { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn build(&mut self) {}

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn nb_surfaces(&self) -> i32 {
        0
    }
}

impl Default for MakeShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_shape_creation() {
        let shape = MakeShape::new();
        assert!(!shape.is_done());
    }

    #[test]
    fn test_set_done() {
        let mut shape = MakeShape::new();
        shape.set_done();
        assert!(shape.is_done());
    }

    #[test]
    fn test_nb_surfaces() {
        let shape = MakeShape::new();
        assert_eq!(shape.nb_surfaces(), 0);
    }
}
