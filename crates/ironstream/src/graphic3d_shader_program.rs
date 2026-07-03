// FILE: graphic3d_shader_program.rs
// occt: Graphic3d_ShaderProgram

use std::sync::atomic::{AtomicUsize, Ordering};

/// Shader program for GLSL rendering.
pub struct Graphic3dShaderProgram {
    id: String,
    header: String,
    nb_lights_max: i32,
    nb_shadow_maps: i32,
    nb_clip_planes_max: i32,
    nb_fragment_outputs: i32,
    texture_set_bits: i32,
    has_alpha_test: bool,
    has_default_sampler: bool,
    is_pbr: bool,
}

impl Graphic3dShaderProgram {
    /// Default value of THE_MAX_LIGHTS macro.
    pub const THE_MAX_LIGHTS_DEFAULT: i32 = 8;

    /// Default value of THE_MAX_CLIP_PLANES macro.
    pub const THE_MAX_CLIP_PLANES_DEFAULT: i32 = 8;

    /// Default value of THE_NB_FRAG_OUTPUTS macro.
    pub const THE_NB_FRAG_OUTPUTS: i32 = 1;

    /// Creates a new empty program object.
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        Graphic3dShaderProgram {
            id: format!("shader_program_{}", id),
            header: String::new(),
            nb_lights_max: Self::THE_MAX_LIGHTS_DEFAULT,
            nb_shadow_maps: 0,
            nb_clip_planes_max: Self::THE_MAX_CLIP_PLANES_DEFAULT,
            nb_fragment_outputs: Self::THE_NB_FRAG_OUTPUTS,
            texture_set_bits: 0,
            has_alpha_test: false,
            has_default_sampler: true,
            is_pbr: false,
        }
    }

    /// Returns the unique ID used to manage resource in graphic driver.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Sets the unique ID.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Returns GLSL header.
    pub fn header(&self) -> &str {
        &self.header
    }

    /// Sets GLSL header.
    pub fn set_header(&mut self, header: String) {
        self.header = header;
    }

    /// Appends a line to GLSL header.
    pub fn append_to_header(&mut self, line: &str) {
        if !self.header.is_empty() {
            self.header.push('\n');
        }
        self.header.push_str(line);
    }

    /// Returns the length of array of light sources.
    pub fn nb_lights_max(&self) -> i32 {
        self.nb_lights_max
    }

    /// Sets the length of array of light sources.
    pub fn set_nb_lights_max(&mut self, count: i32) {
        self.nb_lights_max = count;
    }

    /// Returns the length of array of shadow maps.
    pub fn nb_shadow_maps(&self) -> i32 {
        self.nb_shadow_maps
    }

    /// Sets the length of array of shadow maps.
    pub fn set_nb_shadow_maps(&mut self, count: i32) {
        self.nb_shadow_maps = count;
    }

    /// Returns the length of array of clipping planes.
    pub fn nb_clip_planes_max(&self) -> i32 {
        self.nb_clip_planes_max
    }

    /// Sets the length of array of clipping planes.
    pub fn set_nb_clip_planes_max(&mut self, count: i32) {
        self.nb_clip_planes_max = count;
    }

    /// Returns the number of Fragment Shader outputs.
    pub fn nb_fragment_outputs(&self) -> i32 {
        self.nb_fragment_outputs
    }

    /// Sets the number of Fragment Shader outputs.
    pub fn set_nb_fragment_outputs(&mut self, count: i32) {
        self.nb_fragment_outputs = count;
    }

    /// Returns whether Fragment Shader should perform alpha test.
    pub fn has_alpha_test(&self) -> bool {
        self.has_alpha_test
    }

    /// Sets whether Fragment Shader should perform alpha test.
    pub fn set_alpha_test(&mut self, enable: bool) {
        self.has_alpha_test = enable;
    }

    /// Returns whether standard program header should define default texture sampler.
    pub fn has_default_sampler(&self) -> bool {
        self.has_default_sampler
    }

    /// Sets whether standard program header should define default texture sampler.
    pub fn set_default_sampler(&mut self, enable: bool) {
        self.has_default_sampler = enable;
    }

    /// Returns whether standard program header should define PBR functions.
    pub fn is_pbr(&self) -> bool {
        self.is_pbr
    }

    /// Sets whether standard program header should define PBR functions.
    pub fn set_pbr(&mut self, enable: bool) {
        self.is_pbr = enable;
    }

    /// Returns texture units declared within the program.
    pub fn texture_set_bits(&self) -> i32 {
        self.texture_set_bits
    }

    /// Sets texture units declared within the program.
    pub fn set_texture_set_bits(&mut self, bits: i32) {
        self.texture_set_bits = bits;
    }

    /// Checks if the program object is valid or not.
    pub fn is_done(&self) -> bool {
        !self.id.is_empty()
    }
}

