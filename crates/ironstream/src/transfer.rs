// FILE: transfer.rs
//! `Transfer` — framework for mapping ("transferring") objects between two
//! representations, mirroring OpenCascade's `Transfer` package
//! (`src/DataExchange/TKXSBase/Transfer/`).
//!
//! # Scope
//!
//! * [`BinderStatus`]            — status codes for a single transfer result.
//! * [`Transfer_Binder`]         — a typed container that holds the result of
//!                                  translating one source object.
//! * [`Transfer_VoidBinder`]     — a binder whose result is intentionally empty
//!                                  (used to mark objects with no geometric
//!                                  representation).
//! * [`Transfer_FinderProcess`]  — process that maps "finder" (source) objects
//!                                  identified by a `u64` key to binders.
//! * [`Transfer_TransientProcess`]— process that maps transient (heap) objects
//!                                  identified by a `u64` pointer-like key to
//!                                  binders, and carries an ordered transfer
//!                                  model (list of root objects).
//!
//! # OCCT correspondence
//!
//! | OCCT class                     | IronStream type                  |
//! |--------------------------------|----------------------------------|
//! | `Transfer_Binder`              | [`Transfer_Binder`]              |
//! | `Transfer_VoidBinder`          | [`Transfer_VoidBinder`]          |
//! | `Transfer_FinderProcess`       | [`Transfer_FinderProcess`]       |
//! | `Transfer_TransientProcess`    | [`Transfer_TransientProcess`]    |

use std::any::Any;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// BinderStatus
// ---------------------------------------------------------------------------

/// Status of a single transfer operation, mirroring `Transfer_StatusExec`.
// occt: Transfer_StatusExec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinderStatus {
    /// Transfer has not yet been attempted.
    Void,
    /// Transfer is running (loop-guard).
    Run,
    /// Transfer completed successfully.
    Done,
    /// Transfer failed or produced an error.
    Error,
}

impl Default for BinderStatus {
    fn default() -> Self {
        Self::Void
    }
}

impl std::fmt::Display for BinderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinderStatus::Void  => "Void",
            BinderStatus::Run   => "Run",
            BinderStatus::Done  => "Done",
            BinderStatus::Error => "Error",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// Transfer_Binder
// ---------------------------------------------------------------------------

/// A container that holds the result of translating one source object.
///
/// In OCCT, `Transfer_Binder` is an abstract base class with typed
/// subclasses.  Here we use a generic struct so that any result type can be
/// stored.  The `result` field is boxed as `dyn Any` to allow heterogeneous
/// storage inside a process map.
// occt: Transfer_Binder
#[derive(Debug)]
pub struct Transfer_Binder {
    status: BinderStatus,
    /// Boxed result value.  `None` for void binders or when the transfer has
    /// not yet run.
    result: Option<Box<dyn Any>>,
    /// Human-readable error message set when `status == Error`.
    error_message: Option<String>,
    /// An optional chain of additional binders (OCCT's "next" binder link
    /// for types that need to transport multiple results).
    next: Option<Box<Transfer_Binder>>,
    /// Whether this binder is a "void" binder (no result by design).
    is_void: bool,
}

impl Transfer_Binder {
    // --- Constructors ----------------------------------------------------------

    /// Create an empty binder in `Void` status.
    pub fn new() -> Self {
        Self {
            status: BinderStatus::Void,
            result: None,
            error_message: None,
            next: None,
            is_void: false,
        }
    }

    /// Create a binder that has already been completed with a result value.
    pub fn with_result<T: Any>(value: T) -> Self {
        Self {
            status: BinderStatus::Done,
            result: Some(Box::new(value)),
            error_message: None,
            next: None,
            is_void: false,
        }
    }

    /// Create a void binder (OCCT `Transfer_VoidBinder` equivalent for inline use).
    pub fn void() -> Self {
        Self {
            status: BinderStatus::Done,
            result: None,
            error_message: None,
            next: None,
            is_void: true,
        }
    }

