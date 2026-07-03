// FILE: graphic3d_cube_map_order.rs
// occt: Graphic3d_CubeMapOrder Graphic3d_ValidatedCubeMapOrder

use std::collections::HashSet;

/// Graphic3d_CubeMapOrder maps sides of cubemap on tiles in packed cubemap image
/// to support different tiles order in such images.
/// Also it can be considered as permutation of numbers from 0 to 5.
/// It stores permutation in one integer as convolution.
#[derive(Clone, Debug)]
pub struct Graphic3dCubeMapOrder {
    /// Array storing the position (0-5) for each cubemap side (0=PosX, 1=NegX, 2=PosY, 3=NegY, 4=PosZ, 5=NegZ)
    positions: [u8; 6],
    /// Indicates if there are attempts to assign index greater than 5
    has_overflows: bool,
}

impl Graphic3dCubeMapOrder {
    /// Create a new empty order with zero convolution.
    pub fn new() -> Self {
        Self {
            positions: [0; 6],
            has_overflows: false,
        }
    }

    /// Initialize order with values for each of the 6 cubemap sides.
    pub fn new_with_order(
        pos_x: u8,
        neg_x: u8,
        pos_y: u8,
        neg_y: u8,
        pos_z: u8,
        neg_z: u8,
    ) -> Self {
        let mut order = Self::new();
        order.set(0, pos_x);
        order.set(1, neg_x);
        order.set(2, pos_y);
        order.set(3, neg_y);
        order.set(4, pos_z);
        order.set(5, neg_z);
        order
    }

    /// Set default order (just from 0 to 5)
    pub fn set_default(&mut self) {
        self.positions = [0, 1, 2, 3, 4, 5];
        self.has_overflows = false;
    }

    /// Sets number of tile in packed cubemap image according passed cubemap side.
    /// side: 0=PosX, 1=NegX, 2=PosY, 3=NegY, 4=PosZ, 5=NegZ
    pub fn set(&mut self, side: u8, value: u8) {
        if side <= 5 {
            // OCCT: values greater than 5 only raise the overflow flag
            // and are not stored.
            if value > 5 {
                self.has_overflows = true;
            } else {
                self.positions[side as usize] = value;
            }
        }
    }

    /// Returns value of passed cubemap side.
    pub fn get(&self, side: u8) -> Option<u8> {
        if side <= 5 {
            Some(self.positions[side as usize])
        } else {
            None
        }
    }

    /// Makes order empty.
    pub fn clear(&mut self) {
        self.positions = [0; 6];
        self.has_overflows = false;
    }

    /// Checks whether order is empty.
    /// OCCT: IsEmpty() only checks the convolution (all stored positions zero).
    pub fn is_empty(&self) -> bool {
        self.positions.iter().all(|&v| v == 0)
    }

    /// Checks whether order has repetitions.
    pub fn has_repetitions(&self) -> bool {
        let mut seen = HashSet::new();
        for &pos in &self.positions {
            if pos <= 5 && !seen.insert(pos) {
                return true;
            }
        }
        false
    }

    /// Checks whether attempts to assign index greater than 5 to any side happened.
    pub fn has_overflows(&self) -> bool {
        self.has_overflows
    }

    /// Checks whether order is valid.
    /// Order is valid when it doesn't have repetitions and there were not attempts to assign
    /// indexes greater than 5.
    pub fn is_valid(&self) -> bool {
        !self.has_repetitions() && !self.has_overflows()
    }

    /// Returns default order.
    /// It is guaranteed to be valid.
    pub fn default_order() -> Self {
        let mut order = Self::new();
        order.set_default();
        order
    }

    /// Returns a validated version of this order if it is valid.
    pub fn validated(&self) -> Option<Graphic3dValidatedCubeMapOrder> {
        if self.is_valid() {
            Some(Graphic3dValidatedCubeMapOrder {
                order: self.clone(),
            })
        } else {
            None
        }
    }
}

impl Default for Graphic3dCubeMapOrder {
    fn default() -> Self {
        Self::new()
    }
}

/// Graphic3d_ValidatedCubeMapOrder contains completely valid order object.
/// The only way to create this class except copy constructor is 'Validated' method of
/// Graphic3d_CubeMapOrder. This class can initialize Graphic3d_CubeMapOrder.
#[derive(Clone, Debug)]
pub struct Graphic3dValidatedCubeMapOrder {
    /// Completely valid order
    order: Graphic3dCubeMapOrder,
}

impl Graphic3dValidatedCubeMapOrder {
    /// Access the underlying valid order.
    pub fn order(&self) -> &Graphic3dCubeMapOrder {
        &self.order
    }

    /// Return the default validated order.
    pub fn default() -> Self {
        Graphic3dValidatedCubeMapOrder {
            order: Graphic3dCubeMapOrder::default_order(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order() {
        let order = Graphic3dCubeMapOrder::new();
        assert!(order.is_empty());
        assert!(!order.has_overflows());
        // OCCT: an empty order maps every side to tile 0 (zero convolution),
        // so HasRepetitions() reports true.
        assert!(order.has_repetitions());
    }

    #[test]
    fn test_new_with_order() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        assert_eq!(order.get(0), Some(0));
        assert_eq!(order.get(1), Some(1));
        assert_eq!(order.get(5), Some(5));
        assert!(order.is_valid());
    }

    #[test]
    fn test_set_and_get() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 2);
        assert_eq!(order.get(0), Some(2));
    }

    #[test]
    fn test_overflow_detection() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 10);
        assert!(order.has_overflows());
        assert!(!order.is_valid());
    }

    #[test]
    fn test_repetition_detection() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 1);
        order.set(1, 1);
        assert!(order.has_repetitions());
        assert!(!order.is_valid());
    }

    #[test]
    fn test_is_valid_order() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        assert!(order.is_valid());
    }

    #[test]
    fn test_default_order() {
        let order = Graphic3dCubeMapOrder::default_order();
        assert!(order.is_valid());
        assert_eq!(order.get(0), Some(0));
        assert_eq!(order.get(1), Some(1));
    }

    #[test]
    fn test_validated() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let validated = order.validated();
        assert!(validated.is_some());
    }

    #[test]
    fn test_validated_invalid() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 1);
        order.set(1, 1);
        let validated = order.validated();
        assert!(validated.is_none());
    }

    #[test]
    fn test_clear() {
        let mut order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        assert!(!order.is_empty());
        order.clear();
        assert!(order.is_empty());
    }

    #[test]
    fn test_set_default() {
        let mut order = Graphic3dCubeMapOrder::new();
        order.set(0, 5);
        order.set(1, 4);
        order.set_default();
        assert_eq!(order.get(0), Some(0));
        assert_eq!(order.get(1), Some(1));
        assert!(order.is_valid());
    }

    #[test]
    fn test_validated_cubemap_order_default() {
        let validated = Graphic3dValidatedCubeMapOrder::default();
        assert!(validated.order().is_valid());
    }

    #[test]
    fn test_validated_cubemap_order_access() {
        let order = Graphic3dCubeMapOrder::new_with_order(0, 1, 2, 3, 4, 5);
        let validated = order.validated().unwrap();
        assert_eq!(validated.order().get(0), Some(0));
    }
}
