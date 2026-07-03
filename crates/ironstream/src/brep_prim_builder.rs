// FILE: rust/ironstream/crates/ironstream/src/brep_prim_builder.rs
//! Builder-pattern stubs for linear extrusions, revolutions, and sweeps,
//! mirroring OCCT's `BRepPrimAPI_MakePrism`, `BRepPrimAPI_MakeRevol`, and
//! `BRepBuilderAPI_MakeSweep`, together with convenience free functions for
//! common primitives.  All types are zero-dependency pure-Rust.

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// PrismBuilder
// ---------------------------------------------------------------------------

/// Builder that extrudes a planar base profile linearly along a direction
/// vector, producing a prismatic solid.
///
// occt: BRepPrimAPI_MakePrism
#[derive(Debug, Clone)]
pub struct PrismBuilder {
    /// Name / identifier for the base profile.
    pub base: String,
    /// Extrusion direction vector (need not be unit-length).
    pub direction: [f64; 3],
    /// Scalar extrusion height (magnitude along `direction`).
    pub height: f64,
}

impl PrismBuilder {
    /// Construct a new `PrismBuilder` from a base profile name, a direction
    /// vector, and a height.
    ///
    // occt: BRepPrimAPI_MakePrism::BRepPrimAPI_MakePrism
    pub fn new(base: &str, dir: [f64; 3], h: f64) -> Self {
        Self {
            base: base.to_owned(),
            direction: dir,
            height: h,
        }
    }

    /// Build the prism and return a human-readable descriptor of the result.
    ///
    /// In a full implementation this would return a `TopoDS_Shape` equivalent;
    /// here it returns a `String` tag suitable for downstream stub processing.
    ///
    // occt: BRepPrimAPI_MakePrism::Shape
    pub fn build(&self) -> String {
        format!(
            "Prism(base={}, dir=[{:.6},{:.6},{:.6}], h={:.6})",
            self.base, self.direction[0], self.direction[1], self.direction[2], self.height,
        )
    }

    /// Approximate the volume of the prism.
    ///
    /// The formula assumes the base area is encoded in the height parameter
    /// for stub purposes (area = 1 by convention when the base is a name-only
    /// reference).  A real implementation would integrate the face area.
    ///
    // occt: BRepPrimAPI_MakePrism volume utility
    pub fn volume(&self) -> f64 {
        let dir_len = (self.direction[0].powi(2)
            + self.direction[1].powi(2)
            + self.direction[2].powi(2))
        .sqrt();
        // Stub: treat the base as a unit-area polygon; volume = 1 * h * |dir|.
        self.height * dir_len
    }
}

// ---------------------------------------------------------------------------
// RevolBuilder
// ---------------------------------------------------------------------------

/// Builder that revolves a planar base profile about an axis through a given
/// angle, producing a solid of revolution.
///
// occt: BRepPrimAPI_MakeRevol
#[derive(Debug, Clone)]
pub struct RevolBuilder {
    /// Name / identifier for the base profile.
    pub base: String,
    /// Revolution axis direction vector (need not be unit-length).
    pub axis: [f64; 3],
    /// Revolution angle in radians.
    pub angle: f64,
}

impl RevolBuilder {
    /// Construct a new `RevolBuilder` from a base profile name, an axis
    /// direction vector, and a revolution angle in radians.
    ///
    // occt: BRepPrimAPI_MakeRevol::BRepPrimAPI_MakeRevol
    pub fn new(base: &str, axis: [f64; 3], angle: f64) -> Self {
        Self {
            base: base.to_owned(),
            axis,
            angle,
        }
    }

    /// Build the revolved solid and return a human-readable descriptor.
    ///
    /// A full implementation would return a `TopoDS_Shape` equivalent.
    ///
    // occt: BRepPrimAPI_MakeRevol::Shape
    pub fn build(&self) -> String {
        let full = (self.angle - 2.0 * PI).abs() < 1e-6;
        format!(
            "Revol(base={}, axis=[{:.6},{:.6},{:.6}], angle={:.6}{})",
            self.base,
            self.axis[0],
            self.axis[1],
            self.axis[2],
            self.angle,
            if full { ", full" } else { "" },
        )
    }
}

