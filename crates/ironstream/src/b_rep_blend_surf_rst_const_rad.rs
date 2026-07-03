// FILE: b_rep_blend_surf_rst_const_rad.rs
// occt: BRepBlend_SurfRstConstRad

pub use crate::blend_func_const_rad::BlendFuncConstRad as BRepBlendSurfRstConstRad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfRstConstRad>();
    }
}
