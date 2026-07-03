// FILE: geom2d_gcc_lin2d2_tan.rs
// occt: Geom2dGcc_Lin2d2Tan

/// A 2D line represented by coefficients ax + by + c = 0
#[derive(Clone, Copy, Debug)]
pub struct Lin2d {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Lin2d {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Lin2d { a, b, c }
    }

    /// Compute distance from point to line
    pub fn distance_to_point(&self, x: f64, y: f64) -> f64 {
        (self.a * x + self.b * y + self.c).abs() / (self.a * self.a + self.b * self.b).sqrt()
    }
}

/// Algorithm to find lines tangent to two entities
#[derive(Clone, Debug)]
pub struct Geom2dGccLin2d2Tan {
    solutions: Vec<Lin2d>,
    done: bool,
}

impl Geom2dGccLin2d2Tan {
    pub fn new() -> Self {
        Geom2dGccLin2d2Tan {
            solutions: Vec::new(),
            done: false,
        }
    }

    pub fn add_solution(&mut self, line: Lin2d) {
        self.solutions.push(line);
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn nb_solutions(&self) -> usize {
        self.solutions.len()
    }

    pub fn this_solution(&self, index: usize) -> Option<Lin2d> {
        if index > 0 && index <= self.solutions.len() {
            Some(self.solutions[index - 1])
        } else {
            None
        }
    }
}

impl Default for Geom2dGccLin2d2Tan {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Lin2d::new(1.0, 0.0, 0.0);
        assert_eq!(line.a, 1.0);
    }

    #[test]
    fn test_algorithm_creation() {
        let algo = Geom2dGccLin2d2Tan::new();
        assert!(!algo.is_done());
        assert_eq!(algo.nb_solutions(), 0);
    }

    #[test]
    fn test_add_solution() {
        let mut algo = Geom2dGccLin2d2Tan::new();
        algo.add_solution(Lin2d::new(1.0, 1.0, -2.0));
        algo.set_done();

        assert!(algo.is_done());
        assert_eq!(algo.nb_solutions(), 1);
    }

    #[test]
    fn test_retrieve_solution() {
        let mut algo = Geom2dGccLin2d2Tan::new();
        let line = Lin2d::new(2.0, 3.0, 4.0);
        algo.add_solution(line);

        let sol = algo.this_solution(1);
        assert!(sol.is_some());
        let sol = sol.unwrap();
        assert_eq!(sol.a, 2.0);
        assert_eq!(sol.b, 3.0);
        assert_eq!(sol.c, 4.0);
    }
}
