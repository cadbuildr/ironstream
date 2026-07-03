// FILE: tests/occt_message_alert.rs
extern crate ironstream;

use ironstream::message_alert::{
    BufferedPrinter, MessageAlert, MessageGravity, MessageLevel, MessageMessenger, MessageMsg,
    MessageMsgFile, MessagePrinterSend,
};

#[test]
fn message_gravity_ordering_and_is_error() {
    assert!(MessageGravity::Fail > MessageGravity::Alarm);
    assert!(MessageGravity::Alarm > MessageGravity::Warning);
    assert!(MessageGravity::Warning > MessageGravity::Info);
    assert!(MessageGravity::Alarm.is_error());
    assert!(MessageGravity::Fail.is_error());
    assert!(!MessageGravity::Warning.is_error());
    assert!(!MessageGravity::Info.is_error());
    assert!(!MessageGravity::Trace.is_error());
}

#[test]
fn message_alert_fail_warning_info() {
    let fail = MessageAlert::fail("fatal error");
    assert!(fail.is_error());
    assert_eq!(fail.gravity, MessageGravity::Fail);

    let warn = MessageAlert::warning("caution");
    assert!(!warn.is_error());
    assert_eq!(warn.description(), "caution");

    let info = MessageAlert::info("note");
    assert_eq!(info.gravity, MessageGravity::Info);
}

#[test]
fn message_level_nb_errors_nb_warnings() {
    let mut lvl = MessageLevel::new("validation", MessageGravity::Info);
    lvl.add_alert(MessageAlert::info("ok"));
    lvl.add_alert(MessageAlert::warning("w1"));
    lvl.add_alert(MessageAlert::warning("w2"));
    lvl.add_alert(MessageAlert::fail("e1"));
    assert_eq!(lvl.nb_alerts(), 4);
    assert_eq!(lvl.nb_warnings(), 2);
    assert_eq!(lvl.nb_errors(), 1);
    assert!(lvl.has_errors());
}

#[test]
fn message_msg_format_with_args() {
    let msg = MessageMsg::new("FILE_NOT_FOUND")
        .with_text("File %1 missing at line %2")
        .arg("model.stp")
        .arg(99);
    let formatted = msg.format();
    assert!(formatted.contains("model.stp"));
    assert!(formatted.contains("99"));
}

#[test]
fn message_msg_file_has_msg() {
    let mut mf = MessageMsgFile::new("en");
    mf.load_string("ERR_001", "Something went wrong");
    mf.load_string("OK", "Success");
    assert!(mf.has_msg("ERR_001"));
    assert!(mf.has_msg("OK"));
    assert!(!mf.has_msg("MISSING_KEY"));
    assert_eq!(mf.nb_messages(), 2);
}
