// FILE: geom2d_gcc_circ2d2_tan_on_geo_o.rs
// occt: Geom2dGcc_Circ2d2TanOnGeo

/// Circle tangent to two geometries and on another geometry.
pub struct Circ2d2TanOnGeo {
    done: bool,
    nb_solutions: i32,
}

impl Circ2d2TanOnGeo {
    /// Initialize solver.
    pub fn new() -> Self {
        Circ2d2TanOnGeo {
            done: false,
            nb_solutions: 0,
        }
    }

    /// Returns computation status.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns number of solutions.
    pub fn nb_solutions(&self) -> i32 {
        self.nb_solutions
    }
}

impl Default for Circ2d2TanOnGeo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2d2TanOnGeo::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_solutions(), 0);
    }

    #[test]
    fn test_default() {
        let solver = Circ2d2TanOnGeo::default();
        assert!(!solver.is_done());
    }
}
