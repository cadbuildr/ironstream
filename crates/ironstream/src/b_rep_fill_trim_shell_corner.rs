// FILE: b_rep_fill_trim_shell_corner.rs
// occt: BRepFill_TrimShellCorner

#[derive(Debug, Clone)]
pub struct BRepFillTrimShellCorner {
    transition: u32,
    axis_origin_x: f64,
    axis_origin_y: f64,
    axis_origin_z: f64,
    is_done: bool,
    has_section: bool,
}

impl BRepFillTrimShellCorner {
    pub fn new(transition: u32) -> Self {
        BRepFillTrimShellCorner {
            transition,
            axis_origin_x: 0.0,
            axis_origin_y: 0.0,
            axis_origin_z: 0.0,
            is_done: false,
            has_section: false,
        }
    }

    pub fn with_axis(transition: u32, x: f64, y: f64, z: f64) -> Self {
        BRepFillTrimShellCorner {
            transition,
            axis_origin_x: x,
            axis_origin_y: y,
            axis_origin_z: z,
            is_done: false,
            has_section: false,
        }
    }

    pub fn perform(&mut self) {
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn has_section(&self) -> bool {
        self.has_section
    }

    pub fn set_has_section(&mut self, has_section: bool) {
        self.has_section = has_section;
    }

    pub fn axis_origin(&self) -> (f64, f64, f64) {
        (self.axis_origin_x, self.axis_origin_y, self.axis_origin_z)
    }

    pub fn transition(&self) -> u32 {
        self.transition
    }
}

impl Default for BRepFillTrimShellCorner {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_shell_corner_new() {
        let tsc = BRepFillTrimShellCorner::new(1);
        assert!(!tsc.is_done());
        assert!(!tsc.has_section());
    }

    #[test]
    fn test_trim_shell_corner_with_axis() {
        let tsc = BRepFillTrimShellCorner::with_axis(2, 1.0, 2.0, 3.0);
        assert_eq!(tsc.axis_origin(), (1.0, 2.0, 3.0));
        assert_eq!(tsc.transition(), 2);
    }

    #[test]
    fn test_trim_shell_corner_perform() {
        let mut tsc = BRepFillTrimShellCorner::new(1);
        assert!(!tsc.is_done());
        tsc.perform();
        assert!(tsc.is_done());
    }

    #[test]
    fn test_trim_shell_corner_has_section() {
        let mut tsc = BRepFillTrimShellCorner::new(1);
        assert!(!tsc.has_section());
        tsc.set_has_section(true);
        assert!(tsc.has_section());
    }

    #[test]
    fn test_trim_shell_corner_default() {
        let tsc = BRepFillTrimShellCorner::default();
        assert!(!tsc.is_done());
    }
}
