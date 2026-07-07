// FILE: vrml_directional_light.rs
// occt: Vrml_DirectionalLight
//
// Faithful port of OCCT Vrml_DirectionalLight (DataExchange/TKDEVRML/Vrml/
// Vrml_DirectionalLight.hxx/.cxx): VRML 1.0 DirectionalLight node.
// Represents parallel light rays with direction, intensity, and color.
// Default: on TRUE, intensity 1, white color, direction (0,0,-1).

use std::cell::RefCell;
use std::rc::Rc;

/// Simple 3D vector for light direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalLightVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DirectionalLightVec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        DirectionalLightVec { x, y, z }
    }

    /// Normalize in-place.
    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 1e-10 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    /// Get the normalized version.
    pub fn normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 1e-10 {
            DirectionalLightVec {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

impl Default for DirectionalLightVec {
    fn default() -> Self {
        DirectionalLightVec::new(0.0, 0.0, -1.0)
    }
}

/// Linear-RGB color for light.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalLightColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl DirectionalLightColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        DirectionalLightColor { r, g, b }
    }

    /// White color (default).
    pub fn white() -> Self {
        DirectionalLightColor::new(1.0, 1.0, 1.0)
    }

    /// Linear to sRGB conversion.
    fn linear_to_srgb(v: f64) -> f64 {
        if v <= 0.0031308 {
            v * 12.92
        } else {
            1.055 * v.powf(1.0 / 2.4) - 0.055
        }
    }

    /// Get sRGB values for output.
    pub fn values_srgb(&self) -> (f64, f64, f64) {
        (
            Self::linear_to_srgb(self.r),
            Self::linear_to_srgb(self.g),
            Self::linear_to_srgb(self.b),
        )
    }
}

impl Default for DirectionalLightColor {
    fn default() -> Self {
        DirectionalLightColor::white()
    }
}

