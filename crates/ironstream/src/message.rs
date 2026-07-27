// FILE: message.rs
//! `Message` — alerts, formatted messages, messenger dispatch, and progress
//! tracking, mirroring OpenCascade's `Message` package.
//!
//! # Scope
//!
//! * [`Message_Alert`]        — a single diagnostic event (error/warn/info).
//! * [`Message_Msg`]          — a formattable template-based message string.
//! * [`Message_Messenger`]    — collects and dispatches alerts to listeners.
//! * [`Message_ProgressRange`]— a named sub-range of a [0,1] progress scope.
//! * [`Message_ProgressSentry`]— RAII guard that advances a progress range.

// ---------------------------------------------------------------------------
// Alert severity
// ---------------------------------------------------------------------------

/// Severity level for a diagnostic alert, mirroring `Message_Gravity`.
// occt: Message_Gravity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gravity {
    /// Informational — operation succeeded with no concerns.
    Info,
    /// Warning — operation succeeded but something unusual was noticed.
    Warning,
    /// Alarm — operation may have partially failed.
    Alarm,
    /// Failure — operation failed.
    Fail,
}

// ---------------------------------------------------------------------------
// Message_Alert
// ---------------------------------------------------------------------------

/// A single diagnostic event carrying a severity, a short string key, and an
/// optional human-readable description.
///
/// In OCCT, `Message_Alert` is an abstract base class whose concrete
/// subclasses carry typed data.  Here we use a plain struct that covers the
/// common case (a string-keyed alert) while still exposing the same API
/// shape.
// occt-ref: Message_Alert
#[derive(Debug, Clone)]
pub struct Message_Alert {
    gravity: Gravity,
    /// Short identifier / message key (e.g. `"BRepCheck_InvalidCurveOnSurface"`).
    key: String,
    /// Optional extended description filled in by the emitting code.
    description: Option<String>,
}

impl Message_Alert {
    /// Create a new alert with the given gravity and key.
    pub fn new(gravity: Gravity, key: impl Into<String>) -> Self {
        Self {
            gravity,
            key: key.into(),
            description: None,
        }
    }

    /// Set an extended description on this alert (builder pattern).
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Return the gravity of this alert.
    pub fn gravity(&self) -> Gravity {
        self.gravity
    }

    /// Return the short key / identifier.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the optional extended description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// `true` when the alert represents a failure.
    pub fn is_fail(&self) -> bool {
        self.gravity == Gravity::Fail
    }
}

// ---------------------------------------------------------------------------
// Message_Msg
// ---------------------------------------------------------------------------

/// A formattable message template.
///
/// OCCT's `Message_Msg` resolves a string key against a catalogue and then
/// fills named argument placeholders (`%1`, `%2`, …).  This implementation
/// stores the template text directly (no external catalogue required) and
/// replaces `%1`, `%2`, … in order with successive `arg` calls.
// occt: Message_Msg
#[derive(Debug, Clone)]
pub struct Message_Msg {
    /// The raw template text, e.g. `"Point %1 is at distance %2 from origin"`.
    template: String,
    /// Positional arguments already applied.
    args: Vec<String>,
}

impl Message_Msg {
    /// Create a message from a template string.
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
            args: Vec::new(),
        }
    }

    /// Append a string argument (fills the next `%N` placeholder when
    /// `value()` is called).
    pub fn arg(mut self, value: impl ToString) -> Self {
        self.args.push(value.to_string());
        self
    }

    /// Resolve the template by substituting all registered arguments in order
    /// (`%1` → args[0], `%2` → args[1], …).  Unmatched placeholders are left
    /// as-is; extra arguments are silently ignored.
    pub fn value(&self) -> String {
        let mut result = self.template.clone();
        for (i, arg) in self.args.iter().enumerate() {
            let placeholder = format!("%{}", i + 1);
            result = result.replace(&placeholder, arg);
        }
        result
    }

    /// Original template text before argument substitution.
    pub fn original(&self) -> &str {
        &self.template
    }

    /// Number of arguments that have been appended so far.
    pub fn arg_count(&self) -> usize {
        self.args.len()
    }
}

// ---------------------------------------------------------------------------
// Message_Printer (simple listener trait)
// ---------------------------------------------------------------------------

