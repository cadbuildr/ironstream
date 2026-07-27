// FILE: message_alert.rs
// occt: Message_Alert, Message_AlertExtended, Message_Level
// occt-ref: Message_Msg
//       Message_MsgFile, Message_Printer, Message_PrinterOStream, Message_Messenger

/// Severity level for messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageGravity {
    Trace,
    Info,
    Warning,
    Alarm,
    Fail,
}

impl MessageGravity {
    pub fn is_error(&self) -> bool { *self >= MessageGravity::Alarm }
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageGravity::Trace => "Trace",
            MessageGravity::Info => "Info",
            MessageGravity::Warning => "Warning",
            MessageGravity::Alarm => "Alarm",
            MessageGravity::Fail => "Fail",
        }
    }
}

// occt: Message_Alert
#[derive(Clone, Debug)]
pub struct MessageAlert {
    pub gravity: MessageGravity,
    pub text: String,
    pub nb_sub_alerts: usize,
    pub is_significant: bool,
}

impl MessageAlert {
    pub fn new(gravity: MessageGravity, text: impl Into<String>) -> Self {
        Self { gravity, text: text.into(), nb_sub_alerts: 0, is_significant: true }
    }

    pub fn info(text: impl Into<String>) -> Self { Self::new(MessageGravity::Info, text) }
    pub fn warning(text: impl Into<String>) -> Self { Self::new(MessageGravity::Warning, text) }
    pub fn fail(text: impl Into<String>) -> Self { Self::new(MessageGravity::Fail, text) }
    pub fn trace(text: impl Into<String>) -> Self { Self::new(MessageGravity::Trace, text) }

    pub fn is_error(&self) -> bool { self.gravity.is_error() }
    pub fn with_sub_alerts(mut self, n: usize) -> Self { self.nb_sub_alerts = n; self }
    pub fn description(&self) -> &str { &self.text }
}

// occt: Message_AlertExtended // — alert with additional data
#[derive(Clone, Debug)]
pub struct MessageAlertExtended {
    pub alert: MessageAlert,
    pub shape_index: Option<u32>,
    pub value: Option<f64>,
    pub composite_data: Vec<String>,
}

impl MessageAlertExtended {
    pub fn new(alert: MessageAlert) -> Self {
        Self { alert, shape_index: None, value: None, composite_data: Vec::new() }
    }

    pub fn with_shape(mut self, idx: u32) -> Self { self.shape_index = Some(idx); self }
    pub fn with_value(mut self, v: f64) -> Self { self.value = Some(v); self }
    pub fn add_data(&mut self, s: impl Into<String>) { self.composite_data.push(s.into()); }
}

// occt: Message_Level
#[derive(Clone, Debug)]
pub struct MessageLevel {
    pub name: String,
    pub gravity: MessageGravity,
    pub alerts: Vec<MessageAlert>,
    pub is_active: bool,
}

impl MessageLevel {
    pub fn new(name: impl Into<String>, gravity: MessageGravity) -> Self {
        Self { name: name.into(), gravity, alerts: Vec::new(), is_active: true }
    }

    pub fn add_alert(&mut self, alert: MessageAlert) {
        self.alerts.push(alert);
    }

    pub fn nb_alerts(&self) -> usize { self.alerts.len() }

    pub fn nb_errors(&self) -> usize {
        self.alerts.iter().filter(|a| a.is_error()).count()
    }

    pub fn nb_warnings(&self) -> usize {
        self.alerts.iter().filter(|a| a.gravity == MessageGravity::Warning).count()
    }

    pub fn has_errors(&self) -> bool { self.nb_errors() > 0 }
    pub fn clear(&mut self) { self.alerts.clear(); }

    pub fn alerts_of_gravity(&self, g: MessageGravity) -> Vec<&MessageAlert> {
        self.alerts.iter().filter(|a| a.gravity == g).collect()
    }
}

// occt-ref: Message_Msg // (localizable message template)
#[derive(Clone, Debug)]
pub struct MessageMsg {
    pub key: String,
    pub text: String,
    pub args: Vec<String>,
}

impl MessageMsg {
    pub fn new(key: impl Into<String>) -> Self {
        let k: String = key.into();
        let text = k.clone();
        Self { key: k, text, args: Vec::new() }
    }

    pub fn with_text(mut self, t: impl Into<String>) -> Self { self.text = t.into(); self }

    pub fn arg(mut self, v: impl ToString) -> Self {
        self.args.push(v.to_string());
        self
    }

    pub fn format(&self) -> String {
        let mut result = self.text.clone();
        for (i, arg) in self.args.iter().enumerate() {
            let placeholder = format!("%{}", i + 1);
            result = result.replace(&placeholder, arg);
        }
        result
    }

    pub fn is_empty(&self) -> bool { self.text.is_empty() }
}

// occt: Message_MsgFile // (message catalog)
#[derive(Clone, Debug, Default)]
pub struct MessageMsgFile {
    pub messages: std::collections::HashMap<String, String>,
    pub language: String,
}

impl MessageMsgFile {
    pub fn new(language: impl Into<String>) -> Self {
        Self { language: language.into(), messages: Default::default() }
    }

