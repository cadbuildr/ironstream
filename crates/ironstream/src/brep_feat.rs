// FILE: brep_feat.rs
//! `BRepFeat` — feature-based construction operations, mirroring OpenCascade's
//! `BRepFeat` package.
//!
//! # Overview
//!
//! * [`BRepFeat_MakePrism`] — adds or removes a prismatic (linear-extrusion)
//!   feature from a base solid. Mirrors `BRepFeat_MakePrism`.
//!
//! * [`BRepFeat_MakeRevol`] — adds or removes a revolutionary (rotation-sweep)
//!   feature from a base solid. Mirrors `BRepFeat_MakeRevol`.
//!
//! * [`BRepFeat_MakePipe`] — adds or removes a pipe (sweep-along-spine) feature
//!   from a base solid. Mirrors `BRepFeat_MakePipe`.
//!
//! # Feature operations
//!
//! Each builder accepts a *base solid*, a *tool shape* (constructed internally
//! from a profile and parameters), and a [`FeatMode`] that selects whether to
//! fuse (additive) or cut (subtractive) the tool from the base.  The result is
//! obtained by calling `build()` / `into_solid()`.
//!
//! # Algorithm
//!
//! The prism, revol and pipe tools are generated using the same tessellating
//! sweepers from `brep_prim_api` and `brep_offset_api`.  The boolean fusion or
//! cut is performed by `brep_algo_api::{fuse, cut}`.

use crate::brep_algo_api::{cut, fuse};
use crate::brep_prim_api::{make_prism as prim_prism, make_revol as prim_revol, MeshParams};
use crate::brep_offset_api::MakePipe as OffsetPipe;
use crate::gp::{Ax1, Pnt};
use crate::topods::{Face, Solid, Wire};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// FeatMode
// ─────────────────────────────────────────────────────────────────────────────

/// Whether a feature is *added* to (fused with) the base, or *removed* from
/// (cut from) the base.
///
/// Mirrors `BRepFeat_ModeType` / the sign convention in OCCT's
/// `BRepFeat_MakePrism::Perform`.
// occt-ref: BRepFeat_ModeType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatMode {
    /// Additive: tool is fused with the base (boss, pad, …).
    Add,
    /// Subtractive: tool is cut from the base (pocket, hole, slot, …).
    Remove,
}

