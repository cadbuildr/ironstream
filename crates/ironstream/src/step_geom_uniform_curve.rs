// FILE: step_geom_uniform_curve.rs
// occt: StepGeom_UniformCurve

pub struct UniformCurve;

impl UniformCurve {
    pub fn new() -> Self {
        UniformCurve
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_curve_creation() {
        let _curve = UniformCurve::new();
    }
}