    // --- Status ----------------------------------------------------------------

    /// Current execution status.
    pub fn status(&self) -> BinderStatus {
        self.status
    }

    /// Mark the binder as actively running (loop-guard sentinel).
    pub fn set_status_run(&mut self) {
        self.status = BinderStatus::Run;
    }

    /// Mark the binder as done.  Does not store a result; use `set_result`
    /// afterwards if needed.
    pub fn set_status_done(&mut self) {
        self.status = BinderStatus::Done;
    }

    /// Mark the binder as failed with a message.
    pub fn set_status_error(&mut self, message: impl Into<String>) {
        self.status = BinderStatus::Error;
        self.error_message = Some(message.into());
    }

    /// `true` when the transfer completed without error.
    pub fn is_done(&self) -> bool {
        self.status == BinderStatus::Done
    }

    /// `true` when the transfer is intentionally void (no result expected).
    pub fn is_void_binder(&self) -> bool {
        self.is_void
    }

    /// `true` when the transfer failed.
    pub fn has_failed(&self) -> bool {
        self.status == BinderStatus::Error
    }

    // --- Result ----------------------------------------------------------------

    /// Store a typed result value and set status to `Done`.
    pub fn set_result<T: Any>(&mut self, value: T) {
        self.result = Some(Box::new(value));
        self.status = BinderStatus::Done;
    }

    /// Try to downcast the stored result to type `T`.  Returns `None` if the
    /// binder carries no result or the type does not match.
    pub fn result<T: Any>(&self) -> Option<&T> {
        self.result.as_ref().and_then(|b| b.downcast_ref::<T>())
    }

    /// `true` when a result value is stored.
    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // --- Error message ---------------------------------------------------------

    /// Error message set when the status is `Error`.
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    // --- Next-binder chain -----------------------------------------------------

    /// Attach another binder as the "next" element in the chain.  OCCT uses
    /// this for objects that have multiple associated results (e.g. both a
    /// solid and a wire produced for the same source entity).
    pub fn set_next(&mut self, next: Transfer_Binder) {
        self.next = Some(Box::new(next));
    }

    /// Immutable access to the next binder in the chain.
    pub fn next(&self) -> Option<&Transfer_Binder> {
        self.next.as_deref()
    }

    /// `true` when a next binder is attached.
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Count the number of binders in the chain including `self`.
    pub fn chain_length(&self) -> usize {
        let mut count = 1usize;
        let mut cur = self.next.as_deref();
        while let Some(b) = cur {
            count += 1;
            cur = b.next.as_deref();
        }
        count
    }
}

impl Default for Transfer_Binder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Transfer_VoidBinder
// ---------------------------------------------------------------------------

/// A binder whose result is intentionally empty.
///
/// OCCT uses `Transfer_VoidBinder` to record that a source entity was visited
/// and intentionally produced no geometry (e.g. a purely structural entity in
/// a STEP file that has no shape representation).
// occt: Transfer_VoidBinder
#[derive(Debug, Clone)]
pub struct Transfer_VoidBinder {
    /// Optional annotation explaining why the result is void.
    annotation: Option<String>,
}

impl Transfer_VoidBinder {
    /// Create a void binder with no annotation.
    pub fn new() -> Self {
        Self { annotation: None }
    }

    /// Create a void binder with an explanatory annotation.
    pub fn with_annotation(msg: impl Into<String>) -> Self {
        Self { annotation: Some(msg.into()) }
    }

    /// The annotation, if any.
    pub fn annotation(&self) -> Option<&str> {
        self.annotation.as_deref()
    }

    /// Convert into a generic [`Transfer_Binder`] in Done/void state.
    pub fn into_binder(self) -> Transfer_Binder {
        let mut b = Transfer_Binder::void();
        if let Some(ann) = self.annotation {
            b.error_message = Some(ann); // repurpose field for the annotation text
        }
        b
    }