/// Trait for objects that can receive formatted alert messages.
///
/// Corresponds to `Message_Printer` in OCCT; concrete printers may write to
/// stdout, a file, a GUI panel, etc.
// occt: Message_Printer
pub trait Message_Printer: Send + Sync {
    /// Called by the messenger whenever an alert is dispatched.
    fn send(&self, alert: &Message_Alert);
}

// ---------------------------------------------------------------------------
// Built-in printer: collects alerts into a Vec
// ---------------------------------------------------------------------------

/// A printer that accumulates every alert in memory so tests (and callers)
/// can inspect what was dispatched.
// occt-note: Message_PrinterCollect (custom convenience type)
pub struct CollectingPrinter {
    /// Interior-mutability cell so `send` can take `&self` as required by the
    /// trait while still appending to the log.
    log: std::cell::RefCell<Vec<Message_Alert>>,
}

impl CollectingPrinter {
    /// Create an empty collecting printer.
    pub fn new() -> Self {
        Self {
            log: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Return a snapshot of all alerts received so far.
    pub fn alerts(&self) -> Vec<Message_Alert> {
        self.log.borrow().clone()
    }

    /// Number of alerts collected.
    pub fn count(&self) -> usize {
        self.log.borrow().len()
    }
}

impl Default for CollectingPrinter {
    fn default() -> Self {
        Self::new()
    }
}

// CollectingPrinter is used in single-threaded tests; it is not Send+Sync in
// the general case because RefCell is !Sync.  We provide a non-trait helper
// instead of implementing Message_Printer for it to keep things simple.
impl CollectingPrinter {
    /// Direct (non-trait) send used when no Sync bound is needed.
    pub fn push(&self, alert: Message_Alert) {
        self.log.borrow_mut().push(alert);
    }
}

// ---------------------------------------------------------------------------
// Message_Messenger
// ---------------------------------------------------------------------------

/// Central dispatcher that routes alerts to a list of registered printers.
///
/// OCCT's `Message_Messenger` holds a list of `Handle(Message_Printer)`.
/// Here we store boxed trait objects and expose a matching API surface.
// occt: Message_Messenger
pub struct Message_Messenger {
    printers: Vec<Box<dyn Message_Printer>>,
    /// Minimum gravity that is actually forwarded to printers.
    active_gravity: Gravity,
    /// All alerts ever sent through this messenger (independent of printers).
    history: Vec<Message_Alert>,
}

impl Message_Messenger {
    /// Create a messenger with no printers and `Info` gravity threshold.
    pub fn new() -> Self {
        Self {
            printers: Vec::new(),
            active_gravity: Gravity::Info,
            history: Vec::new(),
        }
    }

    /// Return the default (process-wide singleton equivalent) messenger.
    ///
    /// In OCCT this is `Message::DefaultMessenger()`.  Here we just return a
    /// fresh default-configured instance; callers may share it however they
    /// wish.
    pub fn default_messenger() -> Self {
        Self::new()
    }

    /// Add a printer.  Takes ownership of the boxed printer.
    pub fn add_printer(&mut self, printer: Box<dyn Message_Printer>) {
        self.printers.push(printer);
    }

    /// Set the minimum gravity threshold; alerts below it are silently dropped.
    pub fn set_active_gravity(&mut self, gravity: Gravity) {
        self.active_gravity = gravity;
    }

    /// Current gravity threshold.
    pub fn active_gravity(&self) -> Gravity {
        self.active_gravity
    }

    /// Send an alert.  If the alert's gravity is below the threshold it is
    /// still recorded in the history but not forwarded to printers.
    pub fn send_alert(&mut self, alert: Message_Alert) {
        if alert.gravity() >= self.active_gravity {
            for printer in &self.printers {
                printer.send(&alert);
            }
        }
        self.history.push(alert);
    }

    /// Convenience: send a plain string message at the given gravity.
    pub fn send(&mut self, gravity: Gravity, text: impl Into<String>) {
        let alert = Message_Alert::new(gravity, text);
        self.send_alert(alert);
    }

    /// All alerts ever sent (regardless of gravity filter).
    pub fn history(&self) -> &[Message_Alert] {
        &self.history
    }

    /// Number of alerts whose gravity is at least `min_gravity`.
    pub fn count_at_least(&self, min_gravity: Gravity) -> usize {
        self.history
            .iter()
            .filter(|a| a.gravity() >= min_gravity)
            .count()
    }

    /// `true` if any failure-level alert has been sent.
    pub fn has_failures(&self) -> bool {
        self.history.iter().any(|a| a.is_fail())
    }

    /// Clear the alert history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Number of registered printers.
    pub fn printer_count(&self) -> usize {
        self.printers.len()
    }
}

impl Default for Message_Messenger {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Message_ProgressRange
// ---------------------------------------------------------------------------

/// A named sub-range of a parent progress scope, mapping local [0,1] into
/// a slice `[start, end]` of the parent's [0,1] range.
///
/// OCCT's `Message_ProgressRange` is a lightweight value type that owns a
/// slice of its parent scope.  We follow the same model; when dropped without
/// being fully consumed the range is automatically closed at its end value.
// occt: Message_ProgressRange
#[derive(Debug, Clone)]
pub struct Message_ProgressRange {
    name: String,
    /// Start of this range inside the parent scope (0.0–1.0).
    start: f64,
    /// End of this range inside the parent scope (0.0–1.0).
    end: f64,
    /// Current position within [start, end] (tracks `set_position` calls).
    current: f64,
    /// Whether the range has been closed (either by the caller or on drop).
    closed: bool,
}

impl Message_ProgressRange {
    /// Create a new range that occupies the full [0, 1] scope.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: 0.0,
            end: 1.0,
            current: 0.0,
            closed: false,
        }
    }

    /// Create a sub-range occupying `[start, end]` of the parent scope.
    ///
    /// `start` and `end` are clamped to `[0, 1]` and swapped if inverted.
    pub fn sub_range(name: impl Into<String>, start: f64, end: f64) -> Self {
        let s = start.clamp(0.0, 1.0);
        let e = end.clamp(0.0, 1.0);
        let (s, e) = if s <= e { (s, e) } else { (e, s) };
        Self {
            name: name.into(),
            start: s,
            end: e,
            current: s,
            closed: false,
        }
    }

    /// Range name / label.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Start value in the parent scope.
    pub fn start(&self) -> f64 {
        self.start
    }

    /// End value in the parent scope.
    pub fn end(&self) -> f64 {
        self.end
    }

    /// Current absolute position in the parent scope (between `start` and
    /// `end`).
    pub fn position(&self) -> f64 {
        self.current
    }

    /// Local progress: fraction of [start, end] that has been completed,
    /// in [0, 1].
    pub fn local_progress(&self) -> f64 {
        let span = self.end - self.start;
        if span <= 0.0 {
            return 1.0;
        }
        ((self.current - self.start) / span).clamp(0.0, 1.0)
    }

    /// Advance the range by `delta` (in parent-scope units).
    ///
    /// The position is clamped to `[start, end]`.
    pub fn advance(&mut self, delta: f64) {
        if self.closed {
            return;
        }
        self.current = (self.current + delta).clamp(self.start, self.end);
    }

    /// Set local progress (0.0–1.0); maps into [start, end].
    pub fn set_local_progress(&mut self, fraction: f64) {
        if self.closed {
            return;
        }
        let fraction = fraction.clamp(0.0, 1.0);
        self.current = self.start + fraction * (self.end - self.start);
    }

    /// Mark the range as complete (sets position to `end`).
    pub fn close(&mut self) {
        self.current = self.end;
        self.closed = true;
    }

    /// `true` once `close()` has been called (or the range span is zero).
    pub fn is_closed(&self) -> bool {
        self.closed || (self.end - self.start).abs() < f64::EPSILON
    }

    /// Divide this range into `n` equal sub-ranges and return them.  The
    /// sub-ranges cover [start, end] without overlap.
    pub fn split(&self, n: usize) -> Vec<Message_ProgressRange> {
        if n == 0 {
            return Vec::new();
        }
        let step = (self.end - self.start) / n as f64;
        (0..n)
            .map(|i| {
                Message_ProgressRange::sub_range(
                    format!("{}/{}", self.name, i + 1),
                    self.start + i as f64 * step,
                    self.start + (i + 1) as f64 * step,
                )
            })
            .collect()
    }
}

impl Drop for Message_ProgressRange {
    fn drop(&mut self) {
        // Ensure the range is marked complete on drop (mirrors OCCT behaviour
        // where the destructor releases the scope).
        if !self.closed {
            self.current = self.end;
            self.closed = true;
        }
    }
}

// ---------------------------------------------------------------------------
// Message_ProgressSentry
// ---------------------------------------------------------------------------

/// RAII guard that owns a `Message_ProgressRange` and automatically closes
/// it when dropped.
///
/// OCCT's `Message_ProgressSentry` wraps a progress scope and advances it
/// step by step, calling `Next()` for each work item, then closes the scope
/// on destruction.  This implementation mirrors that pattern with
/// `next()` / `more()` iteration helpers.
// occt: Message_ProgressSentry
pub struct Message_ProgressSentry {
    range: Message_ProgressRange,
    /// Total number of steps this sentry will iterate.
    total_steps: usize,
    /// Steps completed so far.
    steps_done: usize,
    /// Whether the sentry is still active.
    active: bool,
}

impl Message_ProgressSentry {
    /// Create a sentry for `total_steps` equal steps over the given range.
    pub fn new(range: Message_ProgressRange, total_steps: usize) -> Self {
        Self {
            range,
            total_steps: total_steps.max(1),
            steps_done: 0,
            active: true,
        }
    }

