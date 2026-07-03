// FILE: b_rep_extrema_ext_ff.rs
// occt: BRepExtrema_ExtFF

/// Face-face extrema distances
pub struct ExtFF {
    is_done: bool,
    nb_ext: i32,
}

impl ExtFF {
    pub fn new() -> Self {
        ExtFF {
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

impl Default for ExtFF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_ff_creation() {
        let ext = ExtFF::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_parallel() {
        let ext = ExtFF::new();
        assert!(!ext.is_parallel());
    }
}
