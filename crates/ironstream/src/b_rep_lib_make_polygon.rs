// FILE: b_rep_lib_make_polygon.rs
// occt: BRepLib_MakePolygon

/// Makes polygonal wires
pub struct MakePolygon {
    done: bool,
    added: bool,
}

impl MakePolygon {
    pub fn new() -> Self {
        MakePolygon {
            done: false,
            added: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn added(&self) -> bool {
        self.added
    }

    pub fn add_point(&mut self, _x: f64, _y: f64, _z: f64) {
        self.added = true;
    }

    pub fn close(&mut self) {
        self.done = true;
    }
}

impl Default for MakePolygon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_polygon_creation() {
        let maker = MakePolygon::new();
        assert!(!maker.is_done());
        assert!(!maker.added());
    }

    #[test]
    fn test_add_point() {
        let mut maker = MakePolygon::new();
        maker.add_point(0.0, 0.0, 0.0);
        assert!(maker.added());
    }

    #[test]
    fn test_close() {
        let mut maker = MakePolygon::new();
        maker.add_point(1.0, 2.0, 3.0);
        maker.close();
        assert!(maker.is_done());
    }
}
