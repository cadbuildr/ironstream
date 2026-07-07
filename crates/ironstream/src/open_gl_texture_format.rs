// FILE: open_gl_texture_format.rs
// occt: OpenGl_TextureFormat

//! Stores parameters of OpenGL texture format.

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Unknown,
    Gray,
    GrayAlpha,
    Rgb,
    Rgba,
    Bgr,
    Bgra,
}

/// Component data type for texture format selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
    SignedByte,
    SignedShort,
    SignedInt,
    Float,
}

/// OpenGL texture format descriptor
#[derive(Debug, Clone, Copy)]
pub struct OpenGlTextureFormat {
    image_format: ImageFormat,
    internal_format: u32,
    pixel_format: u32,
    data_type: u32,
    nb_components: u32,
}

impl OpenGlTextureFormat {
    /// Empty constructor (invalid texture format)
    pub fn new() -> Self {
        Self {
            image_format: ImageFormat::Unknown,
            internal_format: 0,
            pixel_format: 0,
            data_type: 0,
            nb_components: 0,
        }
    }

    /// Return TRUE if format is defined
    pub fn is_valid(&self) -> bool {
        self.internal_format != 0 && self.pixel_format != 0 && self.data_type != 0
    }

    /// Returns OpenGL internal format of the pixel data (example: GL_R32F)
    pub fn internal_format(&self) -> u32 {
        self.internal_format
    }

    /// Sets texture internal format
    pub fn set_internal_format(&mut self, format: u32) {
        self.internal_format = format;
    }

    /// Returns OpenGL format of the pixel data (example: GL_RED)
    pub fn pixel_format(&self) -> u32 {
        self.pixel_format
    }

    /// Sets OpenGL format of the pixel data
    pub fn set_pixel_format(&mut self, format: u32) {
        self.pixel_format = format;
    }

    /// Returns OpenGL data type of the pixel data (example: GL_FLOAT)
    pub fn data_type(&self) -> u32 {
        self.data_type
    }

    /// Sets OpenGL data type of the pixel data
    pub fn set_data_type(&mut self, data_type: u32) {
        self.data_type = data_type;
    }

    /// Returns number of components (channels)
    pub fn nb_components(&self) -> u32 {
        self.nb_components
    }

    /// Sets number of components (channels)
    pub fn set_nb_components(&mut self, nb: u32) {
        self.nb_components = nb;
    }

    /// Return TRUE if internal texture format is sRGB(A)
    pub fn is_srgb(&self) -> bool {
        const GL_SRGB8: u32 = 0x8C41;
        const GL_SRGB8_ALPHA8: u32 = 0x8C43;
        self.internal_format == GL_SRGB8 || self.internal_format == GL_SRGB8_ALPHA8
    }

    /// Returns image format (best match or Unknown if no suitable fit)
    pub fn image_format(&self) -> ImageFormat {
        self.image_format
    }

    /// Sets image format
    pub fn set_image_format(&mut self, format: ImageFormat) {
        self.image_format = format;
    }

    /// Alias: Returns OpenGL internal format
    pub fn internal(&self) -> u32 {
        self.internal_format
    }

    /// Alias: Returns OpenGL format of the pixel data
    pub fn format(&self) -> u32 {
        self.pixel_format
    }

    /// Create texture format for unsigned byte data
    pub fn create_ubyte(nb_components: u32) -> Self {
        const GL_UNSIGNED_BYTE: u32 = 0x1401;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_R8: u32 = 0x8229;
        const GL_RG8: u32 = 0x822B;
        const GL_RGB8: u32 = 0x8051;
        const GL_RGBA8: u32 = 0x8058;

        let internal = match nb_components {
            1 => GL_R8,
            2 => GL_RG8,
            3 => GL_RGB8,
            4 => GL_RGBA8,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_UNSIGNED_BYTE,
            nb_components,
        }
    }

    /// Create texture format for unsigned short data
    pub fn create_ushort(nb_components: u32) -> Self {
        const GL_UNSIGNED_SHORT: u32 = 0x1403;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_R16: u32 = 0x822A;
        const GL_RG16: u32 = 0x822C;
        const GL_RGB16: u32 = 0x8054;
        const GL_RGBA16: u32 = 0x805B;

        let internal = match nb_components {
            1 => GL_R16,
            2 => GL_RG16,
            3 => GL_RGB16,
            4 => GL_RGBA16,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_UNSIGNED_SHORT,
            nb_components,
        }
    }