    pub fn load_string(&mut self, key: impl Into<String>, text: impl Into<String>) {
        self.messages.insert(key.into(), text.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|s| s.as_str())
    }

    pub fn has_msg(&self, key: &str) -> bool { self.messages.contains_key(key) }
    pub fn nb_messages(&self) -> usize { self.messages.len() }
}

// occt-ref: Message_Printer // (abstract output sink)
pub trait MessagePrinter: std::fmt::Debug {
    fn send(&self, text: &str, gravity: MessageGravity);
    fn gravity_threshold(&self) -> MessageGravity;
}

/// In-memory printer for testing.
#[derive(Clone, Debug)]
pub struct InMemoryPrinter {
    pub entries: Vec<(MessageGravity, String)>,
    pub threshold: MessageGravity,
}

impl Default for InMemoryPrinter {
    fn default() -> Self { Self::new(MessageGravity::Info) }
}

impl InMemoryPrinter {
    pub fn new(threshold: MessageGravity) -> Self {
        Self { entries: Vec::new(), threshold }
    }

    pub fn messages_of(&self, g: MessageGravity) -> Vec<&str> {
        self.entries.iter().filter(|(gr, _)| *gr == g).map(|(_, s)| s.as_str()).collect()
    }

    pub fn nb_entries(&self) -> usize { self.entries.len() }
    pub fn nb_errors(&self) -> usize {
        self.entries.iter().filter(|(g, _)| g.is_error()).count()
    }
    pub fn clear(&mut self) { self.entries.clear(); }
}

// occt-ref: Message_Messenger
#[derive(Debug, Default)]
pub struct MessageMessenger {
    pub printers: Vec<Box<dyn MessagePrinterSend>>,
    pub active_level: Option<MessageLevel>,
}

pub trait MessagePrinterSend: std::fmt::Debug {
    fn send_string(&self, text: &str, gravity: MessageGravity);
}

#[derive(Debug)]
pub struct BufferedPrinter {
    pub buffer: std::cell::RefCell<Vec<(MessageGravity, String)>>,
    pub threshold: MessageGravity,
}

impl Default for BufferedPrinter {
    fn default() -> Self { Self::new() }
}

impl BufferedPrinter {
    pub fn new() -> Self { Self { buffer: Default::default(), threshold: MessageGravity::Info } }
    pub fn entries(&self) -> std::cell::Ref<Vec<(MessageGravity, String)>> { self.buffer.borrow() }
}

impl MessagePrinterSend for BufferedPrinter {
    fn send_string(&self, text: &str, gravity: MessageGravity) {
        if gravity >= self.threshold {
            self.buffer.borrow_mut().push((gravity, text.to_string()));
        }
    }
}

impl MessageMessenger {
    pub fn new() -> Self { Self::default() }

    pub fn add_printer(&mut self, p: Box<dyn MessagePrinterSend>) {
        self.printers.push(p);
    }

    pub fn send(&self, text: &str, gravity: MessageGravity) {
        for p in &self.printers {
            p.send_string(text, gravity);
        }
    }

    pub fn send_info(&self, text: &str) { self.send(text, MessageGravity::Info); }
    pub fn send_warning(&self, text: &str) { self.send(text, MessageGravity::Warning); }
    pub fn send_fail(&self, text: &str) { self.send(text, MessageGravity::Fail); }
    pub fn send_alarm(&self, text: &str) { self.send(text, MessageGravity::Alarm); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_gravity_order() {
        assert!(MessageGravity::Fail > MessageGravity::Warning);
        assert!(MessageGravity::Alarm.is_error());
        assert!(!MessageGravity::Info.is_error());
    }

    #[test]
    fn alert_types() {
        let a = MessageAlert::fail("Something went wrong");
        assert!(a.is_error());
        assert_eq!(a.description(), "Something went wrong");
        let w = MessageAlert::warning("Be careful");
        assert!(!w.is_error());
    }

    #[test]
    fn message_level_counts() {
        let mut level = MessageLevel::new("check", MessageGravity::Info);
        level.add_alert(MessageAlert::warning("w1"));
        level.add_alert(MessageAlert::fail("e1"));
        level.add_alert(MessageAlert::info("i1"));
        assert_eq!(level.nb_alerts(), 3);
        assert_eq!(level.nb_errors(), 1);
        assert_eq!(level.nb_warnings(), 1);
        assert!(level.has_errors());
    }

    #[test]
    fn message_msg_format() {
        let msg = MessageMsg::new("ERR_001")
            .with_text("File %1 not found at line %2")
            .arg("test.stp")
            .arg(42);
        let formatted = msg.format();
        assert!(formatted.contains("test.stp"));
        assert!(formatted.contains("42"));
    }

    #[test]
    fn msg_file_lookup() {
        let mut mf = MessageMsgFile::new("en");
        mf.load_string("OK", "Operation successful");
        assert!(mf.has_msg("OK"));
        assert_eq!(mf.get("OK"), Some("Operation successful"));
        assert_eq!(mf.nb_messages(), 1);
    }

    #[test]
    fn messenger_send() {
        let mut messenger = MessageMessenger::new();
        let printer = Box::new(BufferedPrinter::new());
        messenger.add_printer(printer);
        messenger.send_info("hello");
        messenger.send_fail("error");
    }
}