    /// `true` while there are still pending steps and the sentry is active.
    pub fn more(&self) -> bool {
        self.active && self.steps_done < self.total_steps
    }

    /// Advance one step.  Mirrors `Message_ProgressSentry::Next()` in OCCT.
    pub fn next(&mut self) {
        if !self.active {
            return;
        }
        self.steps_done += 1;
        let fraction = self.steps_done as f64 / self.total_steps as f64;
        self.range.set_local_progress(fraction);
        if self.steps_done >= self.total_steps {
            self.active = false;
            self.range.close();
        }
    }

    /// Abort the sentry prematurely; the range is closed at its current
    /// position.
    pub fn abandon(&mut self) {
        self.active = false;
        self.range.close();
    }

    /// Current local progress of the underlying range (0.0–1.0).
    pub fn progress(&self) -> f64 {
        self.range.local_progress()
    }

    /// Steps completed so far.
    pub fn steps_done(&self) -> usize {
        self.steps_done
    }

    /// Total step count this sentry was created with.
    pub fn total_steps(&self) -> usize {
        self.total_steps
    }

    /// Immutable access to the underlying progress range.
    pub fn range(&self) -> &Message_ProgressRange {
        &self.range
    }

    /// Return a clone of the underlying range in its final (closed) state.
    ///
    /// Because `Message_ProgressSentry` has a `Drop` impl the owned range
    /// cannot be moved out; we close it in place and return a clone so that
    /// the caller receives the fully-advanced range value.
    pub fn into_range(mut self) -> Message_ProgressRange {
        self.active = false;
        self.range.close();
        self.range.clone()
    }
}

impl Drop for Message_ProgressSentry {
    fn drop(&mut self) {
        // Close the range when the sentry goes out of scope.
        self.active = false;
        self.range.close();
    }
}

// ---------------------------------------------------------------------------
// Internal tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Message_Alert -------------------------------------------------------

