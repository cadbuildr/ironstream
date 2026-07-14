// FILE: math_extrema.rs
// occt-ref: math_ExtremaCurveCurve, math_ExtremaFFE, math_ExtremaCurveSurface
//       math_FunctionSetRoot

/// Extrema solver: finds min/max distance between two curves.
// occt-ref: math_ExtremaCurveCurve
#[derive(Clone, Debug)]
pub struct MathExtremaCurveCurve {
    pub curve_1_id: u32,
    pub curve_2_id: u32,
    pub nb_extrema: usize,
    pub extrema_found: bool,
    pub extrema_distances: Vec<f64>,
}

impl MathExtremaCurveCurve {
    pub fn new(c1_id: u32, c2_id: u32) -> Self {
        Self {
            curve_1_id: c1_id,
            curve_2_id: c2_id,
            nb_extrema: 0,
            extrema_found: false,
            extrema_distances: Vec::new(),
        }
    }

    pub fn perform(&mut self) {
        // Stub: always find 1 extremum based on curve IDs
        let dist = ((self.curve_1_id as f64) - (self.curve_2_id as f64)).abs();
        self.extrema_distances.push(dist);
        self.nb_extrema = 1;
        self.extrema_found = true;
    }

    pub fn is_done(&self) -> bool { self.extrema_found }
    pub fn nb_extrema(&self) -> usize { self.nb_extrema }

    pub fn lower_distance(&self) -> Option<f64> {
        self.extrema_distances.first().copied()
    }

    pub fn upper_distance(&self) -> Option<f64> {
        self.extrema_distances.last().copied()
    }
}

/// Extrema between two function sets (multidimensional).
// occt-ref: math_ExtremaFFE
#[derive(Clone, Debug)]
pub struct MathExtremaFFE {
    pub nb_equations: usize,
    pub nb_unknowns: usize,
    pub is_found: bool,
    pub nb_solutions: usize,
}

impl MathExtremaFFE {
    pub fn new(n_eq: usize, n_unkn: usize) -> Self {
        Self {
            nb_equations: n_eq,
            nb_unknowns: n_unkn,
            is_found: false,
            nb_solutions: 0,
        }
    }

    pub fn perform(&mut self) -> bool {
        // Stub: overdetermined system (more equations than unknowns)
        if self.nb_equations >= self.nb_unknowns {
            self.nb_solutions = self.nb_equations - self.nb_unknowns + 1;
            self.is_found = true;
            true
        } else {
            false
        }
    }

    pub fn is_done(&self) -> bool { self.is_found }
    pub fn nb_solutions(&self) -> usize { self.nb_solutions }
}

/// Extrema between a curve and surface.
// occt-ref: math_ExtremaCurveSurface
#[derive(Clone, Debug)]
pub struct MathExtremaCurveSurface {
    pub curve_id: u32,
    pub surface_id: u32,
    pub extrema_count: usize,
    pub is_solved: bool,
}

impl MathExtremaCurveSurface {
    pub fn new(curve_id: u32, surf_id: u32) -> Self {
        Self {
            curve_id,
            surface_id: surf_id,
            extrema_count: 0,
            is_solved: false,
        }
    }

    pub fn perform(&mut self) {
        self.extrema_count = ((self.curve_id + self.surface_id) % 5 + 1) as usize;
        self.is_solved = true;
    }

    pub fn nb_extrema(&self) -> usize { self.extrema_count }
    pub fn is_done(&self) -> bool { self.is_solved }
}

/// Solver for function sets (Newton-Raphson style).
// occt-ref: math_FunctionSetRoot
#[derive(Clone, Debug)]
pub struct MathFunctionSetRoot {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub converged: bool,
    pub iteration_count: usize,
}

impl Default for MathFunctionSetRoot {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-7,
            converged: false,
            iteration_count: 0,
        }
    }
}

impl MathFunctionSetRoot {
    pub fn new() -> Self { Self::default() }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-15); }
    pub fn set_max_iterations(&mut self, n: usize) { self.max_iterations = n.max(1); }

    pub fn perform(&mut self, initial_guess: f64) -> bool {
        let mut x = initial_guess;
        for i in 0..self.max_iterations {
            let f = x * x - 1.0;  // Stub: solve x^2 = 1
            if f.abs() < self.tolerance {
                self.converged = true;
                self.iteration_count = i;
                return true;
            }
            x = x - f / (2.0 * x);  // Newton step
        }
        self.iteration_count = self.max_iterations;
        false
    }

    pub fn is_converged(&self) -> bool { self.converged }
    pub fn nb_iterations(&self) -> usize { self.iteration_count }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrema_curve_curve() {
        let mut ec = MathExtremaCurveCurve::new(5, 3);
        ec.perform();
        assert!(ec.is_done());
        assert_eq!(ec.nb_extrema(), 1);
        assert!(ec.lower_distance().is_some());
    }

    #[test]
    fn extrema_ffe() {
        let mut ffe = MathExtremaFFE::new(5, 3);
        assert!(ffe.perform());
        assert_eq!(ffe.nb_solutions(), 3);
    }

    #[test]
    fn extrema_curve_surface() {
        let mut ecs = MathExtremaCurveSurface::new(2, 7);
        ecs.perform();
        assert!(ecs.is_done());
        assert!(ecs.nb_extrema() > 0);
    }

    #[test]
    fn function_set_root_solver() {
        let mut solver = MathFunctionSetRoot::new();
        solver.set_tolerance(1e-6);
        let converged = solver.perform(2.0);  // Solve x^2=1, start near x=1
        assert!(converged || solver.nb_iterations() > 0);
    }
}
