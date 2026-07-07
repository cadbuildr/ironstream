//! `TopoDS` — topological data structures, mirroring OpenCascade's `TopoDS`
//! package: `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`.
//!
//! IronStream is a tessellating kernel: a [`Solid`] always carries a watertight
//! [`TriMesh`]. Profiles (sketch geometry) flow in as [`Wire`]s and become
//! planar [`Face`]s that the sweepers (`brep_prim_api`) turn into solids. Faces
//! retain their supporting plane so triangulation and normals stay exact.

use crate::gp::{Pnt, Trsf, EPS};
use crate::mesh::{BBox, TriMesh};

/// A point in space (`TopoDS_Vertex`).
#[derive(Clone, Copy, Debug)]
// occt: TopoDS_Vertex
pub struct Vertex(pub Pnt);

/// A closed loop of points in world space (`TopoDS_Wire`). Arcs/circles are
/// pre-discretized into points by the builders, so a wire is always a polyline.
#[derive(Clone, Debug)]
// occt: TopoDS_Wire
pub struct Wire {
    /// Ordered loop points. The closing edge from `pts.last()` to `pts[0]` is
    /// implicit; we do not duplicate the first point.
    pub pts: Vec<Pnt>,
}

impl Wire {
    pub fn new(pts: Vec<Pnt>) -> Self {
        let mut pts = pts;
        // Drop a duplicated closing point if present.
        if pts.len() >= 2 && pts[0].distance(*pts.last().unwrap()) < EPS {
            pts.pop();
        }
        Wire { pts }
    }

    pub fn is_valid(&self) -> bool {
        self.pts.len() >= 3
    }

    /// Best-fit plane normal via Newell's method (robust for non-planar noise).
    pub fn normal(&self) -> Pnt {
        let mut n = Pnt::origin();
        let m = self.pts.len();
        for i in 0..m {
            let a = self.pts[i];
            let b = self.pts[(i + 1) % m];
            n = n + Pnt::new(
                (a.y - b.y) * (a.z + b.z),
                (a.z - b.z) * (a.x + b.x),
                (a.x - b.x) * (a.y + b.y),
            );
        }
        n.normalized()
    }

    pub fn centroid(&self) -> Pnt {
        let mut c = Pnt::origin();
        for p in &self.pts {
            c = c + *p;
        }
        c * (1.0 / self.pts.len().max(1) as f64)
    }
}

/// A planar face: an outer boundary wire plus optional hole wires
/// (`TopoDS_Face`). All wires are assumed coplanar.
#[derive(Clone, Debug)]
// occt: TopoDS_Face
pub struct Face {
    pub outer: Wire,
    pub holes: Vec<Wire>,
}

impl Face {
    pub fn new(outer: Wire) -> Self {
        Face {
            outer,
            holes: Vec::new(),
        }
    }

    pub fn with_holes(outer: Wire, holes: Vec<Wire>) -> Self {
        Face { outer, holes }
    }

    /// Plane normal of the face (from the outer wire).
    pub fn normal(&self) -> Pnt {
        self.outer.normal()
    }

    /// Triangulate the face into world-space triangles (CCW about `normal`).
    /// Outer + holes are merged into a single simple polygon by keyhole
    /// bridging, then ear-clipped. Returns triangles as flat point triples.
    pub fn triangulate(&self) -> Vec<[Pnt; 3]> {
        crate::tess::triangulate_face(self)
    }
}

/// A solid (`TopoDS_Solid`): a watertight boundary mesh, optionally tagged with
/// the analytic surface kind it came from (handy for debugging / future exact
/// ops). The mesh is the single source of truth for volume and booleans.
#[derive(Clone, Debug)]
// occt: TopoDS_Solid, TopoDS_Shell
pub struct Solid {
    mesh: TriMesh,
    /// Analytic provenance: curved surfaces this solid's boundary is known to
    /// sample — stamped by the primitive builders and carried through
    /// booleans/transforms. Consumed by `analytic_step` to emit true B-rep
    /// STEP faces instead of tessellation. Advisory only: an empty list just
    /// means every face falls back to planar/faceted output.
    hints: Vec<crate::geom::Surface>,
    /// Exact boundary representation, when the construction path could build
    /// one (see [`crate::brep`]). Preferred for volume; booleans use it for
    /// the exact drill path and drop it when they can't carry it.
    brep: Option<crate::brep::BSolid>,
}

