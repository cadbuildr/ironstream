// FILE: iges_dimen_section.rs
// occt: IGESDimen_Section

/// Defines Section, Type <404> Form <1>
/// in package IGESDimen
pub struct IgesDimen_Section {
    base_point: (f64, f64),
    normal: (f64, f64),
    direction: (f64, f64),
}

impl IgesDimen_Section {
    pub fn new() -> Self {
        IgesDimen_Section {
            base_point: (0.0, 0.0),
            normal: (0.0, 0.0),
            direction: (1.0, 0.0),
        }
    }

    pub fn init(&mut self, base: (f64, f64), normal: (f64, f64), direction: (f64, f64)) {
        self.base_point = base;
        self.normal = normal;
        self.direction = direction;
    }

    pub fn base_point(&self) -> (f64, f64) {
        self.base_point
    }

    pub fn normal(&self) -> (f64, f64) {
        self.normal
    }

    pub fn direction(&self) -> (f64, f64) {
        self.direction
    }
}

impl Default for IgesDimen_Section {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = IgesDimen_Section::new();
        assert_eq!(section.direction(), (1.0, 0.0));
    }
}
