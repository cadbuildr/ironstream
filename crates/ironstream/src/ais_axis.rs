// FILE: ais_axis.rs
// occt: AIS_Axis, AIS_Point, AIS_Circle
// occt-ref: AIS_ConnectedInteractive

/// An AIS axis (line defined by origin + direction).
/// occt: AIS_Axis
#[derive(Clone, Debug)]
pub struct AisAxis {
    pub object_id: u32,
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub length: f64,
    pub is_infinite: bool,
    pub is_visible: bool,
    pub color: [f64; 3],
}

impl AisAxis {
    pub fn new(object_id: u32, origin: [f64; 3], direction: [f64; 3]) -> Self {
        let len = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let d = if len > 1e-14 { [direction[0]/len, direction[1]/len, direction[2]/len] } else { [0.0, 0.0, 1.0] };
        Self {
            object_id, origin, direction: d,
            length: 100.0, is_infinite: false, is_visible: true,
            color: [1.0, 0.0, 0.0],
        }
    }

    pub fn set_length(&mut self, l: f64) { self.length = l.max(0.0); }
    pub fn set_infinite(&mut self, v: bool) { self.is_infinite = v; }
    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }

    pub fn end_point(&self) -> [f64; 3] {
        [self.origin[0]+self.direction[0]*self.length,
         self.origin[1]+self.direction[1]*self.length,
         self.origin[2]+self.direction[2]*self.length]
    }
}

/// An AIS point (marker at a 3D position).
/// occt: AIS_Point
#[derive(Clone, Debug)]
pub struct AisPoint {
    pub object_id: u32,
    pub position: [f64; 3],
    pub marker_type: AisMarkerType,
    pub color: [f64; 3],
    pub is_visible: bool,
    pub marker_size: f64,
}

/// Marker style for AIS_Point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AisMarkerType {
    #[default]
    Point,
    Plus,
    Star,
    Circle,
    Square,
    Cross,
    Custom,
}

impl AisPoint {
    pub fn new(object_id: u32, position: [f64; 3]) -> Self {
        Self {
            object_id, position,
            marker_type: AisMarkerType::Point,
            color: [1.0, 1.0, 0.0],
            is_visible: true,
            marker_size: 1.0,
        }
    }

    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_marker(&mut self, m: AisMarkerType) { self.marker_type = m; }
    pub fn set_marker_size(&mut self, s: f64) { self.marker_size = s.max(0.1); }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }

    pub fn position(&self) -> [f64; 3] { self.position }
    pub fn is_visible(&self) -> bool { self.is_visible }
}

/// An AIS circle.
/// occt: AIS_Circle
#[derive(Clone, Debug)]
pub struct AisCircle {
    pub object_id: u32,
    pub center: [f64; 3],
    pub normal: [f64; 3],
    pub radius: f64,
    pub param_start: f64,
    pub param_end: f64,
    pub is_filled: bool,
    pub color: [f64; 3],
    pub is_visible: bool,
}

impl AisCircle {
    pub fn new(object_id: u32, center: [f64; 3], normal: [f64; 3], radius: f64) -> Self {
        let len = (normal[0]*normal[0]+normal[1]*normal[1]+normal[2]*normal[2]).sqrt();
        let n = if len > 1e-14 { [normal[0]/len, normal[1]/len, normal[2]/len] } else { [0.0, 0.0, 1.0] };
        Self {
            object_id, center, normal: n, radius: radius.max(0.0),
            param_start: 0.0, param_end: std::f64::consts::TAU,
            is_filled: false, color: [1.0, 1.0, 0.0], is_visible: true,
        }
    }

    pub fn set_radius(&mut self, r: f64) { self.radius = r.max(0.0); }
    pub fn set_range(&mut self, start: f64, end: f64) {
        self.param_start = start;
        self.param_end = end;
    }
    pub fn set_filled(&mut self, v: bool) { self.is_filled = v; }
    pub fn set_color(&mut self, c: [f64; 3]) { self.color = c; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }

    pub fn is_arc(&self) -> bool {
        (self.param_end - self.param_start - std::f64::consts::TAU).abs() > 1e-10
    }

    pub fn arc_length(&self) -> f64 {
        self.radius * (self.param_end - self.param_start).abs()
    }
}

/// A connected interactive object (shares presentation with another).
// occt-ref: AIS_ConnectedInteractive
#[derive(Clone, Debug)]
pub struct AisConnectedInteractive {
    pub object_id: u32,
    pub reference_id: u32,
    pub location: [f64; 16],  // 4×4 flat row-major transform
    pub is_visible: bool,
}

impl AisConnectedInteractive {
    pub fn new(object_id: u32, reference_id: u32) -> Self {
        // Identity 4×4
        let mut loc = [0.0f64; 16];
        loc[0] = 1.0; loc[5] = 1.0; loc[10] = 1.0; loc[15] = 1.0;
        Self { object_id, reference_id, location: loc, is_visible: true }
    }

    pub fn set_translation(&mut self, tx: f64, ty: f64, tz: f64) {
        self.location[3]  = tx;
        self.location[7]  = ty;
        self.location[11] = tz;
    }

    pub fn translation(&self) -> [f64; 3] {
        [self.location[3], self.location[7], self.location[11]]
    }

    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn is_connected(&self) -> bool { self.reference_id > 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_end_point() {
        let a = AisAxis::new(1, [0.0;3], [0.0,0.0,1.0]);
        let ep = a.end_point();
        assert!((ep[2] - 100.0).abs() < 1e-10);
    }

    #[test]
    fn axis_normalizes_direction() {
        let a = AisAxis::new(1, [0.0;3], [0.0,0.0,5.0]);
        assert!((a.direction[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn point_marker() {
        let mut p = AisPoint::new(2, [1.0,2.0,3.0]);
        p.set_marker(AisMarkerType::Star);
        assert_eq!(p.marker_type, AisMarkerType::Star);
        assert!(p.is_visible());
    }

    #[test]
    fn circle_arc_length() {
        use std::f64::consts::PI;
        let mut c = AisCircle::new(3, [0.0;3], [0.0,0.0,1.0], 5.0);
        c.set_range(0.0, PI);
        assert!(c.is_arc());
        assert!((c.arc_length() - 5.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn circle_full_not_arc() {
        let c = AisCircle::new(3, [0.0;3], [0.0,0.0,1.0], 1.0);
        assert!(!c.is_arc());
    }

    #[test]
    fn connected_interactive_translation() {
        let mut ci = AisConnectedInteractive::new(4, 10);
        assert!(ci.is_connected());
        ci.set_translation(1.0, 2.0, 3.0);
        let t = ci.translation();
        assert!((t[0] - 1.0).abs() < 1e-10);
        assert!((t[1] - 2.0).abs() < 1e-10);
        assert!((t[2] - 3.0).abs() < 1e-10);
    }
}
