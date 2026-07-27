// FILE: rust/ironstream/crates/ironstream/src/gccent.rs
//! `GccEnt` — constraint qualification types for geometric construction
//! algorithms, mirroring OpenCascade's `GccEnt` package
//! (`src/ModelingAlgorithms/TKGeomBase/GccEnt`).
//!
//! These types describe *how* a circle or line participates in a tangency
//! constraint (enclosing, enclosed, outside, etc.) when building tangent
//! circles or lines via `Gcc2d` algorithms.
//!
//! # Covered types
//! - [`GccentQualifier`]      — `GccEnt_Position`
//! - [`GccentQualifiedCirc`]  — `GccEnt_QualifiedCirc`
//! - [`GccentQualifiedLin`]   — `GccEnt_QualifiedLin`

// occt-ref: GccEnt_Position
/// Describes the qualifier of a circle or line in a tangency constraint.
///
/// Matches `GccEnt_Position` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GccentQualifier {
    /// No qualifier — any solution is accepted.
    Unqualified,
    /// The solution circle encloses the argument.
    Enclosing,
    /// The solution circle is enclosed by the argument.
    Enclosed,
    /// The solution is outside the argument.
    Outside,
    /// The solution is on the curve of the argument.
    OnCurve,
}

// occt: GccEnt_QualifiedCirc
/// A circle together with a qualifier that constrains how a tangent solution
/// relates to it.
///
/// A circle is represented by its center in 3-D space and a radius.  (The
/// 2-D `Gcc2d` algorithms treat only the XY components of the center; the
/// full `[f64; 3]` is kept for consistency with the rest of IronStream's
/// coordinate conventions.)
///
/// Matches `GccEnt_QualifiedCirc` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GccentQualifiedCirc {
    center: [f64; 3],
    radius: f64,
    qualifier: GccentQualifier,
}

impl GccentQualifiedCirc {
    /// Create a qualified circle with an explicit qualifier.
    pub fn new(center: [f64; 3], radius: f64, qualifier: GccentQualifier) -> Self {
        Self { center, radius, qualifier }
    }

    /// Return the center of the circle.
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// Return the radius of the circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Return the qualifier.
    pub fn qualifier(&self) -> GccentQualifier {
        self.qualifier
    }

    /// Construct with [`GccentQualifier::Unqualified`] — any solution.
    pub fn unqualified(center: [f64; 3], radius: f64) -> Self {
        Self::new(center, radius, GccentQualifier::Unqualified)
    }

    /// Construct with [`GccentQualifier::Enclosing`] — solution encloses this circle.
    pub fn enclosing(center: [f64; 3], radius: f64) -> Self {
        Self::new(center, radius, GccentQualifier::Enclosing)
    }

    /// Construct with [`GccentQualifier::Enclosed`] — solution is enclosed by this circle.
    pub fn enclosed(center: [f64; 3], radius: f64) -> Self {
        Self::new(center, radius, GccentQualifier::Enclosed)
    }

    /// Construct with [`GccentQualifier::Outside`] — solution is outside this circle.
    pub fn outside(center: [f64; 3], radius: f64) -> Self {
        Self::new(center, radius, GccentQualifier::Outside)
    }
}

// occt: GccEnt_QualifiedLin
/// A line together with a qualifier that constrains how a tangent solution
/// relates to it.
///
/// A line is represented by an origin point and a direction vector in 3-D
/// space.  The direction need not be normalised for storage, but algorithms
/// consuming this type may require it to be a unit vector.
///
/// Matches `GccEnt_QualifiedLin` in OpenCascade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GccentQualifiedLin {
    origin: [f64; 3],
    direction: [f64; 3],
    qualifier: GccentQualifier,
}

impl GccentQualifiedLin {
    /// Create a qualified line with an explicit qualifier.
    pub fn new(origin: [f64; 3], direction: [f64; 3], qualifier: GccentQualifier) -> Self {
        Self { origin, direction, qualifier }
    }

    /// Return the origin of the line.
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    /// Return the direction of the line.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Return the qualifier.
    pub fn qualifier(&self) -> GccentQualifier {
        self.qualifier
    }
}
