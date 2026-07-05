// FILE: t_obj_application.rs
// occt: TObj_Application

//! This is a base class for OCAF based TObj models with declared
//! virtual methods.
//! Faithful port of `TObj_Application` (.hxx + .cxx): Save/Load of OCAF
//! documents with error-flag tracking and status-to-message dispatch
//! (SetError switches producing the exact "TObj_Appl_*" message keys),
//! CreateNewDocument, verbose flag, messenger, and ResourcesName()
//! returning "TObj". The TDocStd document machinery is modeled as an
//! in-memory document store keyed by file name.

use std::collections::HashMap;
use std::rc::Rc;

/// PCDM_StoreStatus subset dispatched by SetError.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcdmStoreStatusAp {
    Ok,
    DriverFailure,
    WriteFailure,
    Failure,
    DocIsNull,
    NoObj,
    Info,
}

/// PCDM_ReaderStatus subset dispatched by SetError.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcdmReaderStatusAp {
    Ok,
    UnknownDocument,
    AlreadyRetrieved,
    NoDriver,
    OpenError,
    NoDocument,
    FormatFailure,
    UnrecognizedFileFormat,
    PermissionDenied,
    ReaderException,
}

/// Message gravity (Message_Gravity).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageGravityAp {
    Trace,
    Info,
    Warning,
    Alarm,
    Fail,
}

/// Local stand-in for `TDocStd_Document`.
#[derive(Clone, Debug, PartialEq)]
pub struct TObjDocumentRecAp {
    pub format: String,
    pub content: String,
}

pub type HandleTObjDocumentAp = Rc<TObjDocumentRecAp>;

/// Base class for OCAF based TObj applications.
pub struct TObjApplication {
    is_error: bool,
    is_verbose: bool,
    /// Messenger log: (message, gravity).
    messenger: Vec<(String, MessageGravityAp)>,
    /// In-memory persistent store (file name -> saved document).
    file_system: HashMap<String, TObjDocumentRecAp>,
}

impl Default for TObjApplication {
    fn default() -> Self {
        TObjApplication::new()
    }
}

impl TObjApplication {
    /// Protected constructor (use GetInstance in C++; the message file
    /// loading is modeled by the always-available key table below).
    pub fn new() -> Self {
        TObjApplication {
            is_error: false,
            is_verbose: false,
            messenger: Vec::new(),
            file_system: HashMap::new(),
        }
    }

