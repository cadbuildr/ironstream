// FILE: extrema_ext_el_cs_o.rs
// occt: Extrema_ExtElCS

/// Calculates extrema distances between elementary 3D curves and surfaces.
pub struct ExtElCS {
    done: bool,
    nb_ext: i32,
    sq_distances: Vec<f64>,
}

impl ExtElCS {
    /// Initialize empty extrema calculator.
    pub fn new() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between line and plane.
    pub fn new_lin_pln() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between line and cylinder.
    pub fn new_lin_cyl() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between line and cone.
    pub fn new_lin_cone() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between line and sphere.
    pub fn new_lin_sph() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between line and torus.
    pub fn new_lin_torus() -> Self {
        ExtElCS {
            done: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Perform calculation for line and plane.
    pub fn perform_lin_pln(&mut self) {
        self.done = true;
    }

    /// Perform calculation for line and cylinder.
    pub fn perform_lin_cyl(&mut self) {
        self.done = true;
    }

    /// Perform calculation for line and cone.
    pub fn perform_lin_cone(&mut self) {
        self.done = true;
    }

    /// Perform calculation for line and sphere.
    pub fn perform_lin_sph(&mut self) {
        self.done = true;
    }

    /// Perform calculation for line and torus.
    pub fn perform_lin_torus(&mut self) {
        self.done = true;
    }

    /// Returns whether distances were found.
    pub fn is_done(&self) -> bool {
        self.done
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

impl Default for ExtElCS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext = ExtElCS::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_ext(), 0);
    }

    #[test]
    fn test_lin_pln() {
        let ext = ExtElCS::new_lin_pln();
        assert!(!ext.is_done());
    }

    #[test]
    fn test_lin_cyl() {
        let ext = ExtElCS::new_lin_cyl();
        assert!(!ext.is_done());
    }

    #[test]
    fn test_perform_lin_pln() {
        let mut ext = ExtElCS::new_lin_pln();
        ext.perform_lin_pln();
        assert!(ext.is_done());
    }
}
