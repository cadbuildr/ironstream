// FILE: crates/ironstream/src/graphic3d_mat.rs

// occt: Graphic3d_NameOfMaterial
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialName {
    Brass,
    Bronze,
    Gold,
    Pewter,
    Silver,
    Steel,
    Chrome,
    Aluminium,
    Obsidian,
    UserDefined,
}

// occt: Graphic3d_MaterialAspect
#[derive(Clone, Debug)]
pub struct MaterialAspect {
    pub name: MaterialName,
    pub ambient: [f64; 3],
    pub diffuse: [f64; 3],
    pub specular: [f64; 3],
    pub shininess: f64,
    pub transparency: f64,
}

impl MaterialAspect {
    /// Create a MaterialAspect for the given named material with OCCT-matching defaults.
    pub fn new(name: MaterialName) -> Self {
        match name {
            MaterialName::Brass => Self {
                name,
                ambient: [0.3294, 0.2235, 0.0275],
                diffuse: [0.7804, 0.5686, 0.1137],
                specular: [0.9922, 0.9412, 0.8078],
                shininess: 0.2179,
                transparency: 0.0,
            },
            MaterialName::Bronze => Self {
                name,
                ambient: [0.2125, 0.1275, 0.0540],
                diffuse: [0.7140, 0.4284, 0.1814],
                specular: [0.3935, 0.2719, 0.1667],
                shininess: 0.2000,
                transparency: 0.0,
            },
            MaterialName::Gold => Self {
                name,
                ambient: [0.2473, 0.1995, 0.0745],
                diffuse: [0.7516, 0.6065, 0.2265],
                specular: [0.6283, 0.5559, 0.3661],
                shininess: 0.4000,
                transparency: 0.0,
            },
            MaterialName::Pewter => Self {
                name,
                ambient: [0.1059, 0.0588, 0.1137],
                diffuse: [0.4275, 0.4706, 0.5412],
                specular: [0.3333, 0.3333, 0.5216],
                shininess: 0.0769,
                transparency: 0.0,
            },
            MaterialName::Silver => Self {
                name,
                ambient: [0.1922, 0.1922, 0.1922],
                diffuse: [0.5075, 0.5075, 0.5075],
                specular: [0.5083, 0.5083, 0.5083],
                shininess: 0.4000,
                transparency: 0.0,
            },
            MaterialName::Steel => Self {
                name,
                ambient: [0.0157, 0.0157, 0.0157],
                diffuse: [0.1137, 0.1137, 0.1137],
                specular: [0.9843, 0.9843, 0.9843],
                shininess: 0.9000,
                transparency: 0.0,
            },
            MaterialName::Chrome => Self {
                name,
                ambient: [0.0980, 0.0980, 0.0980],
                diffuse: [0.4000, 0.4000, 0.4000],
                specular: [0.7745, 0.7745, 0.7745],
                shininess: 0.6000,
                transparency: 0.0,
            },
            MaterialName::Aluminium => Self {
                name,
                ambient: [0.1000, 0.1000, 0.1000],
                diffuse: [0.4500, 0.4500, 0.4500],
                specular: [0.7745, 0.7745, 0.7745],
                shininess: 0.5500,
                transparency: 0.0,
            },
            MaterialName::Obsidian => Self {
                name,
                ambient: [0.0536, 0.0500, 0.0664],
                diffuse: [0.1828, 0.1700, 0.2257],
                specular: [0.3328, 0.3286, 0.3464],
                shininess: 0.3000,
                transparency: 0.0,
            },
            MaterialName::UserDefined => Self {
                name,
                ambient: [0.2000, 0.2000, 0.2000],
                diffuse: [0.8000, 0.8000, 0.8000],
                specular: [0.1000, 0.1000, 0.1000],
                shininess: 0.2000,
                transparency: 0.0,
            },
        }
    }

    /// Convenience constructor that returns brass material defaults.
    pub fn default_brass() -> Self {
        Self::new(MaterialName::Brass)
    }

    /// Set the transparency value (0.0 = fully opaque, 1.0 = fully transparent).
    pub fn set_transparency(&mut self, t: f64) {
        self.transparency = t.clamp(0.0, 1.0);
    }

    /// Set the shininess exponent (0.0 = matte, 1.0 = very glossy).
    pub fn set_shininess(&mut self, s: f64) {
        self.shininess = s.clamp(0.0, 1.0);
    }

    /// Returns true when the material has any transparency.
    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.0
    }
}

// occt: Graphic3d_BSDF
/// Bidirectional Scattering Distribution Function coefficients used for
/// physically-based rendering in OCCT's PBR pipeline.
#[derive(Clone, Debug)]
pub struct Bsdf {
    /// Diffuse reflectance (albedo) colour [R, G, B].
    pub kd: [f64; 3],
    /// Specular/glossy reflectance colour [R, G, B].
    pub ks: [f64; 3],
    /// Transmittance (refraction) colour [R, G, B].
    pub kt: [f64; 3],
    /// Emitted radiance colour [R, G, B].
    pub le: [f64; 3],
}

impl Bsdf {
    /// Create a zero-initialised BSDF (black, non-emissive, opaque).
    pub fn new() -> Self {
        Bsdf {
            kd: [0.0; 3],
            ks: [0.0; 3],
            kt: [0.0; 3],
            le: [0.0; 3],
        }
    }

    /// Create a purely Lambertian (diffuse) BSDF for the given albedo colour.
    pub fn diffuse(color: [f64; 3]) -> Self {
        Bsdf {
            kd: color,
            ks: [0.0; 3],
            kt: [0.0; 3],
            le: [0.0; 3],
        }
    }

    /// Create a metallic BSDF.
    ///
    /// `color` is the characteristic reflectance (F0) of the metal.
    /// `roughness` controls the GGX microfacet width (0.0 = mirror, 1.0 = fully rough).
    /// The diffuse term is zeroed for conductors; roughness is packed into the alpha
    /// channel of `ks` following OCCT convention where `ks[3]` encodes 1 - roughness.
    /// Because `ks` here is `[f64; 3]` (RGB only) the roughness is stored in `kd[0]`
    /// as a sentinel so callers can retrieve it without a separate field.
    pub fn metal(color: [f64; 3], roughness: f64) -> Self {
        let roughness = roughness.clamp(0.0, 1.0);
        Bsdf {
            kd: [roughness, 0.0, 0.0],
            ks: color,
            kt: [0.0; 3],
            le: [0.0; 3],
        }
    }
}

impl Default for Bsdf {
    fn default() -> Self {
        Self::new()
    }
}
