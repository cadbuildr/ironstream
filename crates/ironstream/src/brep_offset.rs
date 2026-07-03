// FILE: rust/ironstream/crates/ironstream/src/brep_offset.rs

// occt: BRepOffset / BRepOffsetAPI — thick-solid and 2-D wire offset stubs.
//
// This module contains zero-dependency Rust stubs that mirror the public API
// shape of the OpenCascade `BRepOffset_*` / `BRepOffsetAPI_*` families.
// No geometry is computed; the structs exist so that downstream code can be
// written and compiled against a stable interface before a real implementation
// is provided.

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetMode
// ─────────────────────────────────────────────────────────────────────────────

/// Controls how a thick-solid offset shell is constructed.
///
// occt: BRepOffset_Mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepOffsetMode {
    /// The offset shell follows the outer skin of the shape.
    ///
    // occt: BRepOffset_Skin
    Skin,

    /// The offset shell follows a pipe profile swept along edges.
    ///
    // occt: BRepOffset_Pipe
    Pipe,

    /// The offset shell is built on both sides (recto and verso).
    ///
    // occt: BRepOffset_RectoVerso
    RectoVerso,
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetJoinType
// ─────────────────────────────────────────────────────────────────────────────

/// How adjacent offset surfaces are joined at convex edges.
///
// occt: GeomAbs_JoinType (used by BRepOffsetAPI)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepOffsetJoinType {
    /// Fill the gap with a circular arc (fillet-like join).
    ///
    // occt: GeomAbs_Arc
    Arc,

    /// Extend adjacent surfaces until they meet tangentially.
    ///
    // occt: GeomAbs_Tangent
    Tangent,

    /// Extend adjacent surfaces until they intersect.
    ///
    // occt: GeomAbs_Intersection
    Intersection,
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepOffsetParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters for thick-solid offset construction.
///
/// Acts as a configuration bundle passed to [`BrepMakeThickSolid`].
/// Mirrors the constructor and modifier interface of
/// `BRepOffsetAPI_MakeThickSolid`.
///
// occt: BRepOffsetAPI_MakeThickSolid (parameter bundle)
#[derive(Clone, Debug)]
pub struct BrepOffsetParams {
    offset: f64,
    tolerance: f64,
    mode: BrepOffsetMode,
    join_type: BrepOffsetJoinType,
    intersection: bool,
}

impl BrepOffsetParams {
    /// Create params with the given signed offset distance and linear tolerance.
    ///
    /// Defaults: mode = [`BrepOffsetMode::Skin`],
    /// join_type = [`BrepOffsetJoinType::Arc`],
    /// intersection = `false`.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — primary constructor args
    pub fn new(offset: f64, tolerance: f64) -> Self {
        Self {
            offset,
            tolerance,
            mode: BrepOffsetMode::Skin,
            join_type: BrepOffsetJoinType::Arc,
            intersection: false,
        }
    }

    /// Set the offset mode.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid::SetOffsetMode
    pub fn set_mode(&mut self, mode: BrepOffsetMode) {
        self.mode = mode;
    }

    /// Return the current offset mode.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — mode accessor
    pub fn mode(&self) -> BrepOffsetMode {
        self.mode
    }

    /// Set the join type used when adjacent offset surfaces meet at convex
    /// edges.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid::SetJoinType
    pub fn set_join_type(&mut self, join_type: BrepOffsetJoinType) {
        self.join_type = join_type;
    }

    /// Return the current join type.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — join type accessor
    pub fn join_type(&self) -> BrepOffsetJoinType {
        self.join_type
    }

    /// Return the signed offset distance.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — offset distance accessor
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Return the linear tolerance.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — tolerance accessor
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Return whether self-intersection removal is enabled.
    pub fn intersection(&self) -> bool {
        self.intersection
    }

