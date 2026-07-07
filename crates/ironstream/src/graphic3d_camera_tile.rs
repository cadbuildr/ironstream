// FILE: graphic3d_camera_tile.rs
// occt: Graphic3d_CameraTile

/// Class defines the area (Tile) inside a view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graphic3dCameraTile {
    /// Total size of the View area, in pixels
    pub total_size_x: i32,
    pub total_size_y: i32,
    /// Size of the Tile, in pixels
    pub tile_size_x: i32,
    pub tile_size_y: i32,
    /// The lower-left corner of the Tile relative to the View area (or upper-left if is_top_down is true), in pixels
    pub offset_x: i32,
    pub offset_y: i32,
    /// Indicate the offset coordinate system - lower-left (default) or top-down
    pub is_top_down: bool,
}

impl Default for Graphic3dCameraTile {
    /// Default constructor.
    /// Initializes the empty Tile of zero size and lower-left offset orientation.
    /// Such Tile is considered uninitialized (invalid).
    fn default() -> Self {
        Self {
            total_size_x: 0,
            total_size_y: 0,
            tile_size_x: 0,
            tile_size_y: 0,
            offset_x: 0,
            offset_y: 0,
            is_top_down: false,
        }
    }
}

impl Graphic3dCameraTile {
    /// Return true if Tile has been defined.
    pub fn is_valid(&self) -> bool {
        self.total_size_x > 0
            && self.total_size_y > 0
            && self.tile_size_x > 0
            && self.tile_size_y > 0
    }

    /// Return offset position from lower-left corner.
    pub fn offset_lower_left(&self) -> (i32, i32) {
        let y = if !self.is_top_down {
            self.offset_y
        } else {
            self.total_size_y - self.offset_y - 1
        };
        (self.offset_x, y)
    }

    /// Return the copy cropped by total size.
    pub fn cropped(&self) -> Graphic3dCameraTile {
        let mut tile = *self;
        if !self.is_valid() {
            return tile;
        }

        tile.offset_x = tile.offset_x.max(0);
        tile.offset_y = tile.offset_y.max(0);

        let max_x = (tile.offset_x + tile.tile_size_x).min(tile.total_size_x);
        let max_y = (tile.offset_y + tile.tile_size_y).min(tile.total_size_y);

        tile.tile_size_x = max_x - tile.offset_x;
        tile.tile_size_y = max_y - tile.offset_y;

        tile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_invalid() {
        let tile = Graphic3dCameraTile::default();
        assert!(!tile.is_valid());
    }

    #[test]
    fn test_valid_tile() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: 0,
            offset_y: 0,
            is_top_down: false,
        };
        assert!(tile.is_valid());
    }

    #[test]
    fn test_offset_lower_left_default() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: 100,
            offset_y: 50,
            is_top_down: false,
        };
        let (x, y) = tile.offset_lower_left();
        assert_eq!(x, 100);
        assert_eq!(y, 50);
    }

    #[test]
    fn test_offset_lower_left_top_down() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: 100,
            offset_y: 50,
            is_top_down: true,
        };
        let (x, y) = tile.offset_lower_left();
        assert_eq!(x, 100);
        assert_eq!(y, 549); // 600 - 50 - 1
    }

    #[test]
    fn test_cropped_valid() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: 200,
            offset_y: 100,
            is_top_down: false,
        };
        let cropped = tile.cropped();
        assert_eq!(cropped.tile_size_x, 400);
        assert_eq!(cropped.tile_size_y, 300);
    }

    #[test]
    fn test_cropped_clips_at_boundary() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: 600,
            offset_y: 400,
            is_top_down: false,
        };
        let cropped = tile.cropped();
        assert_eq!(cropped.offset_x, 600);
        assert_eq!(cropped.offset_y, 400);
        assert_eq!(cropped.tile_size_x, 200); // 800 - 600
        assert_eq!(cropped.tile_size_y, 200); // 600 - 400
    }

    #[test]
    fn test_cropped_negative_offset_clamped() {
        let tile = Graphic3dCameraTile {
            total_size_x: 800,
            total_size_y: 600,
            tile_size_x: 400,
            tile_size_y: 300,
            offset_x: -100,
            offset_y: -50,
            is_top_down: false,
        };
        let cropped = tile.cropped();
        assert_eq!(cropped.offset_x, 0);
        assert_eq!(cropped.offset_y, 0);
    }

    #[test]
    fn test_cropped_invalid_tile() {
        let tile = Graphic3dCameraTile::default();
        let cropped = tile.cropped();
        assert_eq!(cropped, tile);
    }
}
