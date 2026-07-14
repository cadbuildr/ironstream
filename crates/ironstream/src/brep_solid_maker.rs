// FILE: rust/ironstream/crates/ironstream/src/brep_solid_maker.rs
//! Solid-construction stubs mirroring `BRepBuilderAPI_MakeSolid` and
//! `BRepPrimAPI_MakeBox` from OCCT.
//!
//! [`SolidMaker`] assembles a closed solid from one or more shell labels,
//! mirroring `BRepBuilderAPI_MakeSolid`.  [`BoxMaker`] is a builder-pattern
//! wrapper for constructing axis-aligned box solids, mirroring
//! `BRepPrimAPI_MakeBox`.  Two free-function helpers, [`make_box`] and
//! [`make_sphere`], provide single-call convenience à la the OCCT API.
//!
//! All shapes are represented as descriptive label strings rather than
//! real topology; no external crates are used.

// ---------------------------------------------------------------------------
// SolidMaker
// ---------------------------------------------------------------------------

// occt: BRepBuilderAPI_MakeSolid
/// Stub mirroring `BRepBuilderAPI_MakeSolid`.
///
/// Accumulates shell labels and combines them into a single solid label.
/// The solid is considered "done" once at least one shell has been added and
/// [`SolidMaker::solid`] has been called.
///
/// # Typical usage
///
/// ```
/// use ironstream::brep_solid_maker::SolidMaker;
///
/// let mut m = SolidMaker::new();
/// m.add_shell("outer_shell_1");
/// let label = m.solid();
/// assert!(label.contains("outer_shell_1"));
/// assert!(m.is_done());
/// ```
#[derive(Debug, Clone)]
pub struct SolidMaker {
    // occt: BRepBuilderAPI_MakeSolid // ::Add(const TopoDS_Shell&)
    /// Labels of the shells that have been added via [`SolidMaker::add_shell`].
    pub shells: Vec<String>,
    // occt: BRepBuilderAPI_MakeSolid // ::IsDone()
    /// Whether a solid has been successfully built (set by [`SolidMaker::solid`]).
    pub done: bool,
}

impl SolidMaker {
    // occt-note: BRepBuilderAPI_MakeSolid()
    /// Create an empty `SolidMaker`.
    ///
    /// Mirrors the default `BRepBuilderAPI_MakeSolid()` constructor.
    pub fn new() -> Self {
        Self {
            shells: Vec::new(),
            done: false,
        }
    }

    // occt: BRepBuilderAPI_MakeSolid // ::Add(const TopoDS_Shell& aShell)
    /// Add a shell label to the solid under construction.
    ///
    /// Empty labels are silently ignored.
    ///
    /// Mirrors `BRepBuilderAPI_MakeSolid::Add(const TopoDS_Shell&)`.
    pub fn add_shell(&mut self, s: &str) {
        if !s.is_empty() {
            self.shells.push(s.to_string());
        }
    }

    // occt: BRepBuilderAPI_MakeSolid // ::Solid() -> const TopoDS_Solid&
    /// Build and return the solid label.
    ///
    /// Sets [`SolidMaker::done`] to `true` when at least one shell is present.
    /// Returns an empty string when no shells have been added.
    ///
    /// Mirrors `BRepBuilderAPI_MakeSolid::Solid()`.
    pub fn solid(&mut self) -> String {
        if self.shells.is_empty() {
            self.done = false;
            return String::new();
        }
        self.done = true;
        format!("solid(shells=[{}])", self.shells.join(","))
    }

