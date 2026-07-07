// FILE: step_geom_cartesian_transformation_operator2d.rs
// occt: StepGeom_CartesianTransformationOperator2d

use std::sync::Arc;

#[derive(Clone)]
pub struct CartesianTransformationOperator2d {
    name: Arc<String>,
    axis1: Option<Arc<String>>,
    local_origin: Option<Arc<String>>,
    scale: f64,
}

impl CartesianTransformationOperator2d {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            axis1: None,
            local_origin: None,
            scale: 1.0,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        axis1: Option<String>,
        local_origin: Option<String>,
        scale: f64,
    ) {
        self.name = Arc::new(name);
        self.axis1 = axis1.map(|a| Arc::new(a));
        self.local_origin = local_origin.map(|o| Arc::new(o));
        self.scale = scale;
    }

    pub fn set_axis1(&mut self, axis: String) {
        self.axis1 = Some(Arc::new(axis));
    }

    pub fn axis1(&self) -> Option<String> {
        self.axis1.as_ref().map(|a| a.as_ref().clone())
    }

    pub fn set_local_origin(&mut self, origin: String) {
        self.local_origin = Some(Arc::new(origin));
    }

    pub fn local_origin(&self) -> Option<String> {
        self.local_origin.as_ref().map(|o| o.as_ref().clone())
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CartesianTransformationOperator2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let op = CartesianTransformationOperator2d::new();
        assert_eq!(op.scale(), 1.0);
    }

    #[test]
    fn test_init() {
        let mut op = CartesianTransformationOperator2d::new();
        op.init("transform2d".to_string(), None, None, 1.5);
        assert_eq!(op.scale(), 1.5);
    }
}