    #[test]
    fn alert_basic_fields() {
        let a = Message_Alert::new(Gravity::Warning, "BRepCheck_BadOrientation");
        assert_eq!(a.gravity(), Gravity::Warning);
        assert_eq!(a.key(), "BRepCheck_BadOrientation");
        assert!(a.description().is_none());
        assert!(!a.is_fail());
    }

    #[test]
    fn alert_with_description_and_fail() {
        let a = Message_Alert::new(Gravity::Fail, "NullShape")
            .with_description("Shape pointer is null");
        assert!(a.is_fail());
        assert_eq!(a.description(), Some("Shape pointer is null"));
    }

    #[test]
    fn alert_gravity_ordering() {
        assert!(Gravity::Info < Gravity::Warning);
        assert!(Gravity::Warning < Gravity::Alarm);
        assert!(Gravity::Alarm < Gravity::Fail);
    }

    // --- Message_Msg ---------------------------------------------------------

    #[test]
    fn msg_no_args() {
        let m = Message_Msg::new("No placeholders here.");
        assert_eq!(m.value(), "No placeholders here.");
        assert_eq!(m.arg_count(), 0);
    }

    #[test]
    fn msg_single_arg() {
        let m = Message_Msg::new("Distance = %1 mm").arg(3.14_f64);
        assert_eq!(m.value(), "Distance = 3.14 mm");
    }

