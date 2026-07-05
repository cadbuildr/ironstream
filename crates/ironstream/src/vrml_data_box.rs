// FILE: vrml_data_box.rs
// occt: VrmlData_Box

#[derive(Clone, Debug)]
pub struct VrmlDataBox {
    width: f64,
    height: f64,
    depth: f64,
}

impl VrmlDataBox {
    pub fn new(w: f64, h: f64, d: f64) -> Self {
        VrmlDataBox {
            width: w,
            height: h,
            depth: d,
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = VrmlDataBox::new(1.0, 2.0, 3.0);
        assert_eq!(b.width(), 1.0);
        assert_eq!(b.height(), 2.0);
        assert_eq!(b.depth(), 3.0);
    }
}
