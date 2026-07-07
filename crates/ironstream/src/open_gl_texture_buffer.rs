// FILE: open_gl_texture_buffer.rs
// occt: OpenGl_TextureBuffer

//! Texture Buffer Object (TBO).
//! A special 1D texture that is VBO-style initialized.
//! Main differences from general 1D texture:
//!  - no interpolation between fields
//!  - greater sizes
//!  - special sampler object in GLSL shader to access data by index
//!
//! Though TBO is inherited from VBO this is to unify design;
//! users shouldn't cast it to the base class.

/// Texture unit enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureUnit {
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
    Unit8,
    Unit9,
    Unit10,
    Unit11,
    Unit12,
    Unit13,
    Unit14,
    Unit15,
}

/// GL context reference (placeholder for actual OpenGL context)
#[derive(Debug, Clone)]
pub struct GlContext;

/// OpenGL Buffer base class (simplified)
#[derive(Debug)]
pub struct GlBuffer {
    buffer_id: u32,
    target: u32,
    is_valid: bool,
}

impl GlBuffer {
    pub fn new() -> Self {
        Self {
            buffer_id: 0,
            target: 0x88EC, // GL_COPY_READ_BUFFER
            is_valid: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn release(&mut self, _ctx: &GlContext) {
        self.is_valid = false;
        self.buffer_id = 0;
    }
}

impl Default for GlBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Texture Buffer Object
#[derive(Debug)]
pub struct OpenGlTextureBuffer {
    buffer: GlBuffer,
    texture_id: u32,
    tex_format: u32,
}

impl OpenGlTextureBuffer {
    /// Constant: no texture
    pub const NO_TEXTURE: u32 = 0;

    /// Create uninitialized TBO
    pub fn new() -> Self {
        Self {
            buffer: GlBuffer::new(),
            texture_id: Self::NO_TEXTURE,
            tex_format: 0,
        }
    }

    /// Returns true if TBO is valid.
    /// Note that no real GL call is performed.
    pub fn is_valid(&self) -> bool {
        self.buffer.is_valid() && self.texture_id != Self::NO_TEXTURE
    }

    /// Get target for this buffer (TBO uses GL_TEXTURE_BUFFER)
    pub fn get_target(&self) -> u32 {
        0x8C2E // GL_TEXTURE_BUFFER
    }

    /// Destroy object - will release GPU memory if any
    pub fn release(&mut self, ctx: &GlContext) {
        self.buffer.release(ctx);
        if self.texture_id != Self::NO_TEXTURE {
            // In real implementation, would call glDeleteTextures
            self.texture_id = Self::NO_TEXTURE;
        }
        self.tex_format = 0;
    }

    /// Creates VBO and Texture names (ids) if not yet generated.
    /// Data should be initialized by another method.
    pub fn create(&mut self, _ctx: &GlContext) -> bool {
        if self.buffer.is_valid() {
            return true;
        }
        // In real implementation, would call glGenBuffers and glGenTextures
        self.buffer.buffer_id = 1;
        self.buffer.is_valid = true;
        self.texture_id = 1;
        true
    }

    /// Perform TBO initialization with float data.
    /// Existing data will be deleted.
    pub fn init_float(
        &mut self,
        ctx: &GlContext,
        components_nb: u32,
        elems_nb: i32,
        data: &[f32],
    ) -> bool {
        self.create(ctx);
        if elems_nb <= 0 || components_nb == 0 {
            return false;
        }

        // Validate data size
        let expected_size = (elems_nb as u32 * components_nb) as usize;
        if data.len() < expected_size {
            return false;
        }

        // In real implementation, would call glBufferData with GL_DYNAMIC_DRAW
        // and set format based on components_nb (R32F, RG32F, RGBA32F, etc.)
        self.tex_format = match components_nb {
            1 => 0x822E, // GL_R32F
            2 => 0x8230, // GL_RG32F
            3 => 0x8815, // GL_RGB32F
            4 => 0x8814, // GL_RGBA32F
            _ => return false,
        };
        true
    }

    /// Perform TBO initialization with unsigned int data.
    /// Existing data will be deleted.
    pub fn init_uint(
        &mut self,
        ctx: &GlContext,
        components_nb: u32,
        elems_nb: i32,
        data: &[u32],
    ) -> bool {
        self.create(ctx);
        if elems_nb <= 0 || components_nb == 0 {
            return false;
        }

        let expected_size = (elems_nb as u32 * components_nb) as usize;
        if data.len() < expected_size {
            return false;
        }

        self.tex_format = match components_nb {
            1 => 0x8236, // GL_R32UI
            2 => 0x8238, // GL_RG32UI
            3 => 0x8D71, // GL_RGB32UI
            4 => 0x8D70, // GL_RGBA32UI
            _ => return false,
        };
        true
    }

    /// Perform TBO initialization with unsigned short data.
    /// Existing data will be deleted.
    pub fn init_ushort(
        &mut self,
        ctx: &GlContext,
        components_nb: u32,
        elems_nb: i32,
        data: &[u16],
    ) -> bool {
        self.create(ctx);
        if elems_nb <= 0 || components_nb == 0 {
            return false;
        }

        let expected_size = (elems_nb as u32 * components_nb) as usize;
        if data.len() < expected_size {
            return false;
        }

        self.tex_format = match components_nb {
            1 => 0x8234, // GL_R16UI
            2 => 0x8235, // GL_RG16UI
            3 => 0x8D89, // GL_RGB16UI
            4 => 0x8D88, // GL_RGBA16UI
            _ => return false,
        };
        true
    }

    /// Perform TBO initialization with uint8_t data.
    /// Existing data will be deleted.
    pub fn init_ubyte(
        &mut self,
        ctx: &GlContext,
        components_nb: u32,
        elems_nb: i32,
        data: &[u8],
    ) -> bool {
        self.create(ctx);
        if elems_nb <= 0 || components_nb == 0 {
            return false;
        }

        let expected_size = (elems_nb as u32 * components_nb) as usize;
        if data.len() < expected_size {
            return false;
        }

        self.tex_format = match components_nb {
            1 => 0x8232, // GL_R8UI
            2 => 0x8233, // GL_RG8UI
            3 => 0x8D7D, // GL_RGB8UI
            4 => 0x8D7C, // GL_RGBA8UI
            _ => return false,
        };
        true
    }

    /// Bind TBO to specified Texture Unit
    pub fn bind_texture(&self, _ctx: &GlContext, _unit: TextureUnit) {
        // In real implementation, would call:
        // glActiveTexture(GL_TEXTURE0 + unit_index)
        // glBindTexture(GL_TEXTURE_BUFFER, self.texture_id)
    }

    /// Unbind TBO
    pub fn unbind_texture(&self, _ctx: &GlContext, _unit: TextureUnit) {
        // In real implementation, would call:
        // glActiveTexture(GL_TEXTURE0 + unit_index)
        // glBindTexture(GL_TEXTURE_BUFFER, 0)
    }

    /// Returns name of TBO
    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    /// Returns internal texture format
    pub fn texture_format(&self) -> u32 {
        self.tex_format
    }
}

impl Default for OpenGlTextureBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_uninitialized() {
        let tbo = OpenGlTextureBuffer::new();
        assert!(!tbo.is_valid());
        assert_eq!(tbo.texture_id(), OpenGlTextureBuffer::NO_TEXTURE);
        assert_eq!(tbo.texture_format(), 0);
    }

    #[test]
    fn test_create() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        assert!(tbo.create(&ctx));
        assert!(tbo.is_valid());
        assert!(tbo.texture_id() != OpenGlTextureBuffer::NO_TEXTURE);
    }

