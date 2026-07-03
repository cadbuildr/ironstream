// FILE: b_rep_blend_ruled_inv.rs
// occt: BRepBlend_RuledInv

pub use crate::blend_func_ruled_inv::BlendFuncRuledInv as BRepBlendRuledInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendRuledInv>();
    }
}
