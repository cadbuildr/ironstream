// FILE: ais_color_scale.rs
// occt: AIS_ColorScale, AIS_ColorRange

/// Color scale label type.
/// occt: AIS_ColorScale::LabelType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScaleLabelType {
    None,
    LabelMin,
    LabelMax,
    LabelRange,
    LabelCenter,
}

impl Default for ColorScaleLabelType {
    fn default() -> Self { Self::LabelRange }
}

/// Position of the color scale on screen.
/// occt: AIS_ColorScale position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScalePosition {
    Left,
    Right,
    Custom,
}

impl Default for ColorScalePosition {
    fn default() -> Self { Self::Right }
}

/// A single color entry in the scale.
#[derive(Clone, Copy, Debug)]
pub struct ColorEntry {
    pub color: [f64; 3],
    pub min_value: f64,
    pub max_value: f64,
}

impl ColorEntry {
    pub fn new(color: [f64; 3], min: f64, max: f64) -> Self {
        Self { color, min_value: min, max_value: max }
    }
}

/// Color scale widget for visualization legends.
/// occt: AIS_ColorScale
#[derive(Clone, Debug)]
pub struct AisColorScale {
    pub object_id: u32,
    pub min_value: f64,
    pub max_value: f64,
    pub nb_intervals: usize,
    pub title: String,
    pub label_type: ColorScaleLabelType,
    pub position: ColorScalePosition,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_reversed: bool,
    pub is_smooth: bool,
    pub is_logarithmic: bool,
    pub is_visible: bool,
    pub custom_colors: Vec<[f64; 3]>,  // empty = use built-in rainbow
    pub format: String,
}

impl AisColorScale {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            min_value: 0.0,
            max_value: 1.0,
            nb_intervals: 20,
            title: String::new(),
            label_type: ColorScaleLabelType::LabelRange,
            position: ColorScalePosition::Right,
            x: 10, y: 10, width: 60, height: 200,
            is_reversed: false,
            is_smooth: false,
            is_logarithmic: false,
            is_visible: true,
            custom_colors: Vec::new(),
            format: String::from("%.4g"),
        }
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        if max > min { self.min_value = min; self.max_value = max; }
    }

    pub fn set_nb_intervals(&mut self, n: usize) { self.nb_intervals = n.max(1).min(1000); }
    pub fn set_title(&mut self, t: &str) { self.title = t.to_string(); }
    pub fn set_reversed(&mut self, v: bool) { self.is_reversed = v; }
    pub fn set_smooth(&mut self, v: bool) { self.is_smooth = v; }
    pub fn set_logarithmic(&mut self, v: bool) { self.is_logarithmic = v; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_format(&mut self, f: &str) { self.format = f.to_string(); }
    pub fn set_position(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.x = x; self.y = y; self.width = w.max(10); self.height = h.max(10);
    }

    pub fn add_custom_color(&mut self, c: [f64; 3]) { self.custom_colors.push(c); }
    pub fn clear_custom_colors(&mut self) { self.custom_colors.clear(); }

    pub fn min_value(&self) -> f64 { self.min_value }
    pub fn max_value(&self) -> f64 { self.max_value }
    pub fn nb_intervals(&self) -> usize { self.nb_intervals }
    pub fn is_visible(&self) -> bool { self.is_visible }
    pub fn has_custom_colors(&self) -> bool { !self.custom_colors.is_empty() }

    /// Get interpolated color for a value in [min, max].
    pub fn color_at_value(&self, val: f64) -> [f64; 3] {
        let t = if self.max_value > self.min_value {
            ((val - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0)
        } else { 0.5 };
        let t = if self.is_reversed { 1.0 - t } else { t };
        if !self.custom_colors.is_empty() {
            let idx = (t * (self.custom_colors.len() - 1) as f64) as usize;
            return self.custom_colors[idx.min(self.custom_colors.len()-1)];
        }
        // Default rainbow: blue→cyan→green→yellow→red
        let hue = t * 240.0; // 0=red, 240=blue, reversed: 240→0
        let hue = 240.0 - hue;
        let h = hue / 60.0;
        let i = h.floor() as usize % 6;
        let f = h - h.floor();
        let r = match i { 0|5 => 1.0, 1 => 1.0-f, 2|3 => 0.0, _ => f };
        let g = match i { 0 => f, 1|2 => 1.0, 3 => 1.0-f, _ => 0.0 };
        let b = match i { 2 => f, 3|4 => 1.0, 5 => 1.0-f, _ => 0.0 };
        [r, g, b]
    }

    /// Format a value according to the format string (stub: use fixed 4 decimals).
    pub fn format_label(&self, val: f64) -> String {
        format!("{:.4}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_scale_defaults() {
        let cs = AisColorScale::new(1);
        assert!((cs.min_value() - 0.0).abs() < 1e-10);
        assert!((cs.max_value() - 1.0).abs() < 1e-10);
        assert_eq!(cs.nb_intervals(), 20);
        assert!(cs.is_visible());
    }

    #[test]
    fn color_scale_set_range() {
        let mut cs = AisColorScale::new(1);
        cs.set_range(-5.0, 100.0);
        assert!((cs.min_value() + 5.0).abs() < 1e-10);
        assert!((cs.max_value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn color_at_value_extremes() {
        let cs = AisColorScale::new(1);
        let c_min = cs.color_at_value(0.0);
        let c_max = cs.color_at_value(1.0);
        // Just check they are valid RGB
        for k in 0..3 { assert!(c_min[k] >= 0.0 && c_min[k] <= 1.0); }
        for k in 0..3 { assert!(c_max[k] >= 0.0 && c_max[k] <= 1.0); }
    }

    #[test]
    fn custom_colors() {
        let mut cs = AisColorScale::new(1);
        cs.add_custom_color([1.0, 0.0, 0.0]);
        cs.add_custom_color([0.0, 1.0, 0.0]);
        assert!(cs.has_custom_colors());
        let c = cs.color_at_value(0.0);
        assert!((c[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn nb_intervals_clamped() {
        let mut cs = AisColorScale::new(1);
        cs.set_nb_intervals(0);
        assert_eq!(cs.nb_intervals(), 1);
        cs.set_nb_intervals(9999);
        assert_eq!(cs.nb_intervals(), 1000);
    }
}
