// FILE: step_geom_composite_curve_segment.rs
// occt: StepGeom_CompositeCurveSegment

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Curve;

#[derive(Clone, Debug, PartialEq)]
pub enum Sense {
    Forward,
    Backward,
}

#[derive(Clone)]
pub struct CompositeCurveSegment {
    curve: Option<Arc<Mutex<Curve>>>,
    sense: Sense,
}

impl CompositeCurveSegment {
    pub fn new() -> Self {
        Self {
            curve: None,
            sense: Sense::Forward,
        }
    }

    pub fn init(&mut self, curve: Option<Arc<Mutex<Curve>>>, sense: Sense) {
        self.curve = curve;
        self.sense = sense;
    }

    pub fn set_curve(&mut self, curve: Arc<Mutex<Curve>>) {
        self.curve = Some(curve);
    }

    pub fn curve(&self) -> Option<Arc<Mutex<Curve>>> {
        self.curve.clone()
    }

    pub fn set_sense(&mut self, sense: Sense) {
        self.sense = sense;
    }

    pub fn sense(&self) -> Sense {
        self.sense.clone()
    }
}

impl Default for CompositeCurveSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let seg = CompositeCurveSegment::new();
        assert_eq!(seg.sense(), Sense::Forward);
    }

    #[test]
    fn test_init() {
        let mut seg = CompositeCurveSegment::new();
        seg.init(None, Sense::Backward);
        assert_eq!(seg.sense(), Sense::Backward);
    }
}
