// FILE: bopds_vector_of_shape_info.rs
// occt: BOPDS_VectorOfShapeInfo

use std::collections::VecDeque;

/// Enumeration for shape type, modeled after TopAbs_ShapeEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape, // default/unknown
}

/// Bounding box representation for shape queries.
#[derive(Debug, Clone, Copy, PartialEq)]
struct BoundingBox {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    is_void: bool,
}

impl BoundingBox {
    fn new() -> Self {
        BoundingBox {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            z_min: 0.0,
            z_max: 0.0,
            is_void: true,
        }
    }
}

/// Stores handy information about a shape in Boolean operations.
/// Mirrors BOPDS_ShapeInfo from OCCT.
#[derive(Debug, Clone)]
struct ShapeInfo {
    shape_id: usize,                   // Reference to actual shape data
    shape_type: ShapeType,             // Type of the shape
    bounding_box: BoundingBox,         // Bounding box of the shape
    sub_shapes: Vec<i32>,              // Indices of sub-shapes
    reference: i32,                    // Reference index (-1 if none)
    flag: i32,                         // Flag value (-1 if no flag set)
}

impl ShapeInfo {
    /// Creates a new ShapeInfo with default values.
    fn new() -> Self {
        ShapeInfo {
            shape_id: 0,
            shape_type: ShapeType::Shape,
            bounding_box: BoundingBox::new(),
            sub_shapes: Vec::new(),
            reference: -1,
            flag: -1,
        }
    }

    /// Sets the shape ID.
    fn set_shape(&mut self, shape_id: usize) {
        self.shape_id = shape_id;
    }

    /// Returns the shape ID.
    fn shape(&self) -> usize {
        self.shape_id
    }

    /// Sets the shape type.
    fn set_shape_type(&mut self, shape_type: ShapeType) {
        self.shape_type = shape_type;
    }

    /// Returns the shape type.
    fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    /// Sets the bounding box.
    fn set_box(&mut self, bb: BoundingBox) {
        self.bounding_box = bb;
    }

    /// Returns a reference to the bounding box.
    fn bnd_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    /// Returns a mutable reference to the bounding box.
    fn change_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    /// Returns the list of sub-shape indices.
    fn sub_shapes(&self) -> &[i32] {
        &self.sub_shapes
    }

    /// Returns a mutable reference to sub-shapes.
    fn change_sub_shapes(&mut self) -> &mut Vec<i32> {
        &mut self.sub_shapes
    }

    /// Checks if the shape has a sub-shape with the given index.
    fn has_sub_shape(&self, index: i32) -> bool {
        self.sub_shapes.contains(&index)
    }

    /// Checks if the shape has a reference.
    fn has_reference(&self) -> bool {
        self.reference >= 0
    }

    /// Sets the reference index.
    fn set_reference(&mut self, index: i32) {
        self.reference = index;
    }

    /// Returns the reference index.
    fn reference(&self) -> i32 {
        self.reference
    }

    /// Checks if the shape has boundary representation (BRep).
    /// Mirrors BOPDS_Tools::HasBRep logic: true for Face, Edge, Vertex.
    fn has_brep(&self) -> bool {
        matches!(
            self.shape_type,
            ShapeType::Face | ShapeType::Edge | ShapeType::Vertex
        )
    }

    /// Returns true if the shape can participate in an interference.
    /// True if it has BRep or is a Solid.
    fn is_interfering(&self) -> bool {
        self.has_brep() || self.shape_type == ShapeType::Solid
    }

    /// Checks if a flag is set.
    fn has_flag(&self) -> bool {
        self.flag >= 0
    }

    /// Gets the flag value if set; returns (has_flag, flag_value).
    fn get_flag(&self) -> (bool, i32) {
        (self.flag >= 0, self.flag)
    }

    /// Sets the flag.
    fn set_flag(&mut self, flag: i32) {
        self.flag = flag;
    }

    /// Returns the flag value.
    fn flag(&self) -> i32 {
        self.flag
    }
}

/// Deprecated type alias: vector of shape info using a dynamic array.
/// This is a newtype wrapping VecDeque<ShapeInfo> to match OCCT's NCollection_DynamicArray semantics.
pub struct BopdsVectorOfShapeInfo {
    data: VecDeque<ShapeInfo>,
}

