//! `V3d_Light` — viewer light source types, mirroring OpenCascade's
//! `V3d_DirectionalLight`, `V3d_PositionalLight`, `V3d_SpotLight`, and
//! `V3d_AmbientLight` from `src/Visualization/TKV3d/V3d/`.
//!
//! Each light carries a type tag ([`LightType`]), a RGB colour, an intensity
//! scalar, a world-space position, a world-space direction (used by directional
//! and spot lights), and an enabled flag.
//!
//! The companion function [`default_lights`] returns a sensible two-light rig
//! (one directional key light + one ambient fill) comparable to the defaults
//! that `V3d_Viewer::SetDefaultLights()` installs.

// ─────────────────────────────── LightType ───────────────────────────────────

// occt-note: V3d_TypeOfLight (V3d_DIRECTIONAL, V3d_POSITIONAL, V3d_SPOT, V3d_AMBIENT)
/// Discriminates the four light categories defined by OpenCascade's viewer.
///
/// Mirrors the `V3d_TypeOfLight` enumeration used across `V3d_DirectionalLight`,
/// `V3d_PositionalLight`, `V3d_SpotLight`, and `V3d_AmbientLight`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LightType {
    /// Infinitely distant, parallel-ray light — `V3d_DIRECTIONAL`.
    Directional,
    /// Point light with omnidirectional emission — `V3d_POSITIONAL`.
    Positional,
    /// Cone-shaped spotlight — `V3d_SPOT`.
    Spot,
    /// Uniform ambient fill — `V3d_AMBIENT`.
    Ambient,
}

// ──────────────────────────────── Light ──────────────────────────────────────

// occt-ref: V3d_DirectionalLight
// occt: V3d_SpotLight
/// A single viewer light source, merging the data carried by
/// `V3d_DirectionalLight`, `V3d_PositionalLight`, `V3d_SpotLight`, and
/// `V3d_AmbientLight`.
///
/// Fields are intentionally `pub` so that callers can inspect and serialise
/// light rigs without going through accessor methods.
#[derive(Clone, Debug)]
pub struct Light {
    /// The kind of light source.
    pub light_type: LightType,
    /// RGB colour in \[0, 1\] linear range (maps to `V3d_Light::SetColor`).
    pub color: [f64; 3],
    /// Photometric intensity / brightness scalar (maps to `V3d_Light::SetIntensity`).
    pub intensity: f64,
    /// World-space position — meaningful for [`LightType::Positional`] and
    /// [`LightType::Spot`] (maps to `V3d_PositionalLight::SetPosition`).
    pub position: [f64; 3],
    /// Unit direction vector — meaningful for [`LightType::Directional`] and
    /// [`LightType::Spot`] (maps to `V3d_DirectionalLight::SetDirection`).
    pub direction: [f64; 3],
    /// Half-angle of the spot cone in radians
    /// (maps to `V3d_SpotLight::SetAngle`; ignored for other types).
    pub spot_angle: f64,
    /// Whether the light contributes to the scene (`V3d_Light::SetEnabled`).
    pub enabled: bool,
}

impl Light {
    // ─────────────────────────── constructors ────────────────────────────────

    // occt-ref: V3d_DirectionalLight
    /// `V3d_DirectionalLight(theDirection)` — create a white directional light
    /// aimed along `dir`.
    ///
    /// `dir` need not be pre-normalised; it is stored as-is and the renderer is
    /// expected to normalise it (matching OCCT behaviour).
    pub fn directional(dir: [f64; 3]) -> Self {
        Self {
            light_type: LightType::Directional,
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            position: [0.0, 0.0, 0.0],
            direction: dir,
            spot_angle: 0.0,
            enabled: true,
        }
    }

    // occt: V3d_PositionalLight
    /// `V3d_PositionalLight(thePosition)` — create a white point light at `pos`.
    pub fn positional(pos: [f64; 3]) -> Self {
        Self {
            light_type: LightType::Positional,
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            position: pos,
            direction: [0.0, 0.0, -1.0],
            spot_angle: 0.0,
            enabled: true,
        }
    }

