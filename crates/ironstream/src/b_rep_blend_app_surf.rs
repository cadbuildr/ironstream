// FILE: b_rep_blend_app_surf.rs
// occt: BRepBlend_AppSurf

#[derive(Clone, Debug)]
pub struct BRepBlendAppSurf {
    degree_u: i32,
    degree_v: i32,
}

impl BRepBlendAppSurf {
    pub fn new() -> Self {
        BRepBlendAppSurf {
            degree_u: 3,
            degree_v: 3,
        }
    }

    pub fn degree_u(&self) -> i32 {
        self.degree_u
    }

    pub fn degree_v(&self) -> i32 {
        self.degree_v
    }
}

impl Default for BRepBlendAppSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let surf = BRepBlendAppSurf::new();
        assert_eq!(surf.degree_u(), 3);
        assert_eq!(surf.degree_v(), 3);
    }
}