impl BopdsVectorOfShapeInfo {
    /// Creates an empty vector.
    pub fn new() -> Self {
        BopdsVectorOfShapeInfo {
            data: VecDeque::new(),
        }
    }

    /// Appends a ShapeInfo to the vector.
    pub fn push(&mut self, info: ShapeInfo) {
        self.data.push_back(info);
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses a ShapeInfo by index.
    pub fn get(&self, index: usize) -> Option<&ShapeInfo> {
        self.data.get(index)
    }

    /// Mutably accesses a ShapeInfo by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ShapeInfo> {
        self.data.get_mut(index)
    }

    /// Clears all elements.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BopdsVectorOfShapeInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_info_default() {
        let info = ShapeInfo::new();
        assert_eq!(info.shape_type(), ShapeType::Shape);
        assert_eq!(info.reference(), -1);
        assert_eq!(info.flag(), -1);
        assert!(!info.has_reference());
        assert!(!info.has_flag());
    }

    #[test]
    fn test_shape_info_setters() {
        let mut info = ShapeInfo::new();
        info.set_shape(42);
        info.set_shape_type(ShapeType::Face);
        info.set_reference(5);
        info.set_flag(3);

        assert_eq!(info.shape(), 42);
        assert_eq!(info.shape_type(), ShapeType::Face);
        assert!(info.has_reference());
        assert_eq!(info.reference(), 5);
        assert!(info.has_flag());
        assert_eq!(info.flag(), 3);
    }

    #[test]
    fn test_has_brep() {
        let mut info = ShapeInfo::new();
        info.set_shape_type(ShapeType::Vertex);
        assert!(info.has_brep());

        info.set_shape_type(ShapeType::Edge);
        assert!(info.has_brep());

        info.set_shape_type(ShapeType::Face);
        assert!(info.has_brep());

        info.set_shape_type(ShapeType::Solid);
        assert!(!info.has_brep());
    }

    #[test]
    fn test_is_interfering() {
        let mut info = ShapeInfo::new();
        info.set_shape_type(ShapeType::Face);
        assert!(info.is_interfering());

        info.set_shape_type(ShapeType::Solid);
        assert!(info.is_interfering());

        info.set_shape_type(ShapeType::Compound);
        assert!(!info.is_interfering());
    }

    #[test]
    fn test_sub_shapes() {
        let mut info = ShapeInfo::new();
        info.change_sub_shapes().push(1);
        info.change_sub_shapes().push(2);
        info.change_sub_shapes().push(3);

        assert!(info.has_sub_shape(1));
        assert!(info.has_sub_shape(2));
        assert!(!info.has_sub_shape(5));
        assert_eq!(info.sub_shapes().len(), 3);
    }

    #[test]
    fn test_vector_basic() {
        let mut vec = BopdsVectorOfShapeInfo::new();
        assert!(vec.is_empty());

        let info = ShapeInfo::new();
        vec.push(info);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_vector_multiple() {
        let mut vec = BopdsVectorOfShapeInfo::new();
        for i in 0..5 {
            let mut info = ShapeInfo::new();
            info.set_reference(i as i32);
            vec.push(info);
        }
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.get(2).unwrap().reference(), 2);
    }

    #[test]
    fn test_vector_mutate() {
        let mut vec = BopdsVectorOfShapeInfo::new();
        let info = ShapeInfo::new();
        vec.push(info);

        if let Some(inf) = vec.get_mut(0) {
            inf.set_flag(99);
        }
        assert_eq!(vec.get(0).unwrap().flag(), 99);
    }

    #[test]
    fn test_vector_clear() {
        let mut vec = BopdsVectorOfShapeInfo::new();
        vec.push(ShapeInfo::new());
        vec.push(ShapeInfo::new());
        assert_eq!(vec.len(), 2);

        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_bounding_box() {
        let mut info = ShapeInfo::new();
        let mut bb = BoundingBox::new();
        bb.x_min = 1.0;
        bb.x_max = 2.0;
        info.set_box(bb);

        assert_eq!(info.bnd_box().x_min, 1.0);
        assert_eq!(info.bnd_box().x_max, 2.0);
    }
}
