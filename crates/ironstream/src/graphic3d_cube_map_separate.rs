// FILE: graphic3d_cube_map_separate.rs
// occt: Graphic3d_CubeMapSeparate

use std::path::PathBuf;

/// Image format enumeration (simplified for this implementation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Unknown = 0,
    RGBA = 1,
    RGB = 2,
}

/// Image pixmap handle (simplified representation)
#[derive(Debug, Clone)]
pub struct PixMap {
    width: usize,
    height: usize,
    format: ImageFormat,
}

impl PixMap {
    pub fn new(width: usize, height: usize, format: ImageFormat) -> Self {
        PixMap {
            width,
            height,
            format,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }
}

/// Class to manage cubemap located in six different images.
#[derive(Debug, Clone)]
pub struct CubeMapSeparate {
    /// array of paths to cubemap images (6 sides)
    paths: [Option<PathBuf>; 6],
    /// array of cubemap images (6 sides)
    images: [Option<PixMap>; 6],
    /// size of each side of cubemap
    size: usize,
    /// format of each side of cubemap
    format: ImageFormat,
    /// current side iterator
    current_side: usize,
}

impl CubeMapSeparate {
    /// Initializes cubemap to be loaded from file paths.
    /// paths - array of paths to separate image files (must have exactly 6 elements)
    pub fn from_paths(paths: [PathBuf; 6]) -> Result<Self, String> {
        Ok(CubeMapSeparate {
            paths: [
                Some(paths[0].clone()),
                Some(paths[1].clone()),
                Some(paths[2].clone()),
                Some(paths[3].clone()),
                Some(paths[4].clone()),
                Some(paths[5].clone()),
            ],
            images: [None, None, None, None, None, None],
            size: 0,
            format: ImageFormat::Unknown,
            current_side: 0,
        })
    }

    /// Initializes cubemap to be set directly from PixMaps.
    /// images - array of PixMaps (must have exactly 6 elements)
    pub fn from_images(images: [Option<PixMap>; 6]) -> Result<Self, String> {
        // Check that first image is not null
        let first_image = images[0]
            .as_ref()
            .ok_or_else(|| "First image is null".to_string())?;

        // Check that first image is square
        if first_image.width() != first_image.height() {
            return Err("Images must be square".to_string());
        }

        let size = first_image.width();
        let format = first_image.format();

        // Validate all remaining images
        for i in 1..6 {
            if let Some(img) = &images[i] {
                if img.width() != size || img.height() != size {
                    return Err(format!(
                        "All images must have same size: expected {}x{}, got {}x{}",
                        size, size, img.width(), img.height()
                    ));
                }
                if img.format() != format {
                    return Err("All images must have same format".to_string());
                }
            } else {
                return Err(format!("Image {} is null", i));
            }
        }

        Ok(CubeMapSeparate {
            paths: [None, None, None, None, None, None],
            images,
            size,
            format,
            current_side: 0,
        })
    }

    /// Returns current side of cubemap as PixMap.
    /// Returns None if current side or whole cubemap is invalid.
    pub fn value(&self) -> Option<&PixMap> {
        if self.current_side >= 6 {
            return None;
        }

        // Return cached image if available
        if let Some(ref img) = self.images[self.current_side] {
            return Some(img);
        }

        // Would load from file path here in a real implementation
        None
    }

    /// Checks if the cubemap is valid or not.
    /// Returns true if the construction is correct.
    pub fn is_done(&self) -> bool {
        // If any image is loaded, cubemap is valid
        if !self.images[0].is_none() {
            return true;
        }

        // Otherwise, check if all paths exist
        for path_opt in &self.paths {
            if let Some(path) = path_opt {
                // In a real implementation, we would check file existence with OSD_File
                // For now, we just check that path is not empty
                if path.as_os_str().len() == 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Resets images array to all null handles
    fn reset_images(&mut self) {
        self.images = [None, None, None, None, None, None];
    }

    /// Returns current cubemap side index (iterator state)
    pub fn current_side(&self) -> usize {
        self.current_side
    }

    /// Moves iterator to next cubemap side
    pub fn next(&mut self) {
        if self.current_side < 5 {
            self.current_side += 1;
        }
    }

    /// Returns size of each side
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns format of each side
    pub fn format(&self) -> ImageFormat {
        self.format
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_map_separate_from_images() {
        let images = [
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let cubemap = CubeMapSeparate::from_images(images).expect("Failed to create cubemap");
        assert!(cubemap.is_done());
        assert_eq!(cubemap.size(), 256);
        assert_eq!(cubemap.format(), ImageFormat::RGBA);
        assert_eq!(cubemap.current_side(), 0);
        assert!(cubemap.value().is_some());
    }

    #[test]
    fn test_cube_map_separate_non_square_images() {
        let images = [
            Some(PixMap::new(256, 512, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let result = CubeMapSeparate::from_images(images);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Images must be square");
    }

    #[test]
    fn test_cube_map_separate_mismatched_sizes() {
        let images = [
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(512, 512, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let result = CubeMapSeparate::from_images(images);
        assert!(result.is_err());
    }

    #[test]
    fn test_cube_map_separate_mismatched_formats() {
        let images = [
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGB)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let result = CubeMapSeparate::from_images(images);
        assert!(result.is_err());
    }

    #[test]
    fn test_cube_map_separate_null_image() {
        let images = [
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            None,
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let result = CubeMapSeparate::from_images(images);
        assert!(result.is_err());
    }

    #[test]
    fn test_cube_map_separate_iterator() {
        let images = [
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
            Some(PixMap::new(256, 256, ImageFormat::RGBA)),
        ];

        let mut cubemap = CubeMapSeparate::from_images(images).expect("Failed to create cubemap");

        // Test iterator traversal
        assert_eq!(cubemap.current_side(), 0);
        cubemap.next();
        assert_eq!(cubemap.current_side(), 1);
        cubemap.next();
        assert_eq!(cubemap.current_side(), 2);

        // Jump to last
        for _ in 0..3 {
            cubemap.next();
        }
        assert_eq!(cubemap.current_side(), 5);

        // Next at end doesn't go further
        cubemap.next();
        assert_eq!(cubemap.current_side(), 5);
    }

    #[test]
    fn test_cube_map_separate_from_paths() {
        let paths = [
            PathBuf::from("pos_x.png"),
            PathBuf::from("neg_x.png"),
            PathBuf::from("pos_y.png"),
            PathBuf::from("neg_y.png"),
            PathBuf::from("pos_z.png"),
            PathBuf::from("neg_z.png"),
        ];

        let cubemap = CubeMapSeparate::from_paths(paths).expect("Failed to create cubemap");
        assert!(cubemap.is_done());
        assert_eq!(cubemap.current_side(), 0);
    }

    #[test]
    fn test_pixmap_creation() {
        let pixmap = PixMap::new(512, 512, ImageFormat::RGB);
        assert_eq!(pixmap.width(), 512);
        assert_eq!(pixmap.height(), 512);
        assert_eq!(pixmap.format(), ImageFormat::RGB);
    }
}
