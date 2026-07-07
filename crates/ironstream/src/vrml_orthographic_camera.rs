// FILE: vrml_orthographic_camera.rs
// occt: Vrml_OrthographicCamera
//
// Faithful port of OCCT Vrml_OrthographicCamera (DataExchange/TKDEVRML/Vrml/
// Vrml_OrthographicCamera.hxx): the VRML 1.0 `OrthographicCamera` node,
// specifying orthographic camera properties (position, orientation, height).
// Defaults per VRML 1.0 spec. Print emits non-default fields.

/// Local model of gp_Pnt (position).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlOrthoCamPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlOrthoCamPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlOrthoCamPnt { x, y, z }
    }
}

/// Local model of gp_Dir (direction, represented as normalized vector).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlOrthoCamDir {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlOrthoCamDir {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlOrthoCamDir { x, y, z }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_ortho_cam_real(v: f64) -> String {
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

/// Port of Vrml_OrthographicCamera.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlOrthographicCamera {
    my_position: VrmlOrthoCamPnt,
    my_orientation: VrmlOrthoCamDir,
    my_height: f64,
}

impl VrmlOrthographicCamera {
    /// Vrml_OrthographicCamera():
    /// position defaults to (0 0 1)
    /// orientation defaults to (0 0 1) [identity]
    /// height defaults to 2
    pub fn new() -> Self {
        VrmlOrthographicCamera {
            my_position: VrmlOrthoCamPnt::new(0.0, 0.0, 1.0),
            my_orientation: VrmlOrthoCamDir::new(0.0, 0.0, 1.0),
            my_height: 2.0,
        }
    }

    pub fn set_position(&mut self, a_position: VrmlOrthoCamPnt) {
        self.my_position = a_position;
    }

    pub fn position(&self) -> VrmlOrthoCamPnt {
        self.my_position
    }

    pub fn set_orientation(&mut self, a_orientation: VrmlOrthoCamDir) {
        self.my_orientation = a_orientation;
    }

    pub fn orientation(&self) -> VrmlOrthoCamDir {
        self.my_orientation
    }

    pub fn set_height(&mut self, a_height: f64) {
        self.my_height = a_height;
    }

    pub fn height(&self) -> f64 {
        self.my_height
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("OrthographicCamera {\n");

        // position (default 0 0 1)
        if (self.my_position.x - 0.0).abs() > 0.0001
            || (self.my_position.y - 0.0).abs() > 0.0001
            || (self.my_position.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    position\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_ortho_cam_real(self.my_position.x),
                vrml_ortho_cam_real(self.my_position.y),
                vrml_ortho_cam_real(self.my_position.z)
            ));
        }

        // orientation (default 0 0 1, which represents identity)
        if (self.my_orientation.x - 0.0).abs() > 0.0001
            || (self.my_orientation.y - 0.0).abs() > 0.0001
            || (self.my_orientation.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    orientation\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_ortho_cam_real(self.my_orientation.x),
                vrml_ortho_cam_real(self.my_orientation.y),
                vrml_ortho_cam_real(self.my_orientation.z)
            ));
        }

        // height (default 2)
        if (self.my_height - 2.0).abs() > 0.0001 {
            an_ostream.push_str("    height\t");
            an_ostream.push_str(&vrml_ortho_cam_real(self.my_height));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlOrthographicCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_camera() {
        let cam = VrmlOrthographicCamera::new();
        let pos = cam.position();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, 1.0);
        assert_eq!(cam.height(), 2.0);
    }

    #[test]
    fn default_prints_empty_node() {
        let cam = VrmlOrthographicCamera::new();
        let mut out = String::new();
        cam.print(&mut out);
        assert_eq!(out, "OrthographicCamera {\n}\n");
    }

    #[test]
    fn custom_position() {
        let mut cam = VrmlOrthographicCamera::new();
        cam.set_position(VrmlOrthoCamPnt::new(5.0, 5.0, 10.0));
        let mut out = String::new();
        cam.print(&mut out);
        assert!(out.contains("position"));
        assert!(out.contains("5"));
        assert!(out.contains("10"));
    }

    #[test]
    fn custom_height() {
        let mut cam = VrmlOrthographicCamera::new();
        cam.set_height(4.0);
        let mut out = String::new();
        cam.print(&mut out);
        assert_eq!(out, "OrthographicCamera {\n    height\t4\n}\n");
    }

    #[test]
    fn all_custom() {
        let mut cam = VrmlOrthographicCamera::new();
        cam.set_position(VrmlOrthoCamPnt::new(1.0, 2.0, 3.0));
        cam.set_orientation(VrmlOrthoCamDir::new(0.5, 0.5, 0.707));
        cam.set_height(5.0);
        let mut out = String::new();
        cam.print(&mut out);
        assert!(out.contains("position"));
        assert!(out.contains("orientation"));
        assert!(out.contains("height"));
    }
}
