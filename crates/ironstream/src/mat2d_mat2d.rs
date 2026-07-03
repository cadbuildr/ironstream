// FILE: mat2d_mat2d.rs
// occt: MAT2d_Mat2d

/// Generic algorithm for computation of the bisecting locus.
#[derive(Debug, Clone)]
pub struct Mat2dMat2d {
    is_done: bool,
    nb_bisectors: i32,
}

impl Mat2dMat2d {
    pub fn new() -> Self {
        Mat2dMat2d {
            is_done: false,
            nb_bisectors: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn number_of_bisectors(&self) -> i32 {
        self.nb_bisectors
    }
}

impl Default for Mat2dMat2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat2d() {
        let m = Mat2dMat2d::new();
        assert!(!m.is_done());
    }
}
