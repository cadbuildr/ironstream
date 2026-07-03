// FILE: graphic3d_cube_map_packed.rs
// occt: Graphic3d_CubeMapPacked

use crate::graphic3d_cube_map_order::Graphic3dCubeMapOrder;

/// Class is intended to process cubemap packed into single image plane.
pub struct Graphic3dCubeMapPacked {
    /// Order mapping tile grid to cubemap sides
    order: Graphic3dCubeMapOrder,
    /// Width of tile grid (number of tiles horizontally)
    tile_number_x: u32,
    /// Image width in pixels
    image_width: u32,
    /// Image height in pixels
    image_height: u32,
}

impl Graphic3dCubeMapPacked {
    /// Create a new packed cubemap with specified order and dimensions.
    pub fn new(
        order: Graphic3dCubeMapOrder,
        image_width: u32,
        image_height: u32,
    ) -> Option<Self> {
        // Check if order is valid
        if !order.is_valid() {
            return None;
        }

        if image_width == 0 || image_height == 0 {
            return None;
        }

        // OCCT checkImage(): only the exact 1x6, 6x1, 2x3 and 3x2 square-tile
        // arrangements are accepted.
        let tile_number_x = if image_height % image_width == 0 && image_height / image_width == 6 {
            // 1x6 vertical strip
            1
        } else if image_width % image_height == 0 && image_width / image_height == 6 {
            // 6x1 horizontal strip
            6
        } else if image_width % 2 == 0
            && image_height % 3 == 0
            && image_width / 2 == image_height / 3
        {
            // 2x3 arrangement
            2
        } else if image_width % 3 == 0
            && image_height % 2 == 0
            && image_width / 3 == image_height / 2
        {
            // 3x2 arrangement
            3
        } else {
            return None;
        };

        Some(Self {
            order,
            tile_number_x,
            image_width,
            image_height,
        })
    }

    /// Return the order mapping.
    pub fn order(&self) -> &Graphic3dCubeMapOrder {
        &self.order
    }

    /// Return the number of tiles per row.
    pub fn tile_number_x(&self) -> u32 {
        self.tile_number_x
    }

    /// Return the image width.
    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    /// Return the image height.
    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    /// Return the size of a single tile in pixels.
    pub fn tile_size(&self) -> Option<u32> {
        if self.tile_number_x == 0 {
            return None;
        }

        let tile_width = self.image_width / self.tile_number_x;
        let tile_height = self.image_height / ((6 + self.tile_number_x - 1) / self.tile_number_x);

        // Tiles should be square
        if tile_width == tile_height {
            Some(tile_width)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_order() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order, 600, 100);
        assert!(packed.is_some());
    }

    #[test]
    fn test_new_with_invalid_order() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 1);
        order.set(1, 1);
        let packed = Graphic3dCubeMapPacked::new(order, 600, 100);
        assert!(packed.is_none());
    }

    #[test]
    fn test_tile_number_x() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order, 600, 100).unwrap();
        assert_eq!(packed.tile_number_x(), 6);
    }

    #[test]
    fn test_image_dimensions() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order, 1200, 200).unwrap();
        assert_eq!(packed.image_width(), 1200);
        assert_eq!(packed.image_height(), 200);
    }

    #[test]
    fn test_tile_size_square_arrangement() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order, 600, 100).unwrap();
        let tile_size = packed.tile_size();
        assert_eq!(tile_size, Some(100));
    }

    #[test]
    fn test_invalid_dimensions() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order, 500, 100);
        assert!(packed.is_none());
    }

    #[test]
    fn test_order_access() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let packed = Graphic3dCubeMapPacked::new(order.clone(), 600, 100).unwrap();
        assert_eq!(packed.order().get(0), Some(0));
        assert_eq!(packed.order().get(1), Some(1));
    }
}
