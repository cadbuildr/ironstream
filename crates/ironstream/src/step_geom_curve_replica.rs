// FILE: step_geom_curve_replica.rs
// occt: StepGeom_CurveReplica

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Curve;

#[derive(Clone)]
pub struct CartesianTransformationOperator;

#[derive(Clone)]
pub struct CurveReplica {
    name: Arc<String>,
    parent_curve: Option<Arc<Mutex<Curve>>>,
    transformation: Option<Arc<Mutex<CartesianTransformationOperator>>>,
}

impl CurveReplica {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            parent_curve: None,
            transformation: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        parent_curve: Option<Arc<Mutex<Curve>>>,
        transformation: Option<Arc<Mutex<CartesianTransformationOperator>>>,
    ) {
        self.name = Arc::new(name);
        self.parent_curve = parent_curve;
        self.transformation = transformation;
    }

    pub fn set_parent_curve(&mut self, curve: Arc<Mutex<Curve>>) {
        self.parent_curve = Some(curve);
    }

    pub fn parent_curve(&self) -> Option<Arc<Mutex<Curve>>> {
        self.parent_curve.clone()
    }

    pub fn set_transformation(&mut self, trans: Arc<Mutex<CartesianTransformationOperator>>) {
        self.transformation = Some(trans);
    }

    pub fn transformation(&self) -> Option<Arc<Mutex<CartesianTransformationOperator>>> {
        self.transformation.clone()
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CurveReplica {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cr = CurveReplica::new();
        assert_eq!(cr.name(), "");
    }
}
