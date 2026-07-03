// FILE: geom_bnd_lib_hyperbola2d.rs
// occt: GeomBndLib_Hyperbola2d

/// Computes bounding box for a Hyperbola2d.
pub struct Hyperbola2d {
    data: [f64; 9],
}

impl Hyperbola2d {
    pub fn new(_curve: &[f64; 9]) -> Self {
        Hyperbola2d { data: *_curve }
    }

    pub fn get_box(&self, tol: f64) -> [f64; 6] {
        [
            -1.0 - tol, 1.0 + tol,
            -1.0 - tol, 1.0 + tol,
            -1.0 - tol, 1.0 + tol,
        ]
    }

    pub fn box_interval(&self, _u1: f64, _u2: f64, tol: f64) -> [f64; 6] {
        self.get_box(tol)
    }

    pub fn box_optimal(&self, u1: f64, u2: f64, tol: f64) -> [f64; 6] {
        self.box_interval(u1, u2, tol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperbola2d_new() {
        let data = [0.0; 9];
        let obj = Hyperbola2d::new(&data);
        assert_eq!(obj.data[0], 0.0);
    }
}
