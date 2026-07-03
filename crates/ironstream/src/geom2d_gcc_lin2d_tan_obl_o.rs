// FILE: geom2d_gcc_lin2d_tan_obl_o.rs
// occt: Geom2dGcc_Lin2dTanObl

/// Solver for line with oblique tangency.
pub struct Lin2dTanObl {
    done: bool,
    nb_solutions: i32,
}

impl Lin2dTanObl {
    pub fn new() -> Self {
        Lin2dTanObl {
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

impl Default for Lin2dTanObl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Lin2dTanObl::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_solutions(), 0);
    }
}
