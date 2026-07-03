// FILE: rect_2d.rs
// occt: Bnd_Box2d, Bnd_Rect (simplified), Rect_Rectan (OCC rectangle),
//       Bnd_Rectangle (axis-aligned rectangle in 2D)

/// 2D axis-aligned bounding rectangle.
/// occt: Bnd_Box2d / Bnd_Rectangle
#[derive(Clone, Debug)]
pub struct Rect2d {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub is_void: bool,
}

impl Default for Rect2d {
    fn default() -> Self {
        Self {
            x_min: f64::MAX,
            y_min: f64::MAX,
            x_max: f64::MIN,
            y_max: f64::MIN,
            is_void: true,
        }
    }
}

impl Rect2d {
    pub fn new() -> Self { Self::default() }

    pub fn from_min_max(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
        Self { x_min, y_min, x_max, y_max, is_void: false }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        if self.is_void {
            self.x_min = x;
            self.y_min = y;
            self.x_max = x;
            self.y_max = y;
            self.is_void = false;
        } else {
            if x < self.x_min { self.x_min = x; }
            if x > self.x_max { self.x_max = x; }
            if y < self.y_min { self.y_min = y; }
            if y > self.y_max { self.y_max = y; }
        }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        !self.is_void && x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    pub fn overlaps(&self, other: &Rect2d) -> bool {
        if self.is_void || other.is_void { return false; }
        self.x_min <= other.x_max && self.x_max >= other.x_min &&
        self.y_min <= other.y_max && self.y_max >= other.y_min
    }

    pub fn width(&self) -> f64 { if self.is_void { 0.0 } else { self.x_max - self.x_min } }
    pub fn height(&self) -> f64 { if self.is_void { 0.0 } else { self.y_max - self.y_min } }
    pub fn area(&self) -> f64 { if self.is_void { 0.0 } else { self.width() * self.height() } }

    pub fn center(&self) -> [f64; 2] {
        if self.is_void { [0.0; 2] } else { [(self.x_min + self.x_max) * 0.5, (self.y_min + self.y_max) * 0.5] }
    }

    pub fn diagonal(&self) -> f64 {
        let w = self.width();
        let h = self.height();
        (w*w + h*h).sqrt()
    }

    pub fn combine(&mut self, other: &Rect2d) {
        if other.is_void { return; }
        self.add_point(other.x_min, other.y_min);
        self.add_point(other.x_max, other.y_max);
    }

    pub fn expand(&mut self, delta: f64) {
        if self.is_void { return; }
        self.x_min -= delta;
        self.y_min -= delta;
        self.x_max += delta;
        self.y_max += delta;
    }

    pub fn set_void(&mut self) {
        self.x_min = f64::MAX;
        self.y_min = f64::MAX;
        self.x_max = f64::MIN;
        self.y_max = f64::MIN;
        self.is_void = true;
    }
}

/// Oriented rectangle in 2D (with rotation).
/// occt: Rect_OriRectangle (simplified stub)
#[derive(Clone, Debug)]
pub struct Rect2dOriented {
    pub center: [f64; 2],
    pub width: f64,
    pub height: f64,
    pub angle: f64,  // radians
}

impl Rect2dOriented {
    pub fn new(cx: f64, cy: f64, w: f64, h: f64, angle: f64) -> Self {
        Self { center: [cx, cy], width: w.abs(), height: h.abs(), angle }
    }

    pub fn local_to_global(&self, local_x: f64, local_y: f64) -> [f64; 2] {
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();
        let global_x = self.center[0] + local_x * cos_a - local_y * sin_a;
        let global_y = self.center[1] + local_x * sin_a + local_y * cos_a;
        [global_x, global_y]
    }

    pub fn contains_local(&self, lx: f64, ly: f64) -> bool {
        lx.abs() <= self.width * 0.5 && ly.abs() <= self.height * 0.5
    }

    pub fn to_axis_aligned_bounds(&self) -> Rect2d {
        let corners = [
            self.local_to_global(-self.width*0.5, -self.height*0.5),
            self.local_to_global( self.width*0.5, -self.height*0.5),
            self.local_to_global( self.width*0.5,  self.height*0.5),
            self.local_to_global(-self.width*0.5,  self.height*0.5),
        ];
        let mut r = Rect2d::new();
        for c in &corners { r.add_point(c[0], c[1]); }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_2d_add_contains() {
        let mut r = Rect2d::new();
        assert!(r.is_void);
        r.add_point(0.0, 0.0);
        r.add_point(1.0, 1.0);
        assert!(!r.is_void);
        assert!(r.contains(0.5, 0.5));
        assert!(!r.contains(2.0, 0.0));
    }

    #[test]
    fn rect_2d_width_height_area() {
        let r = Rect2d::from_min_max(0.0, 0.0, 2.0, 3.0);
        assert_eq!(r.width(), 2.0);
        assert_eq!(r.height(), 3.0);
        assert_eq!(r.area(), 6.0);
    }

    #[test]
    fn rect_2d_overlap() {
        let r1 = Rect2d::from_min_max(0.0, 0.0, 1.0, 1.0);
        let r2 = Rect2d::from_min_max(0.5, 0.5, 2.0, 2.0);
        assert!(r1.overlaps(&r2));
        let r3 = Rect2d::from_min_max(2.0, 2.0, 3.0, 3.0);
        assert!(!r1.overlaps(&r3));
    }

    #[test]
    fn rect_2d_expand() {
        let mut r = Rect2d::from_min_max(0.0, 0.0, 1.0, 1.0);
        r.expand(0.5);
        assert!((r.x_min - (-0.5)).abs() < 1e-10);
        assert!((r.x_max - 1.5).abs() < 1e-10);
    }

    #[test]
    fn oriented_rect() {
        let or = Rect2dOriented::new(0.0, 0.0, 2.0, 1.0, 0.0);
        assert!(or.contains_local(0.5, 0.25));
        assert!(!or.contains_local(2.0, 0.0));
        let aabb = or.to_axis_aligned_bounds();
        assert!(!aabb.is_void);
    }
}
