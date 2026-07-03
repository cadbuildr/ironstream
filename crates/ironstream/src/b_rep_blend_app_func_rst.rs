// FILE: b_rep_blend_app_func_rst.rs
// occt: BRepBlend_AppFuncRst

#[derive(Clone, Debug)]
pub struct BRepBlendAppFuncRst {
    surface_count: i32,
}

impl BRepBlendAppFuncRst {
    pub fn new() -> Self {
        BRepBlendAppFuncRst {
            surface_count: 0,
        }
    }

    pub fn surface_count(&self) -> i32 {
        self.surface_count
    }
}

impl Default for BRepBlendAppFuncRst {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let func = BRepBlendAppFuncRst::new();
        assert_eq!(func.surface_count(), 0);
    }
}
