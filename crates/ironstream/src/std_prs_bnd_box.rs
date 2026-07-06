// FILE: std_prs_bnd_box.rs
// occt: StdPrs_BndBox

//! Presentation builder for bounding boxes, following StdPrs_BndBox.hxx
//! (a typedef of Prs3d_BndBox). Computes the wireframe presentation of an
//! axis-aligned bounding box: 8 vertices and 12 edges, in the exact vertex
//! and edge order of Prs3d_BndBox::fillSegments.
//!
//! External plumbing (Bnd_Box, Graphic3d_ArrayOfSegments, the presentation
//! group and its line aspect) is modelled with local helper types; the
//! segment-filling behaviour is real.

// ---------------------------------------------------------------------------
// Local plumbing
// ---------------------------------------------------------------------------

/// Models gp_Pnt.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Models Bnd_Box: an axis-aligned box which is void until updated.
#[derive(Clone, Debug, Default)]
pub struct BndBox {
    corners: Option<(Pnt, Pnt)>,
}

impl BndBox {
    /// Creates a void box.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_void(&self) -> bool {
        self.corners.is_none()
    }

    /// Bnd_Box::Update with a point: enlarges the box to contain it.
    pub fn update(&mut self, x: f64, y: f64, z: f64) {
        match &mut self.corners {
            None => self.corners = Some((Pnt::new(x, y, z), Pnt::new(x, y, z))),
            Some((min, max)) => {
                min.x = min.x.min(x);
                min.y = min.y.min(y);
                min.z = min.z.min(z);
                max.x = max.x.max(x);
                max.y = max.y.max(y);
                max.z = max.z.max(z);
            }
        }
    }

    /// Bnd_Box::CornerMin; panics on a void box like OCCT raises.
    pub fn corner_min(&self) -> Pnt {
        self.corners.expect("Bnd_Box is void").0
    }

    /// Bnd_Box::CornerMax; panics on a void box like OCCT raises.
    pub fn corner_max(&self) -> Pnt {
        self.corners.expect("Bnd_Box is void").1
    }
}

/// Models Graphic3d_ArrayOfSegments: vertices plus edge index pairs
/// (1-based indices, as in Graphic3d).
#[derive(Clone, Debug, Default)]
pub struct ArrayOfSegments {
    vertices: Vec<Pnt>,
    edges: Vec<(usize, usize)>,
}

impl ArrayOfSegments {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertex_number(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_number(&self) -> usize {
        self.edges.len()
    }

    pub fn add_vertex(&mut self, p: Pnt) {
        self.vertices.push(p);
    }

    /// AddEdges: registers one segment given two 1-based vertex indices.
    pub fn add_edges(&mut self, v1: usize, v2: usize) {
        self.edges.push((v1, v2));
    }

    pub fn vertex(&self, index1: usize) -> Pnt {
        self.vertices[index1 - 1]
    }

    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// The two endpoints of the given (0-based) segment.
    pub fn segment(&self, i: usize) -> (Pnt, Pnt) {
        let (a, b) = self.edges[i];
        (self.vertex(a), self.vertex(b))
    }
}

/// Models Aspect_TypeOfLine (only what Add uses).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfLine {
    Solid,
    DotDash,
}

/// Models the line aspect of a drawer/group.
#[derive(Clone, Debug)]
pub struct LineAspect {
    pub color: [f32; 3],
    pub type_of_line: TypeOfLine,
    pub width: f32,
}

impl Default for LineAspect {
    fn default() -> Self {
        Self { color: [1.0, 1.0, 0.0], type_of_line: TypeOfLine::Solid, width: 1.0 }
    }
}

/// Models Prs3d_Drawer (only the line aspect used by Add).
#[derive(Clone, Debug, Default)]
pub struct Drawer {
    pub line_aspect: LineAspect,
}

/// Models a Graphic3d_Group: an aspect plus primitive arrays.
#[derive(Clone, Debug, Default)]
pub struct Group {
    pub aspect: Option<LineAspect>,
    pub arrays: Vec<ArrayOfSegments>,
}

impl Group {
    pub fn set_group_primitives_aspect(&mut self, aspect: LineAspect) {
        self.aspect = Some(aspect);
    }

    pub fn add_primitive_array(&mut self, array: ArrayOfSegments) {
        self.arrays.push(array);
    }
}

