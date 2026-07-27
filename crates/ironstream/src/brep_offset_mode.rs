// FILE: brep_offset_mode.rs
// occt: BRepOffset_Mode, BRepOffset_Status
// occt-ref: BRepOffset_MakeOffset
//       BRepOffset_Tool

/// Offset shell building mode.
/// occt: BRepOffset_Mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepOffsetMode {
    Skin,
    Pipe,
    RectoVerso,
    ApproxRectOVerso,
}

impl Default for BRepOffsetMode {
    fn default() -> Self { Self::Skin }
}

/// Offset operation result status.
/// occt: BRepOffset_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepOffsetStatus {
    Good,
    BadOrientation,
    BadOrientationOnFace,
    EdgeDone,
    NoGoodFace,
    NoSolidFromShells,
    FaceProducingError,
    NotDone,
}

impl Default for BRepOffsetStatus {
    fn default() -> Self { Self::NotDone }
}

impl BRepOffsetStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Good | Self::EdgeDone)
    }
}

/// Offset surface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRepOffsetJoinType {
    Arc,            // circular arc for convex edges
    Tangent,        // tangent extension (open)
    Intersection,   // intersect extended faces
}

impl Default for BRepOffsetJoinType {
    fn default() -> Self { Self::Arc }
}

/// Offset builder for shells/faces.
// occt-ref: BRepOffset_MakeOffset
#[derive(Clone, Debug)]
pub struct BRepMakeOffset {
    pub source_shape_id: u32,
    pub offset: f64,
    pub tolerance: f64,
    pub mode: BRepOffsetMode,
    pub join_type: BRepOffsetJoinType,
    pub status: BRepOffsetStatus,
    pub result_shape_id: u32,
    pub is_solid: bool,
    pub remove_internal_edges: bool,
    pub generate_edge: bool,
}

impl BRepMakeOffset {
    pub fn new(source_shape_id: u32, offset: f64) -> Self {
        let ok = source_shape_id > 0;
        Self {
            source_shape_id,
            offset,
            tolerance: 1e-6,
            mode: BRepOffsetMode::Skin,
            join_type: BRepOffsetJoinType::Arc,
            status: if ok { BRepOffsetStatus::Good } else { BRepOffsetStatus::NotDone },
            result_shape_id: if ok { source_shape_id + 20000 } else { 0 },
            is_solid: false,
            remove_internal_edges: false,
            generate_edge: true,
        }
    }

    pub fn set_mode(&mut self, m: BRepOffsetMode) { self.mode = m; }
    pub fn set_join_type(&mut self, j: BRepOffsetJoinType) { self.join_type = j; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t.max(1e-12); }
    pub fn set_offset(&mut self, d: f64) { self.offset = d; }
    pub fn set_solid(&mut self, v: bool) { self.is_solid = v; }
    pub fn set_remove_internal_edges(&mut self, v: bool) { self.remove_internal_edges = v; }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn status(&self) -> BRepOffsetStatus { self.status }
    pub fn result_shape_id(&self) -> u32 { self.result_shape_id }
    pub fn offset(&self) -> f64 { self.offset }
    pub fn mode(&self) -> BRepOffsetMode { self.mode }
    pub fn join_type(&self) -> BRepOffsetJoinType { self.join_type }
}

/// Builder for shell from sequences of wires (loft/ruled surface).
// occt-ref: BRepOffsetAPI_ThruSections
#[derive(Clone, Debug)]
pub struct BRepThruSections {
    pub builder_id: u32,
    pub wire_ids: Vec<u32>,
    pub is_solid: bool,
    pub is_ruled: bool,
    pub result_shape_id: u32,
    pub is_done: bool,
}

impl BRepThruSections {
    pub fn new(builder_id: u32, is_solid: bool, is_ruled: bool) -> Self {
        Self {
            builder_id, wire_ids: Vec::new(),
            is_solid, is_ruled,
            result_shape_id: 0, is_done: false,
        }
    }

    pub fn add_wire(&mut self, wire_id: u32) { self.wire_ids.push(wire_id); }
    pub fn nb_wires(&self) -> usize { self.wire_ids.len() }

    pub fn build(&mut self) {
        if self.wire_ids.len() >= 2 {
            self.result_shape_id = self.builder_id + 21000;
            self.is_done = true;
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn result_shape_id(&self) -> u32 { self.result_shape_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_good_status() {
        let o = BRepMakeOffset::new(1, 2.0);
        assert!(o.is_done());
        assert_eq!(o.status(), BRepOffsetStatus::Good);
        assert!((o.offset() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn offset_invalid_source() {
        let o = BRepMakeOffset::new(0, 1.0);
        assert!(!o.is_done());
        assert_eq!(o.status(), BRepOffsetStatus::NotDone);
    }

    #[test]
    fn offset_mode_join() {
        let mut o = BRepMakeOffset::new(1, 1.0);
        o.set_mode(BRepOffsetMode::Pipe);
        o.set_join_type(BRepOffsetJoinType::Intersection);
        assert_eq!(o.mode(), BRepOffsetMode::Pipe);
        assert_eq!(o.join_type(), BRepOffsetJoinType::Intersection);
    }

    #[test]
    fn thru_sections_build() {
        let mut b = BRepThruSections::new(1, true, false);
        b.add_wire(10);
        b.add_wire(20);
        b.build();
        assert!(b.is_done());
        assert_eq!(b.result_shape_id(), 21001);
    }

    #[test]
    fn thru_sections_need_two_wires() {
        let mut b = BRepThruSections::new(1, false, false);
        b.add_wire(10);
        b.build();
        assert!(!b.is_done());
    }
}
