// FILE: b_rep_blend_app_func_rst_rst.rs
// occt: BRepBlend_AppFuncRstRst

#[derive(Clone, Debug)]
pub struct BRepBlendAppFuncRstRst {
    surface_count: i32,
}

impl BRepBlendAppFuncRstRst {
    pub fn new() -> Self {
        BRepBlendAppFuncRstRst {
            surface_count: 0,
        }
    }

    pub fn surface_count(&self) -> i32 {
        self.surface_count
    }
}

impl Default for BRepBlendAppFuncRstRst {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let func = BRepBlendAppFuncRstRst::new();
        assert_eq!(func.surface_count(), 0);
    }
}
