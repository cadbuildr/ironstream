// FILE: vrml_perspective_camera.rs
// occt: Vrml_PerspectiveCamera
//
// Faithful port of OCCT Vrml_PerspectiveCamera (DataExchange/TKDEVRML/Vrml/
// Vrml_PerspectiveCamera.hxx): the VRML 1.0 `PerspectiveCamera` node,
// specifying perspective camera properties (position, orientation, fov).
// Defaults per VRML 1.0 spec. Print emits non-default fields.

/// Local model of gp_Pnt (position).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlPerspCamPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlPerspCamPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlPerspCamPnt { x, y, z }
    }
}

/// Local model of gp_Dir (direction).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlPerspCamDir {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlPerspCamDir {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlPerspCamDir { x, y, z }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_persp_cam_real(v: f64) -> String {
    let p = 6usize;
    let sci = format!("{:.*e}", p - 1, v);
    let epos = sci.find('e').expect("exponent");
    let exp: i32 = sci[epos + 1..].parse().expect("exp digits");
    if exp < -4 || exp >= p as i32 {
        let mant = sci[..epos].trim_end_matches('0').trim_end_matches('.');
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else {
        let prec = (p as i32 - 1 - exp).max(0) as usize;
        let fixed = format!("{:.*}", prec, v);
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}

/// Port of Vrml_PerspectiveCamera.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlPerspectiveCamera {
    my_position: VrmlPerspCamPnt,
    my_orientation: VrmlPerspCamDir,
    my_field_of_view: f64,
}

impl VrmlPerspectiveCamera {
    /// Vrml_PerspectiveCamera():
    /// position defaults to (0 0 1)
    /// orientation defaults to (0 0 1) [identity]
    /// fieldOfView defaults to pi/4 (approximately 0.785398)
    pub fn new() -> Self {
        VrmlPerspectiveCamera {
            my_position: VrmlPerspCamPnt::new(0.0, 0.0, 1.0),
            my_orientation: VrmlPerspCamDir::new(0.0, 0.0, 1.0),
            my_field_of_view: std::f64::consts::PI / 4.0,
        }
    }

    pub fn set_position(&mut self, a_position: VrmlPerspCamPnt) {
        self.my_position = a_position;
    }

    pub fn position(&self) -> VrmlPerspCamPnt {
        self.my_position
    }

    pub fn set_orientation(&mut self, a_orientation: VrmlPerspCamDir) {
        self.my_orientation = a_orientation;
    }

    pub fn orientation(&self) -> VrmlPerspCamDir {
        self.my_orientation
    }

    pub fn set_field_of_view(&mut self, a_fov: f64) {
        self.my_field_of_view = a_fov;
    }

    pub fn field_of_view(&self) -> f64 {
        self.my_field_of_view
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("PerspectiveCamera {\n");

        let default_fov = std::f64::consts::PI / 4.0;

        // position (default 0 0 1)
        if (self.my_position.x - 0.0).abs() > 0.0001
            || (self.my_position.y - 0.0).abs() > 0.0001
            || (self.my_position.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    position\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_persp_cam_real(self.my_position.x),
                vrml_persp_cam_real(self.my_position.y),
                vrml_persp_cam_real(self.my_position.z)
            ));
        }

        // orientation (default 0 0 1)
        if (self.my_orientation.x - 0.0).abs() > 0.0001
            || (self.my_orientation.y - 0.0).abs() > 0.0001
            || (self.my_orientation.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    orientation\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_persp_cam_real(self.my_orientation.x),
                vrml_persp_cam_real(self.my_orientation.y),
                vrml_persp_cam_real(self.my_orientation.z)
            ));
        }

        // fieldOfView (default pi/4)
        if (self.my_field_of_view - default_fov).abs() > 0.0001 {
            an_ostream.push_str("    fieldOfView\t");
            an_ostream.push_str(&vrml_persp_cam_real(self.my_field_of_view));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlPerspectiveCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_camera() {
        let cam = VrmlPerspectiveCamera::new();
        let pos = cam.position();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, 1.0);
        let fov = cam.field_of_view();
        assert!((fov - std::f64::consts::PI / 4.0).abs() < 0.0001);
    }

    #[test]
    fn default_prints_empty_node() {
        let cam = VrmlPerspectiveCamera::new();
        let mut out = String::new();
        cam.print(&mut out);
        assert_eq!(out, "PerspectiveCamera {\n}\n");
    }

    #[test]
    fn custom_position() {
        let mut cam = VrmlPerspectiveCamera::new();
        cam.set_position(VrmlPerspCamPnt::new(5.0, 5.0, 10.0));
        let mut out = String::new();
        cam.print(&mut out);
        assert!(out.contains("position"));
        assert!(out.contains("5"));
        assert!(out.contains("10"));
    }

    #[test]
    fn custom_fov() {
        let mut cam = VrmlPerspectiveCamera::new();
        cam.set_field_of_view(0.5);
        let mut out = String::new();
        cam.print(&mut out);
        assert!(out.contains("fieldOfView"));
        assert!(out.contains("0.5"));
    }

    #[test]
    fn all_custom() {
        let mut cam = VrmlPerspectiveCamera::new();
        cam.set_position(VrmlPerspCamPnt::new(1.0, 2.0, 3.0));
        cam.set_orientation(VrmlPerspCamDir::new(0.5, 0.5, 0.707));
        cam.set_field_of_view(1.0);
        let mut out = String::new();
        cam.print(&mut out);
        assert!(out.contains("position"));
        assert!(out.contains("orientation"));
        assert!(out.contains("fieldOfView"));
    }
}
