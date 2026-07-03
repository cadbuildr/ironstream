// FILE: geom2d_gcc_circ2d_tan_cen_o.rs
// occt: Geom2dGcc_Circ2dTanCen

/// Circle tangent to geometry with given center.
pub struct Circ2dTanCen {
    done: bool,
    nb_solutions: i32,
}

impl Circ2dTanCen {
    pub fn new() -> Self {
        Circ2dTanCen {
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

impl Default for Circ2dTanCen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Circ2dTanCen::new();
        assert!(!solver.is_done());
    }
}
