// FILE: b_rep_blend_blend_tool.rs
// occt: BRepBlend_BlendTool

pub struct BRepBlendBlendTool;

impl BRepBlendBlendTool {
    pub fn nb_samples_u(_surface_type: i32, _u1: f64, _u2: f64) -> i32 {
        10
    }

    pub fn nb_samples_v(_surface_type: i32, _v1: f64, _v2: f64) -> i32 {
        10
    }

    pub fn surface_type(_surface_type: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_samples() {
        assert_eq!(BRepBlendBlendTool::nb_samples_u(0, 0.0, 1.0), 10);
        assert_eq!(BRepBlendBlendTool::nb_samples_v(0, 0.0, 1.0), 10);
    }
}
