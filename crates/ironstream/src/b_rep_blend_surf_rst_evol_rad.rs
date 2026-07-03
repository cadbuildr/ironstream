// FILE: b_rep_blend_surf_rst_evol_rad.rs
// occt: BRepBlend_SurfRstEvolRad

pub use crate::blend_func_evol_rad::BlendFuncEvolRad as BRepBlendSurfRstEvolRad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _ = std::any::type_name::<BRepBlendSurfRstEvolRad>();
    }
}
