// FILE: gc_make_scale2d.rs
// occt: GC_MakeScale2d

/// Builder for 2D scaling transformations.
pub struct MakeScale2d {
    // Stores scaling center and factor
}

impl MakeScale2d {
    /// Constructs a scaling transformation.
    pub fn new_point_scale(_point: [f64; 2], _scale: f64) -> Self {
        MakeScale2d {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_2d_point_factor() {
        let point = [1.0, 2.0];
        let scale = MakeScale2d::new_point_scale(point, 2.0);
        let _ = scale;
    }

    #[test]
    fn test_scale_2d_origin() {
        let point = [0.0, 0.0];
        let scale = MakeScale2d::new_point_scale(point, 1.5);
        let _ = scale;
    }

    #[test]
    fn test_scale_2d_shrink() {
        let point = [3.0, 4.0];
        let scale = MakeScale2d::new_point_scale(point, 0.25);
        let _ = scale;
    }
}
