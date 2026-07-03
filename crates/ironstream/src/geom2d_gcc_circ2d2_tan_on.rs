// FILE: geom2d_gcc_circ2d2_tan_on.rs
// occt: Geom2dGcc_Circ2d2TanOn

use std::f64;

/// Enumeration for position/qualifier types (simplified)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GccEntPosition {
    Enclosed = 0,
    Enclosing = 1,
    Outside = 2,
    Unqualified = 3,
}

/// A point in 2D space
#[derive(Clone, Copy, Debug)]
pub struct Pnt2d {
    pub x: f64,
    pub y: f64,
}

impl Pnt2d {
    pub fn new(x: f64, y: f64) -> Self {
        Pnt2d { x, y }
    }

    pub fn distance_to(&self, other: &Pnt2d) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A 2D circle
#[derive(Clone, Copy, Debug)]
pub struct Circ2d {
    pub center: Pnt2d,
    pub radius: f64,
}

impl Circ2d {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Circ2d {
            center: Pnt2d::new(cx, cy),
            radius: radius.abs(),
        }
    }

    pub fn distance_to_point(&self, p: &Pnt2d) -> f64 {
        (self.center.distance_to(p) - self.radius).abs()
    }
}

/// Algorithm to find 2D circles tangent to two entities and with center on a curve.
#[derive(Clone, Debug)]
pub struct Geom2dGccCirc2d2TanOn {
    solutions: Vec<Circ2d>,
    qualifiers1: Vec<GccEntPosition>,
    qualifiers2: Vec<GccEntPosition>,
    tangency1_points: Vec<Pnt2d>,
    tangency2_points: Vec<Pnt2d>,
    center_points: Vec<Pnt2d>,
    is_same1: Vec<bool>,
    is_same2: Vec<bool>,
    done: bool,
}

impl Geom2dGccCirc2d2TanOn {
    /// Creates a new algorithm instance.
    pub fn new() -> Self {
        Geom2dGccCirc2d2TanOn {
            solutions: Vec::new(),
            qualifiers1: Vec::new(),
            qualifiers2: Vec::new(),
            tangency1_points: Vec::new(),
            tangency2_points: Vec::new(),
            center_points: Vec::new(),
            is_same1: Vec::new(),
            is_same2: Vec::new(),
            done: false,
        }
    }

    /// Adds a solution circle with tangency information.
    pub fn add_solution(
        &mut self,
        circ: Circ2d,
        qual1: GccEntPosition,
        qual2: GccEntPosition,
        tg1: Pnt2d,
        tg2: Pnt2d,
        center: Pnt2d,
        same1: bool,
        same2: bool,
    ) {
        self.solutions.push(circ);
        self.qualifiers1.push(qual1);
        self.qualifiers2.push(qual2);
        self.tangency1_points.push(tg1);
        self.tangency2_points.push(tg2);
        self.center_points.push(center);
        self.is_same1.push(same1);
        self.is_same2.push(same2);
    }

    /// Marks the algorithm as done.
    pub fn set_done(&mut self) {
        self.done = true;
    }

    /// Returns true if algorithm succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of solutions found.
    pub fn nb_solutions(&self) -> usize {
        self.solutions.len()
    }

    /// Returns the solution at the given index.
    pub fn this_solution(&self, index: usize) -> Option<Circ2d> {
        if index > 0 && index <= self.solutions.len() {
            Some(self.solutions[index - 1])
        } else {
            None
        }
    }

    /// Returns the qualifiers for solution at index.
    pub fn which_qualifier(&self, index: usize) -> Option<(GccEntPosition, GccEntPosition)> {
        if index > 0 && index <= self.qualifiers1.len() {
            Some((self.qualifiers1[index - 1], self.qualifiers2[index - 1]))
        } else {
            None
        }
    }

    /// Returns tangency information for first tangency point.
    pub fn tangency1(&self, index: usize) -> Option<Pnt2d> {
        if index > 0 && index <= self.tangency1_points.len() {
            Some(self.tangency1_points[index - 1])
        } else {
            None
        }
    }

