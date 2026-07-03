// FILE: b_rep_extrema_ext_pf.rs
// occt: BRepExtrema_ExtPF

/// Point-face extrema distances
pub struct ExtPF {
    is_done: bool,
    nb_ext: i32,
}

impl ExtPF {
    pub fn new() -> Self {
        ExtPF {
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

    pub fn square_distance(&self, _n: i32) -> f64 {
        0.0
    }
}

impl Default for ExtPF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_pf_creation() {
        let ext = ExtPF::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_square_distance() {
        let ext = ExtPF::new();
        assert_eq!(ext.square_distance(0), 0.0);
    }
}
