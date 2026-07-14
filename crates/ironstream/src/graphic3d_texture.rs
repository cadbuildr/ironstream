// FILE: graphic3d_texture.rs
// occt: Graphic3d_TextureRoot, Graphic3d_Texture2D
// occt-ref: Graphic3d_TextureParams
//       Graphic3d_TextureEnv, Graphic3d_TextureUnit

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureWrap {
    Repeat,
    Clamp,
    ClampToBorder,
    MirroredRepeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureUnit {
    Diffuse,
    Ambient,
    Specular,
    Shininess,
    SelfEmission,
    Normal,
    PbrAlbedo,
    PbrRoughness,
    PbrEmissive,
    PbrOcclusion,
}

// occt-ref: Graphic3d_TextureParams
#[derive(Clone, Debug)]
pub struct TextureParams {
    pub filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
    pub gen_mode: TextureGenMode,
    pub scale: [f32; 2],
    pub translation: [f32; 2],
    pub rotation: f32,
    pub is_modulate: bool,
    pub anisotropy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureGenMode {
    ObjectLinear,
    SphereMap,
    NormalMap,
    Manual,
}

impl TextureParams {
    pub fn new() -> Self {
        Self {
            filter: TextureFilter::Linear,
            wrap_s: TextureWrap::Repeat,
            wrap_t: TextureWrap::Repeat,
            gen_mode: TextureGenMode::Manual,
            scale: [1.0, 1.0],
            translation: [0.0, 0.0],
            rotation: 0.0,
            is_modulate: false,
            anisotropy: 1.0,
        }
    }

    pub fn with_filter(mut self, f: TextureFilter) -> Self { self.filter = f; self }
    pub fn with_wrap(mut self, s: TextureWrap, t: TextureWrap) -> Self { self.wrap_s = s; self.wrap_t = t; self }
    pub fn with_scale(mut self, sx: f32, sy: f32) -> Self { self.scale = [sx, sy]; self }
    pub fn is_mip_mapped(&self) -> bool {
        matches!(self.filter, TextureFilter::NearestMipmapNearest | TextureFilter::LinearMipmapNearest |
                              TextureFilter::NearestMipmapLinear | TextureFilter::LinearMipmapLinear)
    }
}

impl Default for TextureParams {
    fn default() -> Self { Self::new() }
}

// occt: Graphic3d_TextureRoot
#[derive(Clone, Debug)]
pub struct TextureRoot {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub params: TextureParams,
    pub is_done: bool,
    pub texture_id: u64,
}

impl TextureRoot {
    pub fn from_file(path: impl Into<String>, id: u64) -> Self {
        let path = path.into();
        let is_done = !path.is_empty();
        Self { path, width: 0, height: 0, params: TextureParams::new(), is_done, texture_id: id }
    }

    pub fn from_memory(width: u32, height: u32, id: u64) -> Self {
        Self { path: String::new(), width, height, params: TextureParams::new(), is_done: true, texture_id: id }
    }

    pub fn is_valid(&self) -> bool { self.is_done }
    pub fn nb_pixels(&self) -> u64 { self.width as u64 * self.height as u64 }
}

// occt: Graphic3d_Texture2D
#[derive(Clone, Debug)]
pub struct Texture2D {
    pub root: TextureRoot,
    pub unit: TextureUnit,
    pub is_alpha: bool,
}

impl Texture2D {
    pub fn new(root: TextureRoot, unit: TextureUnit) -> Self {
        Self { root, unit, is_alpha: false }
    }

    pub fn path(&self) -> &str { &self.root.path }
    pub fn is_valid(&self) -> bool { self.root.is_valid() }
    pub fn params(&self) -> &TextureParams { &self.root.params }
    pub fn params_mut(&mut self) -> &mut TextureParams { &mut self.root.params }
}

// occt: Graphic3d_TextureEnv // — environment (cube or sphere) map
#[derive(Clone, Debug)]
pub struct TextureEnv {
    pub path: String,
    pub is_cube_map: bool,
    pub cube_faces: Vec<String>,
    pub params: TextureParams,
}

impl TextureEnv {
    pub fn sphere_map(path: impl Into<String>) -> Self {
        Self { path: path.into(), is_cube_map: false, cube_faces: Vec::new(), params: TextureParams::new() }
    }

    pub fn cube_map(faces: Vec<String>) -> Self {
        Self { path: String::new(), is_cube_map: true, cube_faces: faces, params: TextureParams::new() }
    }

    pub fn is_valid(&self) -> bool {
        if self.is_cube_map { self.cube_faces.len() == 6 } else { !self.path.is_empty() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_params_defaults() {
        let p = TextureParams::new();
        assert!(!p.is_mip_mapped());
        assert_eq!(p.filter, TextureFilter::Linear);
    }

    #[test]
    fn texture_params_with_filter() {
        let p = TextureParams::new().with_filter(TextureFilter::LinearMipmapLinear);
        assert!(p.is_mip_mapped());
    }

    #[test]
    fn texture_root_from_file() {
        let t = TextureRoot::from_file("diffuse.png", 1);
        assert!(t.is_valid());
        assert_eq!(t.texture_id, 1);
    }

    #[test]
    fn texture2d_valid() {
        let root = TextureRoot::from_memory(512, 512, 2);
        assert_eq!(root.nb_pixels(), 512*512);
        let t = Texture2D::new(root, TextureUnit::PbrAlbedo);
        assert!(t.is_valid());
    }

    #[test]
    fn texture_env_cube_valid() {
        let faces: Vec<String> = (0..6).map(|i| format!("face{}.png", i)).collect();
        let env = TextureEnv::cube_map(faces);
        assert!(env.is_valid());
        assert!(env.is_cube_map);
    }
}
