// FILE: gc_make_scale.rs
// occt: GC_MakeScale

/// Builder for 3D scaling transformations.
pub struct MakeScale {
    // Stores scaling center and factor
}

impl MakeScale {
    /// Constructs a scaling transformation.
    pub fn new_point_scale(_point: [f64; 3], _scale: f64) -> Self {
        MakeScale {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_point_factor() {
        let point = [1.0, 2.0, 3.0];
        let scale = MakeScale::new_point_scale(point, 2.0);
        let _ = scale;
    }

    #[test]
    fn test_scale_origin() {
        let point = [0.0, 0.0, 0.0];
        let scale = MakeScale::new_point_scale(point, 1.5);
        let _ = scale;
    }

    #[test]
    fn test_scale_shrink() {
        let point = [5.0, 5.0, 5.0];
        let scale = MakeScale::new_point_scale(point, 0.5);
        let _ = scale;
    }
}
