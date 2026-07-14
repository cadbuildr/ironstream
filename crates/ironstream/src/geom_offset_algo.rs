// occt-note: Geom2dOffset_Status, GeomAPI_OffsetCurve, Geom2dAdaptor_Curve offset result

/// Status codes returned by 2D/3D offset curve algorithms.
///
// occt-ref: Geom2dOffset_Status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetAlgoStatus {
    /// No error — the offset was computed successfully.
    // occt-ref: Geom2dOffset_NoError
    NoError,
    /// The basis curve does not have sufficient continuity.
    // occt-ref: Geom2dOffset_NotContinuous
    NotContinuous,
    /// The algorithm could not trim the offset curve.
    // occt-ref: Geom2dOffset_CannotTrim
    CannotTrim,
    /// The algorithm could not close the offset curve.
    // occt-ref: Geom2dOffset_CannotCloseOffset
    CannotCloseOffset,
    /// The offset distance produces a null (degenerate) offset curve.
    // occt-ref: Geom2dOffset_NullOffsetCurve
    NullOffsetCurve,
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetCurveParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters for a 3D offset curve computation.
///
// occt-note: GeomAPI_OffsetCurve params
#[derive(Clone, Debug)]
pub struct GeomOffsetCurveParams {
    /// Signed offset distance.
    // occt-note: GeomAPI_OffsetCurve::Offset
    offset: f64,
    /// Reference direction used to compute the offset normal: N = normalize(direction × T).
    // occt-note: GeomAPI_OffsetCurve::Direction (gp_Dir)
    direction: [f64; 3],
    /// Required geometric continuity order (1 = C1, 2 = C2, …).
    // occt-note: GeomAPI_OffsetCurve::Continuity (GeomAbs_Shape)
    continuity: u8,
    /// Tolerance used during the offset computation.
    // occt-note: GeomAPI_OffsetCurve::Tolerance
    tolerance: f64,
}

impl GeomOffsetCurveParams {
    /// Create params with the given offset distance and reference direction.
    ///
    /// Defaults: `continuity = 1` (C1), `tolerance = 1e-7`.
    pub fn new(offset: f64, direction: [f64; 3]) -> Self {
        Self {
            offset,
            direction,
            continuity: 1,
            tolerance: 1e-7,
        }
    }

    /// Set the required continuity order.
    pub fn set_continuity(&mut self, continuity: u8) {
        self.continuity = continuity;
    }

    /// Required continuity order.
    pub fn continuity(&self) -> u8 {
        self.continuity
    }

    /// Signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Reference direction.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Tolerance used during the offset computation.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomOffsetCurveResult
// ─────────────────────────────────────────────────────────────────────────────

/// Result of a 3D offset curve computation.
///
// occt-note: offset result (GeomAPI_OffsetCurve)
#[derive(Clone, Debug)]
pub struct GeomOffsetCurveResult {
    /// Current status of the algorithm.
    // occt-ref: Geom2dOffset_Status
    status: OffsetAlgoStatus,
    /// Whether the computation completed successfully.
    is_done: bool,
    /// Number of intervals in the resulting offset curve.
    nb_intervals: usize,
}

impl GeomOffsetCurveResult {
    /// Create a fresh (not-done) result.
    pub fn new() -> Self {
        Self {
            status: OffsetAlgoStatus::NoError,
            is_done: false,
            nb_intervals: 0,
        }
    }

    /// Mark the computation as done with `nb_intervals` intervals.
    pub fn set_done(&mut self, nb_intervals: usize) {
        self.is_done = true;
        self.nb_intervals = nb_intervals;
        self.status = OffsetAlgoStatus::NoError;
    }

    /// Whether the computation completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Current algorithm status.
    pub fn status(&self) -> OffsetAlgoStatus {
        self.status
    }

    /// Number of intervals in the offset curve.
    pub fn nb_intervals(&self) -> usize {
        self.nb_intervals
    }
}

impl Default for GeomOffsetCurveResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dOffsetCurveParams
// ─────────────────────────────────────────────────────────────────────────────

/// Parameters for a 2D offset curve computation.
///
// occt-ref: Geom2dAdaptor_Curve // offset params
#[derive(Clone, Debug)]
pub struct Geom2dOffsetCurveParams {
    /// Signed offset distance.
    // occt-ref: Geom2d_OffsetCurve // offset value
    offset: f64,
    /// Tolerance for the offset computation.
    // occt-note: Geom2dOffset tolerance
    tolerance: f64,
    /// Join type: `0` = arc join, `1` = tangent (intersection) join.
    // occt-ref: GeomAbs_JoinType // (0=Arc, 1=Tangent)
    join_type: u8,
}

impl Geom2dOffsetCurveParams {
    /// Create params with the given offset distance and tolerance.
    ///
    /// Default join type is `0` (arc).
    pub fn new(offset: f64, tolerance: f64) -> Self {
        Self {
            offset,
            tolerance,
            join_type: 0,
        }
    }

    /// Join type: `0` = arc, `1` = tangent.
    pub fn join_type(&self) -> u8 {
        self.join_type
    }

    /// Set the join type.
    pub fn set_join_type(&mut self, join_type: u8) {
        self.join_type = join_type;
    }

    /// Signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Tolerance for the computation.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Geom2dOffsetResult
// ─────────────────────────────────────────────────────────────────────────────

/// Result of a 2D offset curve computation.
///
// occt-note: 2D offset curve result (Geom2dAdaptor / Geom2dOffset)
#[derive(Clone, Debug)]
pub struct Geom2dOffsetResult {
    /// Whether the computation completed successfully.
    is_done: bool,
    /// Current status of the algorithm.
    // occt-ref: Geom2dOffset_Status
    status: OffsetAlgoStatus,
    /// Number of intervals in the resulting offset curve.
    nb_intervals: usize,
}

impl Geom2dOffsetResult {
    /// Create a fresh (not-done) result.
    pub fn new() -> Self {
        Self {
            is_done: false,
            status: OffsetAlgoStatus::NoError,
            nb_intervals: 0,
        }
    }

    /// Simulate performing the offset with `n` intervals.
    ///
    /// Sets `is_done = true` and `nb_intervals = n`.
    pub fn perform(&mut self, n: usize) {
        self.is_done = true;
        self.nb_intervals = n;
        self.status = OffsetAlgoStatus::NoError;
    }

    /// Whether the computation completed successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of intervals in the offset curve.
    pub fn nb_intervals(&self) -> usize {
        self.nb_intervals
    }
}

impl Default for Geom2dOffsetResult {
    fn default() -> Self {
        Self::new()
    }
}