impl Solid {
    pub fn from_mesh(mesh: TriMesh) -> Self {
        Solid {
            mesh,
            hints: Vec::new(),
            brep: None,
        }
    }

    /// Attach analytic surface provenance (see the `hints` field docs).
    pub fn with_hints(mesh: TriMesh, hints: Vec<crate::geom::Surface>) -> Self {
        Solid { mesh, hints, brep: None }
    }

    /// Exact boundary representation, when one was carried through.
    pub fn brep(&self) -> Option<&crate::brep::BSolid> {
        self.brep.as_ref()
    }

    pub fn set_brep(&mut self, brep: Option<crate::brep::BSolid>) {
        self.brep = brep;
    }

    /// Curved surfaces this solid's boundary is known to sample.
    pub fn hints(&self) -> &[crate::geom::Surface] {
        &self.hints
    }

    pub fn add_hint(&mut self, surface: crate::geom::Surface) {
        self.hints.push(surface);
    }

    /// Merge another solid's provenance into this one (used by booleans).
    pub fn merge_hints_from(&mut self, other: &Solid) {
        self.hints.extend(other.hints.iter().cloned());
    }

    pub fn empty() -> Self {
        Solid {
            mesh: TriMesh::new(),
            hints: Vec::new(),
            brep: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mesh.is_empty()
    }

    pub fn mesh(&self) -> &TriMesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut TriMesh {
        &mut self.mesh
    }

    pub fn into_mesh(self) -> TriMesh {
        self.mesh
    }

    pub fn volume(&self) -> f64 {
        // the exact B-rep integrates circles as true disks; the mesh is its
        // tessellation, so prefer the exact value when we have it.
        match &self.brep {
            Some(b) => b.volume(),
            None => self.mesh.volume(),
        }
    }

    pub fn surface_area(&self) -> f64 {
        self.mesh.surface_area()
    }

    pub fn bbox(&self) -> BBox {
        self.mesh.bbox()
    }

    pub fn transformed(&self, t: &Trsf) -> Solid {
        Solid {
            mesh: self.mesh.transformed(t),
            hints: crate::geom::transform_hints(&self.hints, t),
            brep: self.brep.as_ref().and_then(|b| b.rigid_transformed(t)),
        }
    }

    pub fn transform(&mut self, t: &Trsf) {
        self.mesh.transform(t);
        self.hints = crate::geom::transform_hints(&self.hints, t);
        self.brep = self.brep.as_ref().and_then(|b| b.rigid_transformed(t));
    }

    /// Weld + clean the boundary (call after booleans / assembly).
    pub fn cleaned(&self, tol: f64) -> Solid {
        Solid {
            mesh: self.mesh.welded(tol),
            hints: self.hints.clone(),
            brep: self.brep.clone(),
        }
    }
}

/// A heterogeneous group of solids (`TopoDS_Compound`) — e.g. an assembly or a
/// multi-region part before fusing.
#[derive(Clone, Debug, Default)]
// occt: TopoDS_Compound
pub struct Compound {
    pub solids: Vec<Solid>,
}

impl Compound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, s: Solid) {
        if !s.is_empty() {
            self.solids.push(s);
        }
    }

    /// Flatten the compound into a single (non-merged) mesh.
    pub fn to_mesh(&self) -> TriMesh {
        let mut m = TriMesh::new();
        for s in &self.solids {
            m.extend(s.mesh());
        }
        m
    }
}
