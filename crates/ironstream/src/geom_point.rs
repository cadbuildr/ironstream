//! `Geom_CartesianPoint` -- a 3D point described by Cartesian coordinates,
//! mirroring OpenCascade's `Geom_CartesianPoint`
//! (`src/ModelingData/TKG3d/Geom/Geom_CartesianPoint.hxx`).
//!
//! A Cartesian point is the simplest geometry object: it wraps a [`gp::Pnt`]
//! and adds the `Geom`-layer identity (mutable, copyable, transformable).
//! The class is derived (conceptually) from `Geom_Point`.
//!
//! Builds on the existing `gp` API (zero third-party deps, only `std`).

use crate::gp::{Pnt, Trsf};

/// `Geom_CartesianPoint` -- a 3D point with Cartesian coordinates.
///
/// Wraps a [`Pnt`] (`gp_Pnt`) and exposes the full `Geom_CartesianPoint` API:
/// coordinate access, mutation, distance, and transformation.
// occt-ref: Geom_CartesianPoint
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomCartesianPoint {
    /// The underlying `gp_Pnt`.
    pnt: Pnt,
}

impl GeomCartesianPoint {
    // ----------------------------- constructors ----------------------------

    /// `Geom_CartesianPoint(const gp_Pnt& P)` -- construct from a `gp_Pnt`.
    #[inline]
    pub fn from_pnt(p: Pnt) -> Self {
        Self { pnt: p }
    }

    /// `Geom_CartesianPoint(const Standard_Real X, const Standard_Real Y,
    ///                      const Standard_Real Z)` -- construct from coordinates.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            pnt: Pnt::new(x, y, z),
        }
    }

    // ------------------------------- setters -------------------------------

    /// `void SetCoord(const Standard_Real X, const Standard_Real Y,
    ///                const Standard_Real Z)` -- change all three coordinates.
    #[inline]
    pub fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.pnt = Pnt::new(x, y, z);
    }

    /// `void SetPnt(const gp_Pnt& P)` -- set the point from a `gp_Pnt`.
    #[inline]
    pub fn set_pnt(&mut self, p: Pnt) {
        self.pnt = p;
    }

    /// `void SetX(const Standard_Real X)` -- change the X coordinate only.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.pnt.x = x;
    }

    /// `void SetY(const Standard_Real Y)` -- change the Y coordinate only.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.pnt.y = y;
    }

    /// `void SetZ(const Standard_Real Z)` -- change the Z coordinate only.
    #[inline]
    pub fn set_z(&mut self, z: f64) {
        self.pnt.z = z;
    }

    // ------------------------------- getters -------------------------------

    /// `gp_Pnt Pnt() const` -- the underlying `gp_Pnt`.
    #[inline]
    pub fn pnt(&self) -> Pnt {
        self.pnt
    }

    /// `Standard_Real X() const` -- the X coordinate.
    #[inline]
    pub fn x(&self) -> f64 {
        self.pnt.x
    }

    /// `Standard_Real Y() const` -- the Y coordinate.
    #[inline]
    pub fn y(&self) -> f64 {
        self.pnt.y
    }

    /// `Standard_Real Z() const` -- the Z coordinate.
    #[inline]
    pub fn z(&self) -> f64 {
        self.pnt.z
    }

    /// `void Coord(Standard_Real& X, Standard_Real& Y, Standard_Real& Z) const`
    /// -- returns all three coordinates as a tuple.
    #[inline]
    pub fn coord(&self) -> (f64, f64, f64) {
        (self.pnt.x, self.pnt.y, self.pnt.z)
    }

    // ------------------------------ distance -------------------------------

    /// `Standard_Real Distance(const Handle(Geom_Point)& Other) const`
    /// -- Euclidean distance to another point.
    #[inline]
    pub fn distance(&self, other: &GeomCartesianPoint) -> f64 {
        self.pnt.distance(other.pnt)
    }

    /// `Standard_Real SquareDistance(const Handle(Geom_Point)& Other) const`
    /// -- squared Euclidean distance to another point.
    #[inline]
    pub fn square_distance(&self, other: &GeomCartesianPoint) -> f64 {
        self.pnt.square_distance(other.pnt)
    }

    // ------------------------------ transform ------------------------------

    /// `void Transform(const gp_Trsf& T)` -- apply the transformation `T`
    /// (in-place). Mirrors `Geom_CartesianPoint::Transform`.
    #[inline]
    pub fn transform(&mut self, t: &Trsf) {
        self.pnt = t.apply_point(self.pnt);
    }

    /// Returns a transformed copy of this point.
    #[inline]
    pub fn transformed(&self, t: &Trsf) -> GeomCartesianPoint {
        let mut c = *self;
        c.transform(t);
        c
    }

    /// `Handle(Geom_Geometry) Copy() const` -- a new point equal to this one.
    #[inline]
    pub fn copy(&self) -> GeomCartesianPoint {
        *self
    }
}
