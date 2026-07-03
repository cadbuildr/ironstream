// FILE: aspect_color.rs
// occt: Aspect_ColorRampColorMap, Aspect_MarkAspect, Aspect_TypeOfColorRampLaw,
//       Aspect_TypeOfMarker

/// Type of color ramp interpolation law.
/// occt: Aspect_TypeOfColorRampLaw
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectColorRampLaw {
    #[default]
    Linear,
    Logarithmic,
    Exponential,
    Sqrt,
}

/// A color ramp entry.
#[derive(Clone, Copy, Debug)]
pub struct AspectColorRampEntry {
    pub value: f64,
    pub color: [f64; 4],  // RGBA
}

impl AspectColorRampEntry {
    pub fn new(value: f64, color: [f64; 4]) -> Self { Self { value, color } }
}

/// A color ramp mapping values to colors.
/// occt: Aspect_ColorRampColorMap
#[derive(Clone, Debug)]
pub struct AspectColorRampColorMap {
    pub entries: Vec<AspectColorRampEntry>,
    pub law: AspectColorRampLaw,
}

impl Default for AspectColorRampColorMap {
    fn default() -> Self {
        Self {
            entries: vec![
                AspectColorRampEntry::new(0.0, [0.0, 0.0, 1.0, 1.0]),
                AspectColorRampEntry::new(1.0, [1.0, 0.0, 0.0, 1.0]),
            ],
            law: AspectColorRampLaw::Linear,
        }
    }
}

impl AspectColorRampColorMap {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, entry: AspectColorRampEntry) {
        // insert in sorted order by value
        let pos = self.entries.partition_point(|e| e.value < entry.value);
        self.entries.insert(pos, entry);
    }

    pub fn set_law(&mut self, l: AspectColorRampLaw) { self.law = l; }

    pub fn sample(&self, t: f64) -> [f64; 4] {
        if self.entries.is_empty() { return [0.0, 0.0, 0.0, 1.0]; }
        if t <= self.entries[0].value { return self.entries[0].color; }
        let n = self.entries.len();
        if t >= self.entries[n-1].value { return self.entries[n-1].color; }
        for i in 0..n-1 {
            let lo = &self.entries[i];
            let hi = &self.entries[i+1];
            if t >= lo.value && t <= hi.value {
                let alpha = (t - lo.value) / (hi.value - lo.value);
                let alpha = match self.law {
                    AspectColorRampLaw::Linear => alpha,
                    AspectColorRampLaw::Sqrt => alpha.sqrt(),
                    AspectColorRampLaw::Logarithmic => (1.0 + alpha * 9.0).log10(),
                    AspectColorRampLaw::Exponential => alpha * alpha,
                };
                let mut c = [0.0f64; 4];
                for k in 0..4 { c[k] = lo.color[k] * (1.0 - alpha) + hi.color[k] * alpha; }
                return c;
            }
        }
        self.entries[n-1].color
    }

    pub fn nb_entries(&self) -> usize { self.entries.len() }
    pub fn clear(&mut self) { self.entries.clear(); }
}

/// Marker shape type.
/// occt: Aspect_TypeOfMarker
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectTypeOfMarker {
    #[default]
    Point,
    Plus,
    Star,
    X,
    O,
    OPoint,
    OPlus,
    OStar,
    OX,
    Ring1,
    Ring2,
    Ring3,
    Ball,
    Dot,
    DotSquare,
    DotCircle,
}

/// Aspect of a marker (point symbol).
/// occt: Aspect_MarkAspect (also covers Graphic3d_MarkerImage)
#[derive(Clone, Debug)]
pub struct AspectMarkAspect {
    pub marker_type: AspectTypeOfMarker,
    pub scale: f32,
    pub color: [f64; 4],
}

impl Default for AspectMarkAspect {
    fn default() -> Self {
        Self { marker_type: AspectTypeOfMarker::Point, scale: 1.0, color: [1.0, 1.0, 0.0, 1.0] }
    }
}

impl AspectMarkAspect {
    pub fn new() -> Self { Self::default() }

    pub fn set_marker(&mut self, m: AspectTypeOfMarker) { self.marker_type = m; }
    pub fn set_scale(&mut self, s: f32) { self.scale = s.max(0.1); }
    pub fn set_color(&mut self, c: [f64; 4]) { self.color = c; }

    pub fn is_custom(&self) -> bool { false }
}

/// Fill method for 2D polygons.
/// occt: Aspect_FillMethod (used with Aspect_Background)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectFillMethod {
    #[default]
    None,
    Solid,
    HorizontalGradient,
    VerticalGradient,
    CornerGradient,
    EllipticGradient,
}

/// Hatch style for filling.
/// occt: Aspect_HatchStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectHatchStyle {
    #[default]
    Solid,
    Vertical,
    Horizontal,
    Diagonal45,
    Diagonal135,
    DiagCross,
    HorizontalVerticalCross,
}

/// Aspect of a filled area / face interior.
/// occt: Aspect_InteriorStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AspectInteriorStyle {
    #[default]
    Empty,
    Hollow,
    Hatch,
    Filled,
    HiddenLine,
    Point,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_ramp_defaults() {
        let c = AspectColorRampColorMap::new();
        assert_eq!(c.nb_entries(), 2);
        // At t=0 should be blue
        let s = c.sample(0.0);
        assert!((s[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn color_ramp_add_sorted() {
        let mut c = AspectColorRampColorMap::new();
        c.add(AspectColorRampEntry::new(0.5, [0.0, 1.0, 0.0, 1.0]));
        assert_eq!(c.nb_entries(), 3);
        // At 0.5, sample should be green
        let s = c.sample(0.5);
        assert!((s[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn color_ramp_clamp() {
        let c = AspectColorRampColorMap::default();
        // Below range → first entry (blue)
        let below = c.sample(-5.0);
        assert!((below[2] - 1.0).abs() < 1e-10);
        // Above range → last entry (red)
        let above = c.sample(10.0);
        assert!((above[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn mark_aspect_defaults() {
        let m = AspectMarkAspect::default();
        assert_eq!(m.marker_type, AspectTypeOfMarker::Point);
        assert!((m.scale - 1.0).abs() < 1e-6);
        assert!(!m.is_custom());
    }

    #[test]
    fn hatch_and_fill_defaults() {
        assert_eq!(AspectFillMethod::default(), AspectFillMethod::None);
        assert_eq!(AspectHatchStyle::default(), AspectHatchStyle::Solid);
        assert_eq!(AspectInteriorStyle::default(), AspectInteriorStyle::Empty);
    }
}