    /// Enable or disable removal of self-intersecting portions of the offset
    /// shell.
    pub fn set_intersection(&mut self, intersection: bool) {
        self.intersection = intersection;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepMakeThickSolid
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for building a thick solid by removing selected faces from a shape
/// and offsetting the remainder outward or inward.
///
/// Call [`perform`](BrepMakeThickSolid::perform) to run the (stub) algorithm.
/// After a successful `perform` call, [`is_done`](BrepMakeThickSolid::is_done)
/// returns `true` and
/// [`nb_faces_removed`](BrepMakeThickSolid::nb_faces_removed) reports how many
/// faces were requested to be removed.
///
// occt: BRepOffsetAPI_MakeThickSolid
#[derive(Clone, Debug)]
pub struct BrepMakeThickSolid {
    /// Parameters controlling the offset.
    pub params: BrepOffsetParams,
    /// Whether [`perform`](BrepMakeThickSolid::perform) has been called
    /// successfully.
    pub is_done: bool,
    /// Number of faces passed to the last
    /// [`perform`](BrepMakeThickSolid::perform) call.
    pub nb_faces_removed: usize,
}

impl BrepMakeThickSolid {
    /// Create a new builder with the given parameters.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid::BRepOffsetAPI_MakeThickSolid
    pub fn new(params: BrepOffsetParams) -> Self {
        Self {
            params,
            is_done: false,
            nb_faces_removed: 0,
        }
    }

    /// Run the thick-solid algorithm (stub).
    ///
    /// `nb_faces_to_remove` simulates the number of faces that would be
    /// removed from the input shape before offsetting. After this call
    /// [`is_done`](BrepMakeThickSolid::is_done) returns `true`.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid::Build / MakeThickSolidBySimple / MakeThickSolidByJoin
    pub fn perform(&mut self, nb_faces_to_remove: usize) {
        self.nb_faces_removed = nb_faces_to_remove;
        self.is_done = true;
    }

    /// Whether the last [`perform`](BrepMakeThickSolid::perform) call
    /// succeeded.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid::IsDone
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of faces that were (nominally) removed during the last
    /// [`perform`](BrepMakeThickSolid::perform) call.
    ///
    // occt: BRepOffsetAPI_MakeThickSolid — removed-faces count
    pub fn nb_faces_removed(&self) -> usize {
        self.nb_faces_removed
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BrepMakeOffset
// ─────────────────────────────────────────────────────────────────────────────

/// Stub for a 2-D wire (planar) offset operation.
///
/// Offsets a closed wire lying in a plane by a signed distance, using the
/// given join type at convex corners. Call
/// [`perform`](BrepMakeOffset::perform) to run the (stub) algorithm.
///
// occt: BRepOffsetAPI_MakeOffset
#[derive(Clone, Debug)]
pub struct BrepMakeOffset {
    /// Signed offset distance (positive = expand, negative = shrink).
    pub offset: f64,
    /// Join type at convex corners.
    pub join_type: BrepOffsetJoinType,
    /// Whether [`perform`](BrepMakeOffset::perform) has been called
    /// successfully.
    pub is_done: bool,
}

impl BrepMakeOffset {
    /// Create a new 2-D wire offset builder.
    ///
    // occt: BRepOffsetAPI_MakeOffset::BRepOffsetAPI_MakeOffset
    pub fn new(offset: f64, join_type: BrepOffsetJoinType) -> Self {
        Self {
            offset,
            join_type,
            is_done: false,
        }
    }

    /// Run the offset algorithm (stub). Sets
    /// [`is_done`](BrepMakeOffset::is_done) to `true`.
    ///
    // occt: BRepOffsetAPI_MakeOffset::Build
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    /// Whether the last [`perform`](BrepMakeOffset::perform) call succeeded.
    ///
    // occt: BRepOffsetAPI_MakeOffset::IsDone
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the signed offset distance.
    ///
    // occt: BRepOffsetAPI_MakeOffset — offset distance accessor
    pub fn offset(&self) -> f64 {
        self.offset
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── BrepOffsetParams ──────────────────────────────────────────────────────

    #[test]
    fn params_new_defaults() {
        let p = BrepOffsetParams::new(1.5, 0.01);
        assert_eq!(p.offset(), 1.5);
        assert_eq!(p.tolerance(), 0.01);
        assert_eq!(p.mode(), BrepOffsetMode::Skin);
        assert_eq!(p.join_type(), BrepOffsetJoinType::Arc);
        assert!(!p.intersection());
    }

    #[test]
    fn params_set_mode() {
        let mut p = BrepOffsetParams::new(1.0, 0.001);
        p.set_mode(BrepOffsetMode::Pipe);
        assert_eq!(p.mode(), BrepOffsetMode::Pipe);
        p.set_mode(BrepOffsetMode::RectoVerso);
        assert_eq!(p.mode(), BrepOffsetMode::RectoVerso);
    }

    #[test]
    fn params_set_join_type() {
        let mut p = BrepOffsetParams::new(1.0, 0.001);
        p.set_join_type(BrepOffsetJoinType::Intersection);
        assert_eq!(p.join_type(), BrepOffsetJoinType::Intersection);
        p.set_join_type(BrepOffsetJoinType::Tangent);
        assert_eq!(p.join_type(), BrepOffsetJoinType::Tangent);
    }

    #[test]
    fn params_set_intersection() {
        let mut p = BrepOffsetParams::new(0.5, 0.01);
        p.set_intersection(true);
        assert!(p.intersection());
        p.set_intersection(false);
        assert!(!p.intersection());
    }

    // ── BrepMakeThickSolid ────────────────────────────────────────────────────

    #[test]
    fn thick_solid_not_done_before_perform() {
        let params = BrepOffsetParams::new(2.0, 0.01);
        let builder = BrepMakeThickSolid::new(params);
        assert!(!builder.is_done());
        assert_eq!(builder.nb_faces_removed(), 0);
    }

    #[test]
    fn thick_solid_done_after_perform() {
        let params = BrepOffsetParams::new(2.0, 0.01);
        let mut builder = BrepMakeThickSolid::new(params);
        builder.perform(3);
        assert!(builder.is_done());
        assert_eq!(builder.nb_faces_removed(), 3);
    }

    #[test]
    fn thick_solid_zero_faces_removed() {
        let params = BrepOffsetParams::new(0.5, 1e-6);
        let mut builder = BrepMakeThickSolid::new(params);
        builder.perform(0);
        assert!(builder.is_done());
        assert_eq!(builder.nb_faces_removed(), 0);
    }

    #[test]
    fn thick_solid_params_accessible() {
        let mut params = BrepOffsetParams::new(3.0, 0.005);
        params.set_mode(BrepOffsetMode::RectoVerso);
        params.set_join_type(BrepOffsetJoinType::Intersection);
        let builder = BrepMakeThickSolid::new(params);
        assert_eq!(builder.params.offset(), 3.0);
        assert_eq!(builder.params.mode(), BrepOffsetMode::RectoVerso);
        assert_eq!(builder.params.join_type(), BrepOffsetJoinType::Intersection);
    }

    // ── BrepMakeOffset ────────────────────────────────────────────────────────

    #[test]
    fn make_offset_not_done_before_perform() {
        let mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
        assert!(!mo.is_done());
    }

    #[test]
    fn make_offset_done_after_perform() {
        let mut mo = BrepMakeOffset::new(1.0, BrepOffsetJoinType::Arc);
        mo.perform();
        assert!(mo.is_done());
    }

    #[test]
    fn make_offset_negative_distance() {
        let mut mo = BrepMakeOffset::new(-0.5, BrepOffsetJoinType::Intersection);
        mo.perform();
        assert!(mo.is_done());
        assert_eq!(mo.offset(), -0.5);
    }

    #[test]
    fn make_offset_join_type_preserved() {
        let mo = BrepMakeOffset::new(2.0, BrepOffsetJoinType::Tangent);
        assert_eq!(mo.join_type, BrepOffsetJoinType::Tangent);
    }

    #[test]
    fn make_offset_zero_distance() {
        let mut mo = BrepMakeOffset::new(0.0, BrepOffsetJoinType::Arc);
        mo.perform();
        assert!(mo.is_done());
        assert_eq!(mo.offset(), 0.0);
    }
}
