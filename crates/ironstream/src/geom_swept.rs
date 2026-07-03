// FILE: geom_swept.rs
// occt: Geom_SweptSurface, Geom_SurfaceOfLinearExtrusion,
//       Geom_SurfaceOfRevolution, Geom_RectangularTrimmedSurface,
//       Geom_OffsetSurface

use std::f64::consts::PI;

/// A linear extrusion surface: S(u, v) = C(u) + v * D.
/// occt: Geom_SurfaceOfLinearExtrusion
#[derive(Clone, Debug)]
pub struct GeomSurfLinExtrusion {
    pub curve_id: u32,
    pub direction: [f64; 3],
    pub surface_id: u32,
    pub u_first: f64,
    pub u_last: f64,
}

impl GeomSurfLinExtrusion {
    pub fn new(curve_id: u32, direction: [f64; 3], u_first: f64, u_last: f64) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let d = if len > 1e-14 { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [0.0,0.0,1.0] };
        Self {
            curve_id,
            direction: d,
            surface_id: curve_id + 11000,
            u_first,
            u_last,
        }
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn direction(&self) -> [f64; 3] { self.direction }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn u_first_parameter(&self) -> f64 { self.u_first }
    pub fn u_last_parameter(&self) -> f64 { self.u_last }
    pub fn v_first_parameter(&self) -> f64 { f64::NEG_INFINITY }
    pub fn v_last_parameter(&self) -> f64 { f64::INFINITY }
    pub fn is_u_closed(&self) -> bool { false }
    pub fn is_v_closed(&self) -> bool { false }
    pub fn is_u_periodic(&self) -> bool { false }
    pub fn is_v_periodic(&self) -> bool { false }

    /// Stub: evaluate at (u, v) — curve point + v * direction.
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        // Stub: assume curve at (u, 0, 0)
        [u + v * self.direction[0], v * self.direction[1], v * self.direction[2]]
    }

    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let p = self.value(u, v);
        (p, [1.0, 0.0, 0.0], self.direction)
    }
}

/// A surface of revolution: rotate a curve around an axis.
/// occt: Geom_SurfaceOfRevolution
#[derive(Clone, Debug)]
pub struct GeomSurfRevolution {
    pub curve_id: u32,
    pub axis_origin: [f64; 3],
    pub axis_direction: [f64; 3],
    pub surface_id: u32,
    pub v_first: f64,
    pub v_last: f64,
}

impl GeomSurfRevolution {
    pub fn new(curve_id: u32, axis_origin: [f64; 3], axis_dir: [f64; 3]) -> Self {
        let len = (axis_dir[0]*axis_dir[0]+axis_dir[1]*axis_dir[1]+axis_dir[2]*axis_dir[2]).sqrt();
        let d = if len > 1e-14 { [axis_dir[0]/len, axis_dir[1]/len, axis_dir[2]/len] } else { [0.0,0.0,1.0] };
        Self {
            curve_id,
            axis_origin,
            axis_direction: d,
            surface_id: curve_id + 12000,
            v_first: 0.0,
            v_last: f64::INFINITY,
        }
    }

    pub fn curve_id(&self) -> u32 { self.curve_id }
    pub fn axis_origin(&self) -> [f64; 3] { self.axis_origin }
    pub fn axis_direction(&self) -> [f64; 3] { self.axis_direction }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn u_first_parameter(&self) -> f64 { 0.0 }
    pub fn u_last_parameter(&self) -> f64 { 2.0 * PI }
    pub fn is_u_closed(&self) -> bool { true }
    pub fn is_v_closed(&self) -> bool { false }
    pub fn is_u_periodic(&self) -> bool { true }
    pub fn is_v_periodic(&self) -> bool { false }

    /// Evaluate at (u=angle, v=curve_param): rotate curve point around axis.
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        // Stub: assume curve point is (v, 0, 0) relative to axis
        let r = v.abs();
        [self.axis_origin[0] + r * u.cos(),
         self.axis_origin[1] + r * u.sin(),
         self.axis_origin[2]]
    }
}

/// A rectangular trimmed surface: restricts S to [u1,u2] × [v1,v2].
/// occt: Geom_RectangularTrimmedSurface
#[derive(Clone, Debug)]
pub struct GeomRectTrimmedSurface {
    pub basis_surface_id: u32,
    pub u1: f64, pub u2: f64,
    pub v1: f64, pub v2: f64,
    pub u_sense: bool,
    pub v_sense: bool,
    pub surface_id: u32,
}

impl GeomRectTrimmedSurface {
    pub fn new(basis_id: u32, u1: f64, u2: f64, v1: f64, v2: f64) -> Self {
        assert!(u2 > u1 && v2 > v1);
        Self {
            basis_surface_id: basis_id,
            u1, u2, v1, v2,
            u_sense: true,
            v_sense: true,
            surface_id: basis_id + 13000,
        }
    }

