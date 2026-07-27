// FILE: crates/ironstream/src/interface_check.rs
//! `interface_check` — validation message types mirroring OpenCascade's
//! `Interface_Check` and `Interface_CheckIterator` from the `TKXSBase`
//! package (`src/DataExchange/TKXSBase/Interface/`).
//!
//! # Scope
//!
//! * [`CheckMessageType`]   — severity level of a single check message.
//! * [`CheckMessage`]       — a single check message (severity + text + entity).
//! * [`InterfaceCheck`]     — a collection of messages for one entity.
//! * [`InterfaceCheckList`] — an ordered list of per-entity checks.
//!
//! # OCCT correspondence
//!
//! | OCCT class                   | IronStream type          |
//! |------------------------------|--------------------------|
//! | `Interface_CheckStatus`      | [`CheckMessageType`]     |
//! | `Interface_Check` (message)  | [`CheckMessage`]         |
//! | `Interface_Check`            | [`InterfaceCheck`]       |
//! | `Interface_CheckIterator`    | [`InterfaceCheckList`]   |

// ---------------------------------------------------------------------------
// CheckMessageType
// ---------------------------------------------------------------------------

/// Severity level of a single validation message.
// occt: Interface_CheckStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckMessageType {
    /// Purely informational — does not affect validity.
    Info,
    /// Non-fatal issue — the result may be usable but is suspect.
    Warning,
    /// Fatal issue — the entity or transfer has failed.
    Fail,
}

// ---------------------------------------------------------------------------
// CheckMessage
// ---------------------------------------------------------------------------

/// A single validation message attached to an entity.
///
/// Carries the severity ([`CheckMessageType`]), a human-readable text, and the
/// identifier of the entity that produced the message.
// occt: Interface_Check // message
#[derive(Debug, Clone)]
pub struct CheckMessage {
    msg_type:  CheckMessageType,
    text:      String,
    entity_id: usize,
}

impl CheckMessage {
    /// Create a new message.
    pub fn new(msg_type: CheckMessageType, text: &str, entity_id: usize) -> Self {
        Self {
            msg_type,
            text: text.to_owned(),
            entity_id,
        }
    }

    /// Severity of this message.
    pub fn msg_type(&self) -> CheckMessageType {
        self.msg_type
    }

    /// Human-readable message text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Identifier of the entity that produced this message.
    pub fn entity_id(&self) -> usize {
        self.entity_id
    }
}

// ---------------------------------------------------------------------------
// InterfaceCheck
// ---------------------------------------------------------------------------

/// Validation check for a single entity: an ordered list of
/// [`CheckMessage`] values (infos, warnings, and fails).
///
/// Corresponds to `Interface_Check` in OCCT, which stores messages in two
/// separate sequences (fails and warnings).  Here a single `Vec` is used with
/// the type encoded in each message, which gives the same query behaviour.
// occt: Interface_Check
#[derive(Debug, Clone)]
pub struct InterfaceCheck {
    entity_id: usize,
    messages:  Vec<CheckMessage>,
}

impl InterfaceCheck {
    /// Create an empty check for `entity_id`.
    pub fn new(entity_id: usize) -> Self {
        Self {
            entity_id,
            messages: Vec::new(),
        }
    }

    // --- Message builders ------------------------------------------------------

    /// Append a warning message.
    pub fn add_warning(&mut self, text: &str) {
        self.messages.push(CheckMessage::new(
            CheckMessageType::Warning,
            text,
            self.entity_id,
        ));
    }

    /// Append a fail message.
    pub fn add_fail(&mut self, text: &str) {
        self.messages.push(CheckMessage::new(
            CheckMessageType::Fail,
            text,
            self.entity_id,
        ));
    }

    /// Append an info message.
    pub fn add_info(&mut self, text: &str) {
        self.messages.push(CheckMessage::new(
            CheckMessageType::Info,
            text,
            self.entity_id,
        ));
    }

    // --- Queries ---------------------------------------------------------------

    /// `true` when there are no fail messages (warnings and infos are allowed).
    pub fn is_ok(&self) -> bool {
        self.messages
            .iter()
            .all(|m| m.msg_type() != CheckMessageType::Fail)
    }

    /// Total number of messages (all severities).
    pub fn nb_messages(&self) -> usize {
        self.messages.len()
    }

