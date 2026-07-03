// FILE: b_rep_extrema_ext_pc.rs
// occt: BRepExtrema_ExtPC

/// Point-curve extrema distances
pub struct ExtPC {
    is_done: bool,
    nb_ext: i32,
}

impl ExtPC {
    pub fn new() -> Self {
        ExtPC {
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

    pub fn is_min(&self, _n: i32) -> bool {
        true
    }

    pub fn square_distance(&self, _n: i32) -> f64 {
        0.0
    }
}

impl Default for ExtPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_pc_creation() {
        let ext = ExtPC::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_is_min() {
        let ext = ExtPC::new();
        assert!(ext.is_min(0));
    }
}
