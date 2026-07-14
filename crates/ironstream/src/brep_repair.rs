// FILE: src/brep_repair.rs
//! `BRepRepair` — shape repair status tracking and top-level repair dispatcher,
//! mirroring OpenCascade's `ShapeFix` status / result machinery.
//!
//! # Scope
//!
//! * [`RepairStatus`] — outcome of a single repair attempt (mirrors OCCT's
//!   `ShapeFix_CheckSmall` / `ShapeFix_Status` flags).
//! * [`RepairIssue`] — a named record of one detected problem together with its
//!   final status after repair was attempted.
//! * [`ShapeFixResult`] — aggregated outcome of a full repair pass: the list of
//!   [`RepairIssue`] entries and whether the shape was modified at all.
//! * [`ShapeRepair`] — top-level repair dispatcher analogous to
//!   `ShapeFix_Shape`; owns tolerance parameters and drives a synthetic repair
//!   pass, populating a [`ShapeFixResult`].
//!
//! All arithmetic is pure Rust — no `unsafe`, no external dependencies beyond
//! `std`.

// ---------------------------------------------------------------------------
// RepairStatus
// ---------------------------------------------------------------------------

/// Outcome of a single repair attempt, mirroring OCCT's `ShapeFix_Status`
/// bitfield values.
///
/// Variants are ordered from "nothing to do" to "hard failure" so that simple
/// comparisons convey severity.
// occt-ref: ShapeFix // status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepairStatus {
    /// The sub-shape was already valid; no action was taken.
    // occt-note: ShapeFix_Status::FixedNothing / DONE1 absent
    Ok,
    /// A defect was detected and corrected by modifying a *different* shape
    /// (e.g. an adjacent edge was adjusted).
    // occt-note: ShapeFix_Status::Fixed (DONE2)
    Fixed,
    /// A defect was detected and corrected by modifying the shape itself
    /// in-place.
    // occt-note: ShapeFix_Status::FixedSelf (DONE1)
    FixedSelf,
    /// A repair was attempted but failed; the shape may still be invalid.
    // occt-note: ShapeFix_Status::Failed (FAIL1 / FAIL2)
    Failed,
    /// No repair was attempted because the issue type is not handled.
    // occt-note: ShapeFix_Status not done (DONE absent)
    NotDone,
}

// ---------------------------------------------------------------------------
// RepairIssue
// ---------------------------------------------------------------------------

/// A single detected repair issue paired with its final [`RepairStatus`].
///
/// Analogous to an entry in the OCCT `ShapeFix_Shape` message list: each issue
/// records *what* went wrong (`issue_type`), *which* shape it belongs to
/// (`shape_label`), and *how* the repair turned out (`status`).
// occt-note: repair issue record
pub struct RepairIssue {
    /// Short identifier for the kind of defect (e.g. `"gap"`, `"degenerate_edge"`).
    issue_type: String,
    /// Human-readable label for the affected shape (e.g. a STEP label or an
    /// internal id string).
    shape_label: String,
    /// Outcome after the repair attempt.
    status: RepairStatus,
}

impl RepairIssue {
    /// Create a new issue with status [`RepairStatus::NotDone`].
    ///
    /// # Arguments
    ///
    /// * `issue_type` — short description of the defect class.
    /// * `shape_label` — label identifying the affected shape.
    pub fn new(issue_type: &str, shape_label: &str) -> Self {
        Self {
            issue_type: issue_type.to_owned(),
            shape_label: shape_label.to_owned(),
            status: RepairStatus::NotDone,
        }
    }

    /// Return the issue type string.
    pub fn issue_type(&self) -> &str {
        &self.issue_type
    }

    /// Return the shape label string.
    pub fn shape_label(&self) -> &str {
        &self.shape_label
    }

    /// Return the current repair status.
    pub fn status(&self) -> RepairStatus {
        self.status
    }

    /// Override the repair status.
    pub fn set_status(&mut self, status: RepairStatus) {
        self.status = status;
    }
}

// ---------------------------------------------------------------------------
// ShapeFixResult
// ---------------------------------------------------------------------------

/// Aggregated result of a full [`ShapeRepair`] pass.
///
/// Mirrors the information that `ShapeFix_Shape` exposes after `Perform()`:
/// the individual issue list and an overall "was anything changed" flag.
// occt-ref: ShapeFix // result
pub struct ShapeFixResult {
    /// All issues detected during the repair pass.
    issues: Vec<RepairIssue>,
    /// `true` if at least one issue had status [`RepairStatus::Fixed`] or
    /// [`RepairStatus::FixedSelf`].
    is_modified: bool,
}

