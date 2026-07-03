// FILE: geom2d_gcc_circ2d_tan_on_rad_o.rs
// occt: Geom2dGcc_Circ2dTanOnRad

/// Circle with given radius, tangent to geometry, on another geometry.
pub struct Circ2dTanOnRad {
    done: bool,
    nb_solutions: i32,
}

impl Circ2dTanOnRad {
    pub fn new() -> Self {
        Circ2dTanOnRad {
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

impl Default for Circ2dTanOnRad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2dTanOnRad::new();
        assert!(!solver.is_done());
    }
}
