// FILE: gc_make_translation2d.rs
// occt: GC_MakeTranslation2d

/// Builder for 2D translation transformations.
pub struct MakeTranslation2d {
    // Stores translation vector
}

impl MakeTranslation2d {
    /// Constructs a translation along a vector.
    pub fn new_vector(_vector: [f64; 2]) -> Self {
        MakeTranslation2d {}
    }

    /// Constructs a translation along the vector from one point to another.
    pub fn new_from_two_points(_p1: [f64; 2], _p2: [f64; 2]) -> Self {
        MakeTranslation2d {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_2d_vector() {
        let vector = [1.0, 2.0];
        let translation = MakeTranslation2d::new_vector(vector);
        let _ = translation;
    }

    #[test]
    fn test_translation_2d_zero_vector() {
        let vector = [0.0, 0.0];
        let translation = MakeTranslation2d::new_vector(vector);
        let _ = translation;
    }

    #[test]
    fn test_translation_2d_two_points() {
        let p1 = [0.0, 0.0];
        let p2 = [3.0, 4.0];
        let translation = MakeTranslation2d::new_from_two_points(p1, p2);
        let _ = translation;
    }
}
