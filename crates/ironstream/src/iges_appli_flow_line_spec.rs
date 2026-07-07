// FILE: iges_appli_flow_line_spec.rs
// occt: IGESAppli_FlowLineSpec

/// Specifies properties for a flow line.
#[derive(Clone, Debug)]
pub struct IgesAppliFlowLineSpec {
    pipe_diameter: f64,
    wall_thickness: f64,
    flow_rate: f64,
}

impl IgesAppliFlowLineSpec {
    pub fn new() -> Self {
        Self {
            pipe_diameter: 0.0,
            wall_thickness: 0.0,
            flow_rate: 0.0,
        }
    }

    pub fn init(&mut self, diameter: f64, thickness: f64, rate: f64) {
        self.pipe_diameter = diameter;
        self.wall_thickness = thickness;
        self.flow_rate = rate;
    }

    pub fn pipe_diameter(&self) -> f64 {
        self.pipe_diameter
    }

    pub fn wall_thickness(&self) -> f64 {
        self.wall_thickness
    }

    pub fn flow_rate(&self) -> f64 {
        self.flow_rate
    }
}

impl Default for IgesAppliFlowLineSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut spec = IgesAppliFlowLineSpec::new();
        spec.init(25.4, 1.5, 100.0);

        assert_eq!(spec.pipe_diameter(), 25.4);
        assert_eq!(spec.wall_thickness(), 1.5);
        assert_eq!(spec.flow_rate(), 100.0);
    }
}