    /// Status is always `Done` for a void binder (nothing to fail).
    pub fn status(&self) -> BinderStatus {
        BinderStatus::Done
    }
}

impl Default for Transfer_VoidBinder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// TransferError
// ---------------------------------------------------------------------------

/// Error returned by transfer process operations.
// occt: Transfer_Failure (represented as exception in OCCT)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferError {
    /// Human-readable description.
    pub message: String,
}

impl TransferError {
    /// Construct with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transfer error: {}", self.message)
    }
}

impl std::error::Error for TransferError {}

// ---------------------------------------------------------------------------
// Internal map type shared by both process types
// ---------------------------------------------------------------------------

/// A map from a `u64` key to a [`Transfer_Binder`], plus an ordered list of
/// root keys (entities whose transfer was explicitly requested by the caller,
/// not just discovered as a dependency).
struct ProcessMap {
    map: HashMap<u64, Transfer_Binder>,
    roots: Vec<u64>,
}

impl ProcessMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            roots: Vec::new(),
        }
    }

    /// Bind `key` to `binder`.  Replaces any existing binding.
    fn bind(&mut self, key: u64, binder: Transfer_Binder) {
        self.map.insert(key, binder);
    }

    /// Return a shared reference to the binder for `key`.
    fn find(&self, key: u64) -> Option<&Transfer_Binder> {
        self.map.get(&key)
    }

    /// Return a mutable reference to the binder for `key`.
    fn find_mut(&mut self, key: u64) -> Option<&mut Transfer_Binder> {
        self.map.get_mut(&key)
    }

    /// `true` if `key` is already bound.
    fn is_bound(&self, key: u64) -> bool {
        self.map.contains_key(&key)
    }

    /// Record `key` as a root (top-level) entity.
    fn add_root(&mut self, key: u64) {
        if !self.roots.contains(&key) {
            self.roots.push(key);
        }
    }

    /// All root keys in the order they were added.
    fn roots(&self) -> &[u64] {
        &self.roots
    }

    /// Total number of bindings.
    fn nb_mapped(&self) -> usize {
        self.map.len()
    }

    /// Number of successful (Done) transfers.
    fn nb_done(&self) -> usize {
        self.map.values().filter(|b| b.is_done()).count()
    }

    /// Number of errored transfers.
    fn nb_errors(&self) -> usize {
        self.map.values().filter(|b| b.has_failed()).count()
    }

    /// Remove all bindings and roots.
    fn clear(&mut self) {
        self.map.clear();
        self.roots.clear();
    }
}

// ---------------------------------------------------------------------------
// Transfer_FinderProcess
// ---------------------------------------------------------------------------

/// Process that maps "finder" objects (identified by opaque `u64` keys) to
/// binders.
///
/// In OCCT, `Transfer_FinderProcess` associates `Handle(Transfer_Finder)`
/// keys — which provide an equality operator — to binders.  In pure Rust we
/// use a `u64` identity key (callers can use an incrementing counter, a hash
/// of the source object, etc.).  This gives the same logical API without
/// requiring a vtable-based `Transfer_Finder` hierarchy.
// occt: Transfer_FinderProcess
pub struct Transfer_FinderProcess {
    inner: ProcessMap,
    /// Human-readable label for diagnostic purposes.
    label: String,
}

impl Transfer_FinderProcess {
    // --- Construction ---------------------------------------------------------

    /// Create a new, empty finder process.
    pub fn new() -> Self {
        Self {
            inner: ProcessMap::new(),
            label: String::new(),
        }
    }

    /// Create a finder process with a diagnostic label.
    pub fn with_label(label: impl Into<String>) -> Self {
        Self {
            inner: ProcessMap::new(),
            label: label.into(),
        }
    }

    /// Diagnostic label.
    pub fn label(&self) -> &str {
        &self.label
    }

    // --- Binding --------------------------------------------------------------

