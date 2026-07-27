// FILE: ais_text.rs
// occt-ref: AIS_Text, AIS_TextLabel, Prs3d_Text
//       AIS_Dimension, AIS_LengthDimension, AIS_AngleDimension

/// Horizontal text alignment.
/// occt: Graphic3d_HorizontalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HorizTextAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizTextAlign {
    fn default() -> Self { Self::Left }
}

/// Vertical text alignment.
/// occt: Graphic3d_VerticalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertTextAlign {
    Bottom,
    Center,
    Top,
}

impl Default for VertTextAlign {
    fn default() -> Self { Self::Bottom }
}

/// Text display type.
/// occt: Aspect_TypeOfDisplayText
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayTextType {
    Normal,
    Subtitle,
    DecimalNumbers,
    Dimension,
}

impl Default for DisplayTextType {
    fn default() -> Self { Self::Normal }
}

/// A 3D text label in an AIS context.
// occt-ref: AIS_TextLabel // (a.k.a. AIS_Text / Prs3d_Text)
#[derive(Clone, Debug)]
pub struct AisTextLabel {
    pub label_id: u32,
    pub text: String,
    pub position: [f64; 3],
    pub height: f64,
    pub color: [f64; 4],   // RGBA
    pub horiz_align: HorizTextAlign,
    pub vert_align: VertTextAlign,
    pub display_type: DisplayTextType,
    pub font_name: String,
    pub has_own_color: bool,
    pub is_3d_zoomable: bool,
}

impl AisTextLabel {
    pub fn new(label_id: u32, text: impl Into<String>, pos: [f64; 3]) -> Self {
        Self {
            label_id,
            text: text.into(),
            position: pos,
            height: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            horiz_align: HorizTextAlign::Left,
            vert_align: VertTextAlign::Bottom,
            display_type: DisplayTextType::Normal,
            font_name: String::from("Courier"),
            has_own_color: false,
            is_3d_zoomable: true,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) { self.text = text.into(); }
    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
    pub fn set_height(&mut self, h: f64) { self.height = h; }
    pub fn set_color(&mut self, rgba: [f64; 4]) { self.color = rgba; self.has_own_color = true; }
    pub fn set_font(&mut self, name: impl Into<String>) { self.font_name = name.into(); }
    pub fn set_horiz_align(&mut self, a: HorizTextAlign) { self.horiz_align = a; }
    pub fn set_vert_align(&mut self, a: VertTextAlign) { self.vert_align = a; }
    pub fn set_display_type(&mut self, t: DisplayTextType) { self.display_type = t; }
    pub fn set_3d_zoomable(&mut self, z: bool) { self.is_3d_zoomable = z; }

    pub fn text(&self) -> &str { &self.text }
    pub fn position(&self) -> [f64; 3] { self.position }
    pub fn height(&self) -> f64 { self.height }
    pub fn color(&self) -> [f64; 4] { self.color }
    pub fn font_name(&self) -> &str { &self.font_name }
    pub fn has_own_color(&self) -> bool { self.has_own_color }
}

/// Dimension kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionKind {
    Length,
    Angle,
    Radius,
    Diameter,
}

impl Default for DimensionKind {
    fn default() -> Self { Self::Length }
}

/// Dimension text position.
// occt-ref: PrsDim_DimensionSelectionMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimTextPosition {
    Auto,
    Flyout,
    Centered,
}

impl Default for DimTextPosition {
    fn default() -> Self { Self::Auto }
}

/// A dimension annotation (length, angle, radius).
/// occt: PrsDim_Dimension
// occt-ref: PrsDim_LengthDimension, PrsDim_AngleDimension
#[derive(Clone, Debug)]
pub struct AisDimension {
    pub dim_id: u32,
    pub kind: DimensionKind,
    pub value: f64,
    pub unit: String,
    pub text_position: DimTextPosition,
    pub flyout: f64,
    pub p1: [f64; 3],
    pub p2: [f64; 3],
    pub is_valid: bool,
    pub display_units: bool,
    pub color: [f64; 3],
    pub arrow_length: f64,
}

