// FILE: geom_surface_props.rs
//! Geometric surface / volume / linear mass-property types.
//!
//! This module is a **pure-Rust, zero-dependency stub** that mirrors the
//! OpenCascade packages `GProp_GProps` and `BRepGProp`.  It exposes the same
//! conceptual API shape so that callers familiar with OCCT find the matching
//! types without needing the C++ bindings.
//!
//! All integration routines currently return placeholder values (`0.0` /
//! origin).  A future revision will wire them to the tessellating kernel in
//! `brep_gprop` once the shape representation is settled.
//!
//! # Relationship to OCCT
//!
//! | OCCT type / function              | IronStream counterpart          |
//! |-----------------------------------|---------------------------------|
//! | `GProp_GProps`                    | [`GProps`]                      |
//! | `BRepGProp::LinearProperties`     | [`linear_props`]                |
//! | `BRepGProp::SurfaceProperties`    | [`surface_props`]               |
//! | `BRepGProp::VolumeProperties`     | [`volume_props`]                |

// ─────────────────────────────────────────────────────────────────────────────
// GProps  (mirrors GProp_GProps)
// ─────────────────────────────────────────────────────────────────────────────

/// Accumulator for global mass properties: scalar measure (volume, area, or
/// arc-length), centre of mass, and the second-moment inertia matrix.
///
/// Multiple contributions can be combined via [`GProps::add`].  The inertia
/// matrix uses the convention `I[row][col]` where the diagonal entries hold
/// the second moments (Ixx, Iyy, Izz) and the off-diagonal entries hold the
/// products of inertia.
// occt: GProp_GProps
#[derive(Clone, Debug, PartialEq)]
pub struct GProps {
    /// Total scalar measure (volume, area, or arc-length depending on context).
    // occt: GProp_GProps::Mass
    pub mass: f64,

    /// Centre of mass (weighted centroid) as `[x, y, z]`.
    // occt: GProp_GProps::CentreOfMass
    pub centre_of_mass: [f64; 3],

    /// Second-moment inertia matrix relative to the centre of mass.
    ///
    /// Row / column layout:
    /// ```text
    ///  [ Ixx  Ixy  Ixz ]
    ///  [ Ixy  Iyy  Iyz ]
    ///  [ Ixz  Iyz  Izz ]
    /// ```
    // occt: GProp_GProps::MatrixOfInertia
    pub inertia: [[f64; 3]; 3],
}

impl GProps {
    /// Create an empty accumulator with zero mass, origin centroid, and zero
    /// inertia matrix.
    // occt: GProp_GProps::GProp_GProps()
    pub fn new() -> Self {
        GProps {
            mass: 0.0,
            centre_of_mass: [0.0; 3],
            inertia: [[0.0; 3]; 3],
        }
    }

    /// Accumulate `other` into `self`, scaling `other`'s mass by `density`.
    ///
    /// The centre of mass is updated as the mass-weighted average of the two
    /// existing centroids.  The inertia matrix is combined component-wise after
    /// scaling.
    ///
    /// # Arguments
    ///
    /// * `other`   — properties to add.
    /// * `density` — scalar multiplier applied to `other.mass` before adding.
    // occt: GProp_GProps::Add(GProps, density)
    pub fn add(&mut self, other: &GProps, density: f64) {
        let other_mass = other.mass * density;
        let total = self.mass + other_mass;

        if total.abs() > 0.0 {
            // Mass-weighted centroid.
            for i in 0..3 {
                self.centre_of_mass[i] = (self.centre_of_mass[i] * self.mass
                    + other.centre_of_mass[i] * other_mass)
                    / total;
            }
        }

        // Component-wise inertia accumulation (scaled).
        for r in 0..3 {
            for c in 0..3 {
                self.inertia[r][c] += other.inertia[r][c] * density;
            }
        }

        self.mass = total;
    }

    /// Total scalar measure (volume, area, or arc-length).
    // occt: GProp_GProps::Mass
    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Centre of mass as `[x, y, z]`.
    // occt: GProp_GProps::CentreOfMass
    pub fn centre_of_mass(&self) -> [f64; 3] {
        self.centre_of_mass
    }
}

impl Default for GProps {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VolumeProps  (mirrors BRepGProp volumetric result)
// ─────────────────────────────────────────────────────────────────────────────

/// Volumetric properties of a closed solid: enclosed volume and centre of
/// gravity (COG).
///
/// Mirrors the data extracted from `BRepGProp::VolumeProperties` in OCCT.
// occt: BRepGProp
#[derive(Clone, Debug, PartialEq)]
pub struct VolumeProps {
    /// Enclosed volume of the solid.
    pub volume: f64,

