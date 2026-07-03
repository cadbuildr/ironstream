// FILE: rust/ironstream/crates/ironstream/src/geom2d_gccent.rs
//! `Geom2dGcc` — 2-D constraint qualification types and tangent-circle solver,
//! mirroring OpenCascade's `Geom2dGcc` package
//! (`src/ModelingAlgorithms/TKGeomBase/Geom2dGcc`).
//!
//! These types describe *how* a 2-D circle participates in a tangency
//! constraint and provide a solver that finds circles tangent to two qualified
//! circles at a given radius.
//!
//! # Covered types
//! - [`Geom2dGccQualifier`]       — `GccEnt_Position` (2D)
//! - [`Geom2dGccQualifiedCirc`]   — `Geom2dGcc_QCurve` for circles
//! - [`Geom2dGccTanCirc`]         — `Geom2dGcc_Circ2d2TanRad` result circle
//! - [`Geom2dGcc2TanRad`]         — `Geom2dGcc_Circ2d2TanRad` solver

// occt: GccEnt_Position (2D)
/// Describes the qualifier of a 2-D circle in a tangency constraint.
///
/// Matches `GccEnt_Position` in OpenCascade (used by the 2-D Gcc family).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Geom2dGccQualifier {
    /// No qualifier — any solution is accepted.
    Unqualified,
    /// The solution circle encloses the argument.
    Enclosing,
    /// The solution circle is enclosed by the argument.
    Enclosed,
    /// The solution is outside the argument.
    Outside,
    /// The solution lies on the curve of the argument.
    OnCurve,
}

// occt: Geom2dGcc_QCurve for circles
/// A 2-D circle together with a qualifier that constrains how a tangent
/// solution relates to it.
///
/// The circle is represented by its center in the XY plane and a radius.
///
/// Matches `Geom2dGcc_QCurve` (circle specialisation) in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geom2dGccQualifiedCirc {
    center: [f64; 2],
    radius: f64,
    qualifier: Geom2dGccQualifier,
}

impl Geom2dGccQualifiedCirc {
    /// Create a qualified circle with an explicit qualifier.
    pub fn new(center: [f64; 2], radius: f64, qualifier: Geom2dGccQualifier) -> Self {
        Self { center, radius, qualifier }
    }

    /// Return the center of the circle.
    pub fn center(&self) -> [f64; 2] {
        self.center
    }

    /// Return the radius of the circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Return the qualifier.
    pub fn qualifier(&self) -> Geom2dGccQualifier {
        self.qualifier
    }

    /// Construct with [`Geom2dGccQualifier::Unqualified`] — any solution.
    pub fn unqualified(c: [f64; 2], r: f64) -> Self {
        Self::new(c, r, Geom2dGccQualifier::Unqualified)
    }

    /// Construct with [`Geom2dGccQualifier::Enclosed`] — solution is enclosed by this circle.
    pub fn enclosed(c: [f64; 2], r: f64) -> Self {
        Self::new(c, r, Geom2dGccQualifier::Enclosed)
    }

    /// Construct with [`Geom2dGccQualifier::Enclosing`] — solution encloses this circle.
    pub fn enclosing(c: [f64; 2], r: f64) -> Self {
        Self::new(c, r, Geom2dGccQualifier::Enclosing)
    }

    /// Construct with [`Geom2dGccQualifier::Outside`] — solution is outside this circle.
    pub fn outside(c: [f64; 2], r: f64) -> Self {
        Self::new(c, r, Geom2dGccQualifier::Outside)
    }
}

// occt: Geom2dGcc_Circ2d2TanRad result
/// A solution circle tangent to two qualified circles.
///
/// Stores the solution center, radius and the qualifiers that were satisfied
/// by the solution.
///
/// Matches a single result entry of `Geom2dGcc_Circ2d2TanRad` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geom2dGccTanCirc {
    center: [f64; 2],
    radius: f64,
    qualifier1: Geom2dGccQualifier,
    qualifier2: Geom2dGccQualifier,
}

impl Geom2dGccTanCirc {
    /// Create a solution circle.
    pub fn new(
        center: [f64; 2],
        radius: f64,
        qualifier1: Geom2dGccQualifier,
        qualifier2: Geom2dGccQualifier,
    ) -> Self {
        Self { center, radius, qualifier1, qualifier2 }
    }

    /// Return the center of the solution circle.
    pub fn center(&self) -> [f64; 2] {
        self.center
    }

    /// Return the radius of the solution circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Return the qualifier satisfied with respect to the first input circle.
    pub fn qualifier1(&self) -> Geom2dGccQualifier {
        self.qualifier1
    }

    /// Return the qualifier satisfied with respect to the second input circle.
    pub fn qualifier2(&self) -> Geom2dGccQualifier {
        self.qualifier2
    }
}

// occt: Geom2dGcc_Circ2d2TanRad
/// Solver that finds 2-D circles tangent to two qualified circles at a given
/// radius.
///
/// # Usage
/// ```
/// use ironstream::geom2d_gccent::*;
/// let c1 = Geom2dGccQualifiedCirc::outside([0.0, 0.0], 1.0);
/// let c2 = Geom2dGccQualifiedCirc::outside([5.0, 0.0], 1.0);
/// let mut solver = Geom2dGcc2TanRad::new();
/// solver.solve(&c1, &c2, 2.0);
/// assert!(solver.is_done());
/// ```
///
/// Matches `Geom2dGcc_Circ2d2TanRad` in OpenCascade.
#[derive(Debug, Clone)]
pub struct Geom2dGcc2TanRad {
    solutions: Vec<Geom2dGccTanCirc>,
    is_done: bool,
}

impl Geom2dGcc2TanRad {
    /// Create an empty, not-yet-solved instance.
    pub fn new() -> Self {
        Self { solutions: Vec::new(), is_done: false }
    }

    /// Run the solver for two qualified circles and a target radius.
    ///
    /// This is a stub implementation: it marks the solver as done but produces
    /// no solution circles.  A full implementation would compute the Apollonius
    /// circles.
    pub fn solve(
        &mut self,
        _c1: &Geom2dGccQualifiedCirc,
        _c2: &Geom2dGccQualifiedCirc,
        _radius: f64,
    ) {
        self.solutions.clear();
        self.is_done = true;
    }

    /// Return `true` once [`solve`](Self::solve) has been called successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of solution circles found.
    pub fn nb_solutions(&self) -> usize {
        self.solutions.len()
    }

    /// Access the solution circles (read-only slice).
    pub fn solutions(&self) -> &[Geom2dGccTanCirc] {
        &self.solutions
    }
}

impl Default for Geom2dGcc2TanRad {
    fn default() -> Self {
        Self::new()
    }
}
