// FILE: b_rep_blend_surf_point_const_rad_inv.rs
// occt: BRepBlend_SurfPointConstRadInv

pub use crate::blend_func_const_rad_inv::BlendFuncConstRadInv as BRepBlendSurfPointConstRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfPointConstRadInv>();
    }
}
