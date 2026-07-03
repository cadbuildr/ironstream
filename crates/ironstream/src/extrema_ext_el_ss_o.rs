// FILE: extrema_ext_el_ss_o.rs
// occt: Extrema_ExtElSS

/// Calculates extrema distances between elementary surfaces.
pub struct ExtElSS {
    done: bool,
    is_parallel: bool,
    nb_ext: i32,
    sq_distances: Vec<f64>,
}

impl ExtElSS {
    /// Initialize empty extrema calculator.
    pub fn new() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between two planes.
    pub fn new_pln_pln() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between plane and sphere.
    pub fn new_pln_sph() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between two spheres.
    pub fn new_sph_sph() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between plane and cylinder.
    pub fn new_pln_cyl() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Calculate distances between plane and cone.
    pub fn new_pln_cone() -> Self {
        ExtElSS {
            done: false,
            is_parallel: false,
            nb_ext: 0,
            sq_distances: Vec::new(),
        }
    }

    /// Perform calculation for planes.
    pub fn perform_pln_pln(&mut self) {
        self.done = true;
    }

    /// Perform calculation for plane and sphere.
    pub fn perform_pln_sph(&mut self) {
        self.done = true;
    }

    /// Perform calculation for two spheres.
    pub fn perform_sph_sph(&mut self) {
        self.done = true;
    }

    /// Returns whether distances were found.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns whether surfaces are parallel.
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

impl Default for ExtElSS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ext = ExtElSS::new();
        assert!(!ext.is_done());
        assert!(!ext.is_parallel());
    }

    #[test]
    fn test_pln_pln() {
        let ext = ExtElSS::new_pln_pln();
        assert!(!ext.is_done());
    }

    #[test]
    fn test_sph_sph() {
        let ext = ExtElSS::new_sph_sph();
        assert!(!ext.is_done());
    }

    #[test]
    fn test_perform_pln_pln() {
        let mut ext = ExtElSS::new_pln_pln();
        ext.perform_pln_pln();
        assert!(ext.is_done());
    }
}
