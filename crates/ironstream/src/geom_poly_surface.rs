// FILE: geom_poly_surface.rs
// occt: Geom_UndefinedValue
// occt-ref: Geom_PolygonOnSurface, Geom_SequenceOfSurface

// occt-note: Geom_SequenceOfSurface (represented as a parameterized surface list)
#[derive(Clone, Debug, Default)]
pub struct SequenceOfSurface {
    pub surfaces: Vec<SurfaceHandle>,
}

#[derive(Clone, Debug)]
pub struct SurfaceHandle {
    pub id: usize,
    pub surface_type: String,
}

impl SequenceOfSurface {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, s: SurfaceHandle) { self.surfaces.push(s); }
    pub fn size(&self) -> usize { self.surfaces.len() }
    pub fn value(&self, i: usize) -> Option<&SurfaceHandle> { self.surfaces.get(i.saturating_sub(1)) }
    pub fn clear(&mut self) { self.surfaces.clear(); }
}

// occt-ref: Geom_PolygonOnSurface
/// Represents a polygon embedded on a parametric surface (as UV-polyline).
#[derive(Clone, Debug)]
pub struct PolygonOnSurface {
    pub uv_nodes: Vec<[f64; 2]>,
    pub surface_id: usize,
}

impl PolygonOnSurface {
    pub fn new(surface_id: usize, uv_nodes: Vec<[f64; 2]>) -> Self {
        Self { uv_nodes, surface_id }
    }

    pub fn nb_nodes(&self) -> usize { self.uv_nodes.len() }

    pub fn node(&self, i: usize) -> Option<[f64; 2]> {
        self.uv_nodes.get(i.saturating_sub(1)).copied()
    }
}

// occt-ref: Geom_OffsetSurface // (parametric offset)
/// Surface offset by a constant distance along its normal.
#[derive(Clone, Debug)]
pub struct OffsetSurface {
    pub basis_surface_id: usize,
    pub offset: f64,
}

impl OffsetSurface {
    pub fn new(basis_surface_id: usize, offset: f64) -> Self {
        Self { basis_surface_id, offset }
    }

    /// Returns the offset amount.
    pub fn offset_value(&self) -> f64 { self.offset }
}

// occt-ref: Geom_RectangularTrimmedSurface
/// A surface trimmed to a rectangular UV domain.
#[derive(Clone, Debug)]
pub struct RectangularTrimmedSurface {
    pub basis_surface_id: usize,
    pub u1: f64,
    pub u2: f64,
    pub v1: f64,
    pub v2: f64,
}

impl RectangularTrimmedSurface {
    pub fn new(basis_surface_id: usize, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        let (u1, u2) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
        let (v1, v2) = if v1 <= v2 { (v1, v2) } else { (v2, v1) };
        Self { basis_surface_id, u1, u2, v1, v2 }
    }

    pub fn u_bounds(&self) -> (f64, f64) { (self.u1, self.u2) }
    pub fn v_bounds(&self) -> (f64, f64) { (self.v1, self.v2) }

    pub fn is_u_periodic(&self) -> bool { false }
    pub fn is_v_periodic(&self) -> bool { false }
}

// occt-ref: Geom_SweptSurface // (abstract base for revolution/extrusion)
#[derive(Clone, Debug)]
pub struct SweptSurface {
    pub direction: [f64; 3],
    pub profile_id: usize,
}

impl SweptSurface {
    pub fn new(profile_id: usize, direction: [f64; 3]) -> Self {
        Self { direction, profile_id }
    }

    pub fn direction(&self) -> [f64; 3] { self.direction }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_of_surface() {
        let mut seq = SequenceOfSurface::new();
        seq.append(SurfaceHandle { id: 1, surface_type: "Plane".into() });
        seq.append(SurfaceHandle { id: 2, surface_type: "Cylinder".into() });
        assert_eq!(seq.size(), 2);
        assert_eq!(seq.value(1).unwrap().id, 1);
    }

    #[test]
    fn polygon_on_surface() {
        let uvs = vec![[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,0.0]];
        let p = PolygonOnSurface::new(1, uvs);
        assert_eq!(p.nb_nodes(), 4);
        assert_eq!(p.node(1).unwrap(), [0.0, 0.0]);
    }

    #[test]
    fn offset_surface() {
        let s = OffsetSurface::new(3, 0.5);
        assert!((s.offset_value() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn rectangular_trimmed() {
        let s = RectangularTrimmedSurface::new(1, 0.0, 1.0, 0.0, 2.0);
        assert_eq!(s.u_bounds(), (0.0, 1.0));
        assert_eq!(s.v_bounds(), (0.0, 2.0));
    }

    #[test]
    fn swept_surface_direction() {
        let s = SweptSurface::new(2, [0.0, 0.0, 1.0]);
        assert_eq!(s.direction(), [0.0, 0.0, 1.0]);
    }
}
