// FILE: b_rep_blend_const_throat_with_penetration_inv.rs
// occt: BRepBlend_ConstThroatWithPenetrationInv

pub use crate::blend_func_const_throat_with_penetration_inv::
    BlendFuncConstThroatWithPenetrationInv as BRepBlendConstThroatWithPenetrationInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendConstThroatWithPenetrationInv>();
    }
}
