// FILE: ais_colored_shape.rs
// occt: AIS_ColoredShape, AIS_ColorScale, AIS_ColoredDrawer

use std::collections::HashMap;

/// Color in RGBA (occt Quantity_ColorRGBA).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for ColorRgba {
    fn default() -> Self { Self::WHITE }
}

impl ColorRgba {
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self { Self::new(r, g, b, 1.0) }
    pub fn is_transparent(&self) -> bool { self.a < 1.0 }
    pub fn to_array(&self) -> [f32; 4] { [self.r, self.g, self.b, self.a] }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(a.r+(b.r-a.r)*t, a.g+(b.g-a.g)*t, a.b+(b.b-a.b)*t, a.a+(b.a-a.a)*t)
    }

    pub fn brightness(&self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }
}

// occt: AIS_ColoredDrawer — per-subshape override drawer
#[derive(Clone, Debug)]
pub struct ColoredDrawer {
    pub color: Option<ColorRgba>,
    pub transparency: Option<f32>,
    pub line_width: Option<f32>,
    pub is_hidden: bool,
}

impl ColoredDrawer {
    pub fn new() -> Self { Self { color: None, transparency: None, line_width: None, is_hidden: false } }
    pub fn with_color(mut self, c: ColorRgba) -> Self { self.color = Some(c); self }
    pub fn with_transparency(mut self, t: f32) -> Self { self.transparency = Some(t); self }
    pub fn is_overriding(&self) -> bool { self.color.is_some() || self.transparency.is_some() }
}

impl Default for ColoredDrawer {
    fn default() -> Self { Self::new() }
}

/// Shape reference by index (stands in for TopoDS_Shape handle).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeRef(pub u64);

// occt: AIS_ColoredShape
#[derive(Clone, Debug, Default)]
pub struct ColoredShape {
    pub shape_id: Option<ShapeRef>,
    pub default_color: ColorRgba,
    pub sub_shape_drawers: HashMap<ShapeRef, ColoredDrawer>,
}

impl ColoredShape {
    pub fn new(shape_id: u64) -> Self {
        Self { shape_id: Some(ShapeRef(shape_id)), default_color: ColorRgba::WHITE, sub_shape_drawers: HashMap::new() }
    }

    pub fn set_custom_color(&mut self, shape: ShapeRef, color: ColorRgba) {
        self.sub_shape_drawers.entry(shape).or_insert_with(ColoredDrawer::new).color = Some(color);
    }

    pub fn set_transparency(&mut self, shape: ShapeRef, t: f32) {
        self.sub_shape_drawers.entry(shape).or_insert_with(ColoredDrawer::new).transparency = Some(t);
    }

    pub fn unset_color(&mut self, shape: &ShapeRef) {
        if let Some(d) = self.sub_shape_drawers.get_mut(shape) { d.color = None; }
    }

    pub fn color_of(&self, shape: &ShapeRef) -> ColorRgba {
        self.sub_shape_drawers.get(shape).and_then(|d| d.color).unwrap_or(self.default_color)
    }

    pub fn nb_custom_shapes(&self) -> usize { self.sub_shape_drawers.len() }
}

// occt: AIS_ColorScale — color legend / color ramp
#[derive(Clone, Debug)]
pub struct ColorScale {
    pub min_value: f64,
    pub max_value: f64,
    pub nb_intervals: usize,
    pub title: String,
    pub color_map: Vec<ColorRgba>,
    pub is_reversed: bool,
    pub is_logarithmic: bool,
}

impl ColorScale {
    pub fn new(min: f64, max: f64, nb: usize) -> Self {
        let colors = Self::default_rainbow(nb);
        Self {
            min_value: min, max_value: max, nb_intervals: nb, title: String::new(),
            color_map: colors, is_reversed: false, is_logarithmic: false,
        }
    }

    pub fn color_for_value(&self, v: f64) -> ColorRgba {
        if self.color_map.is_empty() { return ColorRgba::WHITE; }
        let v_clamped = v.clamp(self.min_value, self.max_value);
        let t = if (self.max_value - self.min_value).abs() < 1e-300 { 0.0 }
            else { (v_clamped - self.min_value) / (self.max_value - self.min_value) };
        let t = if self.is_reversed { 1.0 - t } else { t };
        let idx_f = t * (self.color_map.len() - 1) as f64;
        let idx = idx_f.floor() as usize;
        let frac = (idx_f - idx as f64) as f32;
        if idx + 1 < self.color_map.len() {
            ColorRgba::lerp(self.color_map[idx], self.color_map[idx + 1], frac)
        } else {
            *self.color_map.last().unwrap()
        }
    }

    fn default_rainbow(nb: usize) -> Vec<ColorRgba> {
        (0..nb).map(|i| {
            let t = i as f32 / (nb - 1).max(1) as f32;
            ColorRgba::from_rgb(t, 0.5*(1.0-t), 1.0-t)
        }).collect()
    }

    pub fn heatmap_colors(nb: usize) -> Vec<ColorRgba> {
        // blue → cyan → green → yellow → red
        let palette = [
            ColorRgba::BLUE,
            ColorRgba::from_rgb(0.0,1.0,1.0),
            ColorRgba::GREEN,
            ColorRgba::from_rgb(1.0,1.0,0.0),
            ColorRgba::RED,
        ];
        (0..nb).map(|i| {
            let t = i as f32 / (nb - 1).max(1) as f32;
            let seg = (t * (palette.len()-1) as f32).floor() as usize;
            let seg = seg.min(palette.len()-2);
            let frac = t * (palette.len()-1) as f32 - seg as f32;
            ColorRgba::lerp(palette[seg], palette[seg+1], frac)
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_lerp() {
        let c = ColorRgba::lerp(ColorRgba::BLACK, ColorRgba::WHITE, 0.5);
        assert!((c.r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn colored_shape_custom_color() {
        let mut s = ColoredShape::new(1);
        let face = ShapeRef(10);
        s.set_custom_color(face.clone(), ColorRgba::RED);
        assert_eq!(s.color_of(&face), ColorRgba::RED);
        assert_eq!(s.nb_custom_shapes(), 1);
    }

    #[test]
    fn color_scale_interpolation() {
        let cs = ColorScale::new(0.0, 10.0, 10);
        let c_min = cs.color_for_value(0.0);
        let c_max = cs.color_for_value(10.0);
        // Colors should differ
        assert!((c_min.r - c_max.r).abs() > 0.01 || (c_min.b - c_max.b).abs() > 0.01);
    }

    #[test]
    fn heatmap_colors() {
        let colors = ColorScale::heatmap_colors(5);
        assert_eq!(colors.len(), 5);
    }
}
