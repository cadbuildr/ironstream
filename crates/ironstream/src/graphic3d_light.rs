// FILE: graphic3d_light.rs
// occt: Graphic3d_CLight, Graphic3d_TypeOfLightSource,
//       Graphic3d_DirectionalLight, Graphic3d_PointLight,
//       Graphic3d_SpotLight, Graphic3d_AmbientLight

/// Type of a light source.
/// occt: Graphic3d_TypeOfLightSource
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Graphic3dTypeOfLightSource {
    #[default]
    Ambient,
    Directional,
    Positional,
    Spot,
}

/// RGB colour for lights (f32 components, unclamped HDR allowed).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LightColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self { Self { r, g, b } }
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
    pub fn black() -> Self { Self::new(0.0, 0.0, 0.0) }

    pub fn scaled(&self, s: f32) -> Self {
        Self::new(self.r * s, self.g * s, self.b * s)
    }

    pub fn as_array(&self) -> [f32; 3] { [self.r, self.g, self.b] }
}

impl Default for LightColor {
    fn default() -> Self { Self::white() }
}

/// Attenuation parameters for positional lights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightAttenuation {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Default for LightAttenuation {
    fn default() -> Self { Self { constant: 1.0, linear: 0.0, quadratic: 0.0 } }
}

impl LightAttenuation {
    pub fn new(constant: f32, linear: f32, quadratic: f32) -> Self {
        Self { constant, linear, quadratic }
    }

    /// Evaluate attenuation factor at distance d.
    pub fn factor(&self, d: f32) -> f32 {
        let denom = self.constant + self.linear * d + self.quadratic * d * d;
        if denom.abs() < 1e-14 { 1.0 } else { 1.0 / denom }
    }
}

/// Generic light source.
/// occt: Graphic3d_CLight
#[derive(Clone, Debug)]
pub struct Graphic3dCLight {
    pub light_type: Graphic3dTypeOfLightSource,
    pub name: String,
    pub color: LightColor,
    pub intensity: f32,
    pub is_enabled: bool,
    pub is_headlight: bool,
    /// Direction (for directional/spot).
    pub direction: [f32; 3],
    /// Position (for positional/spot).
    pub position: [f32; 3],
    /// Spot cone half-angle (radians).
    pub spot_angle: f32,
    /// Spot exponent.
    pub spot_exponent: f32,
    pub attenuation: LightAttenuation,
    pub smoothness: f32,
    pub range: f32,
}

impl Default for Graphic3dCLight {
    fn default() -> Self {
        Self {
            light_type: Graphic3dTypeOfLightSource::Ambient,
            name: String::new(),
            color: LightColor::white(),
            intensity: 1.0,
            is_enabled: true,
            is_headlight: false,
            direction: [0.0, 0.0, -1.0],
            position: [0.0; 3],
            spot_angle: std::f32::consts::FRAC_PI_4,
            spot_exponent: 1.0,
            attenuation: LightAttenuation::default(),
            smoothness: 0.0,
            range: f32::MAX,
        }
    }
}

impl Graphic3dCLight {
    pub fn new(light_type: Graphic3dTypeOfLightSource) -> Self {
        Self { light_type, ..Self::default() }
    }

    pub fn ambient() -> Self { Self::new(Graphic3dTypeOfLightSource::Ambient) }

    pub fn directional(direction: [f32; 3]) -> Self {
        Self {
            light_type: Graphic3dTypeOfLightSource::Directional,
            direction,
            ..Self::default()
        }
    }

    pub fn positional(position: [f32; 3]) -> Self {
        Self {
            light_type: Graphic3dTypeOfLightSource::Positional,
            position,
            ..Self::default()
        }
    }

    pub fn spot(position: [f32; 3], direction: [f32; 3], angle: f32) -> Self {
        Self {
            light_type: Graphic3dTypeOfLightSource::Spot,
            position,
            direction,
            spot_angle: angle.clamp(0.0, std::f32::consts::FRAC_PI_2),
            ..Self::default()
        }
    }

