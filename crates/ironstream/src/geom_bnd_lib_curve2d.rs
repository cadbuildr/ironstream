// FILE: geom_bnd_lib_curve2d.rs
// occt: GeomBndLib_Curve2d

//! Bounding box computation for 2D curves.

#[derive(Clone, Debug)]
pub struct BoundingBox2d {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl BoundingBox2d {
    pub fn new() -> Self {
        BoundingBox2d {
            xmin: f64::INFINITY,
            xmax: f64::NEG_INFINITY,
            ymin: f64::INFINITY,
            ymax: f64::NEG_INFINITY,
        }
    }

    pub fn add_point(&mut self, p: (f64, f64)) {
        self.xmin = self.xmin.min(p.0);
        self.xmax = self.xmax.max(p.0);
        self.ymin = self.ymin.min(p.1);
        self.ymax = self.ymax.max(p.1);
    }

    pub fn width(&self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn height(&self) -> f64 {
        self.ymax - self.ymin
    }
}

pub struct Curve2dBoundingBox;

impl Curve2dBoundingBox {
    pub fn from_points(points: &[(f64, f64)]) -> BoundingBox2d {
        let mut bbox = BoundingBox2d::new();
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
    fn test_bbox2d_create() {
        let bbox = BoundingBox2d::new();
        assert!(bbox.xmin.is_infinite());
    }

    #[test]
    fn test_bbox2d_add_point() {
        let mut bbox = BoundingBox2d::new();
        bbox.add_point((1.0, 2.0));
        assert_eq!(bbox.xmin, 1.0);
    }

    #[test]
    fn test_curve2d_bbox() {
        let points = vec![(0.0, 0.0), (1.0, 2.0)];
        let bbox = Curve2dBoundingBox::from_points(&points);
        assert!(( bbox.width() - 1.0).abs() < 1e-9);
        assert!((bbox.height() - 2.0).abs() < 1e-9);
    }
}
