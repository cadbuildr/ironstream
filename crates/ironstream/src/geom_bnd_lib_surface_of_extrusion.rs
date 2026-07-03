// FILE: geom_bnd_lib_surface_of_extrusion.rs
// occt: GeomBndLib_SurfaceOfExtrusion

//! Bounding box for extrusion surfaces.

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

pub struct ExtrusionSurfaceBoundingBox;

impl ExtrusionSurfaceBoundingBox {
    pub fn from_profile_and_direction(
        profile: &[(f64, f64, f64)],
        direction: (f64, f64, f64),
        height: f64,
    ) -> BoundingBox {
        let mut bbox = BoundingBox::new();
        for &p in profile {
            bbox.add_point(p);
            bbox.add_point((
                p.0 + height * direction.0,
                p.1 + height * direction.1,
                p.2 + height * direction.2,
            ));
        }
        bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrusion_bbox() {
        let profile = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 0.0)];
        let bbox = ExtrusionSurfaceBoundingBox::from_profile_and_direction(
            &profile,
            (0.0, 0.0, 1.0),
            2.0,
        );
        assert_eq!(bbox.zmin, 0.0);
        assert_eq!(bbox.zmax, 2.0);
    }
}
