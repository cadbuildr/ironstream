// FILE: v3d_image_dump_options.rs
// occt: V3d_ImageDumpOptions

//! Options for image dump functionality in V3d.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BufferType {
    RGB,
    Depth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StereoDumpOptions {
    Mono,
    LeftEye,
    RightEye,
}

#[derive(Clone, Debug)]
pub struct ImageDumpOptions {
    pub width: i32,
    pub height: i32,
    pub buffer_type: BufferType,
    pub stereo_options: StereoDumpOptions,
    pub tile_size: i32,
    pub to_adjust_aspect: bool,
    pub target_z_layer_id: i32,
    pub is_single_layer: bool,
    pub light_name: String,
}

impl Default for ImageDumpOptions {
    fn default() -> Self {
        ImageDumpOptions {
            width: 0,
            height: 0,
            buffer_type: BufferType::RGB,
            stereo_options: StereoDumpOptions::Mono,
            tile_size: 0,
            to_adjust_aspect: true,
            target_z_layer_id: -1,
            is_single_layer: false,
            light_name: String::new(),
        }
    }
}

impl ImageDumpOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_buffer_type(mut self, buffer_type: BufferType) -> Self {
        self.buffer_type = buffer_type;
        self
    }

    pub fn with_stereo_options(mut self, stereo: StereoDumpOptions) -> Self {
        self.stereo_options = stereo;
        self
    }

    pub fn with_tile_size(mut self, size: i32) -> Self {
        self.tile_size = size;
        self
    }

    pub fn with_adjust_aspect(mut self, adjust: bool) -> Self {
        self.to_adjust_aspect = adjust;
        self
    }

    pub fn with_light_name(mut self, name: impl Into<String>) -> Self {
        self.light_name = name.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_image_dump_options() {
        let opts = ImageDumpOptions::default();
        assert_eq!(opts.width, 0);
        assert_eq!(opts.height, 0);
        assert_eq!(opts.buffer_type, BufferType::RGB);
        assert_eq!(opts.stereo_options, StereoDumpOptions::Mono);
        assert!(opts.to_adjust_aspect);
        assert_eq!(opts.light_name, "");
    }

    #[test]
    fn builder_chain() {
        let opts = ImageDumpOptions::new()
            .with_size(1024, 768)
            .with_buffer_type(BufferType::Depth)
            .with_stereo_options(StereoDumpOptions::LeftEye)
            .with_light_name("MainLight");

        assert_eq!(opts.width, 1024);
        assert_eq!(opts.height, 768);
        assert_eq!(opts.buffer_type, BufferType::Depth);
        assert_eq!(opts.stereo_options, StereoDumpOptions::LeftEye);
        assert_eq!(opts.light_name, "MainLight");
    }
}
