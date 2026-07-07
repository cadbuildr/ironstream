// FILE: geom2d_grid_eval_circle.rs
// occt: Geom2dGridEval_Circle

/// Grid evaluator for circles.
pub struct CircleGridEval {
    radius: f64,
}

impl CircleGridEval {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn evaluate(&self, angle: f64) -> (f64, f64) {
        (self.radius * angle.cos(), self.radius * angle.sin())
    }
}
