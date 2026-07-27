// FILE: brep_edge_curve.rs
// occt: BRep_CurveOnSurface, BRep_Curve3D
// occt-ref: BRep_CurveRepresentation
//       BRep_TEdge, BRep_PolyCurve, BRep_ListOfCurveRepresentation

/// The kind of curve representation stored on a BRep edge.
// occt-ref: BRep_CurveRepresentation // (polymorphic base)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum BrepCurveRepKind {
    #[default]
    Curve3D,
    CurveOnSurface,
    CurveOnClosedSurface,
    CurveOnSurfaceUndefined,
    PolygonOnSurface,
    Polygon3D,
}

impl BrepCurveRepKind {
    pub fn is_3d(&self) -> bool { *self == Self::Curve3D || *self == Self::Polygon3D }
    pub fn is_on_surface(&self) -> bool {
        matches!(self, Self::CurveOnSurface | Self::CurveOnClosedSurface | Self::PolygonOnSurface)
    }
}

/// Parameter range for a curve on an edge [first, last].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrepEdgeParamRange {
    pub first: f64,
    pub last: f64,
}

impl Default for BrepEdgeParamRange {
    fn default() -> Self { Self { first: 0.0, last: 1.0 } }
}

impl BrepEdgeParamRange {
    pub fn new(first: f64, last: f64) -> Self { Self { first, last } }
    pub fn length(&self) -> f64 { (self.last - self.first).abs() }
    pub fn contains(&self, t: f64) -> bool { t >= self.first && t <= self.last }
    pub fn reversed(&self) -> Self { Self::new(self.last, self.first) }
}

/// A 3-D curve representation on a BRep edge.
/// occt: BRep_Curve3D
#[derive(Clone, Debug)]
pub struct BrepCurve3D {
    pub curve_id: u32,
    pub range: BrepEdgeParamRange,
}

impl BrepCurve3D {
    pub fn new(curve_id: u32, first: f64, last: f64) -> Self {
        Self { curve_id, range: BrepEdgeParamRange::new(first, last) }
    }
}

/// A 2-D curve (pcurve) on a surface.
/// occt: BRep_CurveOnSurface
#[derive(Clone, Debug)]
pub struct BrepCurveOnSurface {
    pub pcurve_id: u32,
    pub surface_id: u32,
    pub location_id: u32,
    pub range: BrepEdgeParamRange,
    pub is_seam: bool,
}

impl BrepCurveOnSurface {
    pub fn new(pcurve_id: u32, surface_id: u32) -> Self {
        Self {
            pcurve_id,
            surface_id,
            location_id: 0,
            range: BrepEdgeParamRange::default(),
            is_seam: false,
        }
    }

    pub fn with_range(mut self, first: f64, last: f64) -> Self {
        self.range = BrepEdgeParamRange::new(first, last);
        self
    }

    pub fn set_seam(&mut self, v: bool) { self.is_seam = v; }
    pub fn set_location(&mut self, loc_id: u32) { self.location_id = loc_id; }
}

/// A polygon approximation of a 3D curve on a surface.
/// occt: BRep_PolygonOnSurface // / BRep_PolyCurve
#[derive(Clone, Debug, Default)]
pub struct BrepPolyCurve {
    pub surface_id: u32,
    pub nodes: Vec<[f64; 2]>,  // UV nodes on the surface
    pub params: Vec<f64>,       // parameter values at nodes
    pub deflection: f64,
}

impl BrepPolyCurve {
    pub fn new(surface_id: u32) -> Self { Self { surface_id, ..Self::default() } }

    pub fn add_node(&mut self, uv: [f64; 2], param: f64) {
        self.nodes.push(uv);
        self.params.push(param);
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn set_deflection(&mut self, d: f64) { self.deflection = d.max(0.0); }
}

/// A curve representation entry (tagged union).
// occt-ref: BRep_ListOfCurveRepresentation // element
#[derive(Clone, Debug)]
pub enum BrepCurveRepresentation {
    Curve3D(BrepCurve3D),
    OnSurface(BrepCurveOnSurface),
    Poly(BrepPolyCurve),
}

impl BrepCurveRepresentation {
    pub fn kind(&self) -> BrepCurveRepKind {
        match self {
            Self::Curve3D(_)  => BrepCurveRepKind::Curve3D,
            Self::OnSurface(c) => if c.is_seam { BrepCurveRepKind::CurveOnClosedSurface } else { BrepCurveRepKind::CurveOnSurface },
            Self::Poly(_)     => BrepCurveRepKind::PolygonOnSurface,
        }
    }

