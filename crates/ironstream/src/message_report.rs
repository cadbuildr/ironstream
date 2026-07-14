// FILE: message_report.rs

// occt-ref: Message_Gravity
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageGravity {
    Trace = 0,
    Info = 1,
    Warning = 2,
    Alarm = 3,
    Fail = 4,
}

impl MessageGravity {
    fn label(self) -> &'static str {
        match self {
            MessageGravity::Trace => "TRACE",
            MessageGravity::Info => "INFO",
            MessageGravity::Warning => "WARNING",
            MessageGravity::Alarm => "ALARM",
            MessageGravity::Fail => "FAIL",
        }
    }
}

// occt-ref: Message_Alert
pub struct MessageAlert {
    pub gravity: MessageGravity,
    pub message: String,
    pub location: Option<String>,
}

impl MessageAlert {
    pub fn new(gravity: MessageGravity, msg: &str) -> Self {
        Self {
            gravity,
            message: msg.to_owned(),
            location: None,
        }
    }

    pub fn with_location(mut self, loc: &str) -> Self {
        self.location = Some(loc.to_owned());
        self
    }

    pub fn is_error(&self) -> bool {
        self.gravity >= MessageGravity::Alarm
    }
}

// occt: Message_Report
pub struct MessageReport {
    alerts: Vec<MessageAlert>,
    active_gravity: MessageGravity,
}

impl MessageReport {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            active_gravity: MessageGravity::Trace,
        }
    }

    pub fn add_alert(&mut self, a: MessageAlert) {
        if a.gravity >= self.active_gravity {
            self.alerts.push(a);
        }
    }

    pub fn set_min_gravity(&mut self, g: MessageGravity) {
        self.active_gravity = g;
    }

    pub fn nb_alerts(&self) -> usize {
        self.alerts.len()
    }

    pub fn nb_alerts_of(&self, g: MessageGravity) -> usize {
        self.alerts.iter().filter(|a| a.gravity == g).count()
    }

    pub fn has_alerts_of(&self, g: MessageGravity) -> bool {
        self.alerts.iter().any(|a| a.gravity == g)
    }

    pub fn has_errors(&self) -> bool {
        self.alerts.iter().any(|a| a.is_error())
    }

    pub fn has_warnings(&self) -> bool {
        self.has_alerts_of(MessageGravity::Warning)
    }

    pub fn clear(&mut self) {
        self.alerts.clear();
    }

    pub fn dump(&self) -> String {
        let mut out = String::new();
        for a in &self.alerts {
            out.push('[');
            out.push_str(a.gravity.label());
            out.push_str("] ");
            out.push_str(&a.message);
            out.push('\n');
        }
        out
    }
}

impl Default for MessageReport {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: Message_ProgressRange
pub struct MessageProgress {
    current: f64,
    total: f64,
    title: String,
    is_aborted: bool,
}

impl MessageProgress {
    pub fn new(total: f64, title: &str) -> Self {
        Self {
            current: 0.0,
            total,
            title: title.to_owned(),
            is_aborted: false,
        }
    }

    pub fn increment(&mut self, delta: f64) {
        self.current += delta;
    }

    pub fn set_current(&mut self, v: f64) {
        self.current = v;
    }

    pub fn current(&self) -> f64 {
        self.current
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn percent(&self) -> f64 {
        if self.total <= 0.0 {
            return 100.0;
        }
        (self.current / self.total * 100.0).min(100.0)
    }

    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }

    pub fn abort(&mut self) {
        self.is_aborted = true;
    }

