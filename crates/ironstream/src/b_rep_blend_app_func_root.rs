// FILE: b_rep_blend_app_func_root.rs
// occt: BRepBlend_AppFuncRoot

#[derive(Clone, Debug)]
pub struct BRepBlendAppFuncRoot {
    order: i32,
}

impl BRepBlendAppFuncRoot {
    pub fn new() -> Self {
        BRepBlendAppFuncRoot { order: 0 }
    }

    pub fn order(&self) -> i32 {
        self.order
    }

    pub fn set_order(&mut self, o: i32) {
        self.order = o;
    }
}

impl Default for BRepBlendAppFuncRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let func = BRepBlendAppFuncRoot::new();
        assert_eq!(func.order(), 0);
    }
}
