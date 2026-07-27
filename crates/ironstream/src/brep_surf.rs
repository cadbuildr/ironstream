// FILE: brep_surf.rs
// occt: BRep_PointOnCurve, BRep_PointOnCurveOnSurface
// occt-ref: BRep_Surface
//       BRep_PointOnSurface, BRep_CurveOnSurface, BRep_GCurve

/// Representation type for a curve on a surface.
/// occt-note: BRep_Representation kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepRepKind {
    Curve3D,
    CurveOnSurface,
    CurveOnClosedSurface,
    Polygon3D,
    PolygonOnSurface,
    PolygonOnClosedSurface,
    PolygonOnTriangulation,
}

impl Default for BrepRepKind {
    fn default() -> Self { Self::Curve3D }
}

/// A 3D curve stored on a BRep edge.
// occt-ref: BRep_Curve3D
#[derive(Clone, Debug)]
pub struct BrepCurve3D {
    pub curve_id: u32,
    pub location_id: u32,
    pub first: f64,
    pub last: f64,
}

impl BrepCurve3D {
    pub fn new(curve_id: u32, first: f64, last: f64) -> Self {
        Self { curve_id, location_id: 0, first, last }
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn first(&self) -> f64 { self.first }
    pub fn last(&self) -> f64 { self.last }
    pub fn is_valid(&self) -> bool { self.curve_id > 0 && self.first < self.last }
}

/// A 2D curve (pcurve) on a surface stored on a BRep edge.
// occt-ref: BRep_CurveOnSurface
#[derive(Clone, Debug)]
pub struct BrepCurveOnSurface {
    pub pcurve_id: u32,
    pub surface_id: u32,
    pub surface_location_id: u32,
    pub first: f64,
    pub last: f64,
    pub uv_iso: Option<[f64; 2]>,
}

impl BrepCurveOnSurface {
    pub fn new(pcurve_id: u32, surface_id: u32, first: f64, last: f64) -> Self {
        Self {
            pcurve_id, surface_id,
            surface_location_id: 0,
            first, last,
            uv_iso: None,
        }
    }

    pub fn pcurve_id(&self) -> u32 { self.pcurve_id }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn first(&self) -> f64 { self.first }
    pub fn last(&self) -> f64 { self.last }
    pub fn is_valid(&self) -> bool { self.pcurve_id > 0 && self.surface_id > 0 && self.first < self.last }
}

/// BRep surface representation (stub).
// occt-ref: BRep_Surface
#[derive(Clone, Debug)]
pub struct BrepSurface {
    pub surface_id: u32,
    pub location_id: u32,
    pub tolerance: f64,
    pub natural_restriction: bool,
}

impl BrepSurface {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id, location_id: 0, tolerance: 1e-7, natural_restriction: false }
    }

    pub fn with_location(mut self, loc_id: u32) -> Self { self.location_id = loc_id; self }
    pub fn with_tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn natural_restriction(&self) -> bool { self.natural_restriction }
    pub fn set_natural_restriction(&mut self, v: bool) { self.natural_restriction = v; }
}

/// A point on a 3D curve (used for BRep vertex).
/// occt: BRep_PointOnCurve
#[derive(Clone, Debug)]
pub struct BrepPointOnCurve {
    pub parameter: f64,
    pub curve_id: u32,
    pub location_id: u32,
}

impl BrepPointOnCurve {
    pub fn new(parameter: f64, curve_id: u32) -> Self {
        Self { parameter, curve_id, location_id: 0 }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn curve_id(&self) -> u32 { self.curve_id }
}

/// A point given by UV parameters on a surface.
/// occt: BRep_PointOnSurface
#[derive(Clone, Debug)]
pub struct BrepPointOnSurface {
    pub u: f64,
    pub v: f64,
    pub surface_id: u32,
    pub location_id: u32,
}

impl BrepPointOnSurface {
    pub fn new(u: f64, v: f64, surface_id: u32) -> Self {
        Self { u, v, surface_id, location_id: 0 }
    }

    pub fn u(&self) -> f64 { self.u }
    pub fn v(&self) -> f64 { self.v }
    pub fn surface_id(&self) -> u32 { self.surface_id }
}

/// A point on a 2D curve on a surface.
/// occt: BRep_PointOnCurveOnSurface
#[derive(Clone, Debug)]
pub struct BrepPointOnCurveOnSurface {
    pub parameter: f64,
    pub pcurve_id: u32,
    pub surface_id: u32,
    pub location_id: u32,
}

impl BrepPointOnCurveOnSurface {
    pub fn new(parameter: f64, pcurve_id: u32, surface_id: u32) -> Self {
        Self { parameter, pcurve_id, surface_id, location_id: 0 }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn pcurve_id(&self) -> u32 { self.pcurve_id }
    pub fn surface_id(&self) -> u32 { self.surface_id }
}

/// Polygon on a triangulation.
/// occt: BRep_PolygonOnTriangulation
#[derive(Clone, Debug)]
pub struct BrepPolygonOnTriangulation {
    pub triangulation_id: u32,
    pub node_indices: Vec<usize>,
    pub location_id: u32,
}

impl BrepPolygonOnTriangulation {
    pub fn new(triangulation_id: u32) -> Self {
        Self { triangulation_id, node_indices: Vec::new(), location_id: 0 }
    }

    pub fn add_node(&mut self, idx: usize) { self.node_indices.push(idx); }
    pub fn nb_nodes(&self) -> usize { self.node_indices.len() }
    pub fn node(&self, i: usize) -> Option<usize> {
        if i == 0 { None } else { self.node_indices.get(i - 1).copied() }
    }
    pub fn triangulation_id(&self) -> u32 { self.triangulation_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brep_curve3d_valid() {
        let c = BrepCurve3D::new(5, 0.0, 1.0);
        assert!(c.is_valid());
        assert_eq!(c.curve_id(), 5);
        let bad = BrepCurve3D::new(0, 0.0, 1.0);
        assert!(!bad.is_valid());
    }

    #[test]
    fn brep_curve_on_surface_valid() {
        let c = BrepCurveOnSurface::new(3, 7, 0.0, 2.0);
        assert!(c.is_valid());
        assert_eq!(c.pcurve_id(), 3);
        assert_eq!(c.surface_id(), 7);
    }

    #[test]
    fn brep_surface_attrs() {
        let mut s = BrepSurface::new(10).with_tolerance(1e-6);
        assert_eq!(s.surface_id(), 10);
        assert!((s.tolerance() - 1e-6).abs() < 1e-15);
        s.set_natural_restriction(true);
        assert!(s.natural_restriction());
    }

    #[test]
    fn brep_point_on_curve() {
        let p = BrepPointOnCurve::new(0.5, 3);
        assert!((p.parameter() - 0.5).abs() < 1e-12);
        assert_eq!(p.curve_id(), 3);
    }

    #[test]
    fn brep_point_on_surface() {
        let p = BrepPointOnSurface::new(0.3, 0.7, 5);
        assert!((p.u() - 0.3).abs() < 1e-12);
        assert!((p.v() - 0.7).abs() < 1e-12);
        assert_eq!(p.surface_id(), 5);
    }

    #[test]
    fn brep_polygon_on_triangulation() {
        let mut p = BrepPolygonOnTriangulation::new(1);
        p.add_node(1);
        p.add_node(2);
        p.add_node(3);
        assert_eq!(p.nb_nodes(), 3);
        assert_eq!(p.node(1), Some(1));
        assert_eq!(p.node(0), None);
        assert_eq!(p.triangulation_id(), 1);
    }
}
