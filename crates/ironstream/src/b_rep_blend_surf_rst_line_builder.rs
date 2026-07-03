// FILE: b_rep_blend_surf_rst_line_builder.rs
// occt: BRepBlend_SurfRstLineBuilder

#[derive(Clone)]
pub struct BRepBlendSurfRstLineBuilder {
    start_param: f64,
    end_param: f64,
    step: f64,
}

impl BRepBlendSurfRstLineBuilder {
    pub fn new() -> Self {
        Self {
            start_param: 0.0,
            end_param: 1.0,
            step: 0.1,
        }
    }

    pub fn set_range(&mut self, start: f64, end: f64) {
        self.start_param = start;
        self.end_param = end;
    }

    pub fn set_step(&mut self, step: f64) {
        self.step = step;
    }

    pub fn build(&self) -> Vec<f64> {
        let mut params = Vec::new();
        let mut current = self.start_param;
        while current <= self.end_param {
            params.push(current);
            current += self.step;
        }
        if (self.end_param - params.last().copied().unwrap_or(self.start_param)).abs() > 1e-10 {
            params.push(self.end_param);
        }
        params
    }
}

impl Default for BRepBlendSurfRstLineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let builder = BRepBlendSurfRstLineBuilder::new();
        assert_eq!(builder.start_param, 0.0);
    }

    #[test]
    fn test_build() {
        let mut builder = BRepBlendSurfRstLineBuilder::new();
        builder.set_range(0.0, 1.0);
        let params = builder.build();
        assert!(!params.is_empty());
    }
}
