// FILE: std_prs_bnd_box.rs
// occt: StdPrs_BndBox

/// Bounding box presentation tool.
#[derive(Clone, Debug)]
pub struct BndBox {
    pub box_id: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

impl BndBox {
    pub fn new(box_id: u32) -> Self {
        Self {
            box_id,
            x_min: 0.0,
            x_max: 1.0,
            y_min: 0.0,
            y_max: 1.0,
            z_min: 0.0,
            z_max: 1.0,
        }
    }

    pub fn set_bounds(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64) {
        self.x_min = xmin.min(xmax);
        self.x_max = xmax.max(xmin);
        self.y_min = ymin.min(ymax);
        self.y_max = ymax.max(ymin);
        self.z_min = zmin.min(zmax);
        self.z_max = zmax.max(zmin);
    }

    pub fn is_void(&self) -> bool {
        self.x_min > self.x_max || self.y_min > self.y_max || self.z_min > self.z_max
    }

    pub fn volume(&self) -> f64 {
        if self.is_void() {
            return 0.0;
        }
        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;
        let dz = self.z_max - self.z_min;
        dx * dy * dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_box() {
        let bb = BndBox::new(1);
        assert_eq!(bb.box_id, 1);
        assert!(!bb.is_void());
    }

    #[test]
    fn set_bounds() {
        let mut bb = BndBox::new(1);
        bb.set_bounds(-1.0, 1.0, -2.0, 2.0, -3.0, 3.0);
        assert_eq!(bb.x_min, -1.0);
        assert_eq!(bb.x_max, 1.0);
        assert_eq!(bb.volume(), 24.0);
    }

    #[test]
    fn void_box() {
        let mut bb = BndBox::new(1);
        bb.set_bounds(5.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert!(bb.is_void());
        assert_eq!(bb.volume(), 0.0);
    }
}