    /// Bind `finder_key` to `binder`.  If `finder_key` is already bound the
    /// existing binder is replaced.
    pub fn bind(&mut self, finder_key: u64, binder: Transfer_Binder) {
        self.inner.bind(finder_key, binder);
    }

    /// Convenience: bind `finder_key` to a void binder.
    pub fn bind_void(&mut self, finder_key: u64) {
        self.inner.bind(finder_key, Transfer_Binder::void());
    }

    /// Convenience: bind `finder_key` to a typed result value.
    pub fn bind_result<T: Any>(&mut self, finder_key: u64, value: T) {
        self.inner.bind(finder_key, Transfer_Binder::with_result(value));
    }

    /// Mark `finder_key` as a root entity (top-level transfer request).
    /// The key must already be bound; if not, an error is returned.
    pub fn set_root(&mut self, finder_key: u64) -> Result<(), TransferError> {
        if !self.inner.is_bound(finder_key) {
            return Err(TransferError::new(
                format!("FinderProcess::set_root: key {} is not bound", finder_key),
            ));
        }
        self.inner.add_root(finder_key);
        Ok(())
    }

    // --- Lookup ---------------------------------------------------------------

    /// Find the binder for `finder_key`.  Returns `None` when not bound.
    pub fn find(&self, finder_key: u64) -> Option<&Transfer_Binder> {
        self.inner.find(finder_key)
    }

    /// Mutable access to the binder for `finder_key`.
    pub fn find_mut(&mut self, finder_key: u64) -> Option<&mut Transfer_Binder> {
        self.inner.find_mut(finder_key)
    }

    /// `true` when `finder_key` is already bound.
    pub fn is_bound(&self, finder_key: u64) -> bool {
        self.inner.is_bound(finder_key)
    }

    /// Try to retrieve a typed result for `finder_key`.  Returns `None` if
    /// the key is not bound, has no result, or the result type does not match.
    pub fn find_result<T: Any>(&self, finder_key: u64) -> Option<&T> {
        self.inner.find(finder_key)?.result::<T>()
    }

    // --- Statistics -----------------------------------------------------------

    /// Total number of bound keys.
    pub fn nb_mapped(&self) -> usize {
        self.inner.nb_mapped()
    }

    /// Number of done (successful) transfers.
    pub fn nb_done(&self) -> usize {
        self.inner.nb_done()
    }

    /// Number of errored transfers.
    pub fn nb_errors(&self) -> usize {
        self.inner.nb_errors()
    }

    /// Root keys in insertion order.
    pub fn roots(&self) -> &[u64] {
        self.inner.roots()
    }

    /// Remove all bindings and roots.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    // --- Loop-guard helpers ---------------------------------------------------

    /// Begin the transfer of `finder_key`: creates a binder in `Run` state
    /// and stores it.  Returns `Err` if the key is already in `Run` state
    /// (cycle detected).
    pub fn start_transfer(&mut self, finder_key: u64) -> Result<(), TransferError> {
        if let Some(b) = self.inner.find(finder_key) {
            if b.status() == BinderStatus::Run {
                return Err(TransferError::new(format!(
                    "FinderProcess: cycle detected for key {}",
                    finder_key
                )));
            }
        }
        let mut binder = Transfer_Binder::new();
        binder.set_status_run();
        self.inner.bind(finder_key, binder);
        Ok(())
    }

    /// Mark the transfer of `finder_key` as successfully done with a result.
    pub fn finish_transfer<T: Any>(
        &mut self,
        finder_key: u64,
        value: T,
    ) -> Result<(), TransferError> {
        match self.inner.find_mut(finder_key) {
            Some(b) => {
                b.set_result(value);
                Ok(())
            }
            None => Err(TransferError::new(format!(
                "FinderProcess::finish_transfer: key {} not started",
                finder_key
            ))),
        }
    }

