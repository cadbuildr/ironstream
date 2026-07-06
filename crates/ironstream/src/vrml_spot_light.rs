// FILE: vrml_spot_light.rs
// occt: Vrml_SpotLight
//
// Faithful port of OCCT Vrml_SpotLight (DataExchange/TKDEVRML/Vrml/
// Vrml_SpotLight.hxx/.cxx): the VRML 1.0 `SpotLight` node.
// Defaults: on TRUE, intensity 1, color white, location (0,0,1),
// direction (0,0,-1), dropOffRate 0, cutOffAngle 0.785398.
// Print suppresses fields at their defaults; the color field is written in
// sRGB (Quantity_TOC_sRGB conversion of the stored linear-RGB color) while
// the default test uses the linear components.

/// Local model of gp_Vec.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotLightVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SpotLightVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        SpotLightVec { x, y, z }
    }
}

/// Local model of Quantity_Color: linear-RGB components in [0,1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotLightColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl SpotLightColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        SpotLightColor { r, g, b }
    }

    /// Quantity_NOC_WHITE.
    pub fn white() -> Self {
        SpotLightColor::new(1.0, 1.0, 1.0)
    }

    /// Quantity_Color::Convert_LinearRGB_To_sRGB.
    fn linear_to_srgb(v: f64) -> f64 {
        if v <= 0.0031308 {
            v * 12.92
        } else {
            1.055 * v.powf(1.0 / 2.4) - 0.055
        }
    }

    /// Quantity_Color::Values(..., Quantity_TOC_sRGB).
    pub fn values_srgb(&self) -> (f64, f64, f64) {
        (
            Self::linear_to_srgb(self.r),
            Self::linear_to_srgb(self.g),
            Self::linear_to_srgb(self.b),
        )
    }
}

/// Emulates C++ default-ostream double output (printf "%g", 6 significant digits).
fn spot_light_real(v: f64) -> String {
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

const SPOT_LIGHT_INTENSITY_ERROR: &str =
    "Error : Light intensity must be in the range 0.0 to 1.0, inclusive.";

/// Port of Vrml_SpotLight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VrmlSpotLight {
    my_on_off: bool,
    my_intensity: f64,
    my_color: SpotLightColor,
    my_location: SpotLightVec,
    my_direction: SpotLightVec,
    my_drop_off_rate: f64,
    my_cut_off_angle: f64,
}

impl VrmlSpotLight {
    /// Vrml_SpotLight(): on TRUE, intensity 1, white, location (0,0,1),
    /// direction (0,0,-1), dropOffRate 0, cutOffAngle 0.785398.
    pub fn new() -> Self {
        VrmlSpotLight {
            my_on_off: true,
            my_intensity: 1.0,
            my_color: SpotLightColor::white(),
            my_location: SpotLightVec::new(0.0, 0.0, 1.0),
            my_direction: SpotLightVec::new(0.0, 0.0, -1.0),
            my_drop_off_rate: 0.0,
            my_cut_off_angle: 0.785398,
        }
    }

    /// Full-field constructor; panics (Standard_Failure) when the intensity
    /// is outside [0,1].
    #[allow(clippy::too_many_arguments)]
    pub fn with_fields(
        a_on_off: bool,
        a_intensity: f64,
        a_color: SpotLightColor,
        a_location: SpotLightVec,
        a_direction: SpotLightVec,
        a_drop_off_rate: f64,
        a_cut_off_angle: f64,
    ) -> Self {
        if !(0.0..=1.0).contains(&a_intensity) {
            panic!("{}", SPOT_LIGHT_INTENSITY_ERROR);
        }
        VrmlSpotLight {
            my_on_off: a_on_off,
            my_intensity: a_intensity,
            my_color: a_color,
            my_location: a_location,
            my_direction: a_direction,
            my_drop_off_rate: a_drop_off_rate,
            my_cut_off_angle: a_cut_off_angle,
        }
    }

    pub fn set_on_off(&mut self, a_on_off: bool) {
        self.my_on_off = a_on_off;
    }

    pub fn on_off(&self) -> bool {
        self.my_on_off
    }

    /// Panics (Standard_Failure) when the intensity is outside [0,1].
    pub fn set_intensity(&mut self, a_intensity: f64) {
        if !(0.0..=1.0).contains(&a_intensity) {
            panic!("{}", SPOT_LIGHT_INTENSITY_ERROR);
        }
        self.my_intensity = a_intensity;
    }

    pub fn intensity(&self) -> f64 {
        self.my_intensity
    }

    pub fn set_color(&mut self, a_color: SpotLightColor) {
        self.my_color = a_color;
    }

    pub fn color(&self) -> SpotLightColor {
        self.my_color
    }

    pub fn set_location(&mut self, a_location: SpotLightVec) {
        self.my_location = a_location;
    }

    pub fn location(&self) -> SpotLightVec {
        self.my_location
    }

    pub fn set_direction(&mut self, a_direction: SpotLightVec) {
        self.my_direction = a_direction;
    }

