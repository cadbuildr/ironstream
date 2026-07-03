// FILE: b_rep_top_adaptor_f_class2d.rs
// occt: BRepTopAdaptor_FClass2d

/// Point classification tool for 2D problems on a face.
#[derive(Debug, Clone)]
pub struct BRepTopAdaptorFClass2d {
    toluv: f64,
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
}

impl BRepTopAdaptorFClass2d {
    pub fn new(tol: f64) -> Self {
        BRepTopAdaptorFClass2d {
            toluv: tol,
            u1: 0.0,
            v1: 0.0,
            u2: 1.0,
            v2: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fclass2d() {
        let fc = BRepTopAdaptorFClass2d::new(0.01);
        assert_eq!(fc.toluv, 0.01);
    }
}
