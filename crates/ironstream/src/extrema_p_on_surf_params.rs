// FILE: extrema_p_on_surf_params.rs
// occt: Extrema_POnSurfParams

/// Point on surface with parameters definition.
pub struct ExtremaPOnSurfParams {
    u: f64,
    v: f64,
}

impl ExtremaPOnSurfParams {
    pub fn new() -> Self {
        ExtremaPOnSurfParams { u: 0.0, v: 0.0 }
    }

    pub fn set_uv(&mut self, u: f64, v: f64) {
        self.u = u;
        self.v = v;
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = ExtremaPOnSurfParams::new();
        assert_eq!(p.u(), 0.0);
        assert_eq!(p.v(), 0.0);
    }

    #[test]
    fn test_set_uv() {
        let mut p = ExtremaPOnSurfParams::new();
        p.set_uv(1.5, 2.5);
        assert_eq!(p.u(), 1.5);
        assert_eq!(p.v(), 2.5);
    }
}
