// FILE: adapt_surf.rs
// occt: Adaptor3d_Surface, GeomAdaptor_Surface, BRepAdaptor_Surface

/// Type of underlying surface geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    SurfaceOfRevolution,
    SurfaceOfExtrusion,
    OffsetSurface,
    OtherSurface,
}

impl Default for SurfaceType {
    fn default() -> Self { Self::BSplineSurface }
}

// occt: Adaptor3d_Surface — abstract surface adaptor interface
#[derive(Clone, Debug)]
pub struct Adaptor3dSurface {
    pub surface_id: u32,
    pub surf_type: SurfaceType,
    pub u_first: f64,
    pub u_last: f64,
    pub v_first: f64,
    pub v_last: f64,
    pub u_continuity: u8,
    pub v_continuity: u8,
}

impl Adaptor3dSurface {
    pub fn new(surface_id: u32, surf_type: SurfaceType) -> Self {
        Self {
            surface_id,
            surf_type,
            u_first: 0.0,
            u_last: 1.0,
            v_first: 0.0,
            v_last: 1.0,
            u_continuity: 2,
            v_continuity: 2,
        }
    }

    pub fn set_u_bounds(&mut self, first: f64, last: f64) { self.u_first = first; self.u_last = last; }
    pub fn set_v_bounds(&mut self, first: f64, last: f64) { self.v_first = first; self.v_last = last; }

    pub fn u_first(&self) -> f64 { self.u_first }
    pub fn u_last(&self) -> f64 { self.u_last }
    pub fn v_first(&self) -> f64 { self.v_first }
    pub fn v_last(&self) -> f64 { self.v_last }
    pub fn get_type(&self) -> SurfaceType { self.surf_type }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        match self.surf_type {
            SurfaceType::Plane => [u, v, 0.0],
            SurfaceType::Cylinder => [u.cos(), u.sin(), v],
            SurfaceType::Sphere => {
                let r = 1.0;
                [r * u.cos() * v.cos(), r * u.sin() * v.cos(), r * v.sin()]
            }
            _ => [u, v, 0.0],
        }
    }

    pub fn is_u_periodic(&self) -> bool {
        matches!(self.surf_type, SurfaceType::Cylinder | SurfaceType::Sphere | SurfaceType::Torus)
    }

    pub fn is_v_periodic(&self) -> bool {
        matches!(self.surf_type, SurfaceType::Sphere | SurfaceType::Torus)
    }

    pub fn is_u_closed(&self) -> bool { self.is_u_periodic() }
    pub fn is_v_closed(&self) -> bool { self.is_v_periodic() }

    pub fn u_resolution(&self, _tol: f64) -> f64 { 1e-7 }
    pub fn v_resolution(&self, _tol: f64) -> f64 { 1e-7 }

    pub fn trim(&self, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        let mut s = self.clone();
        s.set_u_bounds(u1, u2);
        s.set_v_bounds(v1, v2);
        s
    }
}

// occt: GeomAdaptor_Surface — wraps Geom_Surface as an Adaptor3d_Surface
#[derive(Clone, Debug)]
pub struct GeomAdaptorSurface {
    pub adaptor: Adaptor3dSurface,
    pub geom_surface_id: u32,
}

impl GeomAdaptorSurface {
    pub fn new(geom_surface_id: u32) -> Self {
        Self {
            adaptor: Adaptor3dSurface::new(geom_surface_id, SurfaceType::BSplineSurface),
            geom_surface_id,
        }
    }

    pub fn new_plane(surface_id: u32) -> Self {
        Self {
            adaptor: Adaptor3dSurface::new(surface_id, SurfaceType::Plane),
            geom_surface_id: surface_id,
        }
    }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] { self.adaptor.value(u, v) }
    pub fn get_type(&self) -> SurfaceType { self.adaptor.get_type() }
    pub fn u_first(&self) -> f64 { self.adaptor.u_first() }
    pub fn u_last(&self) -> f64 { self.adaptor.u_last() }
    pub fn v_first(&self) -> f64 { self.adaptor.v_first() }
    pub fn v_last(&self) -> f64 { self.adaptor.v_last() }
}

// occt: BRepAdaptor_Surface — adaptor for a BRep face's underlying surface
#[derive(Clone, Debug)]
pub struct BrepAdaptorSurface {
    pub face_id: u32,
    pub adaptor: Adaptor3dSurface,
    pub face_tolerance: f64,
    pub location_id: u32,
}

impl BrepAdaptorSurface {
    pub fn new(face_id: u32) -> Self {
        Self {
            face_id,
            adaptor: Adaptor3dSurface::new(face_id, SurfaceType::Plane),
            face_tolerance: 1e-7,
            location_id: 0,
        }
    }

    pub fn face(&self) -> u32 { self.face_id }
    pub fn get_type(&self) -> SurfaceType { self.adaptor.get_type() }
    pub fn tolerance(&self) -> f64 { self.face_tolerance }
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] { self.adaptor.value(u, v) }

    pub fn u_first(&self) -> f64 { self.adaptor.u_first() }
    pub fn u_last(&self) -> f64 { self.adaptor.u_last() }
    pub fn v_first(&self) -> f64 { self.adaptor.v_first() }
    pub fn v_last(&self) -> f64 { self.adaptor.v_last() }
    pub fn is_u_periodic(&self) -> bool { self.adaptor.is_u_periodic() }
    pub fn is_v_periodic(&self) -> bool { self.adaptor.is_v_periodic() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptor_plane_value() {
        let s = Adaptor3dSurface::new(1, SurfaceType::Plane);
        let p = s.value(2.0, 3.0);
        assert!((p[0] - 2.0).abs() < 1e-10);
        assert!((p[1] - 3.0).abs() < 1e-10);
        assert!((p[2] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn adaptor_cylinder_periodic() {
        let s = Adaptor3dSurface::new(1, SurfaceType::Cylinder);
        assert!(s.is_u_periodic());
        assert!(!s.is_v_periodic());
    }

    #[test]
    fn adaptor_trim() {
        let s = Adaptor3dSurface::new(1, SurfaceType::BSplineSurface);
        let t = s.trim(0.0, 2.0, 0.0, 3.0);
        assert!((t.u_last() - 2.0).abs() < 1e-10);
        assert!((t.v_last() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn geom_adaptor_surface_type() {
        let g = GeomAdaptorSurface::new_plane(5);
        assert_eq!(g.get_type(), SurfaceType::Plane);
        let p = g.value(1.0, 2.0);
        assert!((p[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn brep_adaptor_surface_face() {
        let b = BrepAdaptorSurface::new(99);
        assert_eq!(b.face(), 99);
        assert!((b.tolerance() - 1e-7).abs() < 1e-15);
    }

    #[test]
    fn adaptor_sphere_periodic() {
        let s = Adaptor3dSurface::new(1, SurfaceType::Sphere);
        assert!(s.is_u_periodic());
        assert!(s.is_v_periodic());
    }
}
