// FILE: rust/ironstream/crates/ironstream/src/hatch_build.rs
//! `HatchBuild` — hatch line and hatch-pattern builder types, mirroring
//! OpenCascade's `Hatch` package.
//!
//! # Scope
//!
//! This module provides the types used to define and trim 2-D hatch patterns:
//!
//! * [`HatchDomain`] — a parameter interval on a single hatch line (bounded or
//!   fully open), mirroring `HatchGen_Domain` as used by `Hatch_Hatcher`.
//! * [`HatchLine`] — one directed line in a hatch pattern together with its
//!   list of trimmed domains, mirroring `Hatch_Line`.
//! * [`HatchBuilder`] — a collection of [`HatchLine`] values and the logic to
//!   add trimmed domains to them, mirroring `Hatch_Hatcher`.
//!
//! # Notes
//!
//! All types are zero-dependency pure-Rust data structures; no external crates
//! are required.  Coordinates are 2-D `[f64; 2]` pairs throughout.

// ─────────────────────────────────────────────────────────────────────────────
// HatchDomain
// ─────────────────────────────────────────────────────────────────────────────

/// A parameter interval on a hatch line, delimited by a first and/or last
/// parameter value.
///
/// A domain may be:
/// * **Bounded** — both `first_param` and `last_param` are meaningful.
/// * **Open** — neither endpoint is meaningful (the full line is the domain).
///
/// Semi-open domains (one endpoint set, one missing) are not modelled here
/// because `Hatch_Hatcher` only creates bounded or fully-open intervals.
///
// occt: HatchGen_Domain
#[derive(Clone, Debug)]
pub struct HatchDomain {
    has_first: bool,
    has_last: bool,
    first_param: f64,
    last_param: f64,
}

impl HatchDomain {
    /// Create a bounded domain spanning `[first, last]`.
    ///
    /// If `first > last` the values are silently swapped so the invariant
    /// `first_param <= last_param` always holds.
    ///
    /// Mirrors `HatchGen_Domain(First, Last)`.
    pub fn new_bounded(first: f64, last: f64) -> Self {
        let (f, l) = if first <= last {
            (first, last)
        } else {
            (last, first)
        };
        Self {
            has_first: true,
            has_last: true,
            first_param: f,
            last_param: l,
        }
    }

    /// Create a fully-open domain (the hatch line is unbounded in both
    /// directions).
    ///
    /// Mirrors `HatchGen_Domain()`.
    pub fn new_open() -> Self {
        Self {
            has_first: false,
            has_last: false,
            first_param: 0.0,
            last_param: 0.0,
        }
    }

    /// `true` when a lower-bound parameter has been set.
    ///
    /// Mirrors `HatchGen_Domain::HasFirstPoint()`.
    pub fn has_first(&self) -> bool {
        self.has_first
    }

    /// `true` when an upper-bound parameter has been set.
    ///
    /// Mirrors `HatchGen_Domain::HasLastPoint()`.
    pub fn has_last(&self) -> bool {
        self.has_last
    }

    /// The lower-bound parameter.
    ///
    /// The return value is only meaningful when [`has_first`](Self::has_first)
    /// returns `true`.
    ///
    /// Mirrors `HatchGen_Domain::FirstPoint().Parameter()`.
    pub fn first_param(&self) -> f64 {
        self.first_param
    }

    /// The upper-bound parameter.
    ///
    /// The return value is only meaningful when [`has_last`](Self::has_last)
    /// returns `true`.
    ///
    /// Mirrors `HatchGen_Domain::LastPoint().Parameter()`.
    pub fn last_param(&self) -> f64 {
        self.last_param
    }