    pub fn direction(&self) -> SpotLightVec {
        self.my_direction
    }

    pub fn set_drop_off_rate(&mut self, a_drop_off_rate: f64) {
        self.my_drop_off_rate = a_drop_off_rate;
    }

    pub fn drop_off_rate(&self) -> f64 {
        self.my_drop_off_rate
    }

    pub fn set_cut_off_angle(&mut self, a_cut_off_angle: f64) {
        self.my_cut_off_angle = a_cut_off_angle;
    }

    pub fn cut_off_angle(&self) -> f64 {
        self.my_cut_off_angle
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("SpotLight {\n");

        if !self.my_on_off {
            an_ostream.push_str("    on\t\tFALSE\n");
        }

        if (self.my_intensity - 1.0).abs() > 0.0001 {
            an_ostream.push_str("    intensity\t");
            an_ostream.push_str(&spot_light_real(self.my_intensity));
            an_ostream.push('\n');
        }

        if (self.my_color.r - 1.0).abs() > 0.0001
            || (self.my_color.g - 1.0).abs() > 0.0001
            || (self.my_color.b - 1.0).abs() > 0.0001
        {
            let (sr, sg, sb) = self.my_color.values_srgb();
            an_ostream.push_str("    color\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                spot_light_real(sr),
                spot_light_real(sg),
                spot_light_real(sb)
            ));
        }

        if self.my_location.x.abs() > 0.0001
            || self.my_location.y.abs() > 0.0001
            || (self.my_location.z - 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    location\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                spot_light_real(self.my_location.x),
                spot_light_real(self.my_location.y),
                spot_light_real(self.my_location.z)
            ));
        }

        if self.my_direction.x.abs() > 0.0001
            || self.my_direction.y.abs() > 0.0001
            || (self.my_direction.z + 1.0).abs() > 0.0001
        {
            an_ostream.push_str("    direction\t");
            an_ostream.push_str(&format!(
                "{} {} {}\n",
                spot_light_real(self.my_direction.x),
                spot_light_real(self.my_direction.y),
                spot_light_real(self.my_direction.z)
            ));
        }

        if self.my_drop_off_rate.abs() > 0.0001 {
            an_ostream.push_str("    dropOffRate\t");
            an_ostream.push_str(&spot_light_real(self.my_drop_off_rate));
            an_ostream.push('\n');
        }

        if (self.my_cut_off_angle - 0.785398).abs() > 0.0000001 {
            an_ostream.push_str("    cutOffAngle\t");
            an_ostream.push_str(&spot_light_real(self.my_cut_off_angle));
            an_ostream.push('\n');
        }

        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlSpotLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let l = VrmlSpotLight::new();
        assert!(l.on_off());
        assert_eq!(l.intensity(), 1.0);
        assert_eq!(l.cut_off_angle(), 0.785398);
        let mut out = String::new();
        l.print(&mut out);
        assert_eq!(out, "SpotLight {\n}\n");
    }

    #[test]
    fn all_fields_printed_when_non_default() {
        let l = VrmlSpotLight::with_fields(
            false,
            0.5,
            SpotLightColor::new(1.0, 0.0, 0.0), // linear red
            SpotLightVec::new(1.0, 2.0, 3.0),
            SpotLightVec::new(0.0, 1.0, 0.0),
            0.25,
            1.5708,
        );
        let mut out = String::new();
        l.print(&mut out);
        assert_eq!(
            out,
            "SpotLight {\n    on\t\tFALSE\n    intensity\t0.5\n    color\t1 0 0\n    location\t1 2 3\n    direction\t0 1 0\n    dropOffRate\t0.25\n    cutOffAngle\t1.5708\n}\n"
        );
    }

    #[test]
    fn color_written_in_srgb() {
        let mut l = VrmlSpotLight::new();
        // linear 0.5 -> sRGB ~0.735357
        l.set_color(SpotLightColor::new(0.5, 0.5, 0.5));
        let mut out = String::new();
        l.print(&mut out);
        assert_eq!(out, "SpotLight {\n    color\t0.735357 0.735357 0.735357\n}\n");
    }

    #[test]
    #[should_panic(expected = "Light intensity must be in the range")]
    fn intensity_out_of_range_panics() {
        let mut l = VrmlSpotLight::new();
        l.set_intensity(1.5);
    }

    #[test]
    fn cut_off_angle_uses_tighter_tolerance() {
        let mut l = VrmlSpotLight::new();
        l.set_cut_off_angle(0.78539801); // within 1e-7 -> suppressed
        let mut out = String::new();
        l.print(&mut out);
        assert_eq!(out, "SpotLight {\n}\n");

        l.set_cut_off_angle(0.7854); // outside 1e-7 -> printed
        let mut out2 = String::new();
        l.print(&mut out2);
        assert_eq!(out2, "SpotLight {\n    cutOffAngle\t0.7854\n}\n");
    }
}
