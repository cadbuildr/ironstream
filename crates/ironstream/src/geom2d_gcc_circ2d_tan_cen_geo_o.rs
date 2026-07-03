// FILE: geom2d_gcc_circ2d_tan_cen_geo_o.rs
// occt: Geom2dGcc_Circ2dTanCenGeo

/// Circle tangent to geometry with center on geometry.
pub struct Circ2dTanCenGeo {
    done: bool,
    nb_solutions: i32,
}

impl Circ2dTanCenGeo {
    pub fn new() -> Self {
        Circ2dTanCenGeo {
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

impl Default for Circ2dTanCenGeo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2dTanCenGeo::new();
        assert!(!solver.is_done());
    }
}