    /// Create texture format for float data
    pub fn create_float(nb_components: u32) -> Self {
        const GL_FLOAT: u32 = 0x1406;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_R32F: u32 = 0x822E;
        const GL_RG32F: u32 = 0x8230;
        const GL_RGB32F: u32 = 0x8815;
        const GL_RGBA32F: u32 = 0x8814;

        let internal = match nb_components {
            1 => GL_R32F,
            2 => GL_RG32F,
            3 => GL_RGB32F,
            4 => GL_RGBA32F,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_FLOAT,
            nb_components,
        }
    }

    /// Create texture format for unsigned int data
    pub fn create_uint(nb_components: u32) -> Self {
        const GL_UNSIGNED_INT: u32 = 0x1405;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        let internal = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_UNSIGNED_INT,
            nb_components,
        }
    }

    /// Create texture format for signed byte data
    pub fn create_sbyte(nb_components: u32) -> Self {
        const GL_BYTE: u32 = 0x1400;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_R8_SNORM: u32 = 0x8F94;
        const GL_RG8_SNORM: u32 = 0x8F95;
        const GL_RGB8_SNORM: u32 = 0x8F96;
        const GL_RGBA8_SNORM: u32 = 0x8F97;

        let internal = match nb_components {
            1 => GL_R8_SNORM,
            2 => GL_RG8_SNORM,
            3 => GL_RGB8_SNORM,
            4 => GL_RGBA8_SNORM,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_BYTE,
            nb_components,
        }
    }

    /// Create texture format for signed short data
    pub fn create_sshort(nb_components: u32) -> Self {
        const GL_SHORT: u32 = 0x1402;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_R16_SNORM: u32 = 0x8F98;
        const GL_RG16_SNORM: u32 = 0x8F99;
        const GL_RGB16_SNORM: u32 = 0x8F9A;
        const GL_RGBA16_SNORM: u32 = 0x8F9B;

        let internal = match nb_components {
            1 => GL_R16_SNORM,
            2 => GL_RG16_SNORM,
            3 => GL_RGB16_SNORM,
            4 => GL_RGBA16_SNORM,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_SHORT,
            nb_components,
        }
    }

    /// Create texture format for signed int data
    pub fn create_sint(nb_components: u32) -> Self {
        const GL_INT: u32 = 0x1404;
        const GL_RED: u32 = 0x1903;
        const GL_RG: u32 = 0x8227;
        const GL_RGB: u32 = 0x1907;
        const GL_RGBA: u32 = 0x1908;

        const GL_RED_SNORM: u32 = 0x8F90;
        const GL_RG_SNORM: u32 = 0x8F91;
        const GL_RGB_SNORM: u32 = 0x8F92;
        const GL_RGBA_SNORM: u32 = 0x8F93;

        let internal = match nb_components {
            1 => GL_RED_SNORM,
            2 => GL_RG_SNORM,
            3 => GL_RGB_SNORM,
            4 => GL_RGBA_SNORM,
            _ => 0,
        };

        let pixel = match nb_components {
            1 => GL_RED,
            2 => GL_RG,
            3 => GL_RGB,
            4 => GL_RGBA,
            _ => 0,
        };

        Self {
            image_format: ImageFormat::Unknown,
            internal_format: internal,
            pixel_format: pixel,
            data_type: GL_INT,
            nb_components,
        }
    }
}