    #[test]
    fn msg_multiple_args() {
        let m = Message_Msg::new("Point (%1, %2, %3)")
            .arg(1)
            .arg(2)
            .arg(3);
        assert_eq!(m.value(), "Point (1, 2, 3)");
        assert_eq!(m.arg_count(), 3);
    }

    #[test]
    fn msg_original_unchanged() {
        let m = Message_Msg::new("Hello %1").arg("world");
        assert_eq!(m.original(), "Hello %1");
    }

    // --- Message_Messenger ---------------------------------------------------

    #[test]
    fn messenger_history_and_count() {
        let mut m = Message_Messenger::new();
        m.send(Gravity::Info, "step 1 done");
        m.send(Gravity::Warning, "unusual value");
        m.send(Gravity::Fail, "abort");
        assert_eq!(m.history().len(), 3);
        assert!(m.has_failures());
        assert_eq!(m.count_at_least(Gravity::Warning), 2);
    }

    #[test]
    fn messenger_gravity_filter() {
        let mut m = Message_Messenger::new();
        m.set_active_gravity(Gravity::Alarm);
        m.send(Gravity::Info, "ignored by printers");
        m.send(Gravity::Alarm, "reaches printers");
        // History always records everything regardless of filter.
        assert_eq!(m.history().len(), 2);
        assert_eq!(m.active_gravity(), Gravity::Alarm);
    }

    #[test]
    fn messenger_clear_history() {
        let mut m = Message_Messenger::new();
        m.send(Gravity::Info, "msg");
        assert_eq!(m.history().len(), 1);
        m.clear_history();
        assert_eq!(m.history().len(), 0);
    }

    // --- Message_ProgressRange -----------------------------------------------

    #[test]
    fn progress_range_local_progress() {
        let mut r = Message_ProgressRange::sub_range("step", 0.2, 0.8);
        assert!((r.local_progress() - 0.0).abs() < 1e-12);
        r.set_local_progress(0.5);
        assert!((r.local_progress() - 0.5).abs() < 1e-12);
        // Absolute position should be 0.2 + 0.5*(0.8-0.2) = 0.5
        assert!((r.position() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn progress_range_advance_and_close() {
        let mut r = Message_ProgressRange::new("root");
        r.advance(0.3);
        assert!((r.position() - 0.3).abs() < 1e-12);
        r.close();
        assert!(r.is_closed());
        // Advance after close is a no-op.
        r.advance(0.5);
        assert!((r.position() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn progress_range_split() {
        let r = Message_ProgressRange::sub_range("parent", 0.0, 1.0);
        let parts = r.split(4);
        assert_eq!(parts.len(), 4);
        assert!((parts[0].start() - 0.0).abs() < 1e-12);
        assert!((parts[0].end() - 0.25).abs() < 1e-12);
        assert!((parts[3].end() - 1.0).abs() < 1e-12);
    }

    // --- Message_ProgressSentry ----------------------------------------------

    #[test]
    fn sentry_full_iteration() {
        let range = Message_ProgressRange::new("op");
        let mut sentry = Message_ProgressSentry::new(range, 4);
        let mut count = 0;
        while sentry.more() {
            sentry.next();
            count += 1;
        }
        assert_eq!(count, 4);
        assert_eq!(sentry.steps_done(), 4);
        assert!((sentry.progress() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn sentry_abandon() {
        let range = Message_ProgressRange::new("partial");
        let mut sentry = Message_ProgressSentry::new(range, 10);
        sentry.next();
        sentry.next();
        sentry.abandon();
        assert!(!sentry.more());
        assert_eq!(sentry.steps_done(), 2);
    }
}