    // occt: BRepBuilderAPI_MakeSolid // ::IsDone() -> Standard_Boolean
    /// Return `true` when the solid was successfully built.
    ///
    /// Mirrors `BRepBuilderAPI_MakeSolid::IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    // occt-ref: BRepGProp // ::VolumeProperties — stub volume heuristic
    /// Return an approximate volume for the accumulated shells.
    ///
    /// This stub sums a per-shell nominal volume of 1.0 for each shell whose
    /// label does not embed an explicit `vol=` tag, or parses the tag when
    /// present.  It is intended only for unit-test assertions and is not
    /// geometrically accurate.
    pub fn volume(&self) -> f64 {
        self.shells.iter().fold(0.0, |acc, s| {
            // Accept an embedded "vol=<number>" hint in the shell label so
            // tests can inject known volumes without real geometry.
            if let Some(pos) = s.find("vol=") {
                let tail = &s[pos + 4..];
                let end = tail
                    .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                    .unwrap_or(tail.len());
                tail[..end].parse::<f64>().unwrap_or(1.0) + acc
            } else {
                acc + 1.0
            }
        })
    }
}

impl Default for SolidMaker {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BoxMaker
// ---------------------------------------------------------------------------

// occt-ref: BRepPrimAPI_MakeBox
/// Stub mirroring `BRepPrimAPI_MakeBox`.
///
/// Builds an axis-aligned box solid from a corner point and three dimensions.
/// Supports a builder-pattern [`BoxMaker::at`] method to relocate the corner
/// after initial construction.
///
/// # Typical usage
///
/// ```
/// use ironstream::brep_solid_maker::BoxMaker;
///
/// let b = BoxMaker::new(2.0, 3.0, 4.0).at([1.0, 0.0, 0.0]);
/// assert!((b.volume() - 24.0).abs() < 1e-9);
/// let label = b.build();
/// assert!(label.contains("dx=2"));
/// ```
#[derive(Debug, Clone)]
pub struct BoxMaker {
    // occt-ref: BRepPrimAPI_MakeBox // — corner of the box (lower-left-front vertex)
    /// The lower corner (origin vertex) of the box.
    pub corner: [f64; 3],
    // occt-note: BRepPrimAPI_MakeBox(Standard_Real dx, ...)
    /// Extent along the X axis.
    pub dx: f64,
    /// Extent along the Y axis.
    pub dy: f64,
    /// Extent along the Z axis.
    pub dz: f64,
}

impl BoxMaker {
    // occt-note: BRepPrimAPI_MakeBox(Standard_Real dx, Standard_Real dy, Standard_Real dz)
    /// Create a box with its corner at the origin extending by `(dx, dy, dz)`.
    ///
    /// Negative dimensions are stored as their absolute values.
    ///
    /// Mirrors `BRepPrimAPI_MakeBox(Standard_Real, Standard_Real, Standard_Real)`.
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            corner: [0.0, 0.0, 0.0],
            dx: dx.abs(),
            dy: dy.abs(),
            dz: dz.abs(),
        }
    }

    // occt-note: BRepPrimAPI_MakeBox(const gp_Pnt& P, Standard_Real dx, ...)
    /// Relocate the lower corner of the box to `corner` and return `self`.
    ///
    /// Consuming builder-pattern method; mirrors the
    /// `BRepPrimAPI_MakeBox(const gp_Pnt&, dx, dy, dz)` constructor overload.
    pub fn at(mut self, corner: [f64; 3]) -> Self {
        self.corner = corner;
        self
    }

    // occt-ref: BRepPrimAPI_MakeBox // ::Shape() / ::Solid()
    /// Build and return the box solid label.
    ///
    /// Mirrors `BRepPrimAPI_MakeBox::Shape()` / `::Solid()`.
    pub fn build(&self) -> String {
        format!(
            "solid(box,corner=[{},{},{}],dx={},dy={},dz={})",
            self.corner[0],
            self.corner[1],
            self.corner[2],
            self.dx,
            self.dy,
            self.dz,
        )
    }

    // occt-ref: BRepGProp // ::VolumeProperties — analytic volume
    /// Return the exact volume `dx * dy * dz`.
    pub fn volume(&self) -> f64 {
        self.dx * self.dy * self.dz
    }

    // occt-ref: BRepGProp // ::SurfaceProperties — analytic surface area
    /// Return the exact surface area `2*(dx*dy + dy*dz + dz*dx)`.
    pub fn surface_area(&self) -> f64 {
        2.0 * (self.dx * self.dy + self.dy * self.dz + self.dz * self.dx)
    }
}