    /// Mark the transfer of `finder_key` as failed.
    pub fn fail_transfer(
        &mut self,
        finder_key: u64,
        message: impl Into<String>,
    ) -> Result<(), TransferError> {
        match self.inner.find_mut(finder_key) {
            Some(b) => {
                b.set_status_error(message);
                Ok(())
            }
            None => Err(TransferError::new(format!(
                "FinderProcess::fail_transfer: key {} not started",
                finder_key
            ))),
        }
    }
}

impl Default for Transfer_FinderProcess {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Transfer_TransientProcess
// ---------------------------------------------------------------------------

/// Process that maps transient (heap) objects — identified by a `u64`
/// pointer-like key — to binders, and maintains an ordered list of "model"
/// entities (the source objects to be transferred).
///
/// In OCCT, `Transfer_TransientProcess` replaces `Transfer_FinderProcess`'s
/// finder key with a `Handle(Standard_Transient)` — an actual heap pointer.
/// The two classes share the same mapping logic; the difference is the key
/// type.  Here both use `u64` keys for simplicity, but
/// `Transfer_TransientProcess` additionally carries a model: an ordered
/// `Vec<u64>` of entity identifiers representing the entities in the source
/// data model.
// occt: Transfer_TransientProcess
pub struct Transfer_TransientProcess {
    inner: ProcessMap,
    /// Ordered list of entity identifiers in the source model.
    model: Vec<u64>,
    /// Human-readable label for diagnostics.
    label: String,
    /// Whether the process has been "activated" (model is set and ready).
    active: bool,
}

impl Transfer_TransientProcess {
    // --- Construction ---------------------------------------------------------

    /// Create an empty transient process.
    pub fn new() -> Self {
        Self {
            inner: ProcessMap::new(),
            model: Vec::new(),
            label: String::new(),
            active: false,
        }
    }

    /// Create with a diagnostic label.
    pub fn with_label(label: impl Into<String>) -> Self {
        Self {
            inner: ProcessMap::new(),
            model: Vec::new(),
            label: label.into(),
            active: false,
        }
    }

    // --- Model ----------------------------------------------------------------

    /// Set the source model as an ordered list of entity identifiers.
    /// This also activates the process.
    pub fn set_model(&mut self, entities: Vec<u64>) {
        self.model = entities;
        self.active = true;
    }

    /// Append a single entity to the model.
    pub fn add_entity(&mut self, entity_key: u64) {
        if !self.model.contains(&entity_key) {
            self.model.push(entity_key);
        }
        self.active = true;
    }

    /// Return the ordered model entity list.
    pub fn model(&self) -> &[u64] {
        &self.model
    }

    /// Number of entities in the source model.
    pub fn nb_entities(&self) -> usize {
        self.model.len()
    }

    /// `true` when the model has been set.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Rank of `entity_key` in the model (1-based, matching OCCT convention).
    /// Returns `None` when the entity is not in the model.
    pub fn entity_rank(&self, entity_key: u64) -> Option<usize> {
        self.model.iter().position(|&k| k == entity_key).map(|i| i + 1)
    }

    /// Entity key at the given 1-based rank.  Returns `None` for out-of-range
    /// ranks.
    pub fn entity_at(&self, rank: usize) -> Option<u64> {
        if rank == 0 || rank > self.model.len() {
            return None;
        }
        Some(self.model[rank - 1])
    }

    // --- Label ----------------------------------------------------------------

    /// Diagnostic label.
    pub fn label(&self) -> &str {
        &self.label
    }

    // --- Binding (same interface as FinderProcess) ----------------------------

    /// Bind `entity_key` to `binder`.
    pub fn bind(&mut self, entity_key: u64, binder: Transfer_Binder) {
        self.inner.bind(entity_key, binder);
    }

    /// Convenience: bind `entity_key` to a void binder.
    pub fn bind_void(&mut self, entity_key: u64) {
        self.inner.bind(entity_key, Transfer_Binder::void());
    }

    /// Convenience: bind `entity_key` to a typed result value.
    pub fn bind_result<T: Any>(&mut self, entity_key: u64, value: T) {
        self.inner.bind(entity_key, Transfer_Binder::with_result(value));
    }

