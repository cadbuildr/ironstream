// FILE: crates/ironstream/src/geom_surface_tool.rs

// occt: GeomAbs_SurfaceType
/// Surface type discriminant, mirroring OpenCascade's `GeomAbs_SurfaceType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeomSurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    RevolutionSurface,
    ExtrudedSurface,
    OffsetSurface,
    OtherSurface,
}

// occt: GeomAdaptor_Surface info struct
/// Lightweight descriptor of a surface's parametric domain and continuity,
/// mirroring the bookkeeping portion of `GeomAdaptor_Surface`.
#[derive(Debug, Clone, Copy)]
pub struct GeomSurfaceInfo {
    surface_type: GeomSurfaceType,
    u_first: f64,
    u_last: f64,
    v_first: f64,
    v_last: f64,
    continuity: u8,
}

impl GeomSurfaceInfo {
    /// Construct a new `GeomSurfaceInfo`.
    ///
    /// `continuity` defaults to 0 (C0).  Use [`set_continuity`](Self::set_continuity)
    /// to override after construction.
    pub fn new(
        surface_type: GeomSurfaceType,
        u_first: f64,
        u_last: f64,
        v_first: f64,
        v_last: f64,
    ) -> Self {
        Self {
            surface_type,
            u_first,
            u_last,
            v_first,
            v_last,
            continuity: 0,
        }
    }

    /// Return the surface type.
    pub fn surface_type(&self) -> GeomSurfaceType {
        self.surface_type
    }

    /// First parameter in the U direction.
    pub fn u_first(&self) -> f64 {
        self.u_first
    }

    /// Last parameter in the U direction.
    pub fn u_last(&self) -> f64 {
        self.u_last
    }

    /// First parameter in the V direction.
    pub fn v_first(&self) -> f64 {
        self.v_first
    }

    /// Last parameter in the V direction.
    pub fn v_last(&self) -> f64 {
        self.v_last
    }

    /// Continuity level (0 = C0, 1 = C1, …).
    pub fn continuity(&self) -> u8 {
        self.continuity
    }

    /// Set the continuity level.
    pub fn set_continuity(&mut self, c: u8) {
        self.continuity = c;
    }

    // occt: GeomAdaptor_Surface::IsUClosed
    /// Returns `false` in this stub implementation.
    pub fn is_u_closed(&self) -> bool {
        false
    }

    // occt: GeomAdaptor_Surface::IsVClosed
    /// Returns `false` in this stub implementation.
    pub fn is_v_closed(&self) -> bool {
        false
    }

    // occt: GeomAdaptor_Surface::IsUPeriodic
    /// Returns `false` in this stub implementation.
    pub fn is_u_periodic(&self) -> bool {
        false
    }

    // occt: GeomAdaptor_Surface::IsVPeriodic
    /// Returns `false` in this stub implementation.
    pub fn is_v_periodic(&self) -> bool {
        false
    }
}

// occt: surface evaluation at (u,v)
/// Evaluates a surface at parametric coordinates `(u, v)`, mirroring the
/// evaluation methods of `GeomAdaptor_Surface`.
///
/// The evaluation strategy is selected by the surface type stored in
/// the inner [`GeomSurfaceInfo`]:
///
/// - **Plane** : `value(u, v) = [u, v, 0.0]`
/// - **Cylinder**: `value(u, v) = [cos(u)*v, sin(u)*v, 0.0]`
/// - All other types fall back to the same formula as Plane.
#[derive(Debug, Clone, Copy)]
pub struct GeomSurfaceEvaluator {
    surface_info: GeomSurfaceInfo,
}

impl GeomSurfaceEvaluator {
    /// Construct from a [`GeomSurfaceInfo`] descriptor.
    pub fn new(info: GeomSurfaceInfo) -> Self {
        Self { surface_info: info }
    }

    /// Return a reference to the underlying surface descriptor.
    pub fn surface_info(&self) -> &GeomSurfaceInfo {
        &self.surface_info
    }

