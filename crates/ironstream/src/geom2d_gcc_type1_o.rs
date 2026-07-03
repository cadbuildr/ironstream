// FILE: geom2d_gcc_type1_o.rs
// occt: Geom2dGcc_Type1

/// Type 1 tangency problem solver.
pub struct Type1 {
    done: bool,
    nb_solutions: i32,
}

impl Type1 {
    pub fn new() -> Self {
        Type1 {
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

impl Default for Type1 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Type1::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_solutions(), 0);
    }
}