impl Default for OpenGlTextureFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invalid() {
        let fmt = OpenGlTextureFormat::new();
        assert!(!fmt.is_valid());
        assert_eq!(fmt.internal_format(), 0);
        assert_eq!(fmt.pixel_format(), 0);
        assert_eq!(fmt.data_type(), 0);
    }

    #[test]
    fn test_setters() {
        let mut fmt = OpenGlTextureFormat::new();
        fmt.set_internal_format(0x8058); // GL_RGBA8
        fmt.set_pixel_format(0x1908);    // GL_RGBA
        fmt.set_data_type(0x1401);       // GL_UNSIGNED_BYTE
        fmt.set_nb_components(4);

        assert!(fmt.is_valid());
        assert_eq!(fmt.internal_format(), 0x8058);
        assert_eq!(fmt.pixel_format(), 0x1908);
        assert_eq!(fmt.data_type(), 0x1401);
        assert_eq!(fmt.nb_components(), 4);
    }

    #[test]
    fn test_create_ubyte_1ch() {
        let fmt = OpenGlTextureFormat::create_ubyte(1);
        assert!(fmt.is_valid());
        assert_eq!(fmt.nb_components(), 1);
        assert_eq!(fmt.internal_format(), 0x8229); // GL_R8
        assert_eq!(fmt.pixel_format(), 0x1903);    // GL_RED
        assert_eq!(fmt.data_type(), 0x1401);       // GL_UNSIGNED_BYTE
    }

    #[test]
    fn test_create_ubyte_4ch() {
        let fmt = OpenGlTextureFormat::create_ubyte(4);
        assert!(fmt.is_valid());
        assert_eq!(fmt.nb_components(), 4);
        assert_eq!(fmt.internal_format(), 0x8058); // GL_RGBA8
        assert_eq!(fmt.pixel_format(), 0x1908);    // GL_RGBA
    }

    #[test]
    fn test_create_ushort() {
        let fmt = OpenGlTextureFormat::create_ushort(2);
        assert!(fmt.is_valid());
        assert_eq!(fmt.nb_components(), 2);
        assert_eq!(fmt.data_type(), 0x1403); // GL_UNSIGNED_SHORT
        assert_eq!(fmt.pixel_format(), 0x8227); // GL_RG
    }

    #[test]
    fn test_create_float() {
        let fmt = OpenGlTextureFormat::create_float(3);
        assert!(fmt.is_valid());
        assert_eq!(fmt.nb_components(), 3);
        assert_eq!(fmt.internal_format(), 0x8815); // GL_RGB32F
        assert_eq!(fmt.data_type(), 0x1406);       // GL_FLOAT
        assert_eq!(fmt.pixel_format(), 0x1907);    // GL_RGB
    }

    #[test]
    fn test_create_uint() {
        let fmt = OpenGlTextureFormat::create_uint(4);
        assert!(fmt.is_valid());
        assert_eq!(fmt.nb_components(), 4);
        assert_eq!(fmt.data_type(), 0x1405); // GL_UNSIGNED_INT
    }

    #[test]
    fn test_create_sbyte() {
        let fmt = OpenGlTextureFormat::create_sbyte(1);
        assert!(fmt.is_valid());
        assert_eq!(fmt.data_type(), 0x1400); // GL_BYTE
        assert_eq!(fmt.internal_format(), 0x8F94); // GL_R8_SNORM
    }

    #[test]
    fn test_create_sshort() {
        let fmt = OpenGlTextureFormat::create_sshort(4);
        assert!(fmt.is_valid());
        assert_eq!(fmt.data_type(), 0x1402); // GL_SHORT
        assert_eq!(fmt.internal_format(), 0x8F9B); // GL_RGBA16_SNORM
    }

    #[test]
    fn test_create_sint() {
        let fmt = OpenGlTextureFormat::create_sint(2);
        assert!(fmt.is_valid());
        assert_eq!(fmt.data_type(), 0x1404); // GL_INT
        assert_eq!(fmt.internal_format(), 0x8F91); // GL_RG_SNORM
    }

    #[test]
    fn test_is_srgb() {
        let mut fmt = OpenGlTextureFormat::new();
        fmt.set_internal_format(0x8C41); // GL_SRGB8
        assert!(fmt.is_srgb());

        let mut fmt2 = OpenGlTextureFormat::new();
        fmt2.set_internal_format(0x8C43); // GL_SRGB8_ALPHA8
        assert!(fmt2.is_srgb());

        let mut fmt3 = OpenGlTextureFormat::new();
        fmt3.set_internal_format(0x8058); // GL_RGBA8
        assert!(!fmt3.is_srgb());
    }

    #[test]
    fn test_image_format() {
        let mut fmt = OpenGlTextureFormat::new();
        assert_eq!(fmt.image_format(), ImageFormat::Unknown);

        fmt.set_image_format(ImageFormat::Rgba);
        assert_eq!(fmt.image_format(), ImageFormat::Rgba);
    }

    #[test]
    fn test_aliases() {
        let mut fmt = OpenGlTextureFormat::new();
        fmt.set_internal_format(0x8815); // GL_RGB32F
        fmt.set_pixel_format(0x1907);    // GL_RGB

        assert_eq!(fmt.internal(), fmt.internal_format());
        assert_eq!(fmt.format(), fmt.pixel_format());
    }

    #[test]
    fn test_invalid_nb_components() {
        let fmt = OpenGlTextureFormat::create_ubyte(5);
        assert!(!fmt.is_valid()); // 5 components not supported
    }
}
