// FILE: b_rep_blend_evol_rad.rs
// occt: BRepBlend_EvolRad

pub use crate::blend_func_evol_rad::BlendFuncEvolRad as BRepBlendEvolRad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendEvolRad>();
    }
}
