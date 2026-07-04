// FILE: step_visual_colour_rgb.rs
// occt: StepVisual_ColourRgb

/// Represents a StepVisual ColourRgb with red, green, blue components
#[derive(Debug, Clone, Default)]
pub struct StepVisual_ColourRgb {
    name: Option<String>,
    red: f64,
    green: f64,
    blue: f64,
}

impl StepVisual_ColourRgb {
    pub fn new() -> Self {
        StepVisual_ColourRgb {
            name: None,
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_red(&mut self, red: f64) {
        self.red = red;
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn set_green(&mut self, green: f64) {
        self.green = green;
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn set_blue(&mut self, blue: f64) {
        self.blue = blue;
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rgb = StepVisual_ColourRgb::new();
        assert_eq!(rgb.red(), 0.0);
        assert_eq!(rgb.green(), 0.0);
        assert_eq!(rgb.blue(), 0.0);
    }

    #[test]
    fn test_set_colors() {
        let mut rgb = StepVisual_ColourRgb::new();
        rgb.set_red(0.5);
        rgb.set_green(0.75);
        rgb.set_blue(1.0);
        assert_eq!(rgb.red(), 0.5);
        assert_eq!(rgb.green(), 0.75);
        assert_eq!(rgb.blue(), 1.0);
    }
}
