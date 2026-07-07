// FILE: iges_appli_pwb_drilled_hole.rs
// occt: IGESAppli_PWBDrilledHole

/// Represents a PWB drilled hole with extended properties.
#[derive(Clone, Debug)]
pub struct IgesAppliPwbDrilledHole {
    hole_type: i32,
    diameter: f64,
    depth: f64,
}

impl IgesAppliPwbDrilledHole {
    pub fn new() -> Self {
        Self {
            hole_type: 0,
            diameter: 0.0,
            depth: 0.0,
        }
    }

    pub fn init(&mut self, htype: i32, diam: f64, d: f64) {
        self.hole_type = htype;
        self.diameter = diam;
        self.depth = d;
    }

    pub fn hole_type(&self) -> i32 {
        self.hole_type
    }

    pub fn diameter(&self) -> f64 {
        self.diameter
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }
}

impl Default for IgesAppliPwbDrilledHole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut hole = IgesAppliPwbDrilledHole::new();
        hole.init(1, 2.5, 10.0);

        assert_eq!(hole.hole_type(), 1);
        assert_eq!(hole.diameter(), 2.5);
        assert_eq!(hole.depth(), 10.0);
    }
}
