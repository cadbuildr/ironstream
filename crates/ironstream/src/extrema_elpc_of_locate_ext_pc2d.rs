// FILE: extrema_elpc_of_locate_ext_pc2d.rs
// occt: Extrema_ELPCOfLocateExtPC2d

/// Extrema line-point curve locator in 2D.
pub struct ExtremaElpcOfLocateExtPc2d {
    found: bool,
    param: f64,
    distance: f64,
}

impl ExtremaElpcOfLocateExtPc2d {
    /// Creates a new 2D locator.
    pub fn new() -> Self {
        ExtremaElpcOfLocateExtPc2d {
            found: false,
            param: 0.0,
            distance: 0.0,
        }
    }

    /// Indicates if extremum was found.
    pub fn found(&self) -> bool {
        self.found
    }

    /// Returns parameter of extremum.
    pub fn param(&self) -> f64 {
        self.param
    }

    /// Returns distance at extremum.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Sets found flag.
    pub fn set_found(&mut self, found: bool) {
        self.found = found;
    }

    /// Sets parameter.
    pub fn set_param(&mut self, param: f64) {
        self.param = param;
    }

    /// Sets distance.
    pub fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }
}

impl Default for ExtremaElpcOfLocateExtPc2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = ExtremaElpcOfLocateExtPc2d::new();
        assert!(!loc.found());
    }

    #[test]
    fn test_set_values() {
        let mut loc = ExtremaElpcOfLocateExtPc2d::new();
        loc.set_found(true);
        loc.set_param(0.6);
        loc.set_distance(0.8);
        assert!(loc.found());
        assert_eq!(loc.param(), 0.6);
        assert_eq!(loc.distance(), 0.8);
    }
}
