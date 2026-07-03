// FILE: geom_bnd_lib_curve.rs
// occt: GeomBndLib_Curve

//! Bounding box computation for 3D curves.

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            xmin: f64::INFINITY,
            xmax: f64::NEG_INFINITY,
            ymin: f64::INFINITY,
            ymax: f64::NEG_INFINITY,
            zmin: f64::INFINITY,
            zmax: f64::NEG_INFINITY,
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

    pub fn width(&self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn height(&self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn depth(&self) -> f64 {
        self.zmax - self.zmin
    }
}

pub struct CurveBoundingBox;

impl CurveBoundingBox {
    pub fn from_points(points: &[(f64, f64, f64)]) -> BoundingBox {
        let mut bbox = BoundingBox::new();
        for &p in points {
            bbox.add_point(p);
        }
        bbox
    }

    pub fn add_curve_to_box(points: &[(f64, f64, f64)], bbox: &mut BoundingBox) {
        for &p in points {
            bbox.add_point(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_create() {
        let bbox = BoundingBox::new();
        assert!(bbox.xmin.is_infinite());
    }

    #[test]
    fn test_bbox_add_point() {
        let mut bbox = BoundingBox::new();
        bbox.add_point((1.0, 2.0, 3.0));
        assert_eq!(bbox.xmin, 1.0);
        assert_eq!(bbox.ymax, 2.0);
    }

    #[test]
    fn test_bbox_dimensions() {
        let mut bbox = BoundingBox::new();
        bbox.add_point((0.0, 0.0, 0.0));
        bbox.add_point((1.0, 2.0, 3.0));
        assert!(( bbox.width() - 1.0).abs() < 1e-9);
        assert!((bbox.height() - 2.0).abs() < 1e-9);
        assert!((bbox.depth() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_curve_bbox() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.5, 0.5, 0.5)];
        let bbox = CurveBoundingBox::from_points(&points);
        assert_eq!(bbox.xmin, 0.0);
        assert_eq!(bbox.xmax, 1.0);
    }
}
