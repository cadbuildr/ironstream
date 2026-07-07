// FILE: open_gl_tile_sampler.rs
// occt: OpenGl_TileSampler

//! Tool object used for sampling screen tiles according to estimated pixel variance.
//! Used in path tracing engine for GPU thread coherency by splitting rendering window
//! into pixel blocks or tiles. Supports adaptive sampling and tile offset management.

/// 2D vector of integers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn mul_scalar(&self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn min_elem(&self, other: Vec2i) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max_elem(&self, other: Vec2i) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

/// 2D pixmap for tile data (simplified)
#[derive(Debug, Clone)]
pub struct PixMapData {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl PixMapData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn size_x(&self) -> usize {
        self.width
    }

    pub fn size_y(&self) -> usize {
        self.height
    }

    pub fn value(&self, row: usize, col: usize) -> u32 {
        if row < self.height && col < self.width {
            self.data[row * self.width + col]
        } else {
            0
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, val: u32) {
        if row < self.height && col < self.width {
            self.data[row * self.width + col] = val;
        }
    }
}

/// Halton sampler for quasi-random sequences
#[derive(Debug, Clone)]
pub struct HaltonSampler {
    index: u32,
}

impl HaltonSampler {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self) -> (f32, f32) {
        let idx = self.index;
        self.index += 1;
        (0.5, 0.5) // Simplified: actual Halton sequence
    }
}

impl Default for HaltonSampler {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenGL Tile Sampler
pub struct OpenGlTileSampler {
    tiles: PixMapData,
    tile_samples: PixMapData,
    variance_map: Vec<f32>,
    variance_raw: Vec<i32>,
    offsets: PixMapData,
    offsets_shrunk: PixMapData,
    marginal_map: Vec<f32>,
    sampler: HaltonSampler,
    last_sample: u32,
    scale_factor: f32,
    tile_size: i32,
    view_size: Vec2i,
}

impl OpenGlTileSampler {
    /// Creates new tile sampler
    pub fn new() -> Self {
        Self {
            tiles: PixMapData::new(1, 1),
            tile_samples: PixMapData::new(1, 1),
            variance_map: vec![0.0],
            variance_raw: vec![0],
            offsets: PixMapData::new(1, 1),
            offsets_shrunk: PixMapData::new(1, 1),
            marginal_map: Vec::new(),
            sampler: HaltonSampler::new(),
            last_sample: 0,
            scale_factor: 1.0,
            tile_size: 16,
            view_size: Vec2i::new(512, 512),
        }
    }

    /// Size of individual tile in pixels
    pub fn tile_size(&self) -> Vec2i {
        Vec2i::new(self.tile_size, self.tile_size)
    }

    /// Scale factor for quantization of visual error
    pub fn variance_scale_factor(&self) -> f32 {
        self.scale_factor
    }

    /// Returns number of tiles in X dimension
    pub fn nb_tiles_x(&self) -> i32 {
        self.tiles.size_x() as i32
    }

    /// Returns number of tiles in Y dimension
    pub fn nb_tiles_y(&self) -> i32 {
        self.tiles.size_y() as i32
    }

    /// Returns total number of tiles in viewport
    pub fn nb_tiles(&self) -> i32 {
        self.nb_tiles_x() * self.nb_tiles_y()
    }

    /// Returns ray-tracing viewport
    pub fn view_size(&self) -> Vec2i {
        self.view_size
    }

    /// Number of tiles within offsets texture
    pub fn nb_offset_tiles(&self, adaptive: bool) -> Vec2i {
        if adaptive {
            Vec2i::new(
                self.offsets_shrunk.size_x() as i32,
                self.offsets_shrunk.size_y() as i32,
            )
        } else {
            Vec2i::new(self.offsets.size_x() as i32, self.offsets.size_y() as i32)
        }
    }

    /// Maximum number of tiles within offsets texture
    pub fn nb_offset_tiles_max(&self) -> Vec2i {
        let adaptive = self.nb_offset_tiles(true);
        let non_adaptive = self.nb_offset_tiles(false);
        adaptive.max_elem(non_adaptive)
    }

    /// Viewport for rendering using offsets texture
    pub fn offset_tiles_viewport(&self, adaptive: bool) -> Vec2i {
        self.nb_offset_tiles(adaptive).mul_scalar(self.tile_size)
    }

    /// Maximum viewport for rendering using offsets texture
    pub fn offset_tiles_viewport_max(&self) -> Vec2i {
        self.nb_offset_tiles_max().mul_scalar(self.tile_size)
    }

    /// Return maximum number of samples per tile
    pub fn max_tile_samples(&self) -> i32 {
        let mut max_samples = 0;
        for row in 0..self.tiles.size_y() {
            for col in 0..self.tiles.size_x() {
                max_samples = max_samples.max(self.tiles.value(row, col) as i32);
            }
        }
        max_samples
    }

    /// Resets (restart) tile sampler to initial state
    pub fn reset(&mut self) {
        self.last_sample = 0;
    }

