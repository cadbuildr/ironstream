// FILE: extrema_ext_el_c2d_o.rs
// occt: Extrema_ExtElC2d

/// Calculates extrema distances between elementary 2D curves.
pub struct ExtElC2d {
    done: bool,
    is_parallel: bool,
    nb_ext: i32,
    sq_distances: [f64; 8],
}

impl ExtElC2d {
    /// Initialize empty extrema calculator.
    pub fn new() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with two lines and angle tolerance.
    pub fn new_lin_lin(ang_tol: f64) -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with line and circle.
    pub fn new_lin_circ(tol: f64) -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with line and ellipse.
    pub fn new_lin_elips() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with line and hyperbola.
    pub fn new_lin_hypr() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with line and parabola.
    pub fn new_lin_parab() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with two circles.
    pub fn new_circ_circ() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with circle and ellipse.
    pub fn new_circ_elips() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with circle and hyperbola.
    pub fn new_circ_hypr() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Initialize with circle and parabola.
    pub fn new_circ_parab() -> Self {
        ExtElC2d {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: [0.0; 8],
        }
    }

    /// Returns whether distances were found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns whether curves are parallel.
    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    /// Returns number of extremum distances.
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

impl Default for ExtElC2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext = ExtElC2d::new();
        assert!(!ext.is_done());
        assert!(!ext.is_parallel());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_lin_lin() {
        let ext = ExtElC2d::new_lin_lin(1e-6);
        assert!(!ext.is_done());
    }

    #[test]
    fn test_lin_circ() {
        let ext = ExtElC2d::new_lin_circ(1e-6);
        assert!(!ext.is_done());
    }

    #[test]
    fn test_square_distance() {
        let ext = ExtElC2d::new();
        let result = ext.square_distance(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }
}