/// Models Prs3d_Presentation: a list of groups with a current group.
#[derive(Clone, Debug, Default)]
pub struct Presentation {
    groups: Vec<Group>,
}

impl Presentation {
    pub fn new() -> Self {
        Self::default()
    }

    /// CurrentGroup: returns the last group, creating one if needed.
    pub fn current_group(&mut self) -> &mut Group {
        if self.groups.is_empty() {
            self.groups.push(Group::default());
        }
        self.groups.last_mut().unwrap()
    }

    pub fn groups(&self) -> &[Group] {
        &self.groups
    }
}

// ---------------------------------------------------------------------------
// The presentation tool itself (StdPrs_BndBox = Prs3d_BndBox)
// ---------------------------------------------------------------------------

/// Tool for computing a bounding-box presentation.
pub struct StdPrsBndBox;

impl StdPrsBndBox {
    /// Computes the wireframe presentation of a bounding box and adds it to
    /// the presentation with a dot-dash line aspect. Mirrors Add.
    pub fn add(presentation: &mut Presentation, bnd_box: &BndBox, drawer: &Drawer) {
        if bnd_box.is_void() {
            return;
        }
        let group = presentation.current_group();
        group.set_group_primitives_aspect(LineAspect {
            color: drawer.line_aspect.color,
            type_of_line: TypeOfLine::DotDash,
            width: drawer.line_aspect.width,
        });
        group.add_primitive_array(
            Self::fill_segments_new(bnd_box).expect("box is not void"),
        );
    }

    /// Creates an array of segments for the box, or None for a void box.
    /// Mirrors FillSegments(const Bnd_Box&).
    pub fn fill_segments_new(bnd_box: &BndBox) -> Option<ArrayOfSegments> {
        if bnd_box.is_void() {
            return None;
        }
        let mut segs = ArrayOfSegments::new();
        Self::fill_segments(&mut segs, bnd_box);
        Some(segs)
    }

    /// Appends the box wireframe to an existing array of segments.
    /// Mirrors FillSegments(handle, const Bnd_Box&).
    pub fn fill_segments(segments: &mut ArrayOfSegments, bnd_box: &BndBox) {
        if bnd_box.is_void() {
            return;
        }
        let min = bnd_box.corner_min();
        let max = bnd_box.corner_max();
        let xyz = [
            Pnt::new(min.x, min.y, min.z),
            Pnt::new(max.x, min.y, min.z),
            Pnt::new(min.x, max.y, min.z),
            Pnt::new(max.x, max.y, min.z),
            Pnt::new(min.x, min.y, max.z),
            Pnt::new(max.x, min.y, max.z),
            Pnt::new(min.x, max.y, max.z),
            Pnt::new(max.x, max.y, max.z),
        ];
        Self::fill_segments_points(segments, &xyz);
    }

