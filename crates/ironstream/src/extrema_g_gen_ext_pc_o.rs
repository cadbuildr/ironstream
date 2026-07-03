// FILE: extrema_g_gen_ext_pc_o.rs
// occt: Extrema_GGenExtPC

/// Generic extrema solver for point-curve.
pub struct GGenExtPC {
    done: bool,
    nb_ext: i32,
    sq_distances: Vec<f64>,
}

impl GGenExtPC {
    /// Initialize solver.
    pub fn new() -> Self {
        GGenExtPC {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Returns computation status.
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

impl Default for GGenExtPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = GGenExtPC::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_ext(), 0);
    }

    #[test]
    fn test_default() {
        let solver = GGenExtPC::default();
        assert!(!solver.is_done());
    }
}
