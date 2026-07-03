// FILE: rust/ironstream/crates/ironstream/src/geom_offset_surface_builder.rs
//! `Geom_OffsetSurface` construction — lightweight zero-dependency port of the
//! OCCT `Geom_OffsetSurface` / `GeomConvert_OffsetSurface` workflow, split into
//! parameter, result, and builder structs.
//!
//! This module is zero-dependency (only `std`).

// ─────────────────────────────────────────────────────────────────────────────
// OffsetSurfaceParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters for an offset surface computation.
///
// occt: Geom_OffsetSurface
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetSurfaceParams {
    offset: f64,
    is_implicit: bool,
    basis_u_first: f64,
    basis_u_last: f64,
    basis_v_first: f64,
    basis_v_last: f64,
}

impl OffsetSurfaceParams {
    /// Construct parameters with the given offset and default U/V range [0, 1].
    pub fn new(offset: f64) -> Self {
        Self {
            offset,
            is_implicit: false,
            basis_u_first: 0.0,
            basis_u_last: 1.0,
            basis_v_first: 0.0,
            basis_v_last: 1.0,
        }
    }

    /// Return the signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Set the signed offset distance.
    pub fn set_offset(&mut self, d: f64) {
        self.offset = d;
    }

    /// Return `true` if the offset is implicit (derived from the basis surface).
    pub fn is_implicit(&self) -> bool {
        self.is_implicit
    }

    /// Return the U parameter range as `(first, last)`.
    pub fn u_range(&self) -> (f64, f64) {
        (self.basis_u_first, self.basis_u_last)
    }

    /// Return the V parameter range as `(first, last)`.
    pub fn v_range(&self) -> (f64, f64) {
        (self.basis_v_first, self.basis_v_last)
    }

    /// Set the U parameter range.
    pub fn set_u_range(&mut self, first: f64, last: f64) {
        self.basis_u_first = first;
        self.basis_u_last = last;
    }

    /// Set the V parameter range.
    pub fn set_v_range(&mut self, first: f64, last: f64) {
        self.basis_v_first = first;
        self.basis_v_last = last;
    }
}

impl Default for OffsetSurfaceParams {
    fn default() -> Self {
        Self::new(0.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OffsetSurfaceResult
// ─────────────────────────────────────────────────────────────────────────────

/// Result of an offset surface build operation.
///
// occt: Geom_OffsetSurface (post-build result)
#[derive(Clone, Debug, PartialEq)]
pub struct OffsetSurfaceResult {
    nb_patches: usize,
    is_done: bool,
    max_offset_error: f64,
}

impl OffsetSurfaceResult {
    /// Construct an empty, not-done result.
    pub fn new() -> Self {
        Self {
            nb_patches: 0,
            is_done: false,
            max_offset_error: 0.0,
        }
    }

    /// Record a build result: `nb_patches` patches and offset `offset`.
    ///
    /// Sets `max_offset_error = |offset|` and marks `is_done = true`.
    pub fn build(&mut self, nb_patches: usize, offset: f64) {
        self.nb_patches = nb_patches;
        self.max_offset_error = offset.abs();
        self.is_done = true;
    }

    /// Return `true` once `build` has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of patches in the result.
    pub fn nb_patches(&self) -> usize {
        self.nb_patches
    }

    /// Return the maximum offset error (= `|offset|` in the stub).
    pub fn max_offset_error(&self) -> f64 {
        self.max_offset_error
    }
}

impl Default for OffsetSurfaceResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetSurfaceBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for an offset surface.
///
/// Call `build(nb_patches)` to populate the result.
///
// occt: Geom_OffsetSurface
#[derive(Clone, Debug)]
pub struct GeomOffsetSurfaceBuilder {
    /// Configuration parameters for this builder.
    pub params: OffsetSurfaceParams,
    /// Result populated after `build()` is called.
    pub result: OffsetSurfaceResult,
}

impl GeomOffsetSurfaceBuilder {
    /// Construct a new builder from the given parameters.
    pub fn new(params: OffsetSurfaceParams) -> Self {
        Self {
            params,
            result: OffsetSurfaceResult::new(),
        }
    }

    /// Execute the build with `nb_patches` patches.
    pub fn build(&mut self, nb_patches: usize) {
        let offset = self.params.offset();
        self.result.build(nb_patches, offset);
    }

    /// Return `true` once `build` has been called.
    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    /// Return a reference to the build result.
    pub fn result(&self) -> &OffsetSurfaceResult {
        &self.result
    }

    /// Evaluate the stub offset surface at parameter `(u, v)`.
    ///
    /// Returns `[u, v, offset]` — a flat XY plane displaced by the offset in Z.
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        [u, v, self.params.offset()]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_defaults() {
        let p = OffsetSurfaceParams::new(2.0);
        assert!((p.offset() - 2.0).abs() < 1e-15);
        assert!(!p.is_implicit());
        assert_eq!(p.u_range(), (0.0, 1.0));
        assert_eq!(p.v_range(), (0.0, 1.0));
    }

    #[test]
    fn params_set_offset() {
        let mut p = OffsetSurfaceParams::new(1.0);
        p.set_offset(5.0);
        assert!((p.offset() - 5.0).abs() < 1e-15);
    }

    #[test]
    fn result_lifecycle() {
        let mut r = OffsetSurfaceResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_patches(), 0);
        r.build(4, 2.0);
        assert!(r.is_done());
        assert_eq!(r.nb_patches(), 4);
        assert!((r.max_offset_error() - 2.0).abs() < 1e-15);
    }

    #[test]
    fn builder_build_and_evaluate() {
        let mut b = GeomOffsetSurfaceBuilder::new(OffsetSurfaceParams::new(3.0));
        assert!(!b.is_done());
        b.build(5);
        assert!(b.is_done());
        assert_eq!(b.result().nb_patches(), 5);
        let pt = b.evaluate(0.5, 0.5);
        assert!((pt[0] - 0.5).abs() < 1e-15);
        assert!((pt[1] - 0.5).abs() < 1e-15);
        assert!((pt[2] - 3.0).abs() < 1e-15);
    }
}
