// FILE: b_rep_extrema_ext_cc.rs
// occt: BRepExtrema_ExtCC

/// Curve-curve extrema distances
pub struct ExtCC {
    is_done: bool,
    nb_ext: i32,
}

impl ExtCC {
    pub fn new() -> Self {
        ExtCC {
            is_done: false,
            nb_ext: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_ext(&self) -> i32 {
        self.nb_ext
    }

    pub fn is_parallel(&self) -> bool {
        false
    }

    pub fn square_distance(&self, _n: i32) -> f64 {
        0.0
    }
}

impl Default for ExtCC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_cc_creation() {
        let ext = ExtCC::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_parallel() {
        let ext = ExtCC::new();
        assert!(!ext.is_parallel());
    }
}
