// FILE: interface_protocol.rs
// occt: Interface_Protocol

use std::sync::Arc;

/// General description of Interface Protocols
pub trait InterfaceProtocol {
    /// Returns count of Protocol used as Resources (level one)
    fn nb_resources(&self) -> i32;

    /// Returns a Resource, given its rank
    fn resource(&self, num: i32) -> Option<Arc<dyn InterfaceProtocol>>;

    /// Returns a unique positive CaseNumber for each Recognized Object
    fn case_number(&self, obj: &Arc<dyn std::any::Any>) -> i32 {
        0
    }

    /// Returns True if type of obj is that defined from CDL
    fn is_dynamic_type(&self, _obj: &Arc<dyn std::any::Any>) -> bool {
        true
    }

    /// Returns the count of DISTINCT types under which an entity may be processed
    fn nb_types(&self, _obj: &Arc<dyn std::any::Any>) -> i32 {
        1
    }

    /// Returns a unique positive CaseNumber for each Recognized Type
    fn type_number(&self, _atype: &str) -> i32 {
        0
    }

    /// Evaluates a Global Check for a model
    fn global_check(&self, _g: &InterfaceGraph, _ach: &mut CheckHandle) -> bool {
        false
    }

    /// Creates an empty Model of the considered Norm
    fn new_model(&self) -> Arc<dyn std::any::Any>;

    /// Returns True if model is a Model of the considered Norm
    fn is_suitable_model(&self, _model: &Arc<dyn std::any::Any>) -> bool;
}

pub struct InterfaceGraph;
pub type CheckHandle = Arc<dyn std::any::Any>;

/// Default protocol implementation
pub struct DefaultProtocol;

impl InterfaceProtocol for DefaultProtocol {
    fn nb_resources(&self) -> i32 {
        0
    }

    fn resource(&self, _num: i32) -> Option<Arc<dyn InterfaceProtocol>> {
        None
    }

    fn new_model(&self) -> Arc<dyn std::any::Any> {
        Arc::new(())
    }

    fn is_suitable_model(&self, _model: &Arc<dyn std::any::Any>) -> bool {
        true
    }
}

static mut ACTIVE_PROTOCOL: Option<Arc<dyn InterfaceProtocol>> = None;

pub fn set_active_protocol(protocol: Arc<dyn InterfaceProtocol>) {
    unsafe {
        ACTIVE_PROTOCOL = Some(protocol);
    }
}

pub fn active_protocol() -> Option<Arc<dyn InterfaceProtocol>> {
    unsafe { ACTIVE_PROTOCOL.clone() }
}

pub fn clear_active() {
    unsafe {
        ACTIVE_PROTOCOL = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_protocol() {
        let protocol = DefaultProtocol;
        assert_eq!(protocol.nb_resources(), 0);
        assert!(protocol.is_suitable_model(&Arc::new(())));
    }

    #[test]
    fn test_active_protocol() {
        let protocol = Arc::new(DefaultProtocol);
        set_active_protocol(protocol.clone());
        assert!(active_protocol().is_some());
        clear_active();
        assert!(active_protocol().is_none());
    }
}
