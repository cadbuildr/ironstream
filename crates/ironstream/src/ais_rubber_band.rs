// FILE: ais_rubber_band.rs
// occt: AIS_RubberBand, AIS_ViewCube

/// Shape of rubber band selection.
/// occt: AIS_RubberBand (uses rectangle or polygon)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RubberBandType {
    Rectangle,
    Polygon,
    Line,
}

impl Default for RubberBandType {
    fn default() -> Self { Self::Rectangle }
}

/// 2D screen rubber-band selection region.
/// occt: AIS_RubberBand
#[derive(Clone, Debug)]
pub struct AisRubberBand {
    pub band_type: RubberBandType,
    pub points: Vec<[f64; 2]>,  // 2D screen coordinates
    pub line_color: [f64; 3],
    pub fill_color: [f64; 4],   // RGBA (alpha for transparency)
    pub line_width: f64,
    pub is_filled: bool,
    pub is_visible: bool,
}

impl Default for AisRubberBand {
    fn default() -> Self {
        Self {
            band_type: RubberBandType::Rectangle,
            points: Vec::new(),
            line_color: [1.0, 1.0, 0.0],
            fill_color: [1.0, 1.0, 0.0, 0.4],
            line_width: 1.0,
            is_filled: true,
            is_visible: false,
        }
    }
}

impl AisRubberBand {
    pub fn new() -> Self { Self::default() }

    pub fn new_rectangle(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let mut r = Self::default();
        r.band_type = RubberBandType::Rectangle;
        r.points = vec![[x1, y1], [x2, y2]];
        r
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push([x, y]);
        if self.points.len() > 2 { self.band_type = RubberBandType::Polygon; }
    }

    pub fn update_rectangle(&mut self, x2: f64, y2: f64) {
        if self.points.len() >= 2 { self.points[1] = [x2, y2]; }
        else { self.points.push([x2, y2]); }
    }

    pub fn clear_points(&mut self) { self.points.clear(); }

    pub fn set_line_color(&mut self, c: [f64; 3]) { self.line_color = c; }
    pub fn set_fill_color(&mut self, c: [f64; 4]) { self.fill_color = c; }
    pub fn set_fill_transparency(&mut self, t: f64) { self.fill_color[3] = 1.0 - t.clamp(0.0,1.0); }
    pub fn set_filled(&mut self, v: bool) { self.is_filled = v; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_line_width(&mut self, w: f64) { self.line_width = w.max(0.5); }

    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn is_visible(&self) -> bool { self.is_visible }
    pub fn is_filled(&self) -> bool { self.is_filled }
    pub fn band_type(&self) -> RubberBandType { self.band_type }

    /// Bounding rectangle: (min_x, min_y, max_x, max_y) or None if <2 points.
    pub fn bounding_rect(&self) -> Option<[f64; 4]> {
        if self.points.len() < 2 { return None; }
        let (mut xmin, mut ymin) = (f64::INFINITY, f64::INFINITY);
        let (mut xmax, mut ymax) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in &self.points {
            if p[0] < xmin { xmin = p[0]; }
            if p[0] > xmax { xmax = p[0]; }
            if p[1] < ymin { ymin = p[1]; }
            if p[1] > ymax { ymax = p[1]; }
        }
        Some([xmin, ymin, xmax, ymax])
    }
}

/// View orientation cube widget (compass-like).
/// occt: AIS_ViewCube
#[derive(Clone, Debug)]
pub struct AisViewCube {
    pub object_id: u32,
    pub size: f64,
    pub position: [f64; 2],     // screen position (normalized 0-1)
    pub is_visible: bool,
    pub draw_axes: bool,
    pub font_height: f64,
    pub box_facet_color: [f64; 3],
    pub box_edge_color: [f64; 3],
    pub box_corner_color: [f64; 3],
    pub animation_duration: f64,
}

impl AisViewCube {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            size: 70.0,
            position: [0.9, 0.1],
            is_visible: true,
            draw_axes: true,
            font_height: 12.0,
            box_facet_color: [0.7, 0.7, 0.85],
            box_edge_color: [0.3, 0.3, 0.4],
            box_corner_color: [0.5, 0.5, 0.6],
            animation_duration: 0.5,
        }
    }

    pub fn set_size(&mut self, s: f64) { self.size = s.max(10.0); }
    pub fn set_position(&mut self, x: f64, y: f64) { self.position = [x, y]; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_draw_axes(&mut self, v: bool) { self.draw_axes = v; }
    pub fn set_animation_duration(&mut self, d: f64) { self.animation_duration = d.max(0.0); }

    pub fn size(&self) -> f64 { self.size }
    pub fn is_visible(&self) -> bool { self.is_visible }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rubber_band_default() {
        let r = AisRubberBand::new();
        assert_eq!(r.band_type(), RubberBandType::Rectangle);
        assert_eq!(r.nb_points(), 0);
        assert!(!r.is_visible());
    }

    #[test]
    fn rubber_band_rectangle() {
        let mut r = AisRubberBand::new_rectangle(10.0, 20.0, 100.0, 80.0);
        assert_eq!(r.nb_points(), 2);
        let bb = r.bounding_rect().unwrap();
        assert!((bb[0] - 10.0).abs() < 1e-10);
        assert!((bb[2] - 100.0).abs() < 1e-10);
        r.update_rectangle(150.0, 100.0);
        let bb2 = r.bounding_rect().unwrap();
        assert!((bb2[2] - 150.0).abs() < 1e-10);
    }

    #[test]
    fn rubber_band_polygon() {
        let mut r = AisRubberBand::new();
        r.add_point(0.0, 0.0);
        r.add_point(10.0, 0.0);
        r.add_point(5.0, 8.0);
        assert_eq!(r.band_type(), RubberBandType::Polygon);
        assert_eq!(r.nb_points(), 3);
    }

    #[test]
    fn view_cube_defaults() {
        let mut vc = AisViewCube::new(1);
        assert!(vc.is_visible());
        assert!((vc.size() - 70.0).abs() < 1e-10);
        vc.set_visible(false);
        assert!(!vc.is_visible());
    }
}