    /// Returns number of pixels in the given tile
    fn tile_area(&self, tile_x: i32, tile_y: i32) -> i32 {
        let size_x = (self.tile_size).min(self.view_size.x - tile_x * self.tile_size);
        let size_y = (self.tile_size).min(self.view_size.y - tile_y * self.tile_size);
        size_x * size_y
    }

    /// Upload samples to texture (mock implementation)
    pub fn upload_samples(&mut self, adaptive: bool) -> bool {
        self.last_sample += 1;
        true
    }

    /// Upload offsets to texture (mock implementation)
    pub fn upload_offsets(&mut self, adaptive: bool) -> bool {
        true
    }

    /// Set viewport size and recompute tile size
    pub fn set_size(&mut self, width: i32, height: i32, preferred_tiles: i32) {
        self.view_size = Vec2i::new(width, height);

        // Compute tile size to fit approximately preferred_tiles in viewport
        let total_pixels = (width * height) as usize;
        let pixels_per_tile = total_pixels / (preferred_tiles.max(1) as usize).max(1);
        self.tile_size = (pixels_per_tile as f32).sqrt() as i32;
        self.tile_size = self.tile_size.max(1).min(128);

        // Resize tile arrays
        let nb_tiles_x = ((width + self.tile_size - 1) / self.tile_size) as usize;
        let nb_tiles_y = ((height + self.tile_size - 1) / self.tile_size) as usize;

        self.tiles = PixMapData::new(nb_tiles_x, nb_tiles_y);
        self.tile_samples = PixMapData::new(nb_tiles_x, nb_tiles_y);

        // Initialize tiles with 1 sample each
        for row in 0..nb_tiles_y {
            for col in 0..nb_tiles_x {
                let area = self.tile_area(col as i32, row as i32);
                self.tiles.set_value(row, col, 1);
                self.tile_samples.set_value(row, col, area as u32);
            }
        }
    }

    /// Fetch variance map and build discrete distribution
    pub fn grab_variance_map(&mut self) {
        // Simplified implementation: no actual GPU texture fetch
        self.variance_map.resize(self.tiles.size_x() * self.tiles.size_y(), 0.0);
    }

    /// Get current sample index
    pub fn last_sample(&self) -> u32 {
        self.last_sample
    }

    /// Set scale factor
    pub fn set_scale_factor(&mut self, factor: f32) {
        self.scale_factor = factor;
    }
}

impl Default for OpenGlTileSampler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sampler = OpenGlTileSampler::new();
        assert_eq!(sampler.tile_size(), Vec2i::new(16, 16));
        assert_eq!(sampler.view_size(), Vec2i::new(512, 512));
    }

    #[test]
    fn test_tile_size() {
        let sampler = OpenGlTileSampler::new();
        let size = sampler.tile_size();
        assert_eq!(size.x, 16);
        assert_eq!(size.y, 16);
    }

    #[test]
    fn test_nb_tiles() {
        let mut sampler = OpenGlTileSampler::new();
        sampler.set_size(512, 512, 256);

        let tiles_x = sampler.nb_tiles_x();
        let tiles_y = sampler.nb_tiles_y();
        assert!(tiles_x > 0);
        assert!(tiles_y > 0);
        assert_eq!(sampler.nb_tiles(), tiles_x * tiles_y);
    }

    #[test]
    fn test_reset() {
        let mut sampler = OpenGlTileSampler::new();
        sampler.last_sample = 100;
        sampler.reset();
        assert_eq!(sampler.last_sample(), 0);
    }

    #[test]
    fn test_scale_factor() {
        let mut sampler = OpenGlTileSampler::new();
        assert_eq!(sampler.variance_scale_factor(), 1.0);

        sampler.set_scale_factor(2.5);
        assert_eq!(sampler.variance_scale_factor(), 2.5);
    }

    #[test]
    fn test_set_size() {
        let mut sampler = OpenGlTileSampler::new();
        sampler.set_size(1920, 1080, 256);

        assert_eq!(sampler.view_size(), Vec2i::new(1920, 1080));
        // TileSize() returns a Vec2i in OCCT; check both components
        assert!(sampler.tile_size().x > 0);
        assert!(sampler.tile_size().y > 0);
    }

    #[test]
    fn test_vec2i_operations() {
        let v = Vec2i::new(10, 20);
        let scaled = v.mul_scalar(2);
        assert_eq!(scaled.x, 20);
        assert_eq!(scaled.y, 40);

        let v2 = Vec2i::new(5, 25);
        let min = v.min_elem(v2);
        assert_eq!(min.x, 5);
        assert_eq!(min.y, 20);

        let max = v.max_elem(v2);
        assert_eq!(max.x, 10);
        assert_eq!(max.y, 25);
    }

    #[test]
    fn test_upload_samples() {
        let mut sampler = OpenGlTileSampler::new();
        assert!(sampler.upload_samples(false));
        assert_eq!(sampler.last_sample(), 1);
    }

    #[test]
    fn test_max_tile_samples() {
        let sampler = OpenGlTileSampler::new();
        let max = sampler.max_tile_samples();
        assert!(max >= 0);
    }
}
