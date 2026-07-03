// FILE: aspect_frustum_lrbt.rs
// occt: Aspect_FrustumLRBT

/// Frustum defined by left, right, bottom, top clipping planes.
#[derive(Debug, Clone, Copy)]
pub struct AspectFrustumLRBT {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,
}

impl AspectFrustumLRBT {
    /// Create frustum.
    pub fn new(left: f64, right: f64, bottom: f64, top: f64) -> Self {
        AspectFrustumLRBT {
            left,
            right,
            bottom,
            top,
        }
    }

    /// Get width.
    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    /// Get height.
    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let frustum = AspectFrustumLRBT::new(-1.0, 1.0, -1.0, 1.0);
        assert_eq!(frustum.left, -1.0);
        assert_eq!(frustum.width(), 2.0);
        assert_eq!(frustum.height(), 2.0);
    }
}
