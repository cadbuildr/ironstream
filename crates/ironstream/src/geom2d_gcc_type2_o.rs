// FILE: geom2d_gcc_type2_o.rs
// occt: Geom2dGcc_Type2

/// Type 2 tangency problem solver.
pub struct Type2 {
    done: bool,
    nb_solutions: i32,
}

impl Type2 {
    pub fn new() -> Self {
        Type2 {
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

impl Default for Type2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solver = Type2::new();
        assert!(!solver.is_done());
        assert_eq!(solver.nb_solutions(), 0);
    }
}
