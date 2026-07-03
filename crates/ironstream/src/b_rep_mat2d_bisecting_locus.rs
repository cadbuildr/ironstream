// FILE: b_rep_mat2d_bisecting_locus.rs
// occt: BRepMAT2d_BisectingLocus

/// Generates and contains the bisecting locus of a set of lines from Geom2d.
#[derive(Debug, Clone)]
pub struct BRepMat2dBisectingLocus {
    is_done: bool,
    nb_contours: i32,
}

impl BRepMat2dBisectingLocus {
    pub fn new() -> Self {
        BRepMat2dBisectingLocus {
            is_done: false,
            nb_contours: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn number_of_contours(&self) -> i32 {
        self.nb_contours
    }
}

impl Default for BRepMat2dBisectingLocus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisecting_locus() {
        let bl = BRepMat2dBisectingLocus::new();
        assert!(!bl.is_done());
        assert_eq!(bl.number_of_contours(), 0);
    }
}