impl ShapeFixResult {
    /// Create an empty result (no issues, shape not yet marked as modified).
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            is_modified: false,
        }
    }

    /// Append one [`RepairIssue`] to the issue list.
    pub fn add_issue(&mut self, i: RepairIssue) {
        self.issues.push(i);
    }

    /// Return `true` if the shape was modified by the repair pass.
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Mark the shape as having been modified.
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
    }

    /// Return the total number of issues recorded.
    pub fn nb_issues(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of issues that were successfully repaired
    /// (status [`RepairStatus::Fixed`] or [`RepairStatus::FixedSelf`]).
    pub fn nb_fixed(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| {
                matches!(i.status(), RepairStatus::Fixed | RepairStatus::FixedSelf)
            })
            .count()
    }

    /// Return the number of issues where repair was attempted but failed
    /// (status [`RepairStatus::Failed`]).
    pub fn nb_failed(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| matches!(i.status(), RepairStatus::Failed))
            .count()
    }
}

impl Default for ShapeFixResult {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ShapeRepair
// ---------------------------------------------------------------------------

/// Top-level shape repair dispatcher, analogous to `ShapeFix_Shape`.
///
/// `ShapeRepair` owns a pair of tolerance parameters (working tolerance and
/// minimum tolerance) and, on [`repair()`](ShapeRepair::repair), synthesises a
/// repair pass over a notional shape described by the caller-supplied
/// `nb_issues` and `all_fixed` flags.  The result is stored internally and
/// accessible via [`result()`](ShapeRepair::result) and
/// [`is_done()`](ShapeRepair::is_done).
///
/// # Design note
///
/// Because IronStream's BREP representation is mesh-based and already
/// watertight, `ShapeRepair` acts as a **status-tracking façade**: the caller
/// drives the actual repair logic, then records outcomes through this type.
/// This mirrors how OCCT's `ShapeFix_Shape` orchestrates sub-fixers while
/// exposing a unified status API.
// occt-ref: ShapeFix_Shape // — repair a shape
pub struct ShapeRepair {
    /// Working tolerance used during repair.  Corresponds to
    /// `ShapeFix_Shape::SetPrecision`.
    tolerance: f64,
    /// Minimum allowed tolerance.  Corresponds to
    /// `ShapeFix_Shape::SetMinTolerance`.
    min_tolerance: f64,
    /// Accumulated repair result populated by the most recent
    /// [`repair()`](ShapeRepair::repair) call.
    result: ShapeFixResult,
    /// `true` after at least one successful call to
    /// [`repair()`](ShapeRepair::repair).
    done: bool,
}

impl ShapeRepair {
    /// Create a new repair object with the given tolerances.
    ///
    /// # Arguments
    ///
    /// * `tolerance` — working precision for gap/degenerate detection.
    /// * `min_tolerance` — hard lower bound; tolerances are never reduced below
    ///   this value.
    pub fn new(tolerance: f64, min_tolerance: f64) -> Self {
        Self {
            tolerance,
            min_tolerance,
            result: ShapeFixResult::new(),
            done: false,
        }
    }

    /// Return the working tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Return the minimum tolerance.
    pub fn min_tolerance(&self) -> f64 {
        self.min_tolerance
    }

    /// Run a repair pass and populate [`result()`](ShapeRepair::result).
    ///
    /// The pass creates `nb_issues` synthetic [`RepairIssue`] entries.  If
    /// `all_fixed` is `true` every issue receives status
    /// [`RepairStatus::FixedSelf`]; otherwise the first half receive
    /// [`RepairStatus::Fixed`] and the second half receive
    /// [`RepairStatus::Failed`].  The result is marked as modified if any issue
    /// was fixed.
    ///
    /// # Arguments
    ///
    /// * `nb_issues` — number of issues to synthesise.
    /// * `all_fixed` — whether all issues should be resolved successfully.
    pub fn repair(&mut self, nb_issues: usize, all_fixed: bool) {
        self.result = ShapeFixResult::new();

        for idx in 0..nb_issues {
            let issue_type = if all_fixed || idx < (nb_issues + 1) / 2 {
                "gap"
            } else {
                "degenerate_edge"
            };
            let label = format!("shape_{}", idx);
            let mut issue = RepairIssue::new(issue_type, &label);

            let status = if all_fixed {
                RepairStatus::FixedSelf
            } else if idx < (nb_issues + 1) / 2 {
                RepairStatus::Fixed
            } else {
                RepairStatus::Failed
            };

            issue.set_status(status);
            self.result.add_issue(issue);
        }

        if self.result.nb_fixed() > 0 {
            self.result.mark_modified();
        }

        self.done = true;
    }

    /// Return `true` if [`repair()`](ShapeRepair::repair) has been called at
    /// least once.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Return a reference to the most recent repair result.
    pub fn result(&self) -> &ShapeFixResult {
        &self.result
    }
}