impl Default for FeatMode {
    fn default() -> Self {
        FeatMode::Add
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakePrism
// ─────────────────────────────────────────────────────────────────────────────

/// Feature builder that sweeps a planar profile linearly (prismatic extrusion)
/// and fuses or subtracts the result from a base solid.
///
/// Typical usage:
/// ```no_run
/// # use ironstream::brep_feat::{BRepFeat_MakePrism, FeatMode};
/// # use ironstream::gp::Pnt;
/// # use ironstream::topods::{Face, Solid, Wire};
/// # let base = Solid::empty();
/// # let profile = Face::new(Wire::new(vec![Pnt::origin()]));
/// let mut op = BRepFeat_MakePrism::new(base, profile, Pnt::new(0.0, 0.0, 5.0), FeatMode::Add);
/// let result = op.build();
/// ```
// occt: BRepFeat_MakePrism
#[derive(Debug, Clone)]
pub struct BRepFeat_MakePrism {
    base: Solid,
    profile: Face,
    direction: Pnt,
    mode: FeatMode,
    mesh_params: MeshParams,
    result: Option<Solid>,
}

impl BRepFeat_MakePrism {
    /// Construct a prism feature.
    ///
    /// * `base`      — the solid that is modified.
    /// * `profile`   — the planar sketch (cross-section) to extrude.
    /// * `direction` — extrusion vector (length encodes depth).
    /// * `mode`      — `Add` to boss/pad or `Remove` to pocket.
    // occt: BRepFeat_MakePrism // ::BRepFeat_MakePrism
    pub fn new(base: Solid, profile: Face, direction: Pnt, mode: FeatMode) -> Self {
        Self {
            base,
            profile,
            direction,
            mode,
            mesh_params: MeshParams::default(),
            result: None,
        }
    }

    /// Override tessellation parameters (e.g. for smoother swept solids).
    // occt: BRepFeat_MakePrism // (no direct OCCT equivalent; resolution control)
    pub fn with_mesh_params(mut self, mp: MeshParams) -> Self {
        self.mesh_params = mp;
        self.result = None;
        self
    }

    /// Build and cache the result.
    ///
    /// Mirrors `BRepFeat_MakePrism::Perform()` + `Shape()`.
    // occt: BRepFeat_MakePrism // ::Shape
    pub fn build(&mut self) -> &Solid {
        if self.result.is_none() {
            self.result = Some(self.compute());
        }
        self.result.as_ref().unwrap()
    }

    /// `true` iff the last build produced a non-empty result.
    // occt: BRepFeat_MakePrism // ::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }

    /// Consume and return the built solid.
    // occt: BRepFeat_MakePrism // ::Shape (moved)
    pub fn into_solid(mut self) -> Solid {
        self.compute()
    }

    /// Return the extrusion direction vector.
    pub fn direction(&self) -> Pnt {
        self.direction
    }

    /// Return the feature mode (Add / Remove).
    pub fn mode(&self) -> FeatMode {
        self.mode
    }

    fn compute(&self) -> Solid {
        if self.direction.norm() < 1e-12 {
            return self.base.clone();
        }
        let tool = prim_prism(&self.profile, self.direction);
        if tool.is_empty() {
            return self.base.clone();
        }
        match self.mode {
            FeatMode::Add => fuse(&self.base, &tool),
            FeatMode::Remove => cut(&self.base, &tool),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakeRevol
// ─────────────────────────────────────────────────────────────────────────────

/// Feature builder that sweeps a closed wire about an axis (revolutionary
/// extrusion) and fuses or subtracts the result from a base solid.
// occt: BRepFeat_MakeRevol
#[derive(Debug, Clone)]
pub struct BRepFeat_MakeRevol {
    base: Solid,
    profile: Wire,
    axis: Ax1,
    angle: f64,
    mode: FeatMode,
    mesh_params: MeshParams,
    result: Option<Solid>,
}

impl BRepFeat_MakeRevol {
    /// Construct a revol feature.
    ///
    /// * `base`    — the solid that is modified.
    /// * `profile` — closed wire in the half-plane to revolve.
    /// * `axis`    — revolution axis (`Ax1`: origin + direction).
    /// * `angle`   — sweep angle in radians (`2*PI` for full revolution).
    /// * `mode`    — `Add` or `Remove`.
    // occt: BRepFeat_MakeRevol // ::BRepFeat_MakeRevol
    pub fn new(base: Solid, profile: Wire, axis: Ax1, angle: f64, mode: FeatMode) -> Self {
        Self {
            base,
            profile,
            axis,
            angle: angle.clamp(-2.0 * PI, 2.0 * PI),
            mode,
            mesh_params: MeshParams::default(),
            result: None,
        }
    }

    /// Override tessellation resolution.
    pub fn with_mesh_params(mut self, mp: MeshParams) -> Self {
        self.mesh_params = mp;
        self.result = None;
        self
    }

    /// Build and cache the result.
    // occt: BRepFeat_MakeRevol // ::Shape
    pub fn build(&mut self) -> &Solid {
        if self.result.is_none() {
            self.result = Some(self.compute());
        }
        self.result.as_ref().unwrap()
    }

    /// `true` iff the last build produced a non-empty result.
    // occt: BRepFeat_MakeRevol // ::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }

    /// Consume and return the built solid.
    pub fn into_solid(mut self) -> Solid {
        self.compute()
    }

    /// Return the revolution axis.
    pub fn axis(&self) -> Ax1 {
        self.axis
    }

    /// Return the sweep angle in radians.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    fn compute(&self) -> Solid {
        if self.angle.abs() < 1e-12 {
            return self.base.clone();
        }
        let tool = prim_revol(&self.profile, self.axis, self.angle, self.mesh_params);
        if tool.is_empty() {
            return self.base.clone();
        }
        match self.mode {
            FeatMode::Add => fuse(&self.base, &tool),
            FeatMode::Remove => cut(&self.base, &tool),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BRepFeat_MakePipe
// ─────────────────────────────────────────────────────────────────────────────

/// Feature builder that sweeps a circular cross-section along a polyline spine
/// and fuses or subtracts the resulting pipe from a base solid.
// occt: BRepFeat_MakePipe
#[derive(Debug, Clone)]
pub struct BRepFeat_MakePipe {
    base: Solid,
    spine: Wire,
    radius: f64,
    mode: FeatMode,
    facets: usize,
    profile: Option<Wire>,
    result: Option<Solid>,
}

impl BRepFeat_MakePipe {
    /// Construct a pipe feature with a circular cross-section.
    ///
    /// * `base`   — the solid that is modified.
    /// * `spine`  — the polyline path to sweep along.
    /// * `radius` — cross-section radius.
    /// * `mode`   — `Add` or `Remove`.
    // occt: BRepFeat_MakePipe // ::BRepFeat_MakePipe
    pub fn new(base: Solid, spine: Wire, radius: f64, mode: FeatMode) -> Self {
        Self {
            base,
            spine,
            radius: radius.abs(),
            mode,
            facets: 32,
            profile: None,
            result: None,
        }
    }

    /// Override the number of circular facets (resolution).
    pub fn with_facets(mut self, facets: usize) -> Self {
        self.facets = facets.max(3);
        self.result = None;
        self
    }

    /// Supply an explicit closed profile wire (in local XY-plane of the first
    /// spine frame, Z=0 for all points).  Overrides `radius`.
    // occt: BRepFeat_MakePipe // (profile override)
    pub fn with_profile(mut self, profile: Wire) -> Self {
        self.profile = Some(profile);
        self.result = None;
        self
    }

    /// Build and cache the result.
    // occt: BRepFeat_MakePipe // ::Shape
    pub fn build(&mut self) -> &Solid {
        if self.result.is_none() {
            self.result = Some(self.compute());
        }
        self.result.as_ref().unwrap()
    }

    /// `true` iff the last build produced a non-empty result.
    // occt: BRepFeat_MakePipe // ::IsDone
    pub fn is_done(&mut self) -> bool {
        !self.build().is_empty()
    }

    /// Consume and return the built solid.
    pub fn into_solid(mut self) -> Solid {
        self.compute()
    }

    /// Return a reference to the spine wire.
    pub fn spine(&self) -> &Wire {
        &self.spine
    }

    /// Return the cross-section radius (0 if using explicit profile).
    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn compute(&self) -> Solid {
        if self.spine.pts.len() < 2 {
            return self.base.clone();
        }
        let mut pipe_builder = OffsetPipe::new(self.spine.clone(), self.radius)
            .with_facets(self.facets);
        if let Some(ref prof) = self.profile {
            pipe_builder = pipe_builder.with_profile(prof.clone());
        }
        let tool = pipe_builder.into_solid();
        if tool.is_empty() {
            return self.base.clone();
        }
        match self.mode {
            FeatMode::Add => fuse(&self.base, &tool),
            FeatMode::Remove => cut(&self.base, &tool),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_prim_api::make_box;
    use crate::gp::Ax3;
    use std::f64::consts::PI;

    // ── helpers ───────────────────────────────────────────────────────────────

    fn unit_box() -> Solid {
        make_box(Pnt::origin(), 4.0, 4.0, 4.0)
    }

    /// Square profile in the XY plane, centered at origin.
    fn square_profile(side: f64) -> Face {
        let h = side / 2.0;
        Face::new(Wire::new(vec![
            Pnt::new(-h, -h, 0.0),
            Pnt::new(h, -h, 0.0),
            Pnt::new(h, h, 0.0),
            Pnt::new(-h, h, 0.0),
        ]))
    }

    // ── FeatMode ──────────────────────────────────────────────────────────────

    #[test]
    fn feat_mode_default_is_add() {
        assert_eq!(FeatMode::default(), FeatMode::Add);
    }

    #[test]
    fn feat_mode_is_copy() {
        let m = FeatMode::Remove;
        let m2 = m; // Copy
        assert_eq!(m2, FeatMode::Remove);
    }

    // ── BRepFeat_MakePrism ────────────────────────────────────────────────────

    #[test]
    fn prism_add_increases_volume() {
        let base = unit_box();
        let base_vol = base.volume();
        // Boss on top of the box (starts at z=4, extrudes +2 in Z)
        let boss_profile = Face::new(Wire::new(vec![
            Pnt::new(1.0, 1.0, 4.0),
            Pnt::new(3.0, 1.0, 4.0),
            Pnt::new(3.0, 3.0, 4.0),
            Pnt::new(1.0, 3.0, 4.0),
        ]));
        let mut op = BRepFeat_MakePrism::new(
            base,
            boss_profile,
            Pnt::new(0.0, 0.0, 2.0),
            FeatMode::Add,
        );
        let result = op.build();
        assert!(result.volume() > base_vol, "boss should increase volume");
        assert!(op.is_done(), "build should succeed");
    }

    #[test]
    fn prism_remove_decreases_volume() {
        let base = unit_box();
        let base_vol = base.volume();
        // Pocket centered on top face, depth 2.
        let pocket_profile = Face::new(Wire::new(vec![
            Pnt::new(1.0, 1.0, 4.0),
            Pnt::new(3.0, 1.0, 4.0),
            Pnt::new(3.0, 3.0, 4.0),
            Pnt::new(1.0, 3.0, 4.0),
        ]));
        let result = BRepFeat_MakePrism::new(
            base,
            pocket_profile,
            Pnt::new(0.0, 0.0, -2.0),
            FeatMode::Remove,
        )
        .into_solid();
        assert!(result.volume() < base_vol, "pocket should decrease volume");
    }

    #[test]
    fn prism_zero_direction_returns_base() {
        let base = unit_box();
        let base_vol = base.volume();
        let mut op = BRepFeat_MakePrism::new(
            base,
            square_profile(1.0),
            Pnt::origin(), // zero direction
            FeatMode::Add,
        );
        let result = op.build();
        assert!((result.volume() - base_vol).abs() < 1e-6);
    }

    #[test]
    fn prism_direction_accessor() {
        let op = BRepFeat_MakePrism::new(
            unit_box(),
            square_profile(1.0),
            Pnt::new(0.0, 0.0, 3.0),
            FeatMode::Remove,
        );
        assert!((op.direction().z - 3.0).abs() < 1e-12);
        assert_eq!(op.mode(), FeatMode::Remove);
    }

    // ── BRepFeat_MakeRevol ────────────────────────────────────────────────────

    #[test]
    fn revol_full_turn_add_makes_cylinder_boss() {
        // Base: 10x10x1 slab at z=0..1.
        let base = make_box(Pnt::origin(), 10.0, 10.0, 1.0);
        let base_vol = base.volume();

        // Profile: rectangle from (3,0,1) to (4,0,1) to (4,0,3) to (3,0,3)
        // — when revolved about Z this makes an annular tube attached to top of slab.
        let profile = Wire::new(vec![
            Pnt::new(3.0, 0.0, 1.0),
            Pnt::new(4.0, 0.0, 1.0),
            Pnt::new(4.0, 0.0, 3.0),
            Pnt::new(3.0, 0.0, 3.0),
        ]);
        let mut op = BRepFeat_MakeRevol::new(
            base,
            profile,
            Ax1::oz(),
            2.0 * PI,
            FeatMode::Add,
        );
        let result = op.build();
        assert!(result.volume() > base_vol, "annular boss should increase volume");
        assert!(op.is_done());
    }

    #[test]
    fn revol_angle_accessor() {
        let op = BRepFeat_MakeRevol::new(
            unit_box(),
            Wire::new(vec![
                Pnt::new(1.0, 0.0, 0.0),
                Pnt::new(2.0, 0.0, 0.0),
                Pnt::new(2.0, 0.0, 1.0),
                Pnt::new(1.0, 0.0, 1.0),
            ]),
            Ax1::oz(),
            PI,
            FeatMode::Add,
        );
        assert!((op.angle() - PI).abs() < 1e-12);
        assert_eq!(op.axis().location.x, 0.0);
        assert_eq!(op.axis().location.y, 0.0);
        assert_eq!(op.axis().location.z, 0.0);
    }

    #[test]
    fn revol_zero_angle_returns_base() {
        let base = unit_box();
        let base_vol = base.volume();
        let profile = Wire::new(vec![
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 1.0),
        ]);
        let result = BRepFeat_MakeRevol::new(
            base,
            profile,
            Ax1::oz(),
            0.0,
            FeatMode::Remove,
        )
        .into_solid();
        assert!((result.volume() - base_vol).abs() < 1e-6);
    }

    // ── BRepFeat_MakePipe ─────────────────────────────────────────────────────

    #[test]
    fn pipe_add_increases_volume() {
        let base = make_box(Pnt::origin(), 10.0, 10.0, 1.0);
        let base_vol = base.volume();

        // L-shaped spine on top of slab (z=1).
        let spine = Wire::new(vec![
            Pnt::new(2.0, 2.0, 1.0),
            Pnt::new(8.0, 2.0, 1.0),
            Pnt::new(8.0, 8.0, 1.0),
        ]);
        let mut op = BRepFeat_MakePipe::new(base, spine, 0.5, FeatMode::Add);
        let result = op.build();
        assert!(result.volume() > base_vol, "pipe boss should increase volume");
        assert!(op.is_done());
    }

    #[test]
    fn pipe_remove_decreases_volume() {
        let base = make_box(Pnt::origin(), 10.0, 10.0, 10.0);
        let base_vol = base.volume();

        // Straight spine drills through the box along X.
        let spine = Wire::new(vec![
            Pnt::new(-1.0, 5.0, 5.0),
            Pnt::new(11.0, 5.0, 5.0),
        ]);
        let result = BRepFeat_MakePipe::new(base, spine, 1.0, FeatMode::Remove)
            .with_facets(16)
            .into_solid();
        assert!(result.volume() < base_vol, "pipe channel should decrease volume");
    }

    #[test]
    fn pipe_radius_accessor() {
        let spine = Wire::new(vec![Pnt::origin(), Pnt::new(5.0, 0.0, 0.0)]);
        let op = BRepFeat_MakePipe::new(unit_box(), spine, 2.5, FeatMode::Add);
        assert!((op.radius() - 2.5).abs() < 1e-12);
        assert_eq!(op.spine().pts.len(), 2);
    }

    #[test]
    fn pipe_short_spine_returns_base() {
        let base = unit_box();
        let base_vol = base.volume();
        // Single-point spine is invalid; should return base unchanged.
        let spine = Wire::new(vec![Pnt::origin()]);
        let result = BRepFeat_MakePipe::new(base, spine, 1.0, FeatMode::Add)
            .into_solid();
        assert!((result.volume() - base_vol).abs() < 1e-6);
    }
}
