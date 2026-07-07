// FILE: step_visual_fill_area_style_colour.rs
// occt: StepVisual_FillAreaStyleColour

/// A fill area style colour in STEP representation.
///
/// This defines the colour for fill area style.
pub struct FillAreaStyleColour {
    name: String,
    red: f64,
    green: f64,
    blue: f64,
}

impl FillAreaStyleColour {
    /// Creates a new fill area style colour.
    pub fn new(name: String) -> Self {
        FillAreaStyleColour {
            name,
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the RGB values.
    pub fn set_rgb(&mut self, r: f64, g: f64, b: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
    }

    /// Returns the red component.
    pub fn red(&self) -> f64 {
        self.red
    }

    /// Returns the green component.
    pub fn green(&self) -> f64 {
        self.green
    }

    /// Returns the blue component.
    pub fn blue(&self) -> f64 {
        self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_area_style_colour_new() {
        let colour = FillAreaStyleColour::new("Color1".to_string());
        assert_eq!(colour.name(), "Color1");
        assert_eq!(colour.red(), 0.0);
    }

    #[test]
    fn test_set_rgb() {
        let mut colour = FillAreaStyleColour::new("Red".to_string());
        colour.set_rgb(1.0, 0.0, 0.0);
        assert_eq!(colour.red(), 1.0);
        assert_eq!(colour.green(), 0.0);
        assert_eq!(colour.blue(), 0.0);
    }
}
