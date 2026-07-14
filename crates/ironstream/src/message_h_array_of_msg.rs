// FILE: message_h_array_of_msg.rs
// occt: Message_HArrayOfMsg
// occt-ref: Message_ArrayOfMsg

use std::rc::Rc;
use std::cell::RefCell;

/// Message_Msg represents a message with a key and optional arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageMsg {
    key: String,
    text: String,
    arguments: Vec<String>,
}

impl MessageMsg {
    pub fn new(key: String, text: String) -> Self {
        MessageMsg {
            key,
            text,
            arguments: Vec::new(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn add_argument(&mut self, arg: String) {
        self.arguments.push(arg);
    }
}

/// NCollection_Handle is a reference-counted wrapper.
pub type NcollectionHandleMessageMsg = Rc<RefCell<MessageMsg>>;

/// NCollection_Array1 is a 1D array container.
/// Message_ArrayOfMsg is an array of handled Message_Msg objects.
pub struct MessageArrayOfMsg {
    data: Vec<NcollectionHandleMessageMsg>,
    lower: i32,
    upper: i32,
}

impl MessageArrayOfMsg {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        MessageArrayOfMsg {
            data: Vec::with_capacity(size),
            lower,
            upper,
        }
    }

    pub fn set_value(&mut self, index: i32, value: NcollectionHandleMessageMsg) {
        if index >= self.lower && index <= self.upper {
            let idx = (index - self.lower) as usize;
            if idx >= self.data.len() {
                self.data.resize(idx + 1, Rc::new(RefCell::new(
                    MessageMsg::new("".to_string(), "".to_string())
                )));
            }
            self.data[idx] = value;
        }
    }

    pub fn value(&self, index: i32) -> Option<&NcollectionHandleMessageMsg> {
        if index >= self.lower && index <= self.upper {
            let idx = (index - self.lower) as usize;
            if idx < self.data.len() {
                return Some(&self.data[idx]);
            }
        }
        None
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }
}

/// NCollection_Handle<Message_ArrayOfMsg> - a reference-counted handle to the array.
pub type MessageHArrayOfMsg = Rc<RefCell<MessageArrayOfMsg>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = MessageMsg::new("KEY_1".to_string(), "Hello World".to_string());
        assert_eq!(msg.key(), "KEY_1");
        assert_eq!(msg.text(), "Hello World");
        assert_eq!(msg.arguments().len(), 0);
    }

    #[test]
    fn test_message_add_argument() {
        let mut msg = MessageMsg::new("KEY_1".to_string(), "Value: %s".to_string());
        assert_eq!(msg.arguments().len(), 0);

        msg.add_argument("test".to_string());
        assert_eq!(msg.arguments().len(), 1);
        assert_eq!(msg.arguments()[0], "test");

        msg.add_argument("another".to_string());
        assert_eq!(msg.arguments().len(), 2);
    }

    #[test]
    fn test_array_creation() {
        let array = MessageArrayOfMsg::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.length(), 5);
    }

    #[test]
    fn test_array_set_and_get_value() {
        let mut array = MessageArrayOfMsg::new(1, 3);

        let msg1 = Rc::new(RefCell::new(MessageMsg::new(
            "KEY_1".to_string(),
            "Message 1".to_string(),
        )));
        let msg2 = Rc::new(RefCell::new(MessageMsg::new(
            "KEY_2".to_string(),
            "Message 2".to_string(),
        )));

        array.set_value(1, msg1.clone());
        array.set_value(2, msg2.clone());

        let retrieved1 = array.value(1).unwrap();
        assert_eq!(retrieved1.borrow().key(), "KEY_1");

        let retrieved2 = array.value(2).unwrap();
        assert_eq!(retrieved2.borrow().key(), "KEY_2");
    }

    #[test]
    fn test_array_out_of_bounds() {
        let array = MessageArrayOfMsg::new(1, 3);
        assert_eq!(array.value(0), None);
        assert_eq!(array.value(4), None);
        assert_eq!(array.value(10), None);
    }

    #[test]
    fn test_handle_array() {
        let handle = Rc::new(RefCell::new(MessageArrayOfMsg::new(1, 2)));

        {
            let mut array = handle.borrow_mut();
            let msg = Rc::new(RefCell::new(MessageMsg::new(
                "TEST".to_string(),
                "Test Message".to_string(),
            )));
            array.set_value(1, msg);
        }

        {
            let array = handle.borrow();
            let value = array.value(1).unwrap();
            assert_eq!(value.borrow().key(), "TEST");
        }
    }
}