    /// Bind `entity_key` to a failed binder with a message.
    pub fn bind_error(&mut self, entity_key: u64, message: impl Into<String>) {
        let mut b = Transfer_Binder::new();
        b.set_status_error(message);
        self.inner.bind(entity_key, b);
    }

    /// Mark `entity_key` as a root.  Returns `Err` when the key is not bound.
    pub fn set_root(&mut self, entity_key: u64) -> Result<(), TransferError> {
        if !self.inner.is_bound(entity_key) {
            return Err(TransferError::new(format!(
                "TransientProcess::set_root: key {} is not bound",
                entity_key
            )));
        }
        self.inner.add_root(entity_key);
        Ok(())
    }

    // --- Lookup ---------------------------------------------------------------

    /// Find the binder for `entity_key`.
    pub fn find(&self, entity_key: u64) -> Option<&Transfer_Binder> {
        self.inner.find(entity_key)
    }

    /// Mutable access to the binder for `entity_key`.
    pub fn find_mut(&mut self, entity_key: u64) -> Option<&mut Transfer_Binder> {
        self.inner.find_mut(entity_key)
    }

    /// `true` when `entity_key` is bound.
    pub fn is_bound(&self, entity_key: u64) -> bool {
        self.inner.is_bound(entity_key)
    }

    /// Try to retrieve a typed result for `entity_key`.
    pub fn find_result<T: Any>(&self, entity_key: u64) -> Option<&T> {
        self.inner.find(entity_key)?.result::<T>()
    }

    // --- Statistics -----------------------------------------------------------

    /// Total number of bound keys.
    pub fn nb_mapped(&self) -> usize {
        self.inner.nb_mapped()
    }

    /// Number of successful (Done) transfers.
    pub fn nb_done(&self) -> usize {
        self.inner.nb_done()
    }

    /// Number of errored transfers.
    pub fn nb_errors(&self) -> usize {
        self.inner.nb_errors()
    }

    /// Root entity keys in insertion order.
    pub fn roots(&self) -> &[u64] {
        self.inner.roots()
    }

    /// Remove all bindings, roots, and model entities; deactivates the process.
    pub fn clear(&mut self) {
        self.inner.clear();
        self.model.clear();
        self.active = false;
    }

    // --- Loop-guard helpers ---------------------------------------------------

    /// Begin the transfer of `entity_key` (sets binder to `Run`).
    /// Returns `Err` if a cycle is detected.
    pub fn start_transfer(&mut self, entity_key: u64) -> Result<(), TransferError> {
        if let Some(b) = self.inner.find(entity_key) {
            if b.status() == BinderStatus::Run {
                return Err(TransferError::new(format!(
                    "TransientProcess: cycle detected for entity {}",
                    entity_key
                )));
            }
        }
        let mut binder = Transfer_Binder::new();
        binder.set_status_run();
        self.inner.bind(entity_key, binder);
        Ok(())
    }

    /// Finish the transfer of `entity_key` with a typed result.
    pub fn finish_transfer<T: Any>(
        &mut self,
        entity_key: u64,
        value: T,
    ) -> Result<(), TransferError> {
        match self.inner.find_mut(entity_key) {
            Some(b) => {
                b.set_result(value);
                Ok(())
            }
            None => Err(TransferError::new(format!(
                "TransientProcess::finish_transfer: key {} not started",
                entity_key
            ))),
        }
    }

    /// Fail the transfer of `entity_key` with a message.
    pub fn fail_transfer(
        &mut self,
        entity_key: u64,
        message: impl Into<String>,
    ) -> Result<(), TransferError> {
        match self.inner.find_mut(entity_key) {
            Some(b) => {
                b.set_status_error(message);
                Ok(())
            }
            None => Err(TransferError::new(format!(
                "TransientProcess::fail_transfer: key {} not started",
                entity_key
            ))),
        }
    }

