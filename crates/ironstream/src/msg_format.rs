// FILE: msg_format.rs
// occt-ref: Message_Formatter, Message_Printer, Message_Report, Message_Alert

/// Severity level for a message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageGravity {
    Trace,
    Info,
    Warning,
    Alarm,
    Fail,
}

impl Default for MessageGravity {
    fn default() -> Self { Self::Info }
}

impl MessageGravity {
    pub fn is_fail(&self) -> bool { *self >= MessageGravity::Alarm }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Alarm => "ALARM",
            Self::Fail => "FAIL",
        }
    }
}

// occt-ref: Message_Alert // — a single diagnostic message entry
#[derive(Clone, Debug)]
pub struct MessageAlert {
    pub gravity: MessageGravity,
    pub text: String,
    pub arguments: Vec<String>,
}

impl MessageAlert {
    pub fn new(gravity: MessageGravity, text: &str) -> Self {
        Self { gravity, text: text.to_owned(), arguments: Vec::new() }
    }

    pub fn with_arg(mut self, arg: &str) -> Self {
        self.arguments.push(arg.to_owned());
        self
    }

    pub fn formatted(&self) -> String {
        let mut s = self.text.clone();
        for (i, arg) in self.arguments.iter().enumerate() {
            s = s.replace(&format!("%{}", i + 1), arg);
        }
        s
    }

    pub fn is_fail(&self) -> bool { self.gravity.is_fail() }
    pub fn gravity(&self) -> MessageGravity { self.gravity }
}

// occt-note: Message_Formatter — builds a formatted message from a template + arguments
#[derive(Clone, Debug, Default)]
pub struct MessageFormatter {
    pub template: String,
    pub arguments: Vec<String>,
}

impl MessageFormatter {
    pub fn new(template: &str) -> Self {
        Self { template: template.to_owned(), arguments: Vec::new() }
    }

    pub fn arg(mut self, s: &str) -> Self {
        self.arguments.push(s.to_owned());
        self
    }

    pub fn arg_f64(mut self, v: f64) -> Self {
        self.arguments.push(format!("{}", v));
        self
    }

    pub fn arg_i64(mut self, v: i64) -> Self {
        self.arguments.push(format!("{}", v));
        self
    }

    pub fn format(&self) -> String {
        let mut s = self.template.clone();
        for (i, arg) in self.arguments.iter().enumerate() {
            s = s.replace(&format!("%{}", i + 1), arg);
        }
        s
    }

    pub fn is_empty(&self) -> bool { self.template.is_empty() }
}

// occt-ref: Message_Printer // — outputs messages (stub: collects in a buffer)
#[derive(Clone, Debug)]
pub struct MessagePrinter {
    pub gravity_threshold: MessageGravity,
    pub buffer: Vec<String>,
    pub prefix: String,
}

impl Default for MessagePrinter {
    fn default() -> Self {
        Self { gravity_threshold: MessageGravity::Info, buffer: Vec::new(), prefix: String::new() }
    }
}

impl MessagePrinter {
    pub fn new() -> Self { Self::default() }
    pub fn with_prefix(prefix: &str) -> Self { Self { prefix: prefix.to_owned(), ..Default::default() } }

    pub fn set_gravity_threshold(&mut self, g: MessageGravity) { self.gravity_threshold = g; }

    pub fn print(&mut self, gravity: MessageGravity, msg: &str) {
        if gravity >= self.gravity_threshold {
            let entry = if self.prefix.is_empty() {
                format!("[{}] {}", gravity.label(), msg)
            } else {
                format!("[{}][{}] {}", self.prefix, gravity.label(), msg)
            };
            self.buffer.push(entry);
        }
    }

    pub fn print_alert(&mut self, alert: &MessageAlert) {
        self.print(alert.gravity, &alert.formatted());
    }

    pub fn nb_messages(&self) -> usize { self.buffer.len() }
    pub fn messages(&self) -> &[String] { &self.buffer }
    pub fn clear(&mut self) { self.buffer.clear(); }
    pub fn last_message(&self) -> Option<&str> { self.buffer.last().map(|s| s.as_str()) }
}