    pub fn is_aborted(&self) -> bool {
        self.is_aborted
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_ordering() {
        assert!(MessageGravity::Trace < MessageGravity::Info);
        assert!(MessageGravity::Info < MessageGravity::Warning);
        assert!(MessageGravity::Warning < MessageGravity::Alarm);
        assert!(MessageGravity::Alarm < MessageGravity::Fail);
    }

    #[test]
    fn alert_new() {
        let a = MessageAlert::new(MessageGravity::Info, "hello");
        assert_eq!(a.gravity, MessageGravity::Info);
        assert_eq!(a.message, "hello");
        assert!(a.location.is_none());
    }

    #[test]
    fn alert_with_location() {
        let a = MessageAlert::new(MessageGravity::Warning, "w")
            .with_location("src/foo.rs:42");
        assert_eq!(a.location.as_deref(), Some("src/foo.rs:42"));
    }

    #[test]
    fn alert_is_error() {
        assert!(!MessageAlert::new(MessageGravity::Trace, "t").is_error());
        assert!(!MessageAlert::new(MessageGravity::Info, "i").is_error());
        assert!(!MessageAlert::new(MessageGravity::Warning, "w").is_error());
        assert!(MessageAlert::new(MessageGravity::Alarm, "a").is_error());
        assert!(MessageAlert::new(MessageGravity::Fail, "f").is_error());
    }

    #[test]
    fn report_empty() {
        let r = MessageReport::new();
        assert_eq!(r.nb_alerts(), 0);
        assert!(!r.has_errors());
        assert!(!r.has_warnings());
    }

    #[test]
    fn report_add_and_count() {
        let mut r = MessageReport::new();
        r.add_alert(MessageAlert::new(MessageGravity::Info, "i"));
        r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
        r.add_alert(MessageAlert::new(MessageGravity::Fail, "f"));
        assert_eq!(r.nb_alerts(), 3);
        assert_eq!(r.nb_alerts_of(MessageGravity::Info), 1);
        assert_eq!(r.nb_alerts_of(MessageGravity::Warning), 1);
        assert_eq!(r.nb_alerts_of(MessageGravity::Fail), 1);
        assert_eq!(r.nb_alerts_of(MessageGravity::Alarm), 0);
    }

    #[test]
    fn report_has_errors_and_warnings() {
        let mut r = MessageReport::new();
        r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
        assert!(r.has_warnings());
        assert!(!r.has_errors());
        r.add_alert(MessageAlert::new(MessageGravity::Alarm, "a"));
        assert!(r.has_errors());
    }

    #[test]
    fn report_min_gravity_filters() {
        let mut r = MessageReport::new();
        r.set_min_gravity(MessageGravity::Warning);
        r.add_alert(MessageAlert::new(MessageGravity::Trace, "t"));
        r.add_alert(MessageAlert::new(MessageGravity::Info, "i"));
        r.add_alert(MessageAlert::new(MessageGravity::Warning, "w"));
        r.add_alert(MessageAlert::new(MessageGravity::Fail, "f"));
        assert_eq!(r.nb_alerts(), 2);
        assert!(!r.has_alerts_of(MessageGravity::Trace));
        assert!(!r.has_alerts_of(MessageGravity::Info));
    }

    #[test]
    fn report_clear() {
        let mut r = MessageReport::new();
        r.add_alert(MessageAlert::new(MessageGravity::Info, "i"));
        r.clear();
        assert_eq!(r.nb_alerts(), 0);
    }

    #[test]
    fn report_dump_format() {
        let mut r = MessageReport::new();
        r.add_alert(MessageAlert::new(MessageGravity::Info, "msg1"));
        r.add_alert(MessageAlert::new(MessageGravity::Fail, "msg2"));
        let d = r.dump();
        assert!(d.contains("[INFO] msg1"));
        assert!(d.contains("[FAIL] msg2"));
    }

    #[test]
    fn progress_basic() {
        let p = MessageProgress::new(100.0, "task");
        assert_eq!(p.current(), 0.0);
        assert_eq!(p.total(), 100.0);
        assert_eq!(p.percent(), 0.0);
        assert!(!p.is_complete());
        assert!(!p.is_aborted());
    }

    #[test]
    fn progress_increment_to_complete() {
        let mut p = MessageProgress::new(50.0, "load");
        p.increment(25.0);
        assert_eq!(p.percent(), 50.0);
        assert!(!p.is_complete());
        p.increment(25.0);
        assert!(p.is_complete());
        assert_eq!(p.percent(), 100.0);
    }

    #[test]
    fn progress_percent_clamp() {
        let mut p = MessageProgress::new(10.0, "x");
        p.set_current(20.0);
        assert_eq!(p.percent(), 100.0);
    }

    #[test]
    fn progress_abort() {
        let mut p = MessageProgress::new(100.0, "job");
        p.abort();
        assert!(p.is_aborted());
    }
}