    pub fn set_color(&mut self, c: LightColor) { self.color = c; }
    pub fn set_intensity(&mut self, i: f32) { self.intensity = i.max(0.0); }
    pub fn set_enabled(&mut self, v: bool) { self.is_enabled = v; }
    pub fn set_headlight(&mut self, v: bool) { self.is_headlight = v; }
    pub fn set_name(&mut self, n: impl Into<String>) { self.name = n.into(); }
    pub fn set_direction(&mut self, d: [f32; 3]) { self.direction = d; }
    pub fn set_position(&mut self, p: [f32; 3]) { self.position = p; }
    pub fn set_range(&mut self, r: f32) { self.range = r.max(0.0); }

    pub fn effective_color(&self) -> LightColor { self.color.scaled(self.intensity) }

    pub fn is_directional(&self) -> bool { self.light_type == Graphic3dTypeOfLightSource::Directional }
    pub fn is_positional(&self) -> bool  { self.light_type == Graphic3dTypeOfLightSource::Positional }
    pub fn is_spot(&self) -> bool        { self.light_type == Graphic3dTypeOfLightSource::Spot }
    pub fn is_ambient(&self) -> bool     { self.light_type == Graphic3dTypeOfLightSource::Ambient }
}

/// Registry of scene lights.
#[derive(Clone, Debug, Default)]
pub struct Graphic3dLightSet {
    pub lights: Vec<Graphic3dCLight>,
}

impl Graphic3dLightSet {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, light: Graphic3dCLight) { self.lights.push(light); }

    pub fn nb_lights(&self) -> usize { self.lights.len() }
    pub fn nb_enabled(&self) -> usize { self.lights.iter().filter(|l| l.is_enabled).count() }

    pub fn has_ambient(&self) -> bool { self.lights.iter().any(|l| l.is_ambient() && l.is_enabled) }

    pub fn ambient_color(&self) -> LightColor {
        let mut r = 0.0f32; let mut g = 0.0f32; let mut b = 0.0f32;
        for l in self.lights.iter().filter(|l| l.is_ambient() && l.is_enabled) {
            let c = l.effective_color();
            r += c.r; g += c.g; b += c.b;
        }
        LightColor::new(r, g, b)
    }

    pub fn remove(&mut self, name: &str) { self.lights.retain(|l| l.name != name); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_light() {
        let l = Graphic3dCLight::directional([0.0, -1.0, 0.0]);
        assert!(l.is_directional());
        assert!(!l.is_spot());
        assert!(l.is_enabled);
    }

    #[test]
    fn positional_light_attenuation() {
        let l = Graphic3dCLight::positional([0.0, 0.0, 5.0]);
        let a = LightAttenuation::new(1.0, 0.5, 0.0);
        let f = a.factor(2.0);
        assert!((f - 1.0 / 2.0).abs() < 1e-5);
    }

    #[test]
    fn spot_light_angle_clamp() {
        let l = Graphic3dCLight::spot([0.0; 3], [0.0, -1.0, 0.0], 10.0);
        assert!(l.spot_angle <= std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn light_set_ambient() {
        let mut s = Graphic3dLightSet::new();
        s.add(Graphic3dCLight::ambient());
        assert!(s.has_ambient());
        assert_eq!(s.nb_enabled(), 1);
        let col = s.ambient_color();
        assert!((col.r - 1.0).abs() < 1e-6);
    }

    #[test]
    fn light_set_remove() {
        let mut s = Graphic3dLightSet::new();
        let mut l = Graphic3dCLight::directional([1.0, 0.0, 0.0]);
        l.set_name("sun");
        s.add(l);
        assert_eq!(s.nb_lights(), 1);
        s.remove("sun");
        assert_eq!(s.nb_lights(), 0);
    }

    #[test]
    fn effective_color_intensity() {
        let mut l = Graphic3dCLight::ambient();
        l.set_intensity(0.5);
        let c = l.effective_color();
        assert!((c.r - 0.5).abs() < 1e-6);
    }
}