    /// Transfer all model entities using the provided closure.
    ///
    /// For each entity key `k` in the model, `transfer_fn(k)` is called.  If
    /// it returns `Ok(Some(value))` the binder is set to Done; `Ok(None)`
    /// produces a void binder; `Err(msg)` produces an error binder.  The
    /// entity is automatically registered as a root.
    ///
    /// Returns the number of entities successfully transferred.
    pub fn transfer_model<T: Any>(
        &mut self,
        mut transfer_fn: impl FnMut(u64) -> Result<Option<T>, String>,
    ) -> usize {
        // Snapshot the model so we can iterate without borrow issues.
        let keys: Vec<u64> = self.model.clone();
        let mut done_count = 0usize;
        for key in keys {
            match transfer_fn(key) {
                Ok(Some(v)) => {
                    self.inner.bind(key, Transfer_Binder::with_result(v));
                    self.inner.add_root(key);
                    done_count += 1;
                }
                Ok(None) => {
                    self.inner.bind(key, Transfer_Binder::void());
                    self.inner.add_root(key);
                }
                Err(msg) => {
                    let mut b = Transfer_Binder::new();
                    b.set_status_error(msg);
                    self.inner.bind(key, b);
                }
            }
        }
        done_count
    }
}

impl Default for Transfer_TransientProcess {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- BinderStatus ---------------------------------------------------------

    #[test]
    fn binder_status_display() {
        assert_eq!(BinderStatus::Void.to_string(),  "Void");
        assert_eq!(BinderStatus::Run.to_string(),   "Run");
        assert_eq!(BinderStatus::Done.to_string(),  "Done");
        assert_eq!(BinderStatus::Error.to_string(), "Error");
    }

    #[test]
    fn binder_status_default_is_void() {
        let s: BinderStatus = Default::default();
        assert_eq!(s, BinderStatus::Void);
    }

    // --- Transfer_Binder ------------------------------------------------------

    #[test]
    fn binder_new_is_void_status() {
        let b = Transfer_Binder::new();
        assert_eq!(b.status(), BinderStatus::Void);
        assert!(!b.has_result());
        assert!(!b.is_done());
        assert!(!b.has_failed());
    }

    #[test]
    fn binder_with_result_stores_typed_value() {
        let b = Transfer_Binder::with_result(42_i32);
        assert!(b.is_done());
        assert_eq!(b.result::<i32>(), Some(&42_i32));
        assert_eq!(b.result::<f64>(), None); // wrong type
    }

    #[test]
    fn binder_set_result_updates_status() {
        let mut b = Transfer_Binder::new();
        b.set_result(3.14_f64);
        assert!(b.is_done());
        assert!((b.result::<f64>().copied().unwrap() - 3.14).abs() < 1e-12);
    }

    #[test]
    fn binder_set_status_error_records_message() {
        let mut b = Transfer_Binder::new();
        b.set_status_error("conversion failed");
        assert!(b.has_failed());
        assert_eq!(b.error_message(), Some("conversion failed"));
    }

    #[test]
    fn binder_void_is_done_no_result() {
        let b = Transfer_Binder::void();
        assert!(b.is_done());
        assert!(b.is_void_binder());
        assert!(!b.has_result());
    }

    #[test]
    fn binder_chain_next() {
        let mut b = Transfer_Binder::with_result("solid".to_string());
        let next = Transfer_Binder::with_result("wire".to_string());
        b.set_next(next);
        assert!(b.has_next());
        assert_eq!(b.chain_length(), 2);
        let n = b.next().unwrap();
        assert_eq!(n.result::<String>().map(|s| s.as_str()), Some("wire"));
    }

    // --- Transfer_VoidBinder --------------------------------------------------

    #[test]
    fn void_binder_default_no_annotation() {
        let vb = Transfer_VoidBinder::new();
        assert!(vb.annotation().is_none());
        assert_eq!(vb.status(), BinderStatus::Done);
    }