    /// Number of warning messages.
    pub fn nb_warnings(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| m.msg_type() == CheckMessageType::Warning)
            .count()
    }

    /// Number of fail messages.
    pub fn nb_fails(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| m.msg_type() == CheckMessageType::Fail)
            .count()
    }

    /// Return the message at index `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics when `i >= nb_messages()`.
    pub fn message(&self, i: usize) -> &CheckMessage {
        &self.messages[i]
    }

    /// Entity identifier this check belongs to.
    pub fn entity_id(&self) -> usize {
        self.entity_id
    }
}

// ---------------------------------------------------------------------------
// InterfaceCheckList
// ---------------------------------------------------------------------------

/// An ordered list of per-entity [`InterfaceCheck`] values.
///
/// Mirrors `Interface_CheckIterator` in OCCT, which iterates over the checks
/// produced by a transfer or analysis step.  Here the list owns the checks
/// directly rather than iterating over a separate model.
// occt: Interface_CheckIterator
#[derive(Debug, Clone)]
pub struct InterfaceCheckList {
    checks: Vec<InterfaceCheck>,
}

impl InterfaceCheckList {
    /// Create an empty list.
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    /// Append a check.
    pub fn add(&mut self, c: InterfaceCheck) {
        self.checks.push(c);
    }

    /// Total number of checks in the list.
    pub fn nb_checks(&self) -> usize {
        self.checks.len()
    }

    /// Return the check at index `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics when `i >= nb_checks()`.
    pub fn check(&self, i: usize) -> &InterfaceCheck {
        &self.checks[i]
    }

    /// Total number of fail messages across all checks.
    pub fn nb_fails_total(&self) -> usize {
        self.checks.iter().map(|c| c.nb_fails()).sum()
    }

    /// Total number of warning messages across all checks.
    pub fn nb_warnings_total(&self) -> usize {
        self.checks.iter().map(|c| c.nb_warnings()).sum()
    }

    /// `true` when any check contains at least one fail message.
    pub fn has_fails(&self) -> bool {
        self.checks.iter().any(|c| c.nb_fails() > 0)
    }
}

impl Default for InterfaceCheckList {
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

    // --- CheckMessageType ------------------------------------------------------

    #[test]
    fn check_message_type_eq() {
        assert_eq!(CheckMessageType::Info,    CheckMessageType::Info);
        assert_eq!(CheckMessageType::Warning, CheckMessageType::Warning);
        assert_eq!(CheckMessageType::Fail,    CheckMessageType::Fail);
        assert_ne!(CheckMessageType::Info,    CheckMessageType::Fail);
    }

    #[test]
    fn check_message_type_copy() {
        let t = CheckMessageType::Warning;
        let t2 = t; // Copy
        assert_eq!(t, t2);
    }

    // --- CheckMessage ----------------------------------------------------------

    #[test]
    fn check_message_new_fields() {
        let m = CheckMessage::new(CheckMessageType::Fail, "bad edge", 7);
        assert_eq!(m.msg_type(), CheckMessageType::Fail);
        assert_eq!(m.text(), "bad edge");
        assert_eq!(m.entity_id(), 7);
    }

    #[test]
    fn check_message_clone() {
        let m = CheckMessage::new(CheckMessageType::Info, "note", 1);
        let m2 = m.clone();
        assert_eq!(m2.text(), "note");
        assert_eq!(m2.entity_id(), 1);
    }

    // --- InterfaceCheck --------------------------------------------------------

    #[test]
    fn interface_check_empty_is_ok() {
        let c = InterfaceCheck::new(0);
        assert!(c.is_ok());
        assert_eq!(c.nb_messages(), 0);
        assert_eq!(c.nb_warnings(), 0);
        assert_eq!(c.nb_fails(), 0);
    }

    #[test]
    fn interface_check_add_warning() {
        let mut c = InterfaceCheck::new(3);
        c.add_warning("slightly off");
        assert!(c.is_ok()); // warnings do not make is_ok() false
        assert_eq!(c.nb_warnings(), 1);
        assert_eq!(c.nb_fails(), 0);
        assert_eq!(c.nb_messages(), 1);
        assert_eq!(c.message(0).msg_type(), CheckMessageType::Warning);
        assert_eq!(c.message(0).text(), "slightly off");
        assert_eq!(c.message(0).entity_id(), 3);
    }

