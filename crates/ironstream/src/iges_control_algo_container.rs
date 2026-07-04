// FILE: iges_control_algo_container.rs
// occt: IGESControl_AlgoContainer

/// Container for IGES algorithms.
pub struct IgesControlAlgoContainer;

impl IgesControlAlgoContainer {
    pub fn new() -> Self {
        Self
    }

    pub fn get_surface_algo(&self) -> String {
        "SurfaceAlgorithm".to_string()
    }

    pub fn get_curve_algo(&self) -> String {
        "CurveAlgorithm".to_string()
    }

    pub fn get_edge_algo(&self) -> String {
        "EdgeAlgorithm".to_string()
    }
}

impl Default for IgesControlAlgoContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_surface_algo() {
        let container = IgesControlAlgoContainer::new();
        assert_eq!(container.get_surface_algo(), "SurfaceAlgorithm");
    }

    #[test]
    fn test_get_curve_algo() {
        let container = IgesControlAlgoContainer::new();
        assert_eq!(container.get_curve_algo(), "CurveAlgorithm");
    }

    #[test]
    fn test_get_edge_algo() {
        let container = IgesControlAlgoContainer::new();
        assert_eq!(container.get_edge_algo(), "EdgeAlgorithm");
    }
}