impl Default for Graphic3dShaderProgram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_program_creation() {
        let prog = Graphic3dShaderProgram::new();
        assert!(!prog.get_id().is_empty());
        assert_eq!(prog.nb_lights_max(), Graphic3dShaderProgram::THE_MAX_LIGHTS_DEFAULT);
        assert_eq!(prog.nb_clip_planes_max(), Graphic3dShaderProgram::THE_MAX_CLIP_PLANES_DEFAULT);
        assert!(prog.is_done());
    }

    #[test]
    fn test_shader_program_unique_ids() {
        let prog1 = Graphic3dShaderProgram::new();
        let prog2 = Graphic3dShaderProgram::new();
        assert_ne!(prog1.get_id(), prog2.get_id());
    }

    #[test]
    fn test_shader_program_header() {
        let mut prog = Graphic3dShaderProgram::new();
        assert!(prog.header().is_empty());
        prog.set_header("#version 330".to_string());
        assert_eq!(prog.header(), "#version 330");
    }

    #[test]
    fn test_shader_program_append_header() {
        let mut prog = Graphic3dShaderProgram::new();
        prog.append_to_header("#version 330");
        prog.append_to_header("#extension GL_ARB_bindless_texture : require");
        assert!(prog.header().contains("#version 330"));
        assert!(prog.header().contains("GL_ARB_bindless_texture"));
    }

    #[test]
    fn test_shader_program_lights_max() {
        let mut prog = Graphic3dShaderProgram::new();
        assert_eq!(prog.nb_lights_max(), 8);
        prog.set_nb_lights_max(16);
        assert_eq!(prog.nb_lights_max(), 16);
    }

    #[test]
    fn test_shader_program_shadow_maps() {
        let mut prog = Graphic3dShaderProgram::new();
        assert_eq!(prog.nb_shadow_maps(), 0);
        prog.set_nb_shadow_maps(4);
        assert_eq!(prog.nb_shadow_maps(), 4);
    }

    #[test]
    fn test_shader_program_clip_planes() {
        let mut prog = Graphic3dShaderProgram::new();
        assert_eq!(prog.nb_clip_planes_max(), 8);
        prog.set_nb_clip_planes_max(12);
        assert_eq!(prog.nb_clip_planes_max(), 12);
    }

    #[test]
    fn test_shader_program_fragment_outputs() {
        let mut prog = Graphic3dShaderProgram::new();
        assert_eq!(prog.nb_fragment_outputs(), 1);
        prog.set_nb_fragment_outputs(3);
        assert_eq!(prog.nb_fragment_outputs(), 3);
    }

    #[test]
    fn test_shader_program_alpha_test() {
        let mut prog = Graphic3dShaderProgram::new();
        assert!(!prog.has_alpha_test());
        prog.set_alpha_test(true);
        assert!(prog.has_alpha_test());
    }

    #[test]
    fn test_shader_program_default_sampler() {
        let mut prog = Graphic3dShaderProgram::new();
        assert!(prog.has_default_sampler());
        prog.set_default_sampler(false);
        assert!(!prog.has_default_sampler());
    }

    #[test]
    fn test_shader_program_pbr() {
        let mut prog = Graphic3dShaderProgram::new();
        assert!(!prog.is_pbr());
        prog.set_pbr(true);
        assert!(prog.is_pbr());
    }

    #[test]
    fn test_shader_program_texture_bits() {
        let mut prog = Graphic3dShaderProgram::new();
        assert_eq!(prog.texture_set_bits(), 0);
        prog.set_texture_set_bits(0xFF);
        assert_eq!(prog.texture_set_bits(), 0xFF);
    }
}
