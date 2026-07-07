// FILE: vrml_point_light.rs
// occt: Vrml_PointLight
//
// Faithful port of OCCT Vrml_PointLight (DataExchange/TKDEVRML/Vrml/
// Vrml_PointLight.hxx): the VRML 1.0 `PointLight` node, specifying
// a point light source with position, color, intensity, and attenuation.
// Defaults per VRML 1.0 spec. Print emits non-default fields.

/// Local RGB color type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlPointLightColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl VrmlPointLightColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        VrmlPointLightColor { r, g, b }
    }
}

/// Local model of gp_Pnt (position).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlPointLightPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VrmlPointLightPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlPointLightPnt { x, y, z }
    }
}

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_point_light_real(v: f64) -> String {
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

/// Port of Vrml_PointLight.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlPointLight {
    my_on: bool,
    my_intensity: f64,
    my_color: VrmlPointLightColor,
    my_location: VrmlPointLightPnt,
    my_attenuation: [f64; 3], // [constant, linear, quadratic]
}

impl VrmlPointLight {
    /// Vrml_PointLight():
    /// on defaults to true
    /// intensity defaults to 1
    /// color defaults to (1 1 1) white
    /// location defaults to (0 0 1)
    /// attenuation defaults to (1, 0, 0) = constant only
    pub fn new() -> Self {
        VrmlPointLight {
            my_on: true,
            my_intensity: 1.0,
            my_color: VrmlPointLightColor::new(1.0, 1.0, 1.0),
            my_location: VrmlPointLightPnt::new(0.0, 0.0, 1.0),
            my_attenuation: [1.0, 0.0, 0.0],
        }
    }

    pub fn set_on(&mut self, a_on: bool) {
        self.my_on = a_on;
    }

    pub fn on(&self) -> bool {
        self.my_on
    }

    pub fn set_intensity(&mut self, a_intensity: f64) {
        self.my_intensity = a_intensity;
    }

    pub fn intensity(&self) -> f64 {
        self.my_intensity
    }

    pub fn set_color(&mut self, a_color: VrmlPointLightColor) {
        self.my_color = a_color;
    }

    pub fn color(&self) -> VrmlPointLightColor {
        self.my_color
    }

    pub fn set_location(&mut self, a_location: VrmlPointLightPnt) {
        self.my_location = a_location;
    }

    pub fn location(&self) -> VrmlPointLightPnt {
        self.my_location
    }

    pub fn set_attenuation(&mut self, constant: f64, linear: f64, quadratic: f64) {
        self.my_attenuation = [constant, linear, quadratic];
    }

    pub fn attenuation(&self) -> [f64; 3] {
        self.my_attenuation
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("PointLight {\n");

        // on field (default true)
        if !self.my_on {
            an_ostream.push_str("    on\tFALSE\n");
        }

        // intensity (default 1)
        if (self.my_intensity - 1.0).abs() > 0.0001 {
            an_ostream.push_str("    intensity\t");
            an_ostream.push_str(&vrml_point_light_real(self.my_intensity));
            an_ostream.push('\n');
        }

        // color (default 1 1 1)
        if (self.my_color.r - 1.0).abs() > 0.0001
            || (self.my_color.g - 1.0).abs() > 0.0001
            || (self.my_color.b - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    color\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_point_light_real(self.my_color.r),
                vrml_point_light_real(self.my_color.g),
                vrml_point_light_real(self.my_color.b)
            ));
        }

        // location (default 0 0 1)
        if (self.my_location.x - 0.0).abs() > 0.0001
            || (self.my_location.y - 0.0).abs() > 0.0001
            || (self.my_location.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    location\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_point_light_real(self.my_location.x),
                vrml_point_light_real(self.my_location.y),
                vrml_point_light_real(self.my_location.z)
            ));
        }

        // attenuation (default 1 0 0)
        if (self.my_attenuation[0] - 1.0).abs() > 0.0001
            || self.my_attenuation[1] > 0.0001
            || self.my_attenuation[2] > 0.0001
        {
            an_ostream.push_str("    attenuation\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                vrml_point_light_real(self.my_attenuation[0]),
                vrml_point_light_real(self.my_attenuation[1]),
                vrml_point_light_real(self.my_attenuation[2])
            ));
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlPointLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_light() {
        let light = VrmlPointLight::new();
        assert_eq!(light.on(), true);
        assert_eq!(light.intensity(), 1.0);
        assert_eq!(light.color(), VrmlPointLightColor::new(1.0, 1.0, 1.0));
        assert_eq!(light.location(), VrmlPointLightPnt::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn default_prints_empty_node() {
        let light = VrmlPointLight::new();
        let mut out = String::new();
        light.print(&mut out);
        assert_eq!(out, "PointLight {\n}\n");
    }

    #[test]
    fn light_turned_off() {
        let mut light = VrmlPointLight::new();
        light.set_on(false);
        let mut out = String::new();
        light.print(&mut out);
        assert_eq!(out, "PointLight {\n    on\tFALSE\n}\n");
    }

    #[test]
    fn custom_color() {
        let mut light = VrmlPointLight::new();
        light.set_color(VrmlPointLightColor::new(1.0, 0.5, 0.0));
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("color"));
        assert!(out.contains("1 0.5 0"));
    }

    #[test]
    fn custom_location() {
        let mut light = VrmlPointLight::new();
        light.set_location(VrmlPointLightPnt::new(5.0, 5.0, 10.0));
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("location"));
        assert!(out.contains("5"));
        assert!(out.contains("10"));
    }

    #[test]
    fn custom_intensity() {
        let mut light = VrmlPointLight::new();
        light.set_intensity(0.5);
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("intensity"));
        assert!(out.contains("0.5"));
    }

    #[test]
    fn custom_attenuation() {
        let mut light = VrmlPointLight::new();
        light.set_attenuation(0.8, 0.1, 0.05);
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("attenuation"));
    }

    #[test]
    fn all_custom() {
        let mut light = VrmlPointLight::new();
        light.set_on(false);
        light.set_intensity(0.8);
        light.set_color(VrmlPointLightColor::new(0.9, 0.9, 0.9));
        light.set_location(VrmlPointLightPnt::new(1.0, 1.0, 1.0));
        light.set_attenuation(1.0, 0.1, 0.0);
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("on"));
        assert!(out.contains("intensity"));
        assert!(out.contains("color"));
        assert!(out.contains("location"));
    }
}