    // occt: GeomAdaptor_Surface::Value
    /// Point on the surface at `(u, v)`.
    ///
    /// - Plane    → `[u, v, 0.0]`
    /// - Cylinder → `[cos(u)*v, sin(u)*v, 0.0]`
    /// - Others   → same as Plane (stub)
    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        match self.surface_info.surface_type {
            GeomSurfaceType::Cylinder => [u.cos() * v, u.sin() * v, 0.0],
            _ => [u, v, 0.0],
        }
    }

    // occt: GeomAdaptor_Surface::D1
    /// Point and first-order partial derivatives at `(u, v)`.
    ///
    /// Returns `(point, d_du, d_dv)`.
    ///
    /// - **Plane**: `d_du = [1, 0, 0]`, `d_dv = [0, 1, 0]`
    /// - **Cylinder**: derivatives of `[cos(u)*v, sin(u)*v, 0]`
    ///   - `d_du = [-sin(u)*v, cos(u)*v, 0]`
    ///   - `d_dv = [cos(u), sin(u), 0]`
    pub fn d1(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let pt = self.value(u, v);
        match self.surface_info.surface_type {
            GeomSurfaceType::Cylinder => {
                let d_du = [-u.sin() * v, u.cos() * v, 0.0];
                let d_dv = [u.cos(), u.sin(), 0.0];
                (pt, d_du, d_dv)
            }
            _ => {
                let d_du = [1.0, 0.0, 0.0];
                let d_dv = [0.0, 1.0, 0.0];
                (pt, d_du, d_dv)
            }
        }
    }

    // occt: GeomAdaptor_Surface::Normal
    /// Unit surface normal at `(u, v)`, computed as the cross product of the
    /// partial derivatives `d_du × d_dv`, normalised to unit length.
    ///
    /// Returns `[0, 0, 1]` for degenerate cases (zero-length cross product).
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let (_, du, dv) = self.d1(u, v);
        let nx = du[1] * dv[2] - du[2] * dv[1];
        let ny = du[2] * dv[0] - du[0] * dv[2];
        let nz = du[0] * dv[1] - du[1] * dv[0];
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len < f64::EPSILON {
            [0.0, 0.0, 1.0]
        } else {
            [nx / len, ny / len, nz / len]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPS: f64 = 1e-12;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < EPS, "expected {a} ≈ {b}, diff={}", (a - b).abs());
    }

    fn vec_near(a: [f64; 3], b: [f64; 3]) {
        near(a[0], b[0]);
        near(a[1], b[1]);
        near(a[2], b[2]);
    }

    // ── GeomSurfaceType ──────────────────────────────────────────────────────

    #[test]
    fn surface_type_variants_are_distinct() {
        let all = [
            GeomSurfaceType::Plane,
            GeomSurfaceType::Cylinder,
            GeomSurfaceType::Cone,
            GeomSurfaceType::Sphere,
            GeomSurfaceType::Torus,
            GeomSurfaceType::BezierSurface,
            GeomSurfaceType::BSplineSurface,
            GeomSurfaceType::RevolutionSurface,
            GeomSurfaceType::ExtrudedSurface,
            GeomSurfaceType::OffsetSurface,
            GeomSurfaceType::OtherSurface,
        ];
        for i in 0..all.len() {
            for j in 0..all.len() {
                if i == j {
                    assert_eq!(all[i], all[j]);
                } else {
                    assert_ne!(all[i], all[j]);
                }
            }
        }
    }

    #[test]
    fn surface_type_is_copy() {
        let t = GeomSurfaceType::Sphere;
        let t2 = t;
        assert_eq!(t, t2);
    }

    #[test]
    fn surface_type_debug() {
        assert_eq!(format!("{:?}", GeomSurfaceType::Plane), "Plane");
        assert_eq!(format!("{:?}", GeomSurfaceType::Cylinder), "Cylinder");
        assert_eq!(format!("{:?}", GeomSurfaceType::OtherSurface), "OtherSurface");
    }

    // ── GeomSurfaceInfo ──────────────────────────────────────────────────────

    #[test]
    fn info_new_stores_fields() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, -1.0, 2.0);
        assert_eq!(info.surface_type(), GeomSurfaceType::Plane);
        near(info.u_first(), 0.0);
        near(info.u_last(), 1.0);
        near(info.v_first(), -1.0);
        near(info.v_last(), 2.0);
        assert_eq!(info.continuity(), 0);
    }

    #[test]
    fn info_set_continuity() {
        let mut info = GeomSurfaceInfo::new(GeomSurfaceType::Sphere, 0.0, 2.0 * PI, -PI / 2.0, PI / 2.0);
        assert_eq!(info.continuity(), 0);
        info.set_continuity(2);
        assert_eq!(info.continuity(), 2);
        info.set_continuity(0);
        assert_eq!(info.continuity(), 0);
    }

    #[test]
    fn info_closed_periodic_always_false() {
        for st in [
            GeomSurfaceType::Plane,
            GeomSurfaceType::Cylinder,
            GeomSurfaceType::Sphere,
            GeomSurfaceType::Torus,
        ] {
            let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
            assert!(!info.is_u_closed(), "u_closed should be false for {:?}", st);
            assert!(!info.is_v_closed(), "v_closed should be false for {:?}", st);
            assert!(!info.is_u_periodic(), "u_periodic should be false for {:?}", st);
            assert!(!info.is_v_periodic(), "v_periodic should be false for {:?}", st);
        }
    }

    #[test]
    fn info_is_copy() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cone, 0.0, 1.0, 0.0, 1.0);
        let info2 = info;
        assert_eq!(info.surface_type(), info2.surface_type());
    }

    // ── GeomSurfaceEvaluator — Plane ─────────────────────────────────────────

    #[test]
    fn plane_value_origin() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        vec_near(ev.value(0.0, 0.0), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn plane_value_arbitrary() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 10.0, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        vec_near(ev.value(3.5, -2.0), [3.5, -2.0, 0.0]);
        vec_near(ev.value(-1.0, 7.0), [-1.0, 7.0, 0.0]);
    }

    #[test]
    fn plane_d1_constant_partials() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let (pt, du, dv) = ev.d1(2.0, 3.0);
        vec_near(pt, [2.0, 3.0, 0.0]);
        vec_near(du, [1.0, 0.0, 0.0]);
        vec_near(dv, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn plane_normal_is_z_axis() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        for (u, v) in [(0.0, 0.0), (1.5, -2.3), (-10.0, 7.0)] {
            let n = ev.normal(u, v);
            vec_near(n, [0.0, 0.0, 1.0]);
        }
    }

    #[test]
    fn plane_normal_is_unit_length() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let n = ev.normal(0.5, 0.5);
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        near(len, 1.0);
    }

    // ── GeomSurfaceEvaluator — Cylinder ──────────────────────────────────────

    #[test]
    fn cylinder_value_at_zero() {
        // value(0, v) = [cos(0)*v, sin(0)*v, 0] = [v, 0, 0]
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        vec_near(ev.value(0.0, 1.0), [1.0, 0.0, 0.0]);
        vec_near(ev.value(0.0, 3.0), [3.0, 0.0, 0.0]);
    }

    #[test]
    fn cylinder_value_at_pi_over_2() {
        // value(PI/2, v) = [cos(PI/2)*v, sin(PI/2)*v, 0] = [0, v, 0]
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let pt = ev.value(PI / 2.0, 2.0);
        assert!((pt[0] - 0.0).abs() < 1e-12, "x should be ~0, got {}", pt[0]);
        assert!((pt[1] - 2.0).abs() < 1e-12, "y should be ~2, got {}", pt[1]);
        assert!((pt[2] - 0.0).abs() < 1e-12, "z should be 0, got {}", pt[2]);
    }

    #[test]
    fn cylinder_d1_du_and_dv() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let u = PI / 4.0;
        let v = 2.0;
        let (pt, du, dv) = ev.d1(u, v);
        // point
        vec_near(pt, [u.cos() * v, u.sin() * v, 0.0]);
        // d/du = [-sin(u)*v, cos(u)*v, 0]
        let exp_du = [-u.sin() * v, u.cos() * v, 0.0];
        assert!((du[0] - exp_du[0]).abs() < 1e-12);
        assert!((du[1] - exp_du[1]).abs() < 1e-12);
        assert!((du[2] - exp_du[2]).abs() < 1e-12);
        // d/dv = [cos(u), sin(u), 0]
        let exp_dv = [u.cos(), u.sin(), 0.0];
        assert!((dv[0] - exp_dv[0]).abs() < 1e-12);
        assert!((dv[1] - exp_dv[1]).abs() < 1e-12);
        assert!((dv[2] - exp_dv[2]).abs() < 1e-12);
    }

    #[test]
    fn cylinder_normal_is_unit_length() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        for u in [0.0, PI / 6.0, PI / 2.0, PI, 3.0 * PI / 2.0] {
            let n = ev.normal(u, 1.0);
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-12, "normal not unit at u={u}: len={len}");
        }
    }

    #[test]
    fn cylinder_normal_orthogonal_to_partials() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let u = 0.8;
        let v = 3.0;
        let (_, du, dv) = ev.d1(u, v);
        let n = ev.normal(u, v);
        let dot_du = n[0] * du[0] + n[1] * du[1] + n[2] * du[2];
        let dot_dv = n[0] * dv[0] + n[1] * dv[1] + n[2] * dv[2];
        assert!(dot_du.abs() < 1e-12, "normal not perp to du: dot={dot_du}");
        assert!(dot_dv.abs() < 1e-12, "normal not perp to dv: dot={dot_dv}");
    }

    // ── GeomSurfaceEvaluator — other types fall back to plane formula ─────────

    #[test]
    fn sphere_fallback_value() {
        for st in [
            GeomSurfaceType::Sphere,
            GeomSurfaceType::Torus,
            GeomSurfaceType::Cone,
            GeomSurfaceType::BSplineSurface,
            GeomSurfaceType::OtherSurface,
        ] {
            let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
            let ev = GeomSurfaceEvaluator::new(info);
            let pt = ev.value(2.5, -1.0);
            assert!((pt[0] - 2.5).abs() < 1e-12, "{:?}: x should be 2.5", st);
            assert!((pt[1] + 1.0).abs() < 1e-12, "{:?}: y should be -1.0", st);
            assert!((pt[2] - 0.0).abs() < 1e-12, "{:?}: z should be 0.0", st);
        }
    }

    #[test]
    fn evaluator_surface_info_accessor() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::BezierSurface, -1.0, 1.0, -2.0, 2.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let retrieved = ev.surface_info();
        assert_eq!(retrieved.surface_type(), GeomSurfaceType::BezierSurface);
        near(retrieved.u_first(), -1.0);
        near(retrieved.u_last(), 1.0);
        near(retrieved.v_first(), -2.0);
        near(retrieved.v_last(), 2.0);
    }

    #[test]
    fn evaluator_is_copy() {
        let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let ev2 = ev;
        vec_near(ev.value(1.0, 2.0), ev2.value(1.0, 2.0));
    }
}