/// Emulate C++ %g formatting for double output.
fn directional_light_real(v: f64) -> String {
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

const DIRECTIONAL_LIGHT_INTENSITY_ERROR: &str =
    "Error : Light intensity must be in the range 0.0 to 1.0, inclusive.";

/// VRML 1.0 DirectionalLight: parallel light source with direction.
/// Defaults: on TRUE, intensity 1.0, color white (1,1,1), direction (0,0,-1).
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlDirectionalLight {
    my_on_off: bool,
    my_intensity: f64,
    my_color: DirectionalLightColor,
    my_direction: DirectionalLightVec,
    my_name: String,
}

impl VrmlDirectionalLight {
    /// Constructor: creates a default directional light.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDirectionalLight {
            my_on_off: true,
            my_intensity: 1.0,
            my_color: DirectionalLightColor::white(),
            my_direction: DirectionalLightVec::default(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with all fields.
    pub fn with_fields(
        on_off: bool,
        intensity: f64,
        color: DirectionalLightColor,
        direction: DirectionalLightVec,
        name: Option<&str>,
    ) -> Self {
        if !(0.0..=1.0).contains(&intensity) {
            panic!("{}", DIRECTIONAL_LIGHT_INTENSITY_ERROR);
        }
        VrmlDirectionalLight {
            my_on_off: on_off,
            my_intensity: intensity,
            my_color: color,
            my_direction: direction,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get on/off state.
    pub fn on_off(&self) -> bool {
        self.my_on_off
    }

    /// Set on/off state.
    pub fn set_on_off(&mut self, on_off: bool) {
        self.my_on_off = on_off;
    }

    /// Get intensity [0, 1].
    pub fn intensity(&self) -> f64 {
        self.my_intensity
    }

    /// Set intensity [0, 1]; panics if out of range.
    pub fn set_intensity(&mut self, intensity: f64) {
        if !(0.0..=1.0).contains(&intensity) {
            panic!("{}", DIRECTIONAL_LIGHT_INTENSITY_ERROR);
        }
        self.my_intensity = intensity;
    }

    /// Get the color.
    pub fn color(&self) -> DirectionalLightColor {
        self.my_color
    }

    /// Set the color.
    pub fn set_color(&mut self, color: DirectionalLightColor) {
        self.my_color = color;
    }

    /// Get the direction vector.
    pub fn direction(&self) -> DirectionalLightVec {
        self.my_direction
    }

    /// Set the direction vector.
    pub fn set_direction(&mut self, direction: DirectionalLightVec) {
        self.my_direction = direction;
    }

    /// Check if in default state.
    pub fn is_default(&self) -> bool {
        self.my_on_off
            && (self.my_intensity - 1.0).abs() < 1e-10
            && self.my_color == DirectionalLightColor::white()
            && self.my_direction == DirectionalLightVec::default()
    }

    /// Print the light node (VRML format).
    pub fn print(&self, out: &mut String) {
        out.push_str("DirectionalLight {\n");

        if !self.my_on_off {
            out.push_str("    on\t\tFALSE\n");
        }

        if (self.my_intensity - 1.0).abs() > 0.0001 {
            out.push_str("    intensity\t");
            out.push_str(&directional_light_real(self.my_intensity));
            out.push('\n');
        }

        if (self.my_color.r - 1.0).abs() > 0.0001
            || (self.my_color.g - 1.0).abs() > 0.0001
            || (self.my_color.b - 1.0).abs() > 0.0001
        {
            let (sr, sg, sb) = self.my_color.values_srgb();
            out.push_str("    color\t");
            out.push_str(&format!(
                "{} {} {}\n",
                directional_light_real(sr),
                directional_light_real(sg),
                directional_light_real(sb)
            ));
        }

        if self.my_direction.x.abs() > 0.0001
            || self.my_direction.y.abs() > 0.0001
            || (self.my_direction.z + 1.0).abs() > 0.0001
        {
            out.push_str("    direction\t");
            out.push_str(&format!(
                "{} {} {}\n",
                directional_light_real(self.my_direction.x),
                directional_light_real(self.my_direction.y),
                directional_light_real(self.my_direction.z)
            ));
        }

        out.push_str("}\n");
    }
}

impl Default for VrmlDirectionalLight {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_light() {
        let light = VrmlDirectionalLight::new(None);
        assert!(light.on_off());
        assert_eq!(light.intensity(), 1.0);
        assert_eq!(light.color(), DirectionalLightColor::white());
        assert!(light.is_default());
    }

    #[test]
    fn set_on_off() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_on_off(false);
        assert!(!light.on_off());
    }

    #[test]
    fn set_intensity() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_intensity(0.75);
        assert_eq!(light.intensity(), 0.75);
    }

    #[test]
    #[should_panic(expected = "Light intensity must be in the range")]
    fn intensity_out_of_range() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_intensity(1.5);
    }

    #[test]
    fn set_color() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_color(DirectionalLightColor::new(1.0, 0.0, 0.0));
        assert_eq!(light.color().r, 1.0);
        assert_eq!(light.color().g, 0.0);
    }

    #[test]
    fn set_direction() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_direction(DirectionalLightVec::new(0.0, 1.0, 0.0));
        assert_eq!(light.direction().y, 1.0);
    }

    #[test]
    fn print_default() {
        let light = VrmlDirectionalLight::new(None);
        let mut out = String::new();
        light.print(&mut out);
        assert_eq!(out, "DirectionalLight {\n}\n");
    }

    #[test]
    fn print_with_changes() {
        let mut light = VrmlDirectionalLight::new(None);
        light.set_on_off(false);
        light.set_intensity(0.5);
        let mut out = String::new();
        light.print(&mut out);
        assert!(out.contains("FALSE"));
        assert!(out.contains("0.5"));
    }

    #[test]
    fn clone() {
        let light = VrmlDirectionalLight::with_fields(
            false,
            0.8,
            DirectionalLightColor::new(1.0, 0.5, 0.0),
            DirectionalLightVec::new(1.0, 0.0, 0.0),
            Some("Light1"),
        );
        let cloned = light.clone();
        assert_eq!(cloned.on_off(), false);
        assert_eq!(cloned.intensity(), 0.8);
    }

    #[test]
    fn vector_normalize() {
        let mut v = DirectionalLightVec::new(3.0, 4.0, 0.0);
        v.normalize();
        assert!((v.x - 0.6).abs() < 1e-6);
        assert!((v.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn set_name() {
        let mut light = VrmlDirectionalLight::new(Some("Old"));
        light.set_name("New");
        assert_eq!(light.name(), "New");
    }
}
