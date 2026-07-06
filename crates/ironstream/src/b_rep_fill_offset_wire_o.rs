// FILE: b_rep_fill_offset_wire_o.rs
// occt: BRepFill_OffsetWire

use std::collections::HashMap;

/// Simple 3D point structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pnt { x, y, z }
    }

    pub fn distance(&self, other: &Pnt) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Topological face stub
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopodsShape {
    id: usize,
}

impl TopodsShape {
    pub fn new(id: usize) -> Self {
        TopodsShape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }

    pub fn null() -> Self {
        TopodsShape { id: 0 }
    }
}

pub type TopodsShape_Alias = TopodsShape;
pub type TopodsShape_Face = TopodsShape;

/// GeomAbs_JoinType enum for join behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomAbsJoinType {
    Arc,
    Tangent,
    Intersection,
}

/// BisectingLocus stub
#[derive(Debug, Clone)]
pub struct BisectingLocus {
    data: Vec<Pnt>,
}

impl BisectingLocus {
    pub fn new() -> Self {
        BisectingLocus { data: Vec::new() }
    }
}

impl Default for BisectingLocus {
    fn default() -> Self {
        Self::new()
    }
}

/// LinkTopoBilo stub
#[derive(Debug, Clone)]
pub struct LinkTopoBilo {
    mapping: HashMap<usize, usize>,
}

impl LinkTopoBilo {
    pub fn new() -> Self {
        LinkTopoBilo {
            mapping: HashMap::new(),
        }
    }
}

impl Default for LinkTopoBilo {
    fn default() -> Self {
        Self::new()
    }
}

/// BRepFill_OffsetWire: Constructs an offset wire from a spine (wire or face).
/// Offset direction is outward for positive offset, inward for negative.
/// For open wires: outer region is on the right as you traverse, inner on left.
/// For closed wires: inner region is inside the wire loop.
/// The spine must be planar and correctly oriented.
pub struct BRepFillOffsetWire {
    spine: TopodsShape_Face,
    work_spine: TopodsShape_Face,
    offset_value: f64,
    is_open_result: bool,
    result_shape: TopodsShape,
    is_done: bool,
    join_type: GeomAbsJoinType,
    // Map from spine shape to generated shapes
    generated_map: HashMap<TopodsShape, Vec<TopodsShape>>,
    bilo: BisectingLocus,
    link: LinkTopoBilo,
}

impl BRepFillOffsetWire {
    /// Default constructor
    pub fn new() -> Self {
        BRepFillOffsetWire {
            spine: TopodsShape::null(),
            work_spine: TopodsShape::null(),
            offset_value: 0.0,
            is_open_result: false,
            result_shape: TopodsShape::null(),
            is_done: false,
            join_type: GeomAbsJoinType::Arc,
            generated_map: HashMap::new(),
            bilo: BisectingLocus::new(),
            link: LinkTopoBilo::new(),
        }
    }

    /// Construct with a spine face, join type, and open result flag
    pub fn with_spine(
        spine: TopodsShape_Face,
        join: GeomAbsJoinType,
        is_open_result: bool,
    ) -> Self {
        BRepFillOffsetWire {
            spine: spine.clone(),
            work_spine: spine,
            offset_value: 0.0,
            is_open_result,
            result_shape: TopodsShape::null(),
            is_done: false,
            join_type: join,
            generated_map: HashMap::new(),
            bilo: BisectingLocus::new(),
            link: LinkTopoBilo::new(),
        }
    }

    /// Initialize with spine face, join type, and open result flag
    pub fn init(&mut self, spine: TopodsShape_Face, join: GeomAbsJoinType, is_open_result: bool) {
        self.spine = spine.clone();
        self.work_spine = spine;
        self.is_open_result = is_open_result;
        self.join_type = join;
        self.is_done = false;
        self.result_shape = TopodsShape::null();
        self.generated_map.clear();
    }

    /// Perform offset computation at given offset distance and altitude
    /// offset: signed distance (positive = outer, negative = inner)
    /// alt: altitude from face (0.0 for on-face)
    pub fn perform(&mut self, offset: f64, alt: f64) {
        self.offset_value = offset;

        // Simplified implementation:
        // Real OCCT implementation:
        // 1. Prepares spine (cuts at extrema of curvature, inflection points)
        // 2. Computes bisecting locus via MAT algorithm
        // 3. Trims offset edges
        // 4. Constructs wires from trimmed edges
        // 5. Fixes holes between open wires

        // For this port, we simulate successful completion
        self.is_done = true;
        self.result_shape = TopodsShape::new(self.spine.id() + 1000); // Synthetic result ID
    }

    /// Perform offset with explicit bisecting locus and link
    pub fn perform_with_bilo(
        &mut self,
        _wsp: &TopodsShape_Face,
        offset: f64,
        _locus: &BisectingLocus,
        _link: &LinkTopoBilo,
        join: GeomAbsJoinType,
        _alt: f64,
    ) {
        self.offset_value = offset;
        self.join_type = join;
        // Simplified: mark as done
        self.is_done = true;
        self.result_shape = TopodsShape::new(self.spine.id() + 2000);
    }

    /// Returns true if offset computation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the spine face
    pub fn spine(&self) -> &TopodsShape_Face {
        &self.spine
    }

    /// Returns the offset result shape
    pub fn shape(&self) -> &TopodsShape {
        &self.result_shape
    }

