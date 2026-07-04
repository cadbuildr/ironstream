// FILE: iges_graph_color.rs
// occt: IGESGraph_Color

pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    color_name: Option<String>,
    entity_type: i32,
}

impl Color {
    pub fn new() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            color_name: None,
            entity_type: 314,
        }
    }

    pub fn init(&mut self, red: f64, green: f64, blue: f64, name: Option<String>) {
        self.red = red;
        self.green = green;
        self.blue = blue;
        self.color_name = name;
    }

    pub fn rgb_intensity(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }

    pub fn has_color_name(&self) -> bool {
        self.color_name.is_some()
    }

    pub fn color_name(&self) -> Option<&str> {
        self.color_name.as_deref()
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new();
        assert_eq!(color.entity_type(), 314);
    }

    #[test]
    fn test_init() {
        let mut color = Color::new();
        color.init(100.0, 50.0, 25.0, Some("Red".to_string()));
        let (r, g, b) = color.rgb_intensity();
        assert_eq!(r, 100.0);
        assert_eq!(g, 50.0);
        assert_eq!(b, 25.0);
        assert!(color.has_color_name());
    }
}
