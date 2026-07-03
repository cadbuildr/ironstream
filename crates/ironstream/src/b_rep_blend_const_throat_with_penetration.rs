// FILE: b_rep_blend_const_throat_with_penetration.rs
// occt: BRepBlend_ConstThroatWithPenetration

pub use crate::blend_func_const_throat_with_penetration::BlendFuncConstThroatWithPenetration
    as BRepBlendConstThroatWithPenetration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repblend_const_throat_with_penetration_creation() {
        let _ = std::any::type_name::<BRepBlendConstThroatWithPenetration>();
    }
}
