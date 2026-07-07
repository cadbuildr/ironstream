// FILE: graphic3d_polygon_offset.rs
// occt: Graphic3d_PolygonOffset
// occt: Aspect_PolygonOffsetMode

/// Polygon offset modes enumeration.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonOffsetMode {
    /// All polygon offset modes disabled
    Off = 0x00,
    /// GL_POLYGON_OFFSET_FILL enabled (shaded polygons)
    Fill = 0x01,
    /// GL_POLYGON_OFFSET_LINE enabled (polygons as outlines)
    Line = 0x02,
    /// GL_POLYGON_OFFSET_POINT enabled (polygons as vertices)
    Point = 0x04,
    /// All modes enabled
    All = 0x07,
    /// Do not change current polygon offset mode
    None = 0x08,
}

impl PolygonOffsetMode {
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(value: u32) -> Self {
        match value {
            0x00 => PolygonOffsetMode::Off,
            0x01 => PolygonOffsetMode::Fill,
            0x02 => PolygonOffsetMode::Line,
            0x04 => PolygonOffsetMode::Point,
            0x07 => PolygonOffsetMode::All,
            0x08 => PolygonOffsetMode::None,
            _ => PolygonOffsetMode::Off,
        }
    }
}

/// Polygon offset parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct PolygonOffset {
    pub mode: PolygonOffsetMode,
    pub factor: f32,
    pub units: f32,
}

impl PolygonOffset {
    /// Creates a new polygon offset with default parameters.
    /// Default mode: Fill, factor: 1.0, units: 1.0
    pub fn new() -> Self {
        PolygonOffset {
            mode: PolygonOffsetMode::Fill,
            factor: 1.0f32,
            units: 1.0f32,
        }
    }

    /// Creates a new polygon offset with specified parameters.
    pub fn with_params(mode: PolygonOffsetMode, factor: f32, units: f32) -> Self {
        PolygonOffset {
            mode,
            factor,
            units,
        }
    }

    /// Sets the mode.
    pub fn set_mode(&mut self, mode: PolygonOffsetMode) {
        self.mode = mode;
    }

    /// Sets the factor.
    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }

    /// Sets the units.
    pub fn set_units(&mut self, units: f32) {
        self.units = units;
    }
}

impl Default for PolygonOffset {
    fn default() -> Self {
        PolygonOffset::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_offset_mode_values() {
        assert_eq!(PolygonOffsetMode::Off.as_u32(), 0x00);
        assert_eq!(PolygonOffsetMode::Fill.as_u32(), 0x01);
        assert_eq!(PolygonOffsetMode::Line.as_u32(), 0x02);
        assert_eq!(PolygonOffsetMode::Point.as_u32(), 0x04);
        assert_eq!(PolygonOffsetMode::All.as_u32(), 0x07);
        assert_eq!(PolygonOffsetMode::None.as_u32(), 0x08);
    }

    #[test]
    fn test_polygon_offset_mode_from_u32() {
        assert_eq!(PolygonOffsetMode::from_u32(0x00), PolygonOffsetMode::Off);
        assert_eq!(PolygonOffsetMode::from_u32(0x01), PolygonOffsetMode::Fill);
        assert_eq!(PolygonOffsetMode::from_u32(0x02), PolygonOffsetMode::Line);
        assert_eq!(PolygonOffsetMode::from_u32(0x04), PolygonOffsetMode::Point);
        assert_eq!(PolygonOffsetMode::from_u32(0x07), PolygonOffsetMode::All);
        assert_eq!(PolygonOffsetMode::from_u32(0x08), PolygonOffsetMode::None);
    }

    #[test]
    fn test_polygon_offset_default() {
        let offset = PolygonOffset::new();
        assert_eq!(offset.mode, PolygonOffsetMode::Fill);
        assert_eq!(offset.factor, 1.0f32);
        assert_eq!(offset.units, 1.0f32);
    }

    #[test]
    fn test_polygon_offset_with_params() {
        let offset = PolygonOffset::with_params(PolygonOffsetMode::Line, 2.5f32, 3.5f32);
        assert_eq!(offset.mode, PolygonOffsetMode::Line);
        assert_eq!(offset.factor, 2.5f32);
        assert_eq!(offset.units, 3.5f32);
    }

    #[test]
    fn test_polygon_offset_setters() {
        let mut offset = PolygonOffset::new();
        offset.set_mode(PolygonOffsetMode::Point);
        offset.set_factor(2.0f32);
        offset.set_units(3.0f32);

        assert_eq!(offset.mode, PolygonOffsetMode::Point);
        assert_eq!(offset.factor, 2.0f32);
        assert_eq!(offset.units, 3.0f32);
    }

    #[test]
    fn test_polygon_offset_equality() {
        let offset1 = PolygonOffset::with_params(PolygonOffsetMode::Fill, 1.0f32, 1.0f32);
        let offset2 = PolygonOffset::new();
        assert_eq!(offset1, offset2);
    }

    #[test]
    fn test_polygon_offset_default_trait() {
        let offset = PolygonOffset::default();
        assert_eq!(offset.mode, PolygonOffsetMode::Fill);
        assert_eq!(offset.factor, 1.0f32);
        assert_eq!(offset.units, 1.0f32);
    }
}
