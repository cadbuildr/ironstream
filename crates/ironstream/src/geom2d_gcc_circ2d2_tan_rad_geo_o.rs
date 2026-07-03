// FILE: geom2d_gcc_circ2d2_tan_rad_geo_o.rs
// occt: Geom2dGcc_Circ2d2TanRadGeo

/// Circle with given radius, tangent to two geometries.
pub struct Circ2d2TanRadGeo {
    done: bool,
    nb_solutions: i32,
}

impl Circ2d2TanRadGeo {
    pub fn new() -> Self {
        Circ2d2TanRadGeo {
            done: false,
            nb_solutions: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn nb_solutions(&self) -> i32 {
        self.nb_solutions
    }
}

impl Default for Circ2d2TanRadGeo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2d2TanRadGeo::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_solutions(), 0);
    }
}
