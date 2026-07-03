// FILE: b_rep_blend_surf_curv_evol_rad_inv.rs
// occt: BRepBlend_SurfCurvEvolRadInv

pub use crate::blend_func_evol_rad_inv::BlendFuncEvolRadInv as BRepBlendSurfCurvEvolRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfCurvEvolRadInv>();
    }
}
