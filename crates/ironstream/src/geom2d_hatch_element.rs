// FILE: geom2d_hatch_element.rs
// occt: Geom2dHatch_Element

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Element of a hatched area (curve with orientation).
#[derive(Debug, Clone)]
pub struct Geom2dHatchElement {
    curve_id: usize,
    orientation: Orientation,
}

impl Geom2dHatchElement {
    pub fn new(curve_id: usize, orientation: Orientation) -> Self {
        Geom2dHatchElement { curve_id, orientation }
    }

    pub fn default() -> Self {
        Geom2dHatchElement {
            curve_id: 0,
            orientation: Orientation::Forward,
        }
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn curve_id(&self) -> usize {
        self.curve_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = Geom2dHatchElement::new(1, Orientation::Forward);
        assert_eq!(elem.curve_id(), 1);
        assert_eq!(elem.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_element_default() {
        let elem = Geom2dHatchElement::default();
        assert_eq!(elem.curve_id(), 0);
        assert_eq!(elem.orientation(), Orientation::Forward);
    }

    #[test]
    fn test_set_orientation() {
        let mut elem = Geom2dHatchElement::new(1, Orientation::Forward);
        elem.set_orientation(Orientation::Reversed);
        assert_eq!(elem.orientation(), Orientation::Reversed);
    }
}