    #[test]
    fn void_binder_with_annotation() {
        let vb = Transfer_VoidBinder::with_annotation("no geometry");
        assert_eq!(vb.annotation(), Some("no geometry"));
    }

    #[test]
    fn void_binder_into_binder() {
        let vb = Transfer_VoidBinder::with_annotation("intentionally empty");
        let b = vb.into_binder();
        assert!(b.is_void_binder());
        assert!(b.is_done());
    }

    // --- Transfer_FinderProcess -----------------------------------------------

    #[test]
    fn finder_process_bind_and_find() {
        let mut fp = Transfer_FinderProcess::new();
        fp.bind_result(1u64, "shape_A".to_string());
        assert!(fp.is_bound(1));
        assert_eq!(fp.nb_mapped(), 1);
        let s = fp.find_result::<String>(1).map(|s| s.as_str());
        assert_eq!(s, Some("shape_A"));
    }

    #[test]
    fn finder_process_start_finish_cycle_guard() {
        let mut fp = Transfer_FinderProcess::new();
        fp.start_transfer(10).unwrap();
        // Second start on same running key should return cycle error.
        let r = fp.start_transfer(10);
        assert!(r.is_err());
        assert!(r.unwrap_err().message.contains("cycle"));
        // Finish should resolve the binder.
        fp.finish_transfer(10u64, 99_i32).unwrap();
        assert!(fp.find(10).unwrap().is_done());
    }

    #[test]
    fn finder_process_roots() {
        let mut fp = Transfer_FinderProcess::new();
        fp.bind_result(1u64, 1_i32);
        fp.bind_result(2u64, 2_i32);
        fp.set_root(1).unwrap();
        fp.set_root(2).unwrap();
        assert_eq!(fp.roots(), &[1u64, 2u64]);
    }

    #[test]
    fn finder_process_statistics() {
        let mut fp = Transfer_FinderProcess::new();
        fp.bind_result(1u64, 1_i32);
        fp.bind_void(2u64);
        let mut b = Transfer_Binder::new();
        b.set_status_error("bad");
        fp.bind(3, b);
        assert_eq!(fp.nb_mapped(), 3);
        assert_eq!(fp.nb_done(), 2); // bind_result and bind_void are both Done
        assert_eq!(fp.nb_errors(), 1);
    }

    // --- Transfer_TransientProcess --------------------------------------------

    #[test]
    fn transient_process_model_rank() {
        let mut tp = Transfer_TransientProcess::new();
        tp.set_model(vec![10, 20, 30]);
        assert_eq!(tp.nb_entities(), 3);
        assert!(tp.is_active());
        assert_eq!(tp.entity_rank(20), Some(2));
        assert_eq!(tp.entity_at(3), Some(30));
        assert_eq!(tp.entity_rank(99), None);
        assert_eq!(tp.entity_at(0), None);
    }

    #[test]
    fn transient_process_transfer_model_callback() {
        let mut tp = Transfer_TransientProcess::new();
        tp.set_model(vec![1, 2, 3]);
        let n = tp.transfer_model(|key| {
            if key == 2 {
                Ok(None) // void
            } else {
                Ok(Some(key * 10))
            }
        });
        assert_eq!(n, 2); // keys 1 and 3 produced values
        assert!(tp.find(1).unwrap().is_done());
        assert!(tp.find(2).unwrap().is_void_binder());
        assert_eq!(tp.find_result::<u64>(3).copied(), Some(30));
        assert_eq!(tp.roots().len(), 3); // all three are roots
    }

    #[test]
    fn transient_process_clear() {
        let mut tp = Transfer_TransientProcess::new();
        tp.set_model(vec![1, 2]);
        tp.bind_result(1u64, "x");
        tp.clear();
        assert_eq!(tp.nb_mapped(), 0);
        assert_eq!(tp.nb_entities(), 0);
        assert!(!tp.is_active());
    }
}