    /// The signed length of the domain (`last_param - first_param`).
    ///
    /// Returns `f64::INFINITY` when the domain is open (either endpoint
    /// missing).
    ///
    /// Mirrors the arithmetic on `HatchGen_Domain` endpoint parameters.
    pub fn length(&self) -> f64 {
        if self.has_first && self.has_last {
            self.last_param - self.first_param
        } else {
            f64::INFINITY
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchLine
// ─────────────────────────────────────────────────────────────────────────────

/// A directed 2-D hatch line together with the trimmed parameter domains that
/// fall inside the hatched region.
///
/// The line is represented by an origin point and a unit-direction vector
/// (both as `[f64; 2]`).  It carries a list of [`HatchDomain`] values added
/// by [`HatchBuilder::trim_by_domain`].
///
// occt: Hatch_Line
#[derive(Clone, Debug)]
pub struct HatchLine {
    origin: [f64; 2],
    direction: [f64; 2],
    domains: Vec<HatchDomain>,
}

impl HatchLine {
    /// Create a new hatch line through `origin` in the given `direction`.
    ///
    /// `direction` need not be unit-length; callers are responsible for
    /// normalization if required.
    ///
    /// Mirrors `Hatch_Line(Origin, Direction)`.
    pub fn new(origin: [f64; 2], direction: [f64; 2]) -> Self {
        Self {
            origin,
            direction,
            domains: Vec::new(),
        }
    }

    /// The origin point of the hatch line.
    ///
    /// Mirrors `Hatch_Line::Origin()`.
    pub fn origin(&self) -> [f64; 2] {
        self.origin
    }

    /// The direction vector of the hatch line.
    ///
    /// Mirrors `Hatch_Line::Direction()`.
    pub fn direction(&self) -> [f64; 2] {
        self.direction
    }

    /// Append a domain to this line's domain list.
    ///
    /// Mirrors the internal domain-appending logic of `Hatch_Hatcher`.
    pub fn add_domain(&mut self, d: HatchDomain) {
        self.domains.push(d);
    }

    /// Number of domains on this line.
    ///
    /// Mirrors `Hatch_Line::NbDomains()`.
    pub fn nb_domains(&self) -> usize {
        self.domains.len()
    }

    /// Borrow the domain at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics when `i >= nb_domains()`.
    ///
    /// Mirrors `Hatch_Line::Domain(i)` (OCCT uses 1-based indexing; here
    /// 0-based indexing is used for idiomatic Rust).
    pub fn domain(&self, i: usize) -> &HatchDomain {
        &self.domains[i]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HatchBuilder
// ─────────────────────────────────────────────────────────────────────────────

/// Creates a hatch pattern by accumulating directed hatch lines and trimming
/// them with parameter-space domains.
///
/// Typical usage:
/// 1. Construct with [`HatchBuilder::new`].
/// 2. Add lines with [`add_line`](Self::add_line).
/// 3. For each line, call [`trim_by_domain`](Self::trim_by_domain) once or
///    more to record which parameter intervals lie inside the hatched region.
/// 4. Iterate over [`line`](Self::line) / [`nb_lines`](Self::nb_lines) to read
///    back the trimmed pattern.
///
// occt: Hatch_Hatcher
#[derive(Clone, Debug)]
pub struct HatchBuilder {
    lines: Vec<HatchLine>,
    tolerance: f64,
}

impl HatchBuilder {
    /// Create an empty hatch builder with a given positional tolerance.
    ///
    /// `tolerance` is used to decide whether two parameter values should be
    /// considered identical (callers may pass it to domain-length checks).
    ///
    /// Mirrors `Hatch_Hatcher(Tolerance)`.
    pub fn new(tolerance: f64) -> Self {
        Self {
            lines: Vec::new(),
            tolerance,
        }
    }

    /// Append a new hatch line defined by `origin` and `direction`.
    ///
    /// The line is added with an empty domain list; call
    /// [`trim_by_domain`](Self::trim_by_domain) afterwards to populate it.
    ///
    /// Mirrors `Hatch_Hatcher::AddLine(Origin, Direction)`.
    pub fn add_line(&mut self, origin: [f64; 2], direction: [f64; 2]) {
        self.lines.push(HatchLine::new(origin, direction));
    }

    /// Number of hatch lines currently stored.
    ///
    /// Mirrors `Hatch_Hatcher::NbLines()`.
    pub fn nb_lines(&self) -> usize {
        self.lines.len()
    }

    /// Borrow the hatch line at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics when `i >= nb_lines()`.
    ///
    /// Mirrors `Hatch_Hatcher::Line(i)` (OCCT uses 1-based indexing; here
    /// 0-based indexing is used for idiomatic Rust).
    pub fn line(&self, i: usize) -> &HatchLine {
        &self.lines[i]
    }

    /// Add a bounded domain `[first, last]` to the hatch line at index
    /// `line_idx`.
    ///
    /// The domain is only added when its length exceeds the builder's
    /// `tolerance`; zero-length (degenerate) domains are silently discarded.
    ///
    /// # Panics
    ///
    /// Panics when `line_idx >= nb_lines()`.
    ///
    /// Mirrors `Hatch_Hatcher::Trim(LineIndex, First, Last)`.
    pub fn trim_by_domain(&mut self, line_idx: usize, first: f64, last: f64) {
        let (f, l) = if first <= last {
            (first, last)
        } else {
            (last, first)
        };
        if l - f > self.tolerance {
            self.lines[line_idx].add_domain(HatchDomain::new_bounded(f, l));
        }
    }

    /// The positional tolerance supplied at construction time.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}
