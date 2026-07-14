// FILE: rust/ironstream/crates/ironstream/src/brep_make_face.rs
//! `BRepBuilderAPI_MakeFace` — parametric face builder, mirroring OCCT's
//! `BRepBuilderAPI_MakeFace` class with surface-type metadata and wire management.

// occt-note: type of surface for face — Planar, Cylindrical, Spherical, Conical, Toroidal, BSpline, Offset
/// Surface kind carried by a parametric face.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceType {
    /// Flat plane — `Geom_Plane`.
    // occt-ref: Geom_Plane // (infinite planar surface)
    Planar,
    /// Right circular cylinder — `Geom_CylindricalSurface`.
    // occt-ref: Geom_CylindricalSurface
    Cylindrical,
    /// Sphere — `Geom_SphericalSurface`.
    // occt-ref: Geom_SphericalSurface
    Spherical,
    /// Right circular cone — `Geom_ConicalSurface`.
    // occt-ref: Geom_ConicalSurface
    Conical,
    /// Torus — `Geom_ToroidalSurface`.
    // occt-ref: Geom_ToroidalSurface
    Toroidal,
    /// General B-Spline surface — `Geom_BSplineSurface`.
    // occt-ref: Geom_BSplineSurface
    BSpline,
    /// Offset surface — `Geom_OffsetSurface`.
    // occt-ref: Geom_OffsetSurface
    Offset,
}

// occt-ref: BRepBuilderAPI_MakeFace // input params — surface kind plus UV domain and tolerance
/// Input parameters for constructing a parametric face.
#[derive(Debug, Clone)]
pub struct FaceParams {
    // occt-note: surface type tag
    face_type: FaceType,
    // occt-note: U parameter range [u_min, u_max]
    u_range: (f64, f64),
    // occt-note: V parameter range [v_min, v_max]
    v_range: (f64, f64),
    // occt-ref: BRepBuilderAPI_MakeFace // tolerance for vertex/edge on surface
    tolerance: f64,
}

impl FaceParams {
    /// Construct with default UV range [0, 1] × [0, 1] and tolerance 1e-7.
    // occt-note: BRepBuilderAPI_MakeFace(surface) — default domain
    pub fn new(face_type: FaceType) -> Self {
        Self {
            face_type,
            u_range: (0.0, 1.0),
            v_range: (0.0, 1.0),
            tolerance: 1e-7,
        }
    }

    /// Override the UV parameter domain.
    // occt-note: BRepBuilderAPI_MakeFace(surface, u1, u2, v1, v2, tol)
    pub fn with_range(mut self, u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        self.u_range = (u_min, u_max);
        self.v_range = (v_min, v_max);
        self
    }

    /// Return the modelling tolerance.
    // occt-ref: BRepBuilderAPI_MakeFace // ::IsDone / Tolerance accessor
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the modelling tolerance.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Return the U parameter range.
    pub fn u_range(&self) -> (f64, f64) {
        self.u_range
    }

    /// Return the V parameter range.
    pub fn v_range(&self) -> (f64, f64) {
        self.v_range
    }

    /// Return the surface kind.
    pub fn face_type(&self) -> FaceType {
        self.face_type
    }
}

// occt-note: inner wire on a face — wraps BRep_Wire topology with edge count and orientation flag
/// A wire lying on a face, either the outer boundary or an interior hole.
#[derive(Debug, Clone)]
pub struct FaceWire {
    // occt-note: label / name of the wire for identification
    wire_label: String,
    // occt-note: number of edges in the wire
    nb_edges: usize,
    // occt-note: orientation — true for outer boundary, false for hole
    is_outer: bool,
}

impl FaceWire {
    /// Construct a new face wire.
    // occt-ref: BRep_Builder // ::MakeWire + Add edge loop
    pub fn new(label: &str, nb_edges: usize, is_outer: bool) -> Self {
        Self {
            wire_label: label.to_string(),
            nb_edges,
            is_outer,
        }
    }

    /// Return the identifying label.
    pub fn label(&self) -> &str {
        &self.wire_label
    }

    /// Return the number of edges in this wire.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Return true if this wire is the outer boundary of the face.
    pub fn is_outer(&self) -> bool {
        self.is_outer
    }
}

// occt-ref: BRepBuilderAPI_MakeFace // — stub face builder that collects surface params and wires
/// Stub builder mirroring `BRepBuilderAPI_MakeFace`.
///
/// Holds face parameters, an optional outer wire, zero or more inner wires (holes),
/// and a completion flag set by [`build`](BrepMakeFace::build).
#[derive(Debug, Clone)]
pub struct BrepMakeFace {
    // occt-note: surface + UV-domain descriptor
    params: FaceParams,
    // occt-note: outer boundary wire (orientation FORWARD in OCCT)
    outer_wire: Option<FaceWire>,
    // occt-note: interior hole wires (orientation REVERSED in OCCT)
    inner_wires: Vec<FaceWire>,
    // occt-ref: BRepBuilderAPI_MakeFace // ::IsDone
    is_done: bool,
}

impl BrepMakeFace {
    /// Create a new builder from `FaceParams`.
    // occt-note: BRepBuilderAPI_MakeFace(surface, tol)
    pub fn new(params: FaceParams) -> Self {
        Self {
            params,
            outer_wire: None,
            inner_wires: Vec::new(),
            is_done: false,
        }
    }

    /// Set the outer boundary wire.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Add (outer wire, FORWARD orientation)
    pub fn set_outer_wire(&mut self, w: FaceWire) {
        self.outer_wire = Some(w);
    }

    /// Add an interior hole wire.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Add (inner wire, REVERSED orientation)
    pub fn add_inner_wire(&mut self, w: FaceWire) {
        self.inner_wires.push(w);
    }

    /// Mark the face as built.
    // occt-ref: BRepBuilderAPI_MakeFace // ::Build
    pub fn build(&mut self) {
        self.is_done = true;
    }

    /// Return true if the face has been successfully built.
    // occt-ref: BRepBuilderAPI_MakeFace // ::IsDone
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of interior hole wires.
    // occt-ref: BRepBuilderAPI_MakeFace // — count of inner wires via TopExp_Explorer
    pub fn nb_inner_wires(&self) -> usize {
        self.inner_wires.len()
    }

    /// Stub area: product of the U-span and V-span of the parameter domain.
    // occt-ref: BRepGProp // ::SurfaceProperties stub — returns (u_max-u_min)*(v_max-v_min)
    pub fn area(&self) -> f64 {
        let (u_min, u_max) = self.params.u_range;
        let (v_min, v_max) = self.params.v_range;
        (u_max - u_min) * (v_max - v_min)
    }

    /// Return the surface kind of this face.
    // occt-ref: BRep_Tool // ::Surface — surface type tag
    pub fn face_type(&self) -> FaceType {
        self.params.face_type()
    }
}