    /// Centre of gravity as `[x, y, z]`.
    pub cog: [f64; 3],
}

impl VolumeProps {
    /// Compute volumetric properties for the shape identified by `shape`.
    ///
    /// In this stub the `shape` argument is a string token that names the
    /// solid; real integration will be wired to the tessellating kernel.
    /// Returns zeroed properties until then.
    // occt: BRepGProp::VolumeProperties
    pub fn from_shape(_shape: &str) -> Self {
        VolumeProps {
            volume: 0.0,
            cog: [0.0; 3],
        }
    }

    /// Returns `true` when the computed volume is strictly positive, indicating
    /// that the shape is a non-degenerate closed solid.
    // occt: (validity check on BRepGProp result)
    pub fn is_valid(&self) -> bool {
        self.volume > 0.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SurfaceProps  (mirrors BRepGProp surface-area result)
// ─────────────────────────────────────────────────────────────────────────────

/// Surface-area properties of a shape: total area and the area-weighted centre
/// of gravity.
///
/// Mirrors the data extracted from `BRepGProp::SurfaceProperties` in OCCT.
// occt: BRepGProp
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceProps {
    /// Total surface area.
    pub area: f64,

    /// Area-weighted centre of gravity as `[x, y, z]`.
    pub cog: [f64; 3],
}

impl SurfaceProps {
    /// Compute surface-area properties for the shape identified by `shape`.
    ///
    /// In this stub the `shape` argument is a string token that names the
    /// solid; real integration will be wired to the tessellating kernel.
    /// Returns zeroed properties until then.
    // occt: BRepGProp::SurfaceProperties
    pub fn from_shape(_shape: &str) -> Self {
        SurfaceProps {
            area: 0.0,
            cog: [0.0; 3],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions  (mirrors BRepGProp static interface)
// ─────────────────────────────────────────────────────────────────────────────

/// Compute **linear** (arc-length) properties of the edges of a shape.
///
/// Returns a [`GProps`] where `mass` is the total edge length of `shape`.
/// The centre of mass is the arc-length-weighted centroid of all edges.
///
/// Mirrors `BRepGProp::LinearProperties(shape, props)` from OCCT.
// occt: BRepGProp
pub fn linear_props(_shape: &str) -> GProps {
    // Stub: returns zeroed properties.  Wire to the edge-integration loop in
    // `brep_gprop::BRepGProp::linear_properties` once topology is unified.
    GProps::new()
}

/// Compute **surface-area** properties of a shape.
///
/// Returns a [`GProps`] where `mass` is the total surface area of `shape`.
/// The centre of mass is the area-weighted centroid of all triangles.
///
/// Mirrors `BRepGProp::SurfaceProperties(shape, props)` from OCCT.
// occt: BRepGProp
pub fn surface_props(_shape: &str) -> GProps {
    // Stub: returns zeroed properties.  Wire to the triangle-integration loop
    // in `brep_gprop::BRepGProp::surface_properties` once topology is unified.
    GProps::new()
}

/// Compute **volumetric** properties of a closed solid shape.
///
/// Returns a [`GProps`] where `mass` is the enclosed volume of `shape`.
/// The centre of mass is the volume-weighted centroid.
///
/// Mirrors `BRepGProp::VolumeProperties(shape, props)` from OCCT.
// occt: BRepGProp
pub fn volume_props(_shape: &str) -> GProps {
    // Stub: returns zeroed properties.  Wire to the divergence-theorem
    // integration in `brep_gprop::BRepGProp::volume_properties` once topology
    // is unified.
    GProps::new()
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── GProps ────────────────────────────────────────────────────────────────

    #[test]
    fn gprops_new_is_zero() {
        let g = GProps::new();
        assert_eq!(g.mass(), 0.0);
        assert_eq!(g.centre_of_mass(), [0.0; 3]);
        assert_eq!(g.inertia, [[0.0; 3]; 3]);
    }

    #[test]
    fn gprops_default_matches_new() {
        assert_eq!(GProps::default(), GProps::new());
    }

    #[test]
    fn gprops_add_zero_density_unchanged_mass() {
        let mut base = GProps {
            mass: 5.0,
            centre_of_mass: [1.0, 2.0, 3.0],
            inertia: [[0.0; 3]; 3],
        };
        let other = GProps {
            mass: 10.0,
            centre_of_mass: [4.0, 5.0, 6.0],
            inertia: [[1.0; 3]; 3],
        };
        base.add(&other, 0.0);
        // other_mass = 10 * 0 = 0, so base mass and centroid should be
        // unchanged and inertia contributions are zero.
        assert_eq!(base.mass(), 5.0);
        assert_eq!(base.centre_of_mass(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn gprops_add_equal_density_combines_mass() {
        let mut a = GProps {
            mass: 2.0,
            centre_of_mass: [0.0, 0.0, 0.0],
            inertia: [[0.0; 3]; 3],
        };
        let b = GProps {
            mass: 2.0,
            centre_of_mass: [4.0, 0.0, 0.0],
            inertia: [[0.0; 3]; 3],
        };
        a.add(&b, 1.0);
        assert_eq!(a.mass(), 4.0);
        // Centre of mass should be midpoint: x = 2.0.
        let com = a.centre_of_mass();
        assert!((com[0] - 2.0).abs() < 1e-12, "com_x = {}", com[0]);
        assert!((com[1]).abs() < 1e-12);
        assert!((com[2]).abs() < 1e-12);
    }

    #[test]
    fn gprops_add_scales_inertia_by_density() {
        let mut base = GProps::new();
        let other = GProps {
            mass: 1.0,
            centre_of_mass: [0.0; 3],
            inertia: [
                [2.0, 0.0, 0.0],
                [0.0, 3.0, 0.0],
                [0.0, 0.0, 4.0],
            ],
        };
        base.add(&other, 0.5);
        assert!((base.inertia[0][0] - 1.0).abs() < 1e-12); // 2.0 * 0.5
        assert!((base.inertia[1][1] - 1.5).abs() < 1e-12); // 3.0 * 0.5
        assert!((base.inertia[2][2] - 2.0).abs() < 1e-12); // 4.0 * 0.5
    }

    #[test]
    fn gprops_mass_accessor_matches_field() {
        let g = GProps {
            mass: 7.5,
            centre_of_mass: [0.0; 3],
            inertia: [[0.0; 3]; 3],
        };
        assert_eq!(g.mass(), 7.5);
    }

    #[test]
    fn gprops_centre_of_mass_accessor_matches_field() {
        let g = GProps {
            mass: 1.0,
            centre_of_mass: [1.0, 2.0, 3.0],
            inertia: [[0.0; 3]; 3],
        };
        assert_eq!(g.centre_of_mass(), [1.0, 2.0, 3.0]);
    }

    // ── VolumeProps ───────────────────────────────────────────────────────────

    #[test]
    fn volume_props_from_shape_returns_zero_stub() {
        let vp = VolumeProps::from_shape("box");
        assert_eq!(vp.volume, 0.0);
        assert_eq!(vp.cog, [0.0; 3]);
    }

    #[test]
    fn volume_props_is_valid_false_for_zero_volume() {
        let vp = VolumeProps { volume: 0.0, cog: [0.0; 3] };
        assert!(!vp.is_valid());
    }

    #[test]
    fn volume_props_is_valid_true_for_positive_volume() {
        let vp = VolumeProps { volume: 1.0, cog: [0.0; 3] };
        assert!(vp.is_valid());
    }

    #[test]
    fn volume_props_is_valid_false_for_negative_volume() {
        let vp = VolumeProps { volume: -3.0, cog: [0.0; 3] };
        assert!(!vp.is_valid());
    }

    // ── SurfaceProps ──────────────────────────────────────────────────────────

    #[test]
    fn surface_props_from_shape_returns_zero_stub() {
        let sp = SurfaceProps::from_shape("sphere");
        assert_eq!(sp.area, 0.0);
        assert_eq!(sp.cog, [0.0; 3]);
    }

    // ── Free functions ────────────────────────────────────────────────────────

    #[test]
    fn linear_props_stub_returns_zero() {
        let g = linear_props("wire");
        assert_eq!(g.mass(), 0.0);
    }

    #[test]
    fn surface_props_fn_stub_returns_zero() {
        let g = surface_props("face");
        assert_eq!(g.mass(), 0.0);
    }

    #[test]
    fn volume_props_fn_stub_returns_zero() {
        let g = volume_props("solid");
        assert_eq!(g.mass(), 0.0);
    }
}
