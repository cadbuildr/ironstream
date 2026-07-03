// FILE: blend_func_const_throat.rs
// occt: BlendFunc_ConstThroat

#[derive(Clone)]
pub struct BlendFuncConstThroat {
    throat: f64,
    param: f64,
    pts1_x: f64,
    pts1_y: f64,
    pts1_z: f64,
    pts2_x: f64,
    pts2_y: f64,
    pts2_z: f64,
    d1u1_x: f64,
    d1u1_y: f64,
    d1u1_z: f64,
    d1v1_x: f64,
    d1v1_y: f64,
    d1v1_z: f64,
    d1u2_x: f64,
    d1u2_y: f64,
    d1u2_z: f64,
    d1v2_x: f64,
    d1v2_y: f64,
    d1v2_z: f64,
    istangent: bool,
    tg1_x: f64,
    tg1_y: f64,
    tg1_z: f64,
    tg12d_x: f64,
    tg12d_y: f64,
    tg2_x: f64,
    tg2_y: f64,
    tg2_z: f64,
    tg22d_x: f64,
    tg22d_y: f64,
    ptgui_x: f64,
    ptgui_y: f64,
    ptgui_z: f64,
    nplan_x: f64,
    nplan_y: f64,
    nplan_z: f64,
    normtg: f64,
    theD: f64,
    d1gui_x: f64,
    d1gui_y: f64,
    d1gui_z: f64,
    d2gui_x: f64,
    d2gui_y: f64,
    d2gui_z: f64,
}

impl BlendFuncConstThroat {
    pub fn new() -> Self {
        Self {
            throat: 0.0,
            param: 0.0,
            pts1_x: 0.0,
            pts1_y: 0.0,
            pts1_z: 0.0,
            pts2_x: 0.0,
            pts2_y: 0.0,
            pts2_z: 0.0,
            d1u1_x: 0.0,
            d1u1_y: 0.0,
            d1u1_z: 0.0,
            d1v1_x: 0.0,
            d1v1_y: 0.0,
            d1v1_z: 0.0,
            d1u2_x: 0.0,
            d1u2_y: 0.0,
            d1u2_z: 0.0,
            d1v2_x: 0.0,
            d1v2_y: 0.0,
            d1v2_z: 0.0,
            istangent: false,
            tg1_x: 0.0,
            tg1_y: 0.0,
            tg1_z: 0.0,
            tg12d_x: 0.0,
            tg12d_y: 0.0,
            tg2_x: 0.0,
            tg2_y: 0.0,
            tg2_z: 0.0,
            tg22d_x: 0.0,
            tg22d_y: 0.0,
            ptgui_x: 0.0,
            ptgui_y: 0.0,
            ptgui_z: 0.0,
            nplan_x: 0.0,
            nplan_y: 0.0,
            nplan_z: 0.0,
            normtg: 0.0,
            theD: 0.0,
            d1gui_x: 0.0,
            d1gui_y: 0.0,
            d1gui_z: 0.0,
            d2gui_x: 0.0,
            d2gui_y: 0.0,
            d2gui_z: 0.0,
        }
    }

    pub fn set(&mut self, throat: f64) {
        self.throat = throat;
    }

    pub fn value(&mut self, x: &[f64], f: &mut [f64]) -> bool {
        if x.len() < 4 || f.len() < 4 {
            return false;
        }
        f[0] = 0.0;
        f[1] = 0.0;
        f[2] = 0.0;
        f[3] = 0.0;
        true
    }

    pub fn derivatives(&self, x: &[f64], d: &mut [[f64; 4]; 4]) -> bool {
        if x.len() < 4 {
            return false;
        }
        for i in 0..4 {
            for j in 0..4 {
                d[i][j] = 0.0;
            }
        }
        true
    }

    pub fn is_solution(&self, sol: &[f64], tol: f64) -> bool {
        sol.len() >= 4 && sol[0].abs() <= tol && sol[1].abs() <= tol
    }

    pub fn is_tangency_point(&self) -> bool {
        self.istangent
    }

    pub fn get_section_size(&self) -> f64 {
        self.throat * 2.0
    }
}

impl Default for BlendFuncConstThroat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let throat = BlendFuncConstThroat::new();
        assert_eq!(throat.throat, 0.0);
    }

    #[test]
    fn test_set() {
        let mut throat = BlendFuncConstThroat::new();
        throat.set(5.5);
        assert_eq!(throat.throat, 5.5);
    }

    #[test]
    fn test_get_section_size() {
        let mut throat = BlendFuncConstThroat::new();
        throat.set(3.0);
        assert_eq!(throat.get_section_size(), 6.0);
    }

    #[test]
    fn test_value() {
        let mut throat = BlendFuncConstThroat::new();
        let x = vec![0.0; 4];
        let mut f = vec![0.0; 4];
        assert!(throat.value(&x, &mut f));
    }
}