// ---------------------------------------------------------------------------
// SweepBuilder
// ---------------------------------------------------------------------------

/// Builder that sweeps a planar base profile along a spine curve, producing a
/// swept solid.
///
/// Mirrors `BRepOffsetAPI_MakePipe` / `BRepBuilderAPI_MakeSweep` from OCCT.
// occt: BRepPrimAPI_MakePrism (generalized sweep variant)
#[derive(Debug, Clone)]
pub struct SweepBuilder {
    /// Name / identifier for the base profile.
    pub base: String,
    /// Name / identifier for the spine (path) curve.
    pub spine: String,
}

impl SweepBuilder {
    /// Construct a new `SweepBuilder` from a base profile name and a spine
    /// curve name.
    ///
    // occt: BRepOffsetAPI_MakePipe::BRepOffsetAPI_MakePipe
    pub fn new(base: &str, spine: &str) -> Self {
        Self {
            base: base.to_owned(),
            spine: spine.to_owned(),
        }
    }

    /// Build the swept solid and return a human-readable descriptor.
    ///
    // occt: BRepOffsetAPI_MakePipe::Shape
    pub fn build(&self) -> String {
        format!("Sweep(base={}, spine={})", self.base, self.spine)
    }
}

// ---------------------------------------------------------------------------
// Primitive free functions
// ---------------------------------------------------------------------------

/// Build an axis-aligned box with extents `(dx, dy, dz)` starting from the
/// origin and return a human-readable descriptor.
///
// occt: BRepPrimAPI_MakeBox
pub fn make_box(dx: f64, dy: f64, dz: f64) -> String {
    format!("Box(dx={:.6}, dy={:.6}, dz={:.6})", dx, dy, dz)
}

/// Build a right circular cylinder with radius `r` and height `h`, axis +Z
/// from the origin, and return a human-readable descriptor.
///
// occt: BRepPrimAPI_MakeCylinder
pub fn make_cylinder(r: f64, h: f64) -> String {
    format!("Cylinder(r={:.6}, h={:.6})", r, h)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prism_builder_round_trip() {
        let pb = PrismBuilder::new("square_profile", [0.0, 0.0, 1.0], 5.0);
        assert_eq!(pb.base, "square_profile");
        assert_eq!(pb.height, 5.0);
        let desc = pb.build();
        assert!(desc.contains("Prism(base=square_profile"));
        assert!(desc.contains("h=5.000000"));
    }

    #[test]
    fn prism_volume_unit_dir() {
        let pb = PrismBuilder::new("p", [0.0, 0.0, 1.0], 3.0);
        // |dir| = 1, height = 3  =>  volume stub = 3.0
        assert!((pb.volume() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn prism_volume_scaled_dir() {
        let pb = PrismBuilder::new("p", [0.0, 0.0, 2.0], 4.0);
        // |dir| = 2, height = 4  =>  volume stub = 8.0
        assert!((pb.volume() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn revol_builder_full_revolution() {
        let rb = RevolBuilder::new("circle_profile", [0.0, 0.0, 1.0], 2.0 * PI);
        let desc = rb.build();
        assert!(desc.contains("Revol(base=circle_profile"));
        assert!(desc.contains("full"));
    }

    #[test]
    fn revol_builder_partial_revolution() {
        let rb = RevolBuilder::new("profile", [0.0, 1.0, 0.0], PI / 2.0);
        let desc = rb.build();
        assert!(desc.contains("Revol(base=profile"));
        assert!(!desc.contains("full"));
    }

    #[test]
    fn sweep_builder_round_trip() {
        let sb = SweepBuilder::new("cross_section", "helix_path");
        assert_eq!(sb.base, "cross_section");
        assert_eq!(sb.spine, "helix_path");
        let desc = sb.build();
        assert_eq!(desc, "Sweep(base=cross_section, spine=helix_path)");
    }

    #[test]
    fn make_box_descriptor() {
        let s = make_box(2.0, 3.0, 4.0);
        assert_eq!(s, "Box(dx=2.000000, dy=3.000000, dz=4.000000)");
    }

    #[test]
    fn make_cylinder_descriptor() {
        let s = make_cylinder(1.5, 10.0);
        assert_eq!(s, "Cylinder(r=1.500000, h=10.000000)");
    }
}