    /// Low-level filler: 8 vertices, 12 edges in the OCCT order.
    /// Mirrors fillSegments.
    pub fn fill_segments_points(segments: &mut ArrayOfSegments, bx: &[Pnt; 8]) {
        let from = segments.vertex_number();
        for p in bx {
            segments.add_vertex(*p);
        }
        segments.add_edges(from + 1, from + 2);
        segments.add_edges(from + 3, from + 4);
        segments.add_edges(from + 5, from + 6);
        segments.add_edges(from + 7, from + 8);
        segments.add_edges(from + 1, from + 3);
        segments.add_edges(from + 2, from + 4);
        segments.add_edges(from + 5, from + 7);
        segments.add_edges(from + 6, from + 8);
        segments.add_edges(from + 1, from + 5);
        segments.add_edges(from + 2, from + 6);
        segments.add_edges(from + 3, from + 7);
        segments.add_edges(from + 4, from + 8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_box() -> BndBox {
        let mut bb = BndBox::new();
        bb.update(0.0, 0.0, 0.0);
        bb.update(1.0, 2.0, 3.0);
        bb
    }

    #[test]
    fn test_void_box() {
        let bb = BndBox::new();
        assert!(bb.is_void());
        assert!(StdPrsBndBox::fill_segments_new(&bb).is_none());
    }

    #[test]
    fn test_box_update() {
        let mut bb = BndBox::new();
        bb.update(1.0, 2.0, 3.0);
        assert!(!bb.is_void());
        bb.update(-1.0, 5.0, 0.0);
        assert_eq!(bb.corner_min(), Pnt::new(-1.0, 2.0, 0.0));
        assert_eq!(bb.corner_max(), Pnt::new(1.0, 5.0, 3.0));
    }

    #[test]
    fn test_fill_segments_counts() {
        let segs = StdPrsBndBox::fill_segments_new(&unit_box()).unwrap();
        assert_eq!(segs.vertex_number(), 8);
        assert_eq!(segs.edge_number(), 12);
    }

    #[test]
    fn test_fill_segments_vertex_order() {
        let segs = StdPrsBndBox::fill_segments_new(&unit_box()).unwrap();
        // Vertex order: (min|max combinations) as in Prs3d_BndBox.
        assert_eq!(segs.vertex(1), Pnt::new(0.0, 0.0, 0.0));
        assert_eq!(segs.vertex(2), Pnt::new(1.0, 0.0, 0.0));
        assert_eq!(segs.vertex(3), Pnt::new(0.0, 2.0, 0.0));
        assert_eq!(segs.vertex(4), Pnt::new(1.0, 2.0, 0.0));
        assert_eq!(segs.vertex(5), Pnt::new(0.0, 0.0, 3.0));
        assert_eq!(segs.vertex(6), Pnt::new(1.0, 0.0, 3.0));
        assert_eq!(segs.vertex(7), Pnt::new(0.0, 2.0, 3.0));
        assert_eq!(segs.vertex(8), Pnt::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_fill_segments_edge_order() {
        let segs = StdPrsBndBox::fill_segments_new(&unit_box()).unwrap();
        let expected = [
            (1, 2),
            (3, 4),
            (5, 6),
            (7, 8),
            (1, 3),
            (2, 4),
            (5, 7),
            (6, 8),
            (1, 5),
            (2, 6),
            (3, 7),
            (4, 8),
        ];
        assert_eq!(segs.edges(), &expected);
    }

    #[test]
    fn test_edges_have_correct_lengths() {
        // In a box sized 1 x 2 x 3: 4 edges of each length along each axis.
        let segs = StdPrsBndBox::fill_segments_new(&unit_box()).unwrap();
        let mut lens: Vec<f64> = (0..12)
            .map(|i| {
                let (a, b) = segs.segment(i);
                ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
            })
            .collect();
        lens.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for l in &lens[0..4] {
            assert!((l - 1.0).abs() < 1e-12);
        }
        for l in &lens[4..8] {
            assert!((l - 2.0).abs() < 1e-12);
        }
        for l in &lens[8..12] {
            assert!((l - 3.0).abs() < 1e-12);
        }
    }

    #[test]
    fn test_fill_segments_appends_with_offset() {
        // Filling twice into the same array offsets the second box's edges.
        let mut segs = ArrayOfSegments::new();
        StdPrsBndBox::fill_segments(&mut segs, &unit_box());
        StdPrsBndBox::fill_segments(&mut segs, &unit_box());
        assert_eq!(segs.vertex_number(), 16);
        assert_eq!(segs.edge_number(), 24);
        assert_eq!(segs.edges()[12], (9, 10));
        assert_eq!(segs.edges()[23], (12, 16));
    }

    #[test]
    fn test_add_to_presentation() {
        let mut prs = Presentation::new();
        let drawer = Drawer::default();
        StdPrsBndBox::add(&mut prs, &unit_box(), &drawer);

        assert_eq!(prs.groups().len(), 1);
        let group = &prs.groups()[0];
        let aspect = group.aspect.as_ref().expect("aspect set");
        // Add always uses a dot-dash line, keeping drawer color and width.
        assert_eq!(aspect.type_of_line, TypeOfLine::DotDash);
        assert_eq!(aspect.color, drawer.line_aspect.color);
        assert_eq!(aspect.width, drawer.line_aspect.width);
        assert_eq!(group.arrays.len(), 1);
        assert_eq!(group.arrays[0].vertex_number(), 8);
        assert_eq!(group.arrays[0].edge_number(), 12);
    }

    #[test]
    fn test_add_void_box_is_noop() {
        let mut prs = Presentation::new();
        StdPrsBndBox::add(&mut prs, &BndBox::new(), &Drawer::default());
        assert!(prs.groups().is_empty());
    }
}
