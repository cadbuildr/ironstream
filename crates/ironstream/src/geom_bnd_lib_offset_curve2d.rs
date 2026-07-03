// FILE: geom_bnd_lib_offset_curve2d.rs
// occt: GeomBndLib_OffsetCurve2d

//! Bounding box for offset 2D curves.

#[derive(Clone, Debug)]
pub struct BoundingBox2d {
    pub xmin: f64, pub xmax: f64,
    pub ymin: f64, pub ymax: f64,
}

impl BoundingBox2d {
    pub fn new() -> Self {
        BoundingBox2d {
            xmin: f64::INFINITY, xmax: f64::NEG_INFINITY,
            ymin: f64::INFINITY, ymax: f64::NEG_INFINITY,
        }
    }

    pub fn add_point(&mut self, p: (f64, f64)) {
        self.xmin = self.xmin.min(p.0);
        self.xmax = self.xmax.max(p.0);
        self.ymin = self.ymin.min(p.1);
        self.ymax = self.ymax.max(p.1);
    }
}

pub struct OffsetCurve2dBoundingBox;

impl OffsetCurve2dBoundingBox {
    pub fn from_points_and_offset(points: &[(f64, f64)], offset: f64) -> BoundingBox2d {
        let mut bbox = BoundingBox2d::new();
        for &(x, y) in points {
            bbox.add_point((x - offset, y - offset));
            bbox.add_point((x + offset, y + offset));
        }
        bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_bbox2d() {
        let points = vec![(0.0, 0.0), (1.0, 1.0)];
        let bbox = OffsetCurve2dBoundingBox::from_points_and_offset(&points, 0.5);
        assert!(( bbox.xmin - (-0.5)).abs() < 1e-9);
    }
}
