// FILE: b_rep_blend_app_surface.rs
// occt: BRepBlend_AppSurface

#[derive(Clone, Debug)]
pub struct BRepBlendAppSurface {
    nb_poles_u: i32,
    nb_poles_v: i32,
}

impl BRepBlendAppSurface {
    pub fn new() -> Self {
        BRepBlendAppSurface {
            nb_poles_u: 0,
            nb_poles_v: 0,
        }
    }

    pub fn nb_poles_u(&self) -> i32 {
        self.nb_poles_u
    }

    pub fn nb_poles_v(&self) -> i32 {
        self.nb_poles_v
    }

    pub fn set_nb_poles(&mut self, u: i32, v: i32) {
        self.nb_poles_u = u;
        self.nb_poles_v = v;
    }
}

impl Default for BRepBlendAppSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let surf = BRepBlendAppSurface::new();
        assert_eq!(surf.nb_poles_u(), 0);
    }

    #[test]
    fn test_set_poles() {
        let mut surf = BRepBlendAppSurface::new();
        surf.set_nb_poles(5, 6);
        assert_eq!(surf.nb_poles_u(), 5);
        assert_eq!(surf.nb_poles_v(), 6);
    }
}
