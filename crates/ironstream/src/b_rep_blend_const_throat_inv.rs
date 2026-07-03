// FILE: b_rep_blend_const_throat_inv.rs
// occt: BRepBlend_ConstThroatInv

// BRepBlend_ConstThroatInv is a typedef to BlendFunc_ConstThroatInv
// Re-exporting the actual implementation from BlendFunc
pub use crate::blend_func_const_throat_inv::BlendFuncConstThroatInv as BRepBlendConstThroatInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repblend_const_throat_inv_creation() {
        // BRepBlend_ConstThroatInv is a typedef, so we just verify it compiles
        // and can be created through the underlying BlendFunc implementation
        let _ = std::any::type_name::<BRepBlendConstThroatInv>();
    }
}
