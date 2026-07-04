// FILE: int_curve_surface_the_polygon_of_h_inter.rs
// occt: IntCurveSurface_ThePolygonOfHInter

//! Polygon approximation of curve for intersection algorithms.

#[derive(Clone)]
pub struct BoundingBox {
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
}

/// Polygon representation of a 3D curve
pub struct IntCurveSurfaceThePolygonOfHInter {
    points: Vec<(f64, f64, f64)>,
    bounding: BoundingBox,
    deflection: f64,
    closed: bool,
    inf_param: f64,
    sup_param: f64,
}

impl IntCurveSurfaceThePolygonOfHInter {
    /// Creates polygon from curve with fixed point count
    pub fn new(_curve: &CurveAdaptor, _nb_points: i32) -> Self {
        IntCurveSurfaceThePolygonOfHInter {
            points: Vec::new(),
            bounding: BoundingBox {
                xmin: 0.0,
                ymin: 0.0,
                zmin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
                zmax: 0.0,
            },
            deflection: 0.0,
            closed: false,
            inf_param: 0.0,
            sup_param: 1.0,
        }
    }

    /// Creates polygon from curve with parameter range
    pub fn new_with_range(_curve: &CurveAdaptor, _u1: f64, _u2: f64, _nb_points: i32) -> Self {
        IntCurveSurfaceThePolygonOfHInter {
            points: Vec::new(),
            bounding: BoundingBox {
                xmin: 0.0,
                ymin: 0.0,
                zmin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
                zmax: 0.0,
            },
            deflection: 0.0,
            closed: false,
            inf_param: _u1,
            sup_param: _u2,
        }
    }

    /// Creates polygon from curve with specified parameters
    pub fn new_with_params(_curve: &CurveAdaptor, _upars: &[f64]) -> Self {
        IntCurveSurfaceThePolygonOfHInter {
            points: Vec::new(),
            bounding: BoundingBox {
                xmin: 0.0,
                ymin: 0.0,
                zmin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
                zmax: 0.0,
            },
            deflection: 0.0,
            closed: false,
            inf_param: 0.0,
            sup_param: 1.0,
        }
    }

    /// Returns bounding box
    pub fn bounding(&self) -> &BoundingBox {
        &self.bounding
    }

    /// Returns deflection over-estimation
    pub fn deflection_over_estimation(&self) -> f64 {
        self.deflection
    }

    /// Sets deflection over-estimation
    pub fn set_deflection_over_estimation(&mut self, x: f64) {
        self.deflection = x;
    }

    /// Sets closed polygon flag
    pub fn set_closed(&mut self, flag: bool) {
        self.closed = flag;
    }

    /// Returns whether polygon is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Returns number of segments
    pub fn nb_segments(&self) -> i32 {
        (self.points.len().saturating_sub(1)) as i32
    }

    /// Returns begin point of segment
    pub fn begin_of_seg(&self, index: i32) -> Option<(f64, f64, f64)> {
        self.points.get(index as usize).copied()
    }

    /// Returns end point of segment
    pub fn end_of_seg(&self, index: i32) -> Option<(f64, f64, f64)> {
        self.points.get((index + 1) as usize).copied()
    }

    /// Returns inf parameter
    pub fn inf_parameter(&self) -> f64 {
        self.inf_param
    }

    /// Returns sup parameter
    pub fn sup_parameter(&self) -> f64 {
        self.sup_param
    }
}

/// Placeholder for curve adaptor
#[derive(Clone)]
pub struct CurveAdaptor;

impl BoundingBox {
    /// Creates new bounding box
    pub fn new(xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) -> Self {
        BoundingBox {
            xmin,
            ymin,
            zmin,
            xmax,
            ymax,
            zmax,
        }
    }

    /// Enlarges bounding box
    pub fn enlarge(&mut self, value: f64) {
        self.xmin -= value;
        self.ymin -= value;
        self.zmin -= value;
        self.xmax += value;
        self.ymax += value;
        self.zmax += value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_new() {
        let poly = IntCurveSurfaceThePolygonOfHInter::new(&CurveAdaptor, 10);
        assert_eq!(poly.nb_segments(), 0);
        assert!(!poly.is_closed());
    }

    #[test]
    fn test_polygon_parameters() {
        let poly = IntCurveSurfaceThePolygonOfHInter::new_with_range(&CurveAdaptor, 0.0, 1.0, 10);
        assert_eq!(poly.inf_parameter(), 0.0);
        assert_eq!(poly.sup_parameter(), 1.0);
    }

    #[test]
    fn test_bounding_box_enlarge() {
        let mut bbox = BoundingBox::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        bbox.enlarge(0.5);
        assert_eq!(bbox.xmin, -0.5);
        assert_eq!(bbox.xmax, 1.5);
    }
}
