// FILE: geom2d_gcc_circ2d_tan_on_rad_geo_o.rs
// occt: Geom2dGcc_Circ2dTanOnRadGeo

/// Circle with given radius on geometry, tangent to another.
pub struct Circ2dTanOnRadGeo {
    done: bool,
    nb_solutions: i32,
}

impl Circ2dTanOnRadGeo {
    pub fn new() -> Self {
        Circ2dTanOnRadGeo {
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

impl Default for Circ2dTanOnRadGeo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2dTanOnRadGeo::new();
        assert!(!solver.is_done());
    }
}
