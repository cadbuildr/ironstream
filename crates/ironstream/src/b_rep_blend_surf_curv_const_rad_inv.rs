// FILE: b_rep_blend_surf_curv_const_rad_inv.rs
// occt: BRepBlend_SurfCurvConstRadInv

pub use crate::blend_func_const_rad_inv::BlendFuncConstRadInv as BRepBlendSurfCurvConstRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfCurvConstRadInv>();
    }
}