    /// ResourcesName — "TObj".
    pub fn resources_name(&self) -> &'static str {
        "TObj"
    }

    /// Messenger log access.
    pub fn messenger(&self) -> &[(String, MessageGravityAp)] {
        &self.messenger
    }

    /// SetVerbose / IsVerbose.
    pub fn set_verbose(&mut self, verbose: bool) {
        self.is_verbose = verbose;
    }

    pub fn is_verbose(&self) -> bool {
        self.is_verbose
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    /// ErrorMessage with explicit gravity.
    pub fn error_message(&mut self, msg: &str, level: MessageGravityAp) {
        self.messenger.push((msg.to_string(), level));
    }

    /// ErrorMessage default overload — gravity Message_Alarm.
    pub fn error_message_alarm(&mut self, msg: &str) {
        self.error_message(msg, MessageGravityAp::Alarm);
    }

    /// SaveDocument to a file: SaveAs + error tracking.
    pub fn save_document(&mut self, doc: &TObjDocumentRecAp, target_file: &str) -> bool {
        let status = self.save_as(doc, target_file);
        self.is_error = status != PcdmStoreStatusAp::Ok;
        if self.is_error {
            self.set_store_error(status, target_file);
        }
        !self.is_error
    }

    /// LoadDocument from a file: Open + error tracking.
    pub fn load_document(&mut self, source_file: &str) -> Option<TObjDocumentRecAp> {
        let (status, doc) = self.open(source_file);
        self.is_error = status != PcdmReaderStatusAp::Ok;
        if self.is_error {
            self.set_reader_error(status, source_file);
            return None;
        }
        doc
    }

    /// CreateNewDocument — clears the error flag and builds a document
    /// of the requested format.
    pub fn create_new_document(&mut self, format: &str) -> TObjDocumentRecAp {
        self.is_error = false;
        TObjDocumentRecAp { format: format.to_string(), content: String::new() }
    }

    /// TDocStd_Application::SaveAs model.
    fn save_as(&mut self, doc: &TObjDocumentRecAp, target_file: &str) -> PcdmStoreStatusAp {
        if target_file.is_empty() {
            return PcdmStoreStatusAp::WriteFailure;
        }
        if doc.format.is_empty() {
            return PcdmStoreStatusAp::DriverFailure;
        }
        self.file_system.insert(target_file.to_string(), doc.clone());
        PcdmStoreStatusAp::Ok
    }

    /// TDocStd_Application::Open model.
    fn open(&self, source_file: &str) -> (PcdmReaderStatusAp, Option<TObjDocumentRecAp>) {
        match self.file_system.get(source_file) {
            Some(doc) => (PcdmReaderStatusAp::Ok, Some(doc.clone())),
            None => (PcdmReaderStatusAp::OpenError, None),
        }
    }

    /// SetError for store statuses — exact TObj message keys.
    fn set_store_error(&mut self, status: PcdmStoreStatusAp, info: &str) {
        let key = match status {
            PcdmStoreStatusAp::DriverFailure => "TObj_Appl_SDriverFailure",
            PcdmStoreStatusAp::WriteFailure => "TObj_Appl_SWriteFailure",
            PcdmStoreStatusAp::Failure => "TObj_Appl_SFailure",
            _ => "TObj_Appl_SUnknownFailure",
        };
        self.error_message_alarm(&format!("{key}: {info}"));
    }

    /// SetError for reader statuses — exact TObj message keys.
    fn set_reader_error(&mut self, status: PcdmReaderStatusAp, info: &str) {
        let key = match status {
            PcdmReaderStatusAp::UnknownDocument => "TObj_Appl_RUnknownDocument",
            PcdmReaderStatusAp::AlreadyRetrieved => "TObj_Appl_RAlreadyRetrieved",
            PcdmReaderStatusAp::NoDriver => "TObj_Appl_RNoDriver",
            PcdmReaderStatusAp::OpenError => "TObj_Appl_ROpenError",
            PcdmReaderStatusAp::NoDocument => "TObj_Appl_RNoDocument",
            PcdmReaderStatusAp::FormatFailure => "TObj_Appl_RFormatFailure",
            PcdmReaderStatusAp::UnrecognizedFileFormat => "TObj_Appl_RBadFileFormat",
            PcdmReaderStatusAp::PermissionDenied => "TObj_Appl_RPermissionDenied",
            PcdmReaderStatusAp::ReaderException => "TObj_Appl_RException",
            _ => "TObj_Appl_RUnknownFail",
        };
        self.error_message_alarm(&format!("{key}: {info}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resources_name_is_tobj() {
        let app = TObjApplication::new();
        assert_eq!(app.resources_name(), "TObj");
        assert!(!app.is_verbose());
        assert!(!app.is_error());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let mut app = TObjApplication::new();
        let doc = app.create_new_document("TObjBin");
        let doc = TObjDocumentRecAp { content: "model data".into(), ..doc };
        assert!(app.save_document(&doc, "model.cbf"));
        assert!(!app.is_error());
        let loaded = app.load_document("model.cbf").expect("loaded");
        assert_eq!(loaded, doc);
        assert!(app.messenger().is_empty(), "no errors were reported");
    }

    #[test]
    fn load_missing_file_reports_open_error() {
        let mut app = TObjApplication::new();
        assert!(app.load_document("ghost.cbf").is_none());
        assert!(app.is_error());
        let (msg, level) = &app.messenger()[0];
        assert!(msg.starts_with("TObj_Appl_ROpenError"), "got: {msg}");
        assert!(msg.contains("ghost.cbf"));
        assert_eq!(*level, MessageGravityAp::Alarm);
    }

    #[test]
    fn save_failures_map_to_message_keys() {
        let mut app = TObjApplication::new();
        let doc = app.create_new_document("TObjBin");
        assert!(!app.save_document(&doc, ""), "empty target file fails");
        assert!(app.messenger()[0].0.starts_with("TObj_Appl_SWriteFailure"));
        let formatless = TObjDocumentRecAp { format: String::new(), content: String::new() };
        assert!(!app.save_document(&formatless, "x.cbf"));
        assert!(app.messenger()[1].0.starts_with("TObj_Appl_SDriverFailure"));
    }

    #[test]
    fn create_new_document_clears_error() {
        let mut app = TObjApplication::new();
        app.load_document("missing.cbf");
        assert!(app.is_error());
        let doc = app.create_new_document("XmlTObj");
        assert!(!app.is_error());
        assert_eq!(doc.format, "XmlTObj");
    }

    #[test]
    fn verbose_flag_and_default_gravity() {
        let mut app = TObjApplication::new();
        app.set_verbose(true);
        assert!(app.is_verbose());
        app.error_message_alarm("boom");
        assert_eq!(app.messenger()[0], ("boom".to_string(), MessageGravityAp::Alarm));
    }
}
