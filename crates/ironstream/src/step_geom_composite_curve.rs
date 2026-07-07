// FILE: step_geom_composite_curve.rs
// occt: StepGeom_CompositeCurve

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CompositeCurveSegment;

#[derive(Clone)]
pub struct CompositeCurve {
    name: Arc<String>,
    segments: Option<Vec<Arc<Mutex<CompositeCurveSegment>>>>,
    self_intersect: bool,
}

impl CompositeCurve {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            segments: None,
            self_intersect: false,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        segments: Option<Vec<Arc<Mutex<CompositeCurveSegment>>>>,
        self_intersect: bool,
    ) {
        self.name = Arc::new(name);
        self.segments = segments;
        self.self_intersect = self_intersect;
    }

    pub fn set_segments(&mut self, segments: Vec<Arc<Mutex<CompositeCurveSegment>>>) {
        self.segments = Some(segments);
    }

    pub fn segments(&self) -> Option<Vec<Arc<Mutex<CompositeCurveSegment>>>> {
        self.segments.clone()
    }

    pub fn segments_value(&self, num: i32) -> Option<Arc<Mutex<CompositeCurveSegment>>> {
        self.segments
            .as_ref()
            .and_then(|s| s.get((num - 1) as usize).cloned())
    }

    pub fn nb_segments(&self) -> i32 {
        self.segments.as_ref().map_or(0, |s| s.len() as i32)
    }

    pub fn set_self_intersect(&mut self, intersect: bool) {
        self.self_intersect = intersect;
    }

    pub fn self_intersect(&self) -> bool {
        self.self_intersect
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for CompositeCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cc = CompositeCurve::new();
        assert!(!cc.self_intersect());
    }

    #[test]
    fn test_init() {
        let mut cc = CompositeCurve::new();
        cc.init("composite".to_string(), None, true);
        assert_eq!(cc.name(), "composite");
        assert!(cc.self_intersect());
    }
}