    pub fn range(&self) -> BrepEdgeParamRange {
        match self {
            Self::Curve3D(c)  => c.range,
            Self::OnSurface(c) => c.range,
            Self::Poly(p)     => BrepEdgeParamRange::new(
                p.params.first().copied().unwrap_or(0.0),
                p.params.last().copied().unwrap_or(1.0),
            ),
        }
    }

    pub fn is_3d(&self) -> bool { self.kind().is_3d() }
    pub fn is_on_surface(&self) -> bool { self.kind().is_on_surface() }
}

/// BRep edge topology: holds list of curve representations.
/// occt: BRep_TEdge
#[derive(Clone, Debug, Default)]
pub struct BrepTEdge {
    pub edge_id: u32,
    pub tolerance: f64,
    pub same_parameter: bool,
    pub same_range: bool,
    pub degenerated: bool,
    pub representations: Vec<BrepCurveRepresentation>,
}

impl BrepTEdge {
    pub fn new(edge_id: u32) -> Self {
        Self {
            edge_id,
            tolerance: 1e-7,
            same_parameter: true,
            same_range: true,
            degenerated: false,
            representations: Vec::new(),
        }
    }

    pub fn add(&mut self, rep: BrepCurveRepresentation) { self.representations.push(rep); }

    pub fn curve_3d(&self) -> Option<&BrepCurve3D> {
        self.representations.iter().find_map(|r| {
            if let BrepCurveRepresentation::Curve3D(c) = r { Some(c) } else { None }
        })
    }

    pub fn curves_on_surface(&self, surface_id: u32) -> Vec<&BrepCurveOnSurface> {
        self.representations.iter().filter_map(|r| {
            if let BrepCurveRepresentation::OnSurface(c) = r {
                if c.surface_id == surface_id { Some(c) } else { None }
            } else {
                None
            }
        }).collect()
    }

    pub fn nb_representations(&self) -> usize { self.representations.len() }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(0.0); }
    pub fn is_degenerated(&self) -> bool { self.degenerated }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_range() {
        let r = BrepEdgeParamRange::new(0.0, 2.0);
        assert!((r.length() - 2.0).abs() < 1e-10);
        assert!(r.contains(1.0));
        assert!(!r.contains(3.0));
        let rev = r.reversed();
        assert!((rev.first - 2.0).abs() < 1e-10);
    }

    #[test]
    fn curve_rep_kind() {
        let c3 = BrepCurveRepresentation::Curve3D(BrepCurve3D::new(1, 0.0, 1.0));
        assert!(c3.is_3d());
        assert!(!c3.is_on_surface());

        let mut cos = BrepCurveOnSurface::new(2, 5);
        cos.set_seam(true);
        let rep = BrepCurveRepresentation::OnSurface(cos);
        assert_eq!(rep.kind(), BrepCurveRepKind::CurveOnClosedSurface);
    }

    #[test]
    fn tedge_add_retrieve() {
        let mut e = BrepTEdge::new(10);
        e.add(BrepCurveRepresentation::Curve3D(BrepCurve3D::new(1, 0.0, 5.0)));
        e.add(BrepCurveRepresentation::OnSurface(BrepCurveOnSurface::new(2, 7)));
        assert_eq!(e.nb_representations(), 2);
        assert!(e.curve_3d().is_some());
        assert_eq!(e.curves_on_surface(7).len(), 1);
        assert_eq!(e.curves_on_surface(99).len(), 0);
    }

    #[test]
    fn poly_curve() {
        let mut p = BrepPolyCurve::new(3);
        p.add_node([0.0, 0.0], 0.0);
        p.add_node([1.0, 0.5], 0.5);
        p.add_node([2.0, 1.0], 1.0);
        p.set_deflection(0.001);
        assert_eq!(p.nb_nodes(), 3);
        assert!((p.deflection - 0.001).abs() < 1e-10);
        let rep = BrepCurveRepresentation::Poly(p);
        let r = rep.range();
        assert!((r.first - 0.0).abs() < 1e-10);
        assert!((r.last - 1.0).abs() < 1e-10);
    }

    #[test]
    fn tedge_tolerance() {
        let mut e = BrepTEdge::new(1);
        e.set_tolerance(1e-5);
        assert!((e.tolerance - 1e-5).abs() < 1e-15);
        e.degenerated = true;
        assert!(e.is_degenerated());
    }
}
