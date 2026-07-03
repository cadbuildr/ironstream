// FILE: b_rep_blend_curv_point_rad_inv.rs
// occt: BRepBlend_CurvPointRadInv

pub use crate::blend_func_const_rad_inv::BlendFuncConstRadInv as BRepBlendCurvPointRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendCurvPointRadInv>();
    }
}