    #[test]
    fn test_get_target() {
        let tbo = OpenGlTextureBuffer::new();
        assert_eq!(tbo.get_target(), 0x8C2E); // GL_TEXTURE_BUFFER
    }

    #[test]
    fn test_init_float() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = tbo.init_float(&ctx, 2, 2, &data);

        assert!(result);
        assert!(tbo.is_valid());
        assert_eq!(tbo.texture_format(), 0x8230); // GL_RG32F
    }

    #[test]
    fn test_init_uint() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        let data = vec![1u32, 2, 3, 4, 5, 6];
        let result = tbo.init_uint(&ctx, 3, 2, &data);

        assert!(result);
        assert_eq!(tbo.texture_format(), 0x8D71); // GL_RGB32UI
    }

    #[test]
    fn test_init_ushort() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        let data = vec![1u16, 2, 3, 4];
        let result = tbo.init_ushort(&ctx, 4, 1, &data);

        assert!(result);
        assert_eq!(tbo.texture_format(), 0x8D88); // GL_RGBA16UI
    }

    #[test]
    fn test_init_ubyte() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        let data = vec![255u8, 128, 64, 32];
        let result = tbo.init_ubyte(&ctx, 1, 4, &data);

        assert!(result);
        assert_eq!(tbo.texture_format(), 0x8232); // GL_R8UI
    }

    #[test]
    fn test_init_invalid_size() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        let data = vec![1.0, 2.0];
        // Asking for 3 elements with 2 components but only 2 floats
        let result = tbo.init_float(&ctx, 2, 3, &data);

        assert!(!result);
    }

    #[test]
    fn test_release() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        tbo.create(&ctx);
        assert!(tbo.is_valid());

        tbo.release(&ctx);
        assert!(!tbo.is_valid());
        assert_eq!(tbo.texture_id(), OpenGlTextureBuffer::NO_TEXTURE);
        assert_eq!(tbo.texture_format(), 0);
    }

    #[test]
    fn test_bind_unbind() {
        let mut tbo = OpenGlTextureBuffer::new();
        let ctx = GlContext;

        tbo.create(&ctx);

        // These shouldn't panic
        tbo.bind_texture(&ctx, TextureUnit::Unit0);
        tbo.unbind_texture(&ctx, TextureUnit::Unit5);
    }
}