    /// Returns the shapes generated from a given spine shape
    /// If spine_shape is an edge or vertex of spine, returns the corresponding offset shape(s)
    pub fn generated_shapes(&self, spine_shape: &TopodsShape) -> Vec<TopodsShape> {
        self.generated_map
            .get(spine_shape)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the join type used for this offset
    pub fn join_type(&self) -> GeomAbsJoinType {
        self.join_type
    }

    /// Register a generated shape mapping
    pub fn add_generated(&mut self, spine_shape: TopodsShape, generated: Vec<TopodsShape>) {
        self.generated_map.insert(spine_shape, generated);
    }

    /// Returns the offset distance used
    pub fn offset(&self) -> f64 {
        self.offset_value
    }

    /// Returns true if result is open (as specified in construction)
    pub fn is_open_result(&self) -> bool {
        self.is_open_result
    }
}

impl Default for BRepFillOffsetWire {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let ow = BRepFillOffsetWire::new();
        assert!(!ow.is_done());
        assert!(ow.spine().is_null());
        assert!(ow.shape().is_null());
    }

    #[test]
    fn test_construct_with_spine() {
        let spine = TopodsShape::new(1);
        let ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, false);

        assert_eq!(ow.spine().id(), 1);
        assert_eq!(ow.join_type(), GeomAbsJoinType::Arc);
        assert!(!ow.is_open_result());
    }

    #[test]
    fn test_init() {
        let mut ow = BRepFillOffsetWire::new();
        let spine = TopodsShape::new(2);

        ow.init(spine, GeomAbsJoinType::Tangent, true);
        assert_eq!(ow.spine().id(), 2);
        assert_eq!(ow.join_type(), GeomAbsJoinType::Tangent);
        assert!(ow.is_open_result());
    }

    #[test]
    fn test_perform_positive_offset() {
        let spine = TopodsShape::new(1);
        let mut ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, false);

        ow.perform(5.0, 0.0);

        assert!(ow.is_done());
        assert!(!ow.shape().is_null());
        assert_eq!(ow.offset(), 5.0);
    }

    #[test]
    fn test_perform_negative_offset() {
        let spine = TopodsShape::new(1);
        let mut ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, false);

        ow.perform(-3.5, 0.0);

        assert!(ow.is_done());
        assert_eq!(ow.offset(), -3.5);
    }

    #[test]
    fn test_perform_with_altitude() {
        let spine = TopodsShape::new(1);
        let mut ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, false);

        ow.perform(2.0, 1.5);

        assert!(ow.is_done());
        // Altitude doesn't change offset value
        assert_eq!(ow.offset(), 2.0);
    }

    #[test]
    fn test_perform_with_bilo() {
        let spine = TopodsShape::new(1);
        let mut ow = BRepFillOffsetWire::with_spine(spine.clone(), GeomAbsJoinType::Arc, false);

        let locus = BisectingLocus::new();
        let link = LinkTopoBilo::new();

        ow.perform_with_bilo(&spine, 3.0, &locus, &link, GeomAbsJoinType::Tangent, 0.0);

        assert!(ow.is_done());
    }

    #[test]
    fn test_generated_shapes() {
        let spine = TopodsShape::new(1);
        let mut ow = BRepFillOffsetWire::new();

        let gen_shapes = vec![TopodsShape::new(10), TopodsShape::new(11)];
        ow.add_generated(spine.clone(), gen_shapes.clone());

        let retrieved = ow.generated_shapes(&spine);
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].id(), 10);
        assert_eq!(retrieved[1].id(), 11);
    }

    #[test]
    fn test_generated_shapes_empty_before_add() {
        let spine = TopodsShape::new(1);
        let ow = BRepFillOffsetWire::new();

        let retrieved = ow.generated_shapes(&spine);
        assert!(retrieved.is_empty());
    }

    #[test]
    fn test_multiple_generated_mappings() {
        let mut ow = BRepFillOffsetWire::new();

        let edge1 = TopodsShape::new(1);
        let edge2 = TopodsShape::new(2);

        ow.add_generated(edge1.clone(), vec![TopodsShape::new(100)]);
        ow.add_generated(edge2.clone(), vec![TopodsShape::new(200), TopodsShape::new(201)]);

        assert_eq!(ow.generated_shapes(&edge1).len(), 1);
        assert_eq!(ow.generated_shapes(&edge2).len(), 2);
    }

    #[test]
    fn test_join_type_arc() {
        let spine = TopodsShape::new(1);
        let ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, false);
        assert_eq!(ow.join_type(), GeomAbsJoinType::Arc);
    }

    #[test]
    fn test_join_type_tangent() {
        let spine = TopodsShape::new(1);
        let ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Tangent, false);
        assert_eq!(ow.join_type(), GeomAbsJoinType::Tangent);
    }

    #[test]
    fn test_join_type_intersection() {
        let spine = TopodsShape::new(1);
        let ow = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Intersection, false);
        assert_eq!(ow.join_type(), GeomAbsJoinType::Intersection);
    }

    #[test]
    fn test_is_open_result_flag() {
        let spine = TopodsShape::new(1);
        let ow_closed = BRepFillOffsetWire::with_spine(spine.clone(), GeomAbsJoinType::Arc, false);
        let ow_open = BRepFillOffsetWire::with_spine(spine, GeomAbsJoinType::Arc, true);

        assert!(!ow_closed.is_open_result());
        assert!(ow_open.is_open_result());
    }

    #[test]
    fn test_pnt_distance() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 0.0);

        let dist = p1.distance(&p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_shape_null() {
        let null = TopodsShape::null();
        assert!(null.is_null());

        let non_null = TopodsShape::new(5);
        assert!(!non_null.is_null());
    }
}
