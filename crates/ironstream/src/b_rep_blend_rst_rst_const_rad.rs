// FILE: b_rep_blend_rst_rst_const_rad.rs
// occt: BRepBlend_RstRstConstRad

pub use crate::blend_func_const_rad::BlendFuncConstRad as BRepBlendRstRstConstRad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendRstRstConstRad>();
    }
}
