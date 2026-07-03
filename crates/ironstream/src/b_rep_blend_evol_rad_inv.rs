// FILE: b_rep_blend_evol_rad_inv.rs
// occt: BRepBlend_EvolRadInv

pub use crate::blend_func_evol_rad_inv::BlendFuncEvolRadInv as BRepBlendEvolRadInv;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendEvolRadInv>();
    }
}
