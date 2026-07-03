// FILE: mat2d_mini_path.rs
// occt: MAT2d_MiniPath

/// Computes a path to link all lines in a set of lines.
#[derive(Debug, Clone)]
pub struct Mat2dMiniPath {
    ind_start: i32,
}

impl Mat2dMiniPath {
    pub fn new() -> Self {
        Mat2dMiniPath { ind_start: 0 }
    }
}

impl Default for Mat2dMiniPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_path() {
        let mp = Mat2dMiniPath::new();
        assert_eq!(mp.ind_start, 0);
    }
}
