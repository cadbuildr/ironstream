// FILE: extrema_ext_p_el_c2d_o.rs
// occt: Extrema_ExtPElC2d

/// Extrema calculator for point and elementary 2D curve.
pub struct ExtPElC2d {
    done: bool,
    nb_ext: i32,
    sq_distances: Vec<f64>,
}

impl ExtPElC2d {
    /// Initialize calculator.
    pub fn new() -> Self {
        ExtPElC2d {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Returns whether computation succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns number of extrema.
    pub fn nb_ext(&self) -> i32 {
        self.nb_ext
    }

    /// Returns square distance for Nth extremum.
    pub fn square_distance(&self, n: i32) -> Result<f64, &'static str> {
        let idx = (n - 1) as usize;
        if idx >= self.sq_distances.len() {
            return Err("Index out of range");
        }
        Ok(self.sq_distances[idx])
    }
}

impl Default for ExtPElC2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext = ExtPElC2d::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_default() {
        let ext = ExtPElC2d::default();
        assert!(!ext.is_done());
    }
}
