// FILE: geom_bnd_lib_surface_of_revolution.rs
// occt: GeomBndLib_SurfaceOfRevolution

//! Bounding box for revolution surfaces.

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub xmin: f64, pub xmax: f64,
    pub ymin: f64, pub ymax: f64,
    pub zmin: f64, pub zmax: f64,
}

impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            xmin: f64::INFINITY, xmax: f64::NEG_INFINITY,
            ymin: f64::INFINITY, ymax: f64::NEG_INFINITY,
            zmin: f64::INFINITY, zmax: f64::NEG_INFINITY,
        }
    }

    pub fn add_point(&mut self, p: (f64, f64, f64)) {
        self.xmin = self.xmin.min(p.0);
        self.xmax = self.xmax.max(p.0);
        self.ymin = self.ymin.min(p.1);
        self.ymax = self.ymax.max(p.1);
        self.zmin = self.zmin.min(p.2);
        self.zmax = self.zmax.max(p.2);
    }
}

pub struct RevolutionSurfaceBoundingBox;

impl RevolutionSurfaceBoundingBox {
    pub fn from_profile(profile: &[(f64, f64, f64)]) -> BoundingBox {
        let mut bbox = BoundingBox::new();

        // For revolution around Z-axis, add circular bounds
        for &(x, y, z) in profile {
            let r = (x * x + y * y).sqrt();
            bbox.add_point((r, r, z));
            bbox.add_point((-r, -r, z));
        }

        bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revolution_bbox() {
        let profile = vec![(1.0, 0.0, 0.0), (2.0, 0.0, 1.0)];
        let bbox = RevolutionSurfaceBoundingBox::from_profile(&profile);
        assert!(( bbox.xmin - (-2.0)).abs() < 1e-9);
        assert!(( bbox.xmax - 2.0).abs() < 1e-9);
    }
}
