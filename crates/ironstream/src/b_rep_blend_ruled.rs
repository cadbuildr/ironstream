// FILE: b_rep_blend_ruled.rs
// occt: BRepBlend_Ruled

pub use crate::blend_func_ruled_inv::BlendFuncRuledInv as BRepBlendRuled;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendRuled>();
    }
}