    // occt: V3d_SpotLight
    /// `V3d_SpotLight(thePosition, theDirection, theAngle)` — create a white
    /// spotlight at `pos`, aimed along `dir`, with a half-cone `angle` in
    /// radians.
    pub fn spot(pos: [f64; 3], dir: [f64; 3], angle: f64) -> Self {
        Self {
            light_type: LightType::Spot,
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            position: pos,
            direction: dir,
            spot_angle: angle,
            enabled: true,
        }
    }

    // occt-ref: V3d_AmbientLight
    /// `V3d_AmbientLight()` — create a white ambient fill light.
    pub fn ambient() -> Self {
        Self {
            light_type: LightType::Ambient,
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, -1.0],
            spot_angle: 0.0,
            enabled: true,
        }
    }

    // ─────────────────────────── mutators ────────────────────────────────────

    /// `V3d_Light::SetIntensity(theValue)` — set the brightness scalar.
    pub fn set_intensity(&mut self, i: f64) {
        self.intensity = i;
    }

    /// `V3d_Light::SetColor(theColor)` — set the RGB colour in \[0, 1\] range.
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.color = [r, g, b];
    }

    /// `V3d_Light::SetEnabled(theIsEnabled)` — toggle the light on or off.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    // ─────────────────────────── queries ─────────────────────────────────────

    /// `V3d_Light::IsEnabled()` — return whether the light currently contributes
    /// to the scene.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// ──────────────────────────── default_lights ─────────────────────────────────

/// Return a default two-light rig comparable to `V3d_Viewer::SetDefaultLights()`.
///
/// The rig consists of:
/// 1. A directional key light coming from the upper-front-right
///    (`[-1, -1, -1]` direction, normalised by the renderer), intensity 1.0.
/// 2. A low-intensity ambient fill at 0.3, ensuring shadowed surfaces remain
///    visible.
///
/// Callers may push additional lights, adjust intensities, or disable entries
/// after construction.
pub fn default_lights() -> Vec<Light> {
    let mut key = Light::directional([-1.0, -1.0, -1.0]);
    key.set_intensity(1.0);

    let mut fill = Light::ambient();
    fill.set_intensity(0.3);

    vec![key, fill]
}

// ─────────────────────────────── tests ───────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn directional_defaults() {
        let l = Light::directional([0.0, 0.0, -1.0]);
        assert_eq!(l.light_type, LightType::Directional);
        assert_eq!(l.direction, [0.0, 0.0, -1.0]);
        assert_eq!(l.color, [1.0, 1.0, 1.0]);
        assert!((l.intensity - 1.0).abs() < f64::EPSILON);
        assert!(l.is_enabled());
    }

    #[test]
    fn positional_defaults() {
        let l = Light::positional([1.0, 2.0, 3.0]);
        assert_eq!(l.light_type, LightType::Positional);
        assert_eq!(l.position, [1.0, 2.0, 3.0]);
        assert!(l.is_enabled());
    }

    #[test]
    fn spot_stores_angle() {
        let l = Light::spot([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], FRAC_PI_4);
        assert_eq!(l.light_type, LightType::Spot);
        assert!((l.spot_angle - FRAC_PI_4).abs() < f64::EPSILON);
    }

    #[test]
    fn ambient_type() {
        let l = Light::ambient();
        assert_eq!(l.light_type, LightType::Ambient);
    }

    #[test]
    fn set_intensity_and_color() {
        let mut l = Light::directional([1.0, 0.0, 0.0]);
        l.set_intensity(0.5);
        l.set_color(0.8, 0.6, 0.4);
        assert!((l.intensity - 0.5).abs() < f64::EPSILON);
        assert_eq!(l.color, [0.8, 0.6, 0.4]);
    }

    #[test]
    fn enable_disable() {
        let mut l = Light::ambient();
        assert!(l.is_enabled());
        l.set_enabled(false);
        assert!(!l.is_enabled());
        l.set_enabled(true);
        assert!(l.is_enabled());
    }

    #[test]
    fn default_lights_rig() {
        let lights = default_lights();
        assert_eq!(lights.len(), 2);
        assert_eq!(lights[0].light_type, LightType::Directional);
        assert_eq!(lights[1].light_type, LightType::Ambient);
        assert!(lights[0].is_enabled());
        assert!(lights[1].is_enabled());
        assert!((lights[0].intensity - 1.0).abs() < f64::EPSILON);
        assert!((lights[1].intensity - 0.3).abs() < f64::EPSILON);
    }
}
