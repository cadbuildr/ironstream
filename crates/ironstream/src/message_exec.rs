// FILE: message_exec.rs
// occt: Message_ExecStatus, Message_ProgressScope, Message_ProgressRange

/// Bit-field execution status — mirrors OCCT's Message_ExecStatus.
/// occt: Message_ExecStatus
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageExecStatus(pub u32);

impl MessageExecStatus {
    pub const DONE: u32 = 1 << 0;
    pub const FAIL: u32 = 1 << 1;
    pub const WARN: u32 = 1 << 2;
    pub const ALARM: u32 = 1 << 3;

    pub fn new() -> Self { Self(0) }

    pub fn set_done(&mut self) { self.0 |= Self::DONE; }
    pub fn set_fail(&mut self) { self.0 |= Self::FAIL; }
    pub fn set_warn(&mut self) { self.0 |= Self::WARN; }
    pub fn set_alarm(&mut self) { self.0 |= Self::ALARM; }

    pub fn clear_done(&mut self) { self.0 &= !Self::DONE; }
    pub fn clear_fail(&mut self) { self.0 &= !Self::FAIL; }

    pub fn is_done(&self) -> bool { self.0 & Self::DONE != 0 }
    pub fn is_fail(&self) -> bool { self.0 & Self::FAIL != 0 }
    pub fn has_warnings(&self) -> bool { self.0 & Self::WARN != 0 }
    pub fn has_alarms(&self) -> bool { self.0 & Self::ALARM != 0 }
    pub fn is_ok(&self) -> bool { self.is_done() && !self.is_fail() }

    pub fn add(&mut self, other: MessageExecStatus) { self.0 |= other.0; }
    pub fn bits(&self) -> u32 { self.0 }
}

/// Progress range for a single scope within the progress indicator.
/// occt: Message_ProgressRange
#[derive(Clone, Copy, Debug)]
pub struct MessageProgressRange {
    pub min: f64,
    pub max: f64,
    pub current: f64,
}

impl Default for MessageProgressRange {
    fn default() -> Self { Self { min: 0.0, max: 100.0, current: 0.0 } }
}

impl MessageProgressRange {
    pub fn new(min: f64, max: f64) -> Self {
        let max = if max > min { max } else { min + 1.0 };
        Self { min, max, current: min }
    }

    pub fn advance(&mut self, step: f64) {
        self.current = (self.current + step).min(self.max);
    }

    pub fn fraction(&self) -> f64 {
        if (self.max - self.min).abs() < 1e-14 { return 1.0; }
        ((self.current - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    pub fn is_complete(&self) -> bool { self.current >= self.max - 1e-14 }

    pub fn percent(&self) -> f64 { self.fraction() * 100.0 }
}

/// Named scope within a hierarchical progress indicator.
/// occt: Message_ProgressScope
#[derive(Clone, Debug)]
pub struct MessageProgressScope {
    pub name: String,
    pub range: MessageProgressRange,
    pub nb_steps: usize,
    pub step: usize,
}

impl MessageProgressScope {
    pub fn new(name: &str, nb_steps: usize) -> Self {
        let nb_steps = nb_steps.max(1);
        Self {
            name: name.to_string(),
            range: MessageProgressRange::new(0.0, nb_steps as f64),
            nb_steps,
            step: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.step < self.nb_steps {
            self.step += 1;
            self.range.advance(1.0);
            true
        } else {
            false
        }
    }

    pub fn is_complete(&self) -> bool { self.step >= self.nb_steps }
    pub fn fraction(&self) -> f64 { self.range.fraction() }
    pub fn percent(&self) -> f64 { self.range.percent() }
    pub fn steps_done(&self) -> usize { self.step }
    pub fn steps_left(&self) -> usize { self.nb_steps.saturating_sub(self.step) }
}

/// Composite progress indicator aggregating multiple scopes.
/// occt: Message_ProgressIndicator (simplified)
#[derive(Clone, Debug, Default)]
pub struct MessageProgressIndicator {
    pub scopes: Vec<MessageProgressScope>,
}

impl MessageProgressIndicator {
    pub fn new() -> Self { Self::default() }

    pub fn add_scope(&mut self, name: &str, nb_steps: usize) -> usize {
        let idx = self.scopes.len();
        self.scopes.push(MessageProgressScope::new(name, nb_steps));
        idx
    }

    pub fn next_step(&mut self, scope_idx: usize) -> bool {
        self.scopes.get_mut(scope_idx).map(|s| s.next()).unwrap_or(false)
    }

    pub fn total_fraction(&self) -> f64 {
        if self.scopes.is_empty() { return 0.0; }
        let sum: f64 = self.scopes.iter().map(|s| s.fraction()).sum();
        sum / self.scopes.len() as f64
    }

    pub fn is_complete(&self) -> bool { self.scopes.iter().all(|s| s.is_complete()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_status_bits() {
        let mut s = MessageExecStatus::new();
        s.set_done();
        assert!(s.is_done());
        assert!(!s.is_fail());
        assert!(s.is_ok());
        s.set_fail();
        assert!(!s.is_ok());
    }

    #[test]
    fn exec_status_add() {
        let mut a = MessageExecStatus::new();
        a.set_done();
        let mut b = MessageExecStatus::new();
        b.set_warn();
        a.add(b);
        assert!(a.is_done());
        assert!(a.has_warnings());
    }

    #[test]
    fn progress_range_fraction() {
        let mut r = MessageProgressRange::new(0.0, 10.0);
        r.advance(5.0);
        assert!((r.fraction() - 0.5).abs() < 1e-10);
        assert!(!r.is_complete());
        r.advance(5.0);
        assert!(r.is_complete());
    }

    #[test]
    fn progress_scope_steps() {
        let mut s = MessageProgressScope::new("test", 4);
        assert_eq!(s.steps_left(), 4);
        s.next();
        s.next();
        assert_eq!(s.steps_done(), 2);
        assert!((s.fraction() - 0.5).abs() < 1e-10);
        assert!(!s.is_complete());
    }

    #[test]
    fn indicator_total_fraction() {
        let mut ind = MessageProgressIndicator::new();
        let i = ind.add_scope("load", 2);
        ind.next_step(i);
        assert!((ind.total_fraction() - 0.5).abs() < 1e-10);
        ind.next_step(i);
        assert!(ind.is_complete());
    }
}