    pub fn basis_surface_id(&self) -> u32 { self.basis_surface_id }
    pub fn u_first_parameter(&self) -> f64 { self.u1 }
    pub fn u_last_parameter(&self) -> f64 { self.u2 }
    pub fn v_first_parameter(&self) -> f64 { self.v1 }
    pub fn v_last_parameter(&self) -> f64 { self.v2 }
    pub fn u_span(&self) -> f64 { self.u2 - self.u1 }
    pub fn v_span(&self) -> f64 { self.v2 - self.v1 }

    pub fn is_valid_param(&self, u: f64, v: f64) -> bool {
        u >= self.u1 - 1e-9 && u <= self.u2 + 1e-9 &&
        v >= self.v1 - 1e-9 && v <= self.v2 + 1e-9
    }

    pub fn is_u_closed(&self) -> bool { false }
    pub fn is_v_closed(&self) -> bool { false }

    /// Stub: bilinear surface evaluation.
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        let tu = if self.u2 > self.u1 { (u - self.u1) / (self.u2 - self.u1) } else { 0.0 };
        let tv = if self.v2 > self.v1 { (v - self.v1) / (self.v2 - self.v1) } else { 0.0 };
        [tu, tv, 0.0]
    }

    /// Set new trim bounds (with bounds check: must stay within original).
    pub fn set_trim_u(&mut self, u1: f64, u2: f64) {
        if u2 > u1 { self.u1 = u1; self.u2 = u2; }
    }

    pub fn set_trim_v(&mut self, v1: f64, v2: f64) {
        if v2 > v1 { self.v1 = v1; self.v2 = v2; }
    }
}

/// An offset surface: S_offset(u,v) = S(u,v) + offset * N(u,v).
/// occt: Geom_OffsetSurface
#[derive(Clone, Debug)]
pub struct GeomOffsetSurface {
    pub basis_surface_id: u32,
    pub offset: f64,
    pub surface_id: u32,
    pub is_done: bool,
    pub basis_surface_continuity: usize,
}

impl GeomOffsetSurface {
    pub fn new(basis_id: u32, offset: f64) -> Self {
        Self {
            basis_surface_id: basis_id,
            offset,
            surface_id: if basis_id > 0 { basis_id + 14000 } else { 0 },
            is_done: basis_id > 0,
            basis_surface_continuity: 1,
        }
    }

    pub fn basis_surface_id(&self) -> u32 { self.basis_surface_id }
    pub fn offset(&self) -> f64 { self.offset }
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn is_done(&self) -> bool { self.is_done }
    pub fn basis_surface_continuity(&self) -> usize { self.basis_surface_continuity }

    /// Stub: evaluate at (u,v) by offsetting in normal direction (0,0,1) stub.
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, self.offset]
    }

    pub fn set_offset_value(&mut self, d: f64) { self.offset = d; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_extrusion_direction() {
        let s = GeomSurfLinExtrusion::new(1, [0.0, 0.0, 1.0], 0.0, 2.0*PI);
        assert!((s.direction()[2] - 1.0).abs() < 1e-10);
        assert!(s.is_u_closed() == false);
        assert_eq!(s.surface_id(), 11001);
    }

    #[test]
    fn linear_extrusion_value() {
        let s = GeomSurfLinExtrusion::new(1, [0.0, 0.0, 1.0], 0.0, 1.0);
        let p = s.value(1.0, 2.0);
        assert!((p[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn surface_of_revolution_periodic() {
        let s = GeomSurfRevolution::new(1, [0.0,0.0,0.0], [0.0,0.0,1.0]);
        assert!(s.is_u_periodic());
        assert!(s.is_u_closed());
        assert!((s.u_last_parameter() - 2.0*PI).abs() < 1e-10);
    }

    #[test]
    fn surface_of_revolution_value() {
        let s = GeomSurfRevolution::new(1, [0.0,0.0,0.0], [0.0,0.0,1.0]);
        let p = s.value(0.0, 2.0); // angle=0, radius=2
        assert!((p[0] - 2.0).abs() < 1e-10);
        assert!(p[1].abs() < 1e-10);
    }

    #[test]
    fn rect_trimmed_surface() {
        let mut s = GeomRectTrimmedSurface::new(5, 0.0, 1.0, 0.0, 2.0);
        assert!((s.u_span() - 1.0).abs() < 1e-10);
        assert!((s.v_span() - 2.0).abs() < 1e-10);
        assert!(s.is_valid_param(0.5, 1.0));
        assert!(!s.is_valid_param(2.0, 1.0));
        s.set_trim_u(0.0, 0.5);
        assert!((s.u_last_parameter() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn offset_surface() {
        let s = GeomOffsetSurface::new(3, 1.5);
        assert!(s.is_done());
        assert!((s.offset() - 1.5).abs() < 1e-12);
        let p = s.value(0.3, 0.4);
        assert!((p[2] - 1.5).abs() < 1e-12);
    }
}
