// FILE: hlr_algo_poly_hiding_data.rs
// occt: HLRAlgo_PolyHidingData

/// Data for a hiding triangle (plane equation and indices).
#[derive(Clone, Copy)]
pub struct HlrAlgoPolyHidingData {
    index: i32,      // Triangle index
    min: i32,        // Min index
    max: i32,        // Max index
    normal_x: f64,   // Plane normal X
    normal_y: f64,   // Plane normal Y
    normal_z: f64,   // Plane normal Z
    d: f64,          // Plane D coefficient
}

impl HlrAlgoPolyHidingData {
    /// Create default hiding data.
    pub fn new() -> Self {
        HlrAlgoPolyHidingData {
            index: 0,
            min: 0,
            max: 0,
            normal_x: 0.0,
            normal_y: 0.0,
            normal_z: 0.0,
            d: 0.0,
        }
    }

    /// Set all parameters at once.
    pub fn set(&mut self, idx: i32, min: i32, max: i32, a: f64, b: f64, c: f64, d: f64) {
        self.index = idx;
        self.min = min;
        self.max = max;
        self.normal_x = a;
        self.normal_y = b;
        self.normal_z = c;
        self.d = d;
    }

    /// Get index.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Get min.
    pub fn min(&self) -> i32 {
        self.min
    }

    /// Get max.
    pub fn max(&self) -> i32 {
        self.max
    }

    /// Get plane normal as [x, y, z].
    pub fn normal(&self) -> [f64; 3] {
        [self.normal_x, self.normal_y, self.normal_z]
    }

    /// Get plane D coefficient.
    pub fn d(&self) -> f64 {
        self.d
    }
}

impl Default for HlrAlgoPolyHidingData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hd = HlrAlgoPolyHidingData::new();
        assert_eq!(hd.index(), 0);
        assert_eq!(hd.d(), 0.0);
    }

    #[test]
    fn test_set() {
        let mut hd = HlrAlgoPolyHidingData::new();
        hd.set(5, 1, 10, 1.0, 0.0, 0.0, 2.5);
        assert_eq!(hd.index(), 5);
        assert_eq!(hd.min(), 1);
        assert_eq!(hd.max(), 10);
        assert_eq!(hd.normal(), [1.0, 0.0, 0.0]);
        assert_eq!(hd.d(), 2.5);
    }

    #[test]
    fn test_default() {
        let hd = HlrAlgoPolyHidingData::default();
        assert_eq!(hd.index(), 0);
    }
}
