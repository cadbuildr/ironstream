// FILE: hlrb_rep_surface_tool.rs
// occt: HLRBRep_SurfaceTool

use crate::hlrb_rep_surface::HLRBRepSurface;

/// Tool for extracting data from HLRBRep_Surface.
/// Provides static methods for parameter access and geometric queries.
pub struct HLRBRepSurfaceTool;

impl HLRBRepSurfaceTool {
    pub fn first_u_parameter(surf: &HLRBRepSurface) -> f64 {
        surf.first_u_parameter()
    }

    pub fn first_v_parameter(surf: &HLRBRepSurface) -> f64 {
        surf.first_v_parameter()
    }

    pub fn last_u_parameter(surf: &HLRBRepSurface) -> f64 {
        surf.last_u_parameter()
    }

    pub fn last_v_parameter(surf: &HLRBRepSurface) -> f64 {
        surf.last_v_parameter()
    }

    pub fn nb_u_intervals(surf: &HLRBRepSurface, _sh: i32) -> i32 {
        surf.nb_u_intervals(_sh)
    }

    pub fn nb_v_intervals(surf: &HLRBRepSurface, _sh: i32) -> i32 {
        surf.nb_v_intervals(_sh)
    }

    pub fn is_u_closed(surf: &HLRBRepSurface) -> bool {
        surf.is_u_closed()
    }

    pub fn is_v_closed(surf: &HLRBRepSurface) -> bool {
        surf.is_v_closed()
    }

    pub fn is_u_periodic(surf: &HLRBRepSurface) -> bool {
        surf.is_u_periodic()
    }

    pub fn u_period(surf: &HLRBRepSurface) -> f64 {
        surf.u_period()
    }

    pub fn is_v_periodic(surf: &HLRBRepSurface) -> bool {
        surf.is_v_periodic()
    }

    pub fn v_period(surf: &HLRBRepSurface) -> f64 {
        surf.v_period()
    }

    pub fn value(surf: &HLRBRepSurface, u: f64, v: f64) -> (f64, f64, f64) {
        surf.value(u, v)
    }

    pub fn d0(surf: &HLRBRepSurface, u: f64, v: f64) -> (f64, f64, f64) {
        surf.d0(u, v)
    }

    pub fn d1(
        surf: &HLRBRepSurface,
        u: f64,
        v: f64,
    ) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        surf.d1(u, v)
    }

    pub fn get_type(surf: &HLRBRepSurface) -> i32 {
        surf.get_type()
    }

    pub fn u_resolution(surf: &HLRBRepSurface, r3d: f64) -> f64 {
        r3d
    }

    pub fn v_resolution(surf: &HLRBRepSurface, r3d: f64) -> f64 {
        r3d
    }

    pub fn nb_samples_u(surf: &HLRBRepSurface) -> i32 {
        let u1 = surf.first_u_parameter();
        let u2 = surf.last_u_parameter();
        Self::nb_samples_u_range(surf, u1, u2)
    }

    pub fn nb_samples_v(surf: &HLRBRepSurface) -> i32 {
        let v1 = surf.first_v_parameter();
        let v2 = surf.last_v_parameter();
        Self::nb_samples_v_range(surf, v1, v2)
    }

    pub fn nb_samples_u_range(surf: &HLRBRepSurface, _u1: f64, _u2: f64) -> i32 {
        10
    }

    pub fn nb_samples_v_range(surf: &HLRBRepSurface, _v1: f64, _v2: f64) -> i32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_u_parameter() {
        let mut s = HLRBRepSurface::new();
        s.set_first_u(0.5);
        assert_eq!(HLRBRepSurfaceTool::first_u_parameter(&s), 0.5);
    }

    #[test]
    fn test_last_v_parameter() {
        let mut s = HLRBRepSurface::new();
        s.set_last_v(3.14);
        assert_eq!(HLRBRepSurfaceTool::last_v_parameter(&s), 3.14);
    }

    #[test]
    fn test_is_closed() {
        let mut s = HLRBRepSurface::new();
        s.set_u_closed(true);
        assert!(HLRBRepSurfaceTool::is_u_closed(&s));
        assert!(!HLRBRepSurfaceTool::is_v_closed(&s));
    }

    #[test]
    fn test_nb_samples() {
        let s = HLRBRepSurface::new();
        assert_eq!(HLRBRepSurfaceTool::nb_samples_u(&s), 10);
        assert_eq!(HLRBRepSurfaceTool::nb_samples_v(&s), 10);
    }
}