impl AisDimension {
    pub fn new_length(dim_id: u32, p1: [f64; 3], p2: [f64; 3]) -> Self {
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];
        let dz = p2[2] - p1[2];
        let value = (dx*dx + dy*dy + dz*dz).sqrt();
        Self {
            dim_id, kind: DimensionKind::Length, value,
            unit: "mm".to_owned(), text_position: DimTextPosition::Auto,
            flyout: 10.0, p1, p2, is_valid: true,
            display_units: true, color: [1.0, 1.0, 0.0], arrow_length: 1.0,
        }
    }

    pub fn new_angle(dim_id: u32, angle_deg: f64, vertex: [f64; 3]) -> Self {
        Self {
            dim_id, kind: DimensionKind::Angle, value: angle_deg,
            unit: "deg".to_owned(), text_position: DimTextPosition::Auto,
            flyout: 10.0, p1: vertex, p2: vertex, is_valid: angle_deg > 0.0,
            display_units: true, color: [1.0, 1.0, 0.0], arrow_length: 1.0,
        }
    }

    pub fn new_radius(dim_id: u32, center: [f64; 3], radius: f64) -> Self {
        Self {
            dim_id, kind: DimensionKind::Radius, value: radius,
            unit: "mm".to_owned(), text_position: DimTextPosition::Auto,
            flyout: 5.0, p1: center, p2: center, is_valid: radius > 0.0,
            display_units: true, color: [1.0, 1.0, 0.0], arrow_length: 1.0,
        }
    }

    pub fn set_flyout(&mut self, f: f64) { self.flyout = f; }
    pub fn set_display_units(&mut self, v: bool) { self.display_units = v; }
    pub fn set_text_position(&mut self, p: DimTextPosition) { self.text_position = p; }
    pub fn set_color(&mut self, rgb: [f64; 3]) { self.color = rgb; }
    pub fn set_arrow_length(&mut self, l: f64) { self.arrow_length = l; }

    pub fn value(&self) -> f64 { self.value }
    pub fn kind(&self) -> DimensionKind { self.kind }
    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn unit(&self) -> &str { &self.unit }

    /// Format the value as a display string.
    pub fn formatted_value(&self) -> String {
        if self.display_units {
            format!("{:.3} {}", self.value, self.unit)
        } else {
            format!("{:.3}", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_label_basics() {
        let mut t = AisTextLabel::new(1, "Hello", [0.0, 0.0, 0.0]);
        assert_eq!(t.text(), "Hello");
        t.set_text("World");
        assert_eq!(t.text(), "World");
        t.set_color([1.0, 0.0, 0.0, 1.0]);
        assert!(t.has_own_color());
        assert_eq!(t.color()[0], 1.0);
    }

    #[test]
    fn text_label_position() {
        let mut t = AisTextLabel::new(2, "A", [1.0, 2.0, 3.0]);
        t.set_position([4.0, 5.0, 6.0]);
        assert_eq!(t.position(), [4.0, 5.0, 6.0]);
        t.set_height(2.5);
        assert!((t.height() - 2.5).abs() < 1e-12);
    }

    #[test]
    fn dim_length() {
        let d = AisDimension::new_length(1, [0.0,0.0,0.0], [3.0,4.0,0.0]);
        assert!(d.is_valid());
        assert_eq!(d.kind(), DimensionKind::Length);
        assert!((d.value() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn dim_angle() {
        let d = AisDimension::new_angle(2, 90.0, [0.0,0.0,0.0]);
        assert!(d.is_valid());
        assert_eq!(d.kind(), DimensionKind::Angle);
        assert!((d.value() - 90.0).abs() < 1e-10);
        let bad = AisDimension::new_angle(3, 0.0, [0.0,0.0,0.0]);
        assert!(!bad.is_valid());
    }

    #[test]
    fn dim_radius() {
        let d = AisDimension::new_radius(3, [0.0,0.0,0.0], 5.0);
        assert!(d.is_valid());
        assert_eq!(d.kind(), DimensionKind::Radius);
    }

    #[test]
    fn dim_formatted_value() {
        let d = AisDimension::new_length(1, [0.0,0.0,0.0], [1.0,0.0,0.0]);
        let s = d.formatted_value();
        assert!(s.contains("mm"));
    }
}
