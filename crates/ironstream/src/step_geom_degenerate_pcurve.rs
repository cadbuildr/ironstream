// FILE: step_geom_degenerate_pcurve.rs
// occt: StepGeom_DegeneratePcurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CartesianPoint;

#[derive(Clone)]
pub struct DegeneratePcurve {
    name: Arc<String>,
    reference_to_curve: Option<Arc<Mutex<CartesianPoint>>>,
    point_locations: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
}

impl DegeneratePcurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            reference_to_curve: None,
            point_locations: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        reference_to_curve: Option<Arc<Mutex<CartesianPoint>>>,
        point_locations: Option<Vec<Arc<Mutex<CartesianPoint>>>>,
    ) {
        self.name = Arc::new(name);
        self.reference_to_curve = reference_to_curve;
        self.point_locations = point_locations;
    }

    pub fn set_reference_to_curve(&mut self, pt: Arc<Mutex<CartesianPoint>>) {
        self.reference_to_curve = Some(pt);
    }

    pub fn reference_to_curve(&self) -> Option<Arc<Mutex<CartesianPoint>>> {
        self.reference_to_curve.clone()
    }

    pub fn set_point_locations(&mut self, pts: Vec<Arc<Mutex<CartesianPoint>>>) {
        self.point_locations = Some(pts);
    }

    pub fn point_locations(&self) -> Option<Vec<Arc<Mutex<CartesianPoint>>>> {
        self.point_locations.clone()
    }

    pub fn nb_point_locations(&self) -> i32 {
        self.point_locations.as_ref().map_or(0, |p| p.len() as i32)
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for DegeneratePcurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let dp = DegeneratePcurve::new();
        assert_eq!(dp.nb_point_locations(), 0);
    }
}
