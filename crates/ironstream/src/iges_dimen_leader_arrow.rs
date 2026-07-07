// FILE: iges_dimen_leader_arrow.rs
// occt: IGESDimen_LeaderArrow

/// Defines LeaderArrow, Type <214> Form <0-1>
/// in package IGESDimen
pub struct IgesDimen_LeaderArrow {
    arrow_type: i32,
    start: (f64, f64),
    end: (f64, f64),
}

impl IgesDimen_LeaderArrow {
    pub fn new() -> Self {
        IgesDimen_LeaderArrow {
            arrow_type: 0,
            start: (0.0, 0.0),
            end: (0.0, 0.0),
        }
    }

    pub fn init(&mut self, arrow_type: i32, start: (f64, f64), end: (f64, f64)) {
        self.arrow_type = arrow_type;
        self.start = start;
        self.end = end;
    }

    pub fn arrow_type(&self) -> i32 {
        self.arrow_type
    }

    pub fn start(&self) -> (f64, f64) {
        self.start
    }

    pub fn end(&self) -> (f64, f64) {
        self.end
    }
}

impl Default for IgesDimen_LeaderArrow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leader_arrow_creation() {
        let arrow = IgesDimen_LeaderArrow::new();
        assert_eq!(arrow.arrow_type(), 0);
    }

    #[test]
    fn test_leader_arrow_init() {
        let mut arrow = IgesDimen_LeaderArrow::new();
        arrow.init(1, (0.0, 0.0), (10.0, 10.0));

        assert_eq!(arrow.arrow_type(), 1);
        assert_eq!(arrow.start(), (0.0, 0.0));
        assert_eq!(arrow.end(), (10.0, 10.0));
    }
}