    /// Returns tangency information for second tangency point.
    pub fn tangency2(&self, index: usize) -> Option<Pnt2d> {
        if index > 0 && index <= self.tangency2_points.len() {
            Some(self.tangency2_points[index - 1])
        } else {
            None
        }
    }

    /// Returns center point information.
    pub fn center_on3(&self, index: usize) -> Option<Pnt2d> {
        if index > 0 && index <= self.center_points.len() {
            Some(self.center_points[index - 1])
        } else {
            None
        }
    }

    /// Returns true if solution is the same as first argument.
    pub fn is_the_same1(&self, index: usize) -> Option<bool> {
        if index > 0 && index <= self.is_same1.len() {
            Some(self.is_same1[index - 1])
        } else {
            None
        }
    }

    /// Returns true if solution is the same as second argument.
    pub fn is_the_same2(&self, index: usize) -> Option<bool> {
        if index > 0 && index <= self.is_same2.len() {
            Some(self.is_same2[index - 1])
        } else {
            None
        }
    }
}

impl Default for Geom2dGccCirc2d2TanOn {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnt2d_distance() {
        let p1 = Pnt2d::new(0.0, 0.0);
        let p2 = Pnt2d::new(3.0, 4.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_circ2d_creation() {
        let circ = Circ2d::new(1.0, 2.0, 5.0);
        assert_eq!(circ.center.x, 1.0);
        assert_eq!(circ.center.y, 2.0);
        assert_eq!(circ.radius, 5.0);
    }

    #[test]
    fn test_algorithm_creation() {
        let algo = Geom2dGccCirc2d2TanOn::new();
        assert!(!algo.is_done());
        assert_eq!(algo.nb_solutions(), 0);
    }

    #[test]
    fn test_add_solution() {
        let mut algo = Geom2dGccCirc2d2TanOn::new();
        let circ = Circ2d::new(1.0, 1.0, 2.0);
        let tg1 = Pnt2d::new(2.0, 1.0);
        let tg2 = Pnt2d::new(1.0, 3.0);
        let center = Pnt2d::new(1.0, 1.0);

        algo.add_solution(circ, GccEntPosition::Outside, GccEntPosition::Outside, tg1, tg2, center, false, false);
        algo.set_done();

        assert!(algo.is_done());
        assert_eq!(algo.nb_solutions(), 1);
    }

    #[test]
    fn test_this_solution() {
        let mut algo = Geom2dGccCirc2d2TanOn::new();
        let circ = Circ2d::new(5.0, 6.0, 3.0);
        algo.add_solution(circ, GccEntPosition::Outside, GccEntPosition::Outside, Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), false, false);

        let sol = algo.this_solution(1);
        assert!(sol.is_some());
        let sol = sol.unwrap();
        assert!((sol.center.x - 5.0).abs() < 1e-10);
        assert!((sol.radius - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_qualifiers() {
        let mut algo = Geom2dGccCirc2d2TanOn::new();
        algo.add_solution(Circ2d::new(0.0, 0.0, 1.0), GccEntPosition::Enclosed, GccEntPosition::Enclosing, Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), false, false);

        let quals = algo.which_qualifier(1);
        assert!(quals.is_some());
        let (q1, q2) = quals.unwrap();
        assert_eq!(q1, GccEntPosition::Enclosed);
        assert_eq!(q2, GccEntPosition::Enclosing);
    }

    #[test]
    fn test_is_same_flags() {
        let mut algo = Geom2dGccCirc2d2TanOn::new();
        algo.add_solution(Circ2d::new(0.0, 0.0, 1.0), GccEntPosition::Outside, GccEntPosition::Outside, Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 0.0), true, false);

        assert_eq!(algo.is_the_same1(1), Some(true));
        assert_eq!(algo.is_the_same2(1), Some(false));
    }
}
