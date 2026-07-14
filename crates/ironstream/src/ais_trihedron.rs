// FILE: ais_trihedron.rs
// occt: AIS_Trihedron
// occt-ref: AIS_Axis, AIS_Symbol, AIS_PlaneTrihedron

/// Display mode for an AIS trihedron (XYZ axes display).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrihedronDispMode {
    WireFrame,
    Shaded,
    ShadedWithWireFrame,
}

/// Arrow aspect for axis display.
#[derive(Clone, Debug)]
pub struct AxisAspect {
    pub color: [f32; 3],
    pub line_width: f32,
    pub arrow_length: f64,
    pub label: String,
}

impl AxisAspect {
    pub fn new(color: [f32; 3], label: impl Into<String>) -> Self {
        Self { color, line_width: 1.0, arrow_length: 1.0, label: label.into() }
    }
}

// occt: AIS_Trihedron
#[derive(Clone, Debug)]
pub struct Trihedron {
    pub origin: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
    pub z_axis: [f64; 3],
    pub size: f64,
    pub display_mode: TrihedronDispMode,
    pub x_aspect: AxisAspect,
    pub y_aspect: AxisAspect,
    pub z_aspect: AxisAspect,
    pub is_origin_color_filled: bool,
    pub labels_color: [f32; 3],
}

impl Trihedron {
    pub fn new(origin: [f64; 3], size: f64) -> Self {
        Self {
            origin,
            x_axis: [1.0,0.0,0.0],
            y_axis: [0.0,1.0,0.0],
            z_axis: [0.0,0.0,1.0],
            size,
            display_mode: TrihedronDispMode::WireFrame,
            x_aspect: AxisAspect::new([1.0,0.0,0.0], "X"),
            y_aspect: AxisAspect::new([0.0,1.0,0.0], "Y"),
            z_aspect: AxisAspect::new([0.0,0.0,1.0], "Z"),
            is_origin_color_filled: true,
            labels_color: [1.0,1.0,1.0],
        }
    }

    pub fn set_size(&mut self, size: f64) { self.size = size; }
    pub fn set_display_mode(&mut self, mode: TrihedronDispMode) { self.display_mode = mode; }

    /// Axis endpoints for rendering.
    pub fn x_end(&self) -> [f64; 3] {
        [self.origin[0]+self.size*self.x_axis[0], self.origin[1]+self.size*self.x_axis[1], self.origin[2]+self.size*self.x_axis[2]]
    }
    pub fn y_end(&self) -> [f64; 3] {
        [self.origin[0]+self.size*self.y_axis[0], self.origin[1]+self.size*self.y_axis[1], self.origin[2]+self.size*self.y_axis[2]]
    }
    pub fn z_end(&self) -> [f64; 3] {
        [self.origin[0]+self.size*self.z_axis[0], self.origin[1]+self.size*self.z_axis[1], self.origin[2]+self.size*self.z_axis[2]]
    }

    pub fn axes_are_orthonormal(&self) -> bool {
        let dot_xy = dot(self.x_axis, self.y_axis);
        let dot_xz = dot(self.x_axis, self.z_axis);
        let dot_yz = dot(self.y_axis, self.z_axis);
        dot_xy.abs() < 1e-10 && dot_xz.abs() < 1e-10 && dot_yz.abs() < 1e-10
    }
}

// occt-ref: AIS_Axis // — single axis interactive object
#[derive(Clone, Debug)]
pub struct AisAxis {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub length: f64,
    pub color: [f32; 3],
    pub label: String,
    pub is_infinite: bool,
}

impl AisAxis {
    pub fn new(origin: [f64; 3], direction: [f64; 3], length: f64) -> Self {
        let mag = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        let dir = if mag > 1e-14 { [direction[0]/mag,direction[1]/mag,direction[2]/mag] } else { [0.0,0.0,1.0] };
        Self { origin, direction: dir, length, color: [1.0,1.0,0.0], label: String::new(), is_infinite: false }
    }

    pub fn end_point(&self) -> [f64; 3] {
        [self.origin[0]+self.length*self.direction[0], self.origin[1]+self.length*self.direction[1], self.origin[2]+self.length*self.direction[2]]
    }
}

// occt-ref: AIS_PlaneTrihedron // — trihedron lying in a plane (2D)
#[derive(Clone, Debug)]
pub struct PlaneTrihedron {
    pub origin: [f64; 3],
    pub plane_normal: [f64; 3],
    pub u_axis: [f64; 3],
    pub size: f64,
}

impl PlaneTrihedron {
    pub fn new(origin: [f64; 3], plane_normal: [f64; 3], size: f64) -> Self {
        let n = normalize(plane_normal);
        let u = perp(n);
        Self { origin, plane_normal: n, u_axis: u, size }
    }

    pub fn v_axis(&self) -> [f64; 3] { cross(self.plane_normal, self.u_axis) }

    pub fn u_end(&self) -> [f64; 3] {
        add(self.origin, scale(self.size, self.u_axis))
    }

    pub fn v_end(&self) -> [f64; 3] {
        add(self.origin, scale(self.size, self.v_axis()))
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]] }
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]+b[0],a[1]+b[1],a[2]+b[2]] }
fn scale(s: f64, v: [f64; 3]) -> [f64; 3] { [s*v[0],s*v[1],s*v[2]] }
fn mag(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }
fn normalize(v: [f64; 3]) -> [f64; 3] { let m=mag(v); if m>1e-14{[v[0]/m,v[1]/m,v[2]/m]}else{[0.0,0.0,1.0]} }
fn perp(n: [f64; 3]) -> [f64; 3] { let c=if n[0].abs()<0.9{[1.0,0.0,0.0]}else{[0.0,1.0,0.0]}; normalize(cross(n,c)) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trihedron_orthonormal() {
        let t = Trihedron::new([0.0;3], 1.0);
        assert!(t.axes_are_orthonormal());
    }

    #[test]
    fn trihedron_x_end() {
        let t = Trihedron::new([0.0;3], 5.0);
        let e = t.x_end();
        assert!((e[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn ais_axis_end_point() {
        let ax = AisAxis::new([0.0;3], [1.0,0.0,0.0], 3.0);
        let e = ax.end_point();
        assert!((e[0] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn plane_trihedron_axes() {
        let pt = PlaneTrihedron::new([0.0;3], [0.0,0.0,1.0], 2.0);
        // u_axis and v_axis should be perpendicular to normal
        let u = pt.u_axis; let v = pt.v_axis();
        assert!(dot(u, pt.plane_normal).abs() < 1e-10);
        assert!(dot(v, pt.plane_normal).abs() < 1e-10);
    }
}
