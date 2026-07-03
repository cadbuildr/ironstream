// FILE: b_rep_blend_walking.rs
// occt: BRepBlend_Walking

#[derive(Clone)]
pub struct BRepBlendWalking {
    current_param: f64,
    start_param: f64,
    end_param: f64,
    step_size: f64,
}

impl BRepBlendWalking {
    pub fn new() -> Self {
        Self {
            current_param: 0.0,
            start_param: 0.0,
            end_param: 1.0,
            step_size: 0.1,
        }
    }

    pub fn set_range(&mut self, start: f64, end: f64) {
        self.start_param = start;
        self.end_param = end;
        self.current_param = start;
    }

    pub fn set_step(&mut self, step: f64) {
        self.step_size = step;
    }

    pub fn advance(&mut self) -> bool {
        if self.current_param + self.step_size <= self.end_param {
            self.current_param += self.step_size;
            true
        } else {
            self.current_param = self.end_param;
            false
        }
    }

    pub fn current_parameter(&self) -> f64 {
        self.current_param
    }

    pub fn is_finished(&self) -> bool {
        (self.current_param - self.end_param).abs() < 1e-10
    }
}

impl Default for BRepBlendWalking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let walk = BRepBlendWalking::new();
        assert_eq!(walk.current_parameter(), 0.0);
    }

    #[test]
    fn test_advance() {
        let mut walk = BRepBlendWalking::new();
        walk.set_range(0.0, 1.0);
        walk.set_step(0.25);
        assert!(!walk.is_finished());
        walk.advance();
        assert_eq!(walk.current_parameter(), 0.25);
    }
}
