// FILE: b_rep_prim_one_axis.rs
// occt: BRepPrim_OneAxis

/// Algorithm to build primitives with one axis of revolution.
#[derive(Debug, Clone)]
pub struct BRepPrimOneAxis {
    my_angle: f64,
    my_vmin: f64,
    my_vmax: f64,
}

impl BRepPrimOneAxis {
    pub fn new() -> Self {
        BRepPrimOneAxis {
            my_angle: std::f64::consts::TAU,
            my_vmin: 0.0,
            my_vmax: 1.0,
        }
    }

    pub fn angle(&self) -> f64 {
        self.my_angle
    }

    pub fn set_angle(&mut self, a: f64) {
        self.my_angle = a;
    }

    pub fn vmin(&self) -> f64 {
        self.my_vmin
    }

    pub fn set_vmin(&mut self, v: f64) {
        self.my_vmin = v;
    }

    pub fn vmax(&self) -> f64 {
        self.my_vmax
    }

    pub fn set_vmax(&mut self, v: f64) {
        self.my_vmax = v;
    }
}

impl Default for BRepPrimOneAxis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_axis() {
        let oa = BRepPrimOneAxis::new();
        assert_eq!(oa.vmin(), 0.0);
        assert_eq!(oa.vmax(), 1.0);
    }
}
