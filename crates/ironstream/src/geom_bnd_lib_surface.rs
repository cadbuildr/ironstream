// FILE: geom_bnd_lib_surface.rs
// occt: GeomBndLib_Surface

//! Bounding box for 3D surfaces.

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

pub struct SurfaceBoundingBox;

impl SurfaceBoundingBox {
    pub fn from_points(points: &[(f64, f64, f64)]) -> BoundingBox {
        let mut bbox = BoundingBox::new();
        for &p in points {
            bbox.add_point(p);
        }
        bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_bbox() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 2.0, 3.0)];
        let bbox = SurfaceBoundingBox::from_points(&points);
        assert_eq!(bbox.xmin, 0.0);
        assert_eq!(bbox.zmax, 3.0);
    }
}