// occt-ref: Message_Report // — collects alerts by gravity and reports summary
#[derive(Clone, Debug, Default)]
pub struct MessageReport {
    pub alerts: Vec<MessageAlert>,
}

impl MessageReport {
    pub fn new() -> Self { Self::default() }

    pub fn add_alert(&mut self, alert: MessageAlert) { self.alerts.push(alert); }

    pub fn add(&mut self, gravity: MessageGravity, text: &str) {
        self.alerts.push(MessageAlert::new(gravity, text));
    }

    pub fn has_alert(&self, gravity: MessageGravity) -> bool {
        self.alerts.iter().any(|a| a.gravity == gravity)
    }

    pub fn has_failures(&self) -> bool {
        self.alerts.iter().any(|a| a.is_fail())
    }

    pub fn alerts_by_gravity(&self, gravity: MessageGravity) -> Vec<&MessageAlert> {
        self.alerts.iter().filter(|a| a.gravity == gravity).collect()
    }

    pub fn nb_alerts(&self) -> usize { self.alerts.len() }
    pub fn nb_by_gravity(&self, gravity: MessageGravity) -> usize {
        self.alerts.iter().filter(|a| a.gravity == gravity).count()
    }

    pub fn clear(&mut self) { self.alerts.clear(); }
    pub fn is_empty(&self) -> bool { self.alerts.is_empty() }

    pub fn dump(&self, printer: &mut MessagePrinter) {
        for a in &self.alerts { printer.print_alert(a); }
    }

    pub fn merge(&mut self, other: &MessageReport) {
        for a in &other.alerts { self.alerts.push(a.clone()); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_gravity_ordering() {
        assert!(MessageGravity::Fail > MessageGravity::Warning);
        assert!(MessageGravity::Alarm.is_fail());
        assert!(!MessageGravity::Info.is_fail());
    }

    #[test]
    fn message_alert_format() {
        let a = MessageAlert::new(MessageGravity::Warning, "Value %1 exceeds %2")
            .with_arg("42")
            .with_arg("100");
        assert_eq!(a.formatted(), "Value 42 exceeds 100");
        assert!(!a.is_fail());
    }

    #[test]
    fn message_formatter() {
        let msg = MessageFormatter::new("Point (%1, %2, %3)")
            .arg_f64(1.0)
            .arg_f64(2.0)
            .arg_f64(3.0)
            .format();
        assert_eq!(msg, "Point (1, 2, 3)");
    }

    #[test]
    fn message_printer_threshold() {
        let mut p = MessagePrinter::new();
        p.set_gravity_threshold(MessageGravity::Warning);
        p.print(MessageGravity::Info, "ignored");
        p.print(MessageGravity::Warning, "shown");
        assert_eq!(p.nb_messages(), 1);
        assert!(p.last_message().unwrap().contains("shown"));
    }

    #[test]
    fn message_report_accumulate() {
        let mut r = MessageReport::new();
        r.add(MessageGravity::Info, "Step 1 done");
        r.add(MessageGravity::Warning, "Tolerance relaxed");
        r.add(MessageGravity::Fail, "Algorithm failed");
        assert!(r.has_failures());
        assert_eq!(r.nb_by_gravity(MessageGravity::Warning), 1);
        assert_eq!(r.nb_alerts(), 3);
    }

    #[test]
    fn message_report_dump() {
        let mut r = MessageReport::new();
        r.add(MessageGravity::Info, "Hello");
        let mut p = MessagePrinter::new();
        r.dump(&mut p);
        assert_eq!(p.nb_messages(), 1);
    }

    #[test]
    fn message_report_merge() {
        let mut r1 = MessageReport::new();
        r1.add(MessageGravity::Info, "a");
        let mut r2 = MessageReport::new();
        r2.add(MessageGravity::Warning, "b");
        r1.merge(&r2);
        assert_eq!(r1.nb_alerts(), 2);
    }
}
