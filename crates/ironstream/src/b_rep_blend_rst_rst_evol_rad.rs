// FILE: b_rep_blend_rst_rst_evol_rad.rs
// occt: BRepBlend_RstRstEvolRad

pub use crate::blend_func_evol_rad::BlendFuncEvolRad as BRepBlendRstRstEvolRad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendRstRstEvolRad>();
    }
}
