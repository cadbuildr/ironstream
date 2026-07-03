// FILE: topo_ds_t_shape.rs
// occt: TopoDS_TShape

/// Minimal implementation of TopoDS_TShape
pub struct TopoDS_TShape { }

impl Default for TopoDS_TShape {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = TopoDS_TShape::default();
    }
}
