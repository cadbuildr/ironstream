// FILE: b_rep_blend_surf_point_evol_rad_inv.rs
// occt: BRepBlend_SurfPointEvolRadInv

pub use crate::blend_func_evol_rad_inv::BlendFuncEvolRadInv as BRepBlendSurfPointEvolRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfPointEvolRadInv>();
    }
}
