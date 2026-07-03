// FILE: int_curve_surface_the_h_curve_tool.rs
// occt: IntCurveSurface_TheHCurveTool

pub struct IntCurveSurfaceTheHCurveTool;

impl IntCurveSurfaceTheHCurveTool {
    pub fn new() -> Self {
        IntCurveSurfaceTheHCurveTool
    }

    pub fn first_param() -> f64 {
        0.0
    }

    pub fn last_param() -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let _tool = IntCurveSurfaceTheHCurveTool::new();
    }

    #[test]
    fn test_params() {
        assert_eq!(IntCurveSurfaceTheHCurveTool::first_param(), 0.0);
        assert_eq!(IntCurveSurfaceTheHCurveTool::last_param(), 1.0);
    }
}
