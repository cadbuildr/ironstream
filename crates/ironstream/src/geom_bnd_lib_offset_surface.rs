// FILE: geom_bnd_lib_offset_surface.rs
// occt: GeomBndLib_OffsetSurface

//! Bounding box for offset 3D surfaces.

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

pub struct OffsetSurfaceBoundingBox;

impl OffsetSurfaceBoundingBox {
    pub fn from_points_and_offset(points: &[(f64, f64, f64)], offset: f64) -> BoundingBox {
        let mut bbox = BoundingBox::new();
        for &(x, y, z) in points {
            bbox.add_point((x - offset, y - offset, z - offset));
            bbox.add_point((x + offset, y + offset, z + offset));
        }
        bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_surface_bbox() {
        let points = vec![(0.0, 0.0, 0.0)];
        let bbox = OffsetSurfaceBoundingBox::from_points_and_offset(&points, 1.0);
        assert!(( bbox.xmin - (-1.0)).abs() < 1e-9);
    }
}