// ---------------------------------------------------------------------------
// Free-function helpers
// ---------------------------------------------------------------------------

/// Build an axis-aligned box solid with its corner at the origin.
///
/// Convenience wrapper; mirrors the typical single-call
/// `BRepPrimAPI_MakeBox(dx, dy, dz).Shape()` idiom from OCCT.
///
/// # Example
///
/// ```
/// let label = ironstream::brep_solid_maker::make_box(1.0, 2.0, 3.0);
/// assert!(label.contains("dx=1"));
/// ```
// occt-ref: BRepPrimAPI_MakeBox
pub fn make_box(dx: f64, dy: f64, dz: f64) -> String {
    BoxMaker::new(dx, dy, dz).build()
}

/// Build a sphere solid label centred at `center` with radius `r`.
///
/// The returned label embeds the centre coordinates and radius so that
/// downstream consumers can reconstruct the geometry.
///
/// # Example
///
/// ```
/// let label = ironstream::brep_solid_maker::make_sphere([0.0, 0.0, 0.0], 5.0);
/// assert!(label.contains("r=5"));
/// ```
// occt-ref: BRepPrimAPI_MakeSphere
pub fn make_sphere(center: [f64; 3], r: f64) -> String {
    format!(
        "solid(sphere,center=[{},{},{}],r={})",
        center[0], center[1], center[2],
        r.abs(),
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // -- SolidMaker -----------------------------------------------------------

    #[test]
    fn solid_maker_new_is_empty() {
        let m = SolidMaker::new();
        assert!(m.shells.is_empty());
        assert!(!m.done);
    }

    #[test]
    fn solid_maker_add_shell_accumulates() {
        let mut m = SolidMaker::new();
        m.add_shell("shell_a");
        m.add_shell("shell_b");
        assert_eq!(m.shells.len(), 2);
    }

    #[test]
    fn solid_maker_add_shell_ignores_empty() {
        let mut m = SolidMaker::new();
        m.add_shell("");
        assert!(m.shells.is_empty());
    }

    #[test]
    fn solid_maker_solid_no_shells_returns_empty() {
        let mut m = SolidMaker::new();
        let s = m.solid();
        assert!(s.is_empty());
        assert!(!m.is_done());
    }

    #[test]
    fn solid_maker_solid_with_shells() {
        let mut m = SolidMaker::new();
        m.add_shell("outer");
        let s = m.solid();
        assert!(s.contains("outer"));
        assert!(m.is_done());
    }

    #[test]
    fn solid_maker_solid_multiple_shells() {
        let mut m = SolidMaker::new();
        m.add_shell("shell_1");
        m.add_shell("shell_2");
        m.add_shell("shell_3");
        let s = m.solid();
        assert!(s.contains("shell_1"));
        assert!(s.contains("shell_2"));
        assert!(s.contains("shell_3"));
    }

    #[test]
    fn solid_maker_is_done_after_solid() {
        let mut m = SolidMaker::new();
        m.add_shell("s");
        assert!(!m.is_done()); // not done yet
        let _ = m.solid();
        assert!(m.is_done());
    }

    #[test]
    fn solid_maker_volume_default_one_per_shell() {
        let mut m = SolidMaker::new();
        m.add_shell("shell_a");
        m.add_shell("shell_b");
        assert!((m.volume() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn solid_maker_volume_parses_embedded_vol_tag() {
        let mut m = SolidMaker::new();
        m.add_shell("shell(vol=8.0)");
        m.add_shell("shell(vol=2.5)");
        assert!((m.volume() - 10.5).abs() < 1e-9);
    }

    // -- BoxMaker -------------------------------------------------------------

    #[test]
    fn box_maker_new_defaults_corner_to_origin() {
        let b = BoxMaker::new(1.0, 2.0, 3.0);
        assert_eq!(b.corner, [0.0, 0.0, 0.0]);
        assert_eq!(b.dx, 1.0);
        assert_eq!(b.dy, 2.0);
        assert_eq!(b.dz, 3.0);
    }

    #[test]
    fn box_maker_new_abs_dims() {
        let b = BoxMaker::new(-2.0, -3.0, -4.0);
        assert_eq!(b.dx, 2.0);
        assert_eq!(b.dy, 3.0);
        assert_eq!(b.dz, 4.0);
    }

    #[test]
    fn box_maker_at_sets_corner() {
        let b = BoxMaker::new(1.0, 1.0, 1.0).at([5.0, 6.0, 7.0]);
        assert_eq!(b.corner, [5.0, 6.0, 7.0]);
    }

    #[test]
    fn box_maker_volume_exact() {
        let b = BoxMaker::new(2.0, 3.0, 4.0);
        assert!((b.volume() - 24.0).abs() < 1e-12);
    }

    #[test]
    fn box_maker_surface_area_exact() {
        // 2*(2*3 + 3*4 + 4*2) = 2*(6+12+8) = 52
        let b = BoxMaker::new(2.0, 3.0, 4.0);
        assert!((b.surface_area() - 52.0).abs() < 1e-12);
    }

    #[test]
    fn box_maker_build_contains_dims() {
        let label = BoxMaker::new(2.0, 3.0, 4.0).at([1.0, 0.0, 0.0]).build();
        assert!(label.contains("dx=2"));
        assert!(label.contains("dy=3"));
        assert!(label.contains("dz=4"));
        assert!(label.contains("corner=[1,"));
    }

    #[test]
    fn box_maker_cube_surface_area() {
        // Unit cube: 6 faces each 1x1 -> area = 6
        let b = BoxMaker::new(1.0, 1.0, 1.0);
        assert!((b.surface_area() - 6.0).abs() < 1e-12);
    }

    // -- make_box free function -----------------------------------------------

    #[test]
    fn make_box_label_format() {
        let label = make_box(3.0, 4.0, 5.0);
        assert!(label.contains("box"));
        assert!(label.contains("dx=3"));
        assert!(label.contains("dy=4"));
        assert!(label.contains("dz=5"));
        assert!(label.contains("corner=[0,0,0]"));
    }

    #[test]
    fn make_box_zero_corner() {
        let label = make_box(1.0, 1.0, 1.0);
        assert!(label.contains("corner=[0,0,0]"));
    }

    // -- make_sphere free function --------------------------------------------

    #[test]
    fn make_sphere_label_format() {
        let label = make_sphere([1.0, 2.0, 3.0], 5.0);
        assert!(label.contains("sphere"));
        assert!(label.contains("r=5"));
        assert!(label.contains("center=[1,2,3]"));
    }

    #[test]
    fn make_sphere_negative_radius_abs() {
        let label = make_sphere([0.0, 0.0, 0.0], -3.0);
        assert!(label.contains("r=3"));
    }

    #[test]
    fn make_sphere_origin_center() {
        let label = make_sphere([0.0, 0.0, 0.0], 1.0);
        assert!(label.contains("center=[0,0,0]"));
    }

    // -- analytic cross-checks ------------------------------------------------

    #[test]
    fn sphere_volume_formula() {
        // Verify the well-known formula 4/3 * pi * r^3 independently.
        let r = 2.0_f64;
        let expected = 4.0 / 3.0 * PI * r * r * r;
        // make_sphere is a label stub; test the formula directly.
        let computed = 4.0 / 3.0 * PI * r.powi(3);
        assert!((computed - expected).abs() < 1e-12);
    }

    #[test]
    fn box_volume_matches_surface_area_for_cube() {
        let b = BoxMaker::new(2.0, 2.0, 2.0);
        // vol = 8, area = 24
        assert!((b.volume() - 8.0).abs() < 1e-12);
        assert!((b.surface_area() - 24.0).abs() < 1e-12);
    }
}