    #[test]
    fn interface_check_add_fail() {
        let mut c = InterfaceCheck::new(5);
        c.add_fail("missing face");
        assert!(!c.is_ok());
        assert_eq!(c.nb_fails(), 1);
        assert_eq!(c.nb_warnings(), 0);
    }

    #[test]
    fn interface_check_add_info() {
        let mut c = InterfaceCheck::new(2);
        c.add_info("processed");
        assert!(c.is_ok());
        assert_eq!(c.nb_messages(), 1);
        assert_eq!(c.message(0).msg_type(), CheckMessageType::Info);
    }

    #[test]
    fn interface_check_mixed_messages() {
        let mut c = InterfaceCheck::new(10);
        c.add_info("start");
        c.add_warning("approximate");
        c.add_fail("topology error");
        c.add_warning("degenerate edge");
        assert!(!c.is_ok());
        assert_eq!(c.nb_messages(), 4);
        assert_eq!(c.nb_fails(), 1);
        assert_eq!(c.nb_warnings(), 2);
    }

    #[test]
    fn interface_check_message_order_preserved() {
        let mut c = InterfaceCheck::new(1);
        c.add_fail("first");
        c.add_warning("second");
        c.add_info("third");
        assert_eq!(c.message(0).text(), "first");
        assert_eq!(c.message(1).text(), "second");
        assert_eq!(c.message(2).text(), "third");
    }

    #[test]
    fn interface_check_entity_id() {
        let c = InterfaceCheck::new(42);
        assert_eq!(c.entity_id(), 42);
    }

    // --- InterfaceCheckList ----------------------------------------------------

    #[test]
    fn interface_check_list_empty() {
        let list = InterfaceCheckList::new();
        assert_eq!(list.nb_checks(), 0);
        assert_eq!(list.nb_fails_total(), 0);
        assert_eq!(list.nb_warnings_total(), 0);
        assert!(!list.has_fails());
    }

    #[test]
    fn interface_check_list_add_and_access() {
        let mut list = InterfaceCheckList::new();
        let mut c1 = InterfaceCheck::new(1);
        c1.add_warning("w1");
        list.add(c1);

        let mut c2 = InterfaceCheck::new(2);
        c2.add_fail("f1");
        c2.add_fail("f2");
        list.add(c2);

        assert_eq!(list.nb_checks(), 2);
        assert_eq!(list.check(0).entity_id(), 1);
        assert_eq!(list.check(1).entity_id(), 2);
    }

    #[test]
    fn interface_check_list_nb_fails_total() {
        let mut list = InterfaceCheckList::new();
        let mut c1 = InterfaceCheck::new(1);
        c1.add_fail("a");
        c1.add_fail("b");
        list.add(c1);

        let mut c2 = InterfaceCheck::new(2);
        c2.add_fail("c");
        list.add(c2);

        assert_eq!(list.nb_fails_total(), 3);
    }

    #[test]
    fn interface_check_list_nb_warnings_total() {
        let mut list = InterfaceCheckList::new();
        let mut c1 = InterfaceCheck::new(1);
        c1.add_warning("w1");
        c1.add_warning("w2");
        list.add(c1);

        let mut c2 = InterfaceCheck::new(2);
        c2.add_warning("w3");
        list.add(c2);

        assert_eq!(list.nb_warnings_total(), 3);
    }

    #[test]
    fn interface_check_list_has_fails_true_when_any_fail() {
        let mut list = InterfaceCheckList::new();
        let mut c = InterfaceCheck::new(1);
        c.add_warning("harmless");
        list.add(c);
        assert!(!list.has_fails());

        let mut c2 = InterfaceCheck::new(2);
        c2.add_fail("bad");
        list.add(c2);
        assert!(list.has_fails());
    }

    #[test]
    fn interface_check_list_default_is_empty() {
        let list: InterfaceCheckList = Default::default();
        assert_eq!(list.nb_checks(), 0);
    }

    #[test]
    fn interface_check_list_all_ok_checks_no_fails() {
        let mut list = InterfaceCheckList::new();
        let mut c = InterfaceCheck::new(7);
        c.add_info("fine");
        c.add_warning("minor");
        list.add(c);
        assert!(!list.has_fails());
        assert_eq!(list.nb_fails_total(), 0);
        assert_eq!(list.nb_warnings_total(), 1);
    }
}
