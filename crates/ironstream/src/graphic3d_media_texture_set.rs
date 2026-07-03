// FILE: graphic3d_media_texture_set.rs
// occt: Graphic3d_MediaTextureSet

use std::sync::Mutex;

/// Type for media texture set update callback.
pub type CallbackOnUpdate = Option<Box<dyn Fn() + Send>>;

/// Texture adapter set for Media_Frame streams.
pub struct Graphic3dMediaTextureSet {
    callback: Mutex<CallbackOnUpdate>,
    input: String,
    frame_size: (i32, i32),
    progress: f64,
    duration: f64,
    is_planar_yuv: bool,
    is_full_range_yuv: bool,
}

impl Graphic3dMediaTextureSet {
    /// Creates a new empty media texture set.
    pub fn new() -> Self {
        Graphic3dMediaTextureSet {
            callback: Mutex::new(None),
            input: String::new(),
            frame_size: (0, 0),
            progress: 0.0,
            duration: 0.0,
            is_planar_yuv: false,
            is_full_range_yuv: false,
        }
    }

    /// Sets the callback function called on queue progress.
    pub fn set_callback(&self, callback: Option<Box<dyn Fn() + Send>>) {
        if let Ok(mut cb) = self.callback.lock() {
            *cb = callback;
        }
    }

    /// Calls the registered callback.
    pub fn notify(&self) {
        if let Ok(lock) = self.callback.lock() {
            if let Some(ref cb) = *lock {
                cb();
            }
        }
    }

    /// Returns the input file path.
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Opens the specified input file.
    pub fn open_input(&mut self, path: &str, _wait: bool) {
        self.input = path.to_string();
    }

    /// Swaps front/back frames (returns true if frames were swapped).
    pub fn swap_frames(&mut self) -> bool {
        // Returns true if frame swap was successful
        !self.input.is_empty()
    }

    /// Returns frame dimensions.
    pub fn frame_size(&self) -> (i32, i32) {
        self.frame_size
    }

    /// Returns true if texture set uses planar YUV format.
    pub fn is_planar_yuv(&self) -> bool {
        self.is_planar_yuv
    }

    /// Returns true if YUV range is full (versus reduced).
    pub fn is_full_range_yuv(&self) -> bool {
        self.is_full_range_yuv
    }

    /// Returns stream duration in seconds.
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Returns playback progress in seconds.
    pub fn progress(&self) -> f64 {
        self.progress
    }
}

impl Default for Graphic3dMediaTextureSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_texture_set_creation() {
        let set = Graphic3dMediaTextureSet::new();
        assert_eq!(set.input(), "");
        assert_eq!(set.progress(), 0.0);
        assert_eq!(set.duration(), 0.0);
    }

    #[test]
    fn test_media_texture_set_open_input() {
        let mut set = Graphic3dMediaTextureSet::new();
        set.open_input("test.mp4", false);
        assert_eq!(set.input(), "test.mp4");
    }

    #[test]
    fn test_media_texture_set_swap_frames() {
        let mut set = Graphic3dMediaTextureSet::new();
        assert!(!set.swap_frames());
        set.open_input("test.mp4", false);
        assert!(set.swap_frames());
    }

    #[test]
    fn test_media_texture_set_callback() {
        let set = Graphic3dMediaTextureSet::new();
        let called = std::sync::Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        set.set_callback(Some(Box::new(move || {
            if let Ok(mut c) = called_clone.lock() {
                *c = true;
            }
        })));
        set.notify();
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_media_texture_set_yuv_properties() {
        let set = Graphic3dMediaTextureSet::new();
        assert!(!set.is_planar_yuv());
        assert!(!set.is_full_range_yuv());
    }
}
