// FILE: geom_fill_circular_blend_func.rs
// occt: GeomFill_CircularBlendFunc

/// Circular blending function for surface generation
pub struct CircularBlendFunc {
    radius: f64,
    is_done: bool,
}

impl CircularBlendFunc {
    pub fn new(radius: f64) -> Self {
        CircularBlendFunc {
            radius,
            is_done: false,
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let func = CircularBlendFunc::new(1.5);
        assert_eq!(func.radius(), 1.5);
        assert!(!func.is_done());
    }

    #[test]
    fn test_zero_radius() {
        let func = CircularBlendFunc::new(0.0);
        assert_eq!(func.radius(), 0.0);
    }
}
