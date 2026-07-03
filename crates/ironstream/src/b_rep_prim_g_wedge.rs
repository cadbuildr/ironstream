// FILE: b_rep_prim_g_wedge.rs
// occt: BRepPrim_GWedge

/// Wedge primitive with parametric control.
#[derive(Debug, Clone)]
pub struct BRepPrimGWedge {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    z2_min: f64,
    z2_max: f64,
    x2_min: f64,
    x2_max: f64,
}

impl BRepPrimGWedge {
    pub fn new() -> Self {
        BRepPrimGWedge {
            x_min: 0.0,
            x_max: 1.0,
            y_min: 0.0,
            y_max: 1.0,
            z_min: 0.0,
            z_max: 1.0,
            z2_min: 0.0,
            z2_max: 1.0,
            x2_min: 0.0,
            x2_max: 1.0,
        }
    }

    pub fn get_x_min(&self) -> f64 {
        self.x_min
    }

    pub fn get_x_max(&self) -> f64 {
        self.x_max
    }

    pub fn get_y_min(&self) -> f64 {
        self.y_min
    }

    pub fn get_y_max(&self) -> f64 {
        self.y_max
    }

    pub fn get_z_min(&self) -> f64 {
        self.z_min
    }

    pub fn get_z_max(&self) -> f64 {
        self.z_max
    }
}

impl Default for BRepPrimGWedge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_wedge() {
        let w = BRepPrimGWedge::new();
        assert_eq!(w.get_x_min(), 0.0);
        assert_eq!(w.get_x_max(), 1.0);
    }
}
