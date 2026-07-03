// FILE: gcc_ent.rs
// occt: GccEnt

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GccEntPosition {
    Unqualified,
    Enclosing,
    Enclosed,
    Outside,
    NoQualifier,
}

/// Qualified line for geometric constraint building.
#[derive(Debug, Clone)]
pub struct GccEntQualifiedLin {
    pub x0: f64,
    pub y0: f64,
    pub dx: f64,
    pub dy: f64,
    pub position: GccEntPosition,
}

/// Qualified circle for geometric constraint building.
#[derive(Debug, Clone)]
pub struct GccEntQualifiedCirc {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
    pub position: GccEntPosition,
}

impl GccEntQualifiedLin {
    pub fn new(x0: f64, y0: f64, dx: f64, dy: f64, position: GccEntPosition) -> Self {
        GccEntQualifiedLin { x0, y0, dx, dy, position }
    }

    pub fn unqualified(x0: f64, y0: f64, dx: f64, dy: f64) -> Self {
        GccEntQualifiedLin { x0, y0, dx, dy, position: GccEntPosition::Unqualified }
    }

    pub fn enclosed(x0: f64, y0: f64, dx: f64, dy: f64) -> Self {
        GccEntQualifiedLin { x0, y0, dx, dy, position: GccEntPosition::Enclosed }
    }

    pub fn outside(x0: f64, y0: f64, dx: f64, dy: f64) -> Self {
        GccEntQualifiedLin { x0, y0, dx, dy, position: GccEntPosition::Outside }
    }
}

impl GccEntQualifiedCirc {
    pub fn new(cx: f64, cy: f64, radius: f64, position: GccEntPosition) -> Self {
        GccEntQualifiedCirc { cx, cy, radius, position }
    }

    pub fn unqualified(cx: f64, cy: f64, radius: f64) -> Self {
        GccEntQualifiedCirc { cx, cy, radius, position: GccEntPosition::Unqualified }
    }

    pub fn enclosing(cx: f64, cy: f64, radius: f64) -> Self {
        GccEntQualifiedCirc { cx, cy, radius, position: GccEntPosition::Enclosing }
    }

    pub fn enclosed(cx: f64, cy: f64, radius: f64) -> Self {
        GccEntQualifiedCirc { cx, cy, radius, position: GccEntPosition::Enclosed }
    }

    pub fn outside(cx: f64, cy: f64, radius: f64) -> Self {
        GccEntQualifiedCirc { cx, cy, radius, position: GccEntPosition::Outside }
    }
}

impl GccEntPosition {
    pub fn to_string(&self) -> &'static str {
        match self {
            GccEntPosition::Unqualified => "UNQUALIFIED",
            GccEntPosition::Enclosing => "ENCLOSING",
            GccEntPosition::Enclosed => "ENCLOSED",
            GccEntPosition::Outside => "OUTSIDE",
            GccEntPosition::NoQualifier => "NOQUALIFIER",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "UNQUALIFIED" => GccEntPosition::Unqualified,
            "ENCLOSING" => GccEntPosition::Enclosing,
            "ENCLOSED" => GccEntPosition::Enclosed,
            "OUTSIDE" => GccEntPosition::Outside,
            "NOQUALIFIER" => GccEntPosition::NoQualifier,
            _ => GccEntPosition::Unqualified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualified_line_unqualified() {
        let qline = GccEntQualifiedLin::unqualified(0.0, 0.0, 1.0, 0.0);
        assert_eq!(qline.position, GccEntPosition::Unqualified);
    }

    #[test]
    fn test_qualified_circle_enclosing() {
        let qcirc = GccEntQualifiedCirc::enclosing(1.0, 2.0, 3.0);
        assert_eq!(qcirc.position, GccEntPosition::Enclosing);
    }

    #[test]
    fn test_position_to_string() {
        assert_eq!(GccEntPosition::Enclosing.to_string(), "ENCLOSING");
        assert_eq!(GccEntPosition::Outside.to_string(), "OUTSIDE");
    }

    #[test]
    fn test_position_from_string() {
        assert_eq!(GccEntPosition::from_string("OUTSIDE"), GccEntPosition::Outside);
        assert_eq!(GccEntPosition::from_string("outside"), GccEntPosition::Outside);
        assert_eq!(GccEntPosition::from_string("INVALID"), GccEntPosition::Unqualified);
    }
}
