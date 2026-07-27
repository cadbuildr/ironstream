// FILE: message_list_of_msg.rs
// occt: Message_ListOfMsg
// occt-ref: Message_ListIteratorOfListOfMsg

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

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_List<Message_Msg>`
pub type MessageListOfMsg = Vec<MessageMsg>;

/// Deprecated typedef alias for the iterator.
/// Original OCCT: `NCollection_List<Message_Msg>::Iterator`
pub type MessageListIteratorOfListOfMsg = std::vec::IntoIter<MessageMsg>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = MessageMsg::new("KEY_1".to_string(), "Hello".to_string());
        assert_eq!(msg.key(), "KEY_1");
        assert_eq!(msg.text(), "Hello");
        assert_eq!(msg.arguments().len(), 0);
    }

    #[test]
    fn test_message_with_arguments() {
        let mut msg = MessageMsg::new("KEY_1".to_string(), "Value: %s".to_string());
        msg.add_argument("test".to_string());
        msg.add_argument("data".to_string());

        assert_eq!(msg.arguments().len(), 2);
        assert_eq!(msg.arguments()[0], "test");
        assert_eq!(msg.arguments()[1], "data");
    }

    #[test]
    fn test_list_creation() {
        let list: MessageListOfMsg = Vec::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push_and_access() {
        let mut list: MessageListOfMsg = Vec::new();

        let msg1 = MessageMsg::new("KEY_1".to_string(), "Message 1".to_string());
        let msg2 = MessageMsg::new("KEY_2".to_string(), "Message 2".to_string());

        list.push(msg1.clone());
        list.push(msg2.clone());

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].key(), "KEY_1");
        assert_eq!(list[1].key(), "KEY_2");
    }

    #[test]
    fn test_list_iteration() {
        let mut list: MessageListOfMsg = Vec::new();

        for i in 1..=5 {
            list.push(MessageMsg::new(
                format!("KEY_{}", i),
                format!("Message {}", i),
            ));
        }

        assert_eq!(list.len(), 5);

        let mut keys = Vec::new();
        for msg in &list {
            keys.push(msg.key().to_string());
        }
        assert_eq!(keys.len(), 5);
    }

    #[test]
    fn test_list_remove() {
        let mut list: MessageListOfMsg = Vec::new();

        list.push(MessageMsg::new("KEY_1".to_string(), "Msg 1".to_string()));
        list.push(MessageMsg::new("KEY_2".to_string(), "Msg 2".to_string()));
        list.push(MessageMsg::new("KEY_3".to_string(), "Msg 3".to_string()));

        assert_eq!(list.len(), 3);
        list.remove(1);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].key(), "KEY_1");
        assert_eq!(list[1].key(), "KEY_3");
    }

    #[test]
    fn test_list_clear() {
        let mut list: MessageListOfMsg = Vec::new();

        list.push(MessageMsg::new("KEY_1".to_string(), "Msg 1".to_string()));
        list.push(MessageMsg::new("KEY_2".to_string(), "Msg 2".to_string()));

        assert_eq!(list.len(), 2);
        list.clear();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_iterator() {
        let mut list: MessageListOfMsg = Vec::new();

        list.push(MessageMsg::new("KEY_A".to_string(), "A".to_string()));
        list.push(MessageMsg::new("KEY_B".to_string(), "B".to_string()));
        list.push(MessageMsg::new("KEY_C".to_string(), "C".to_string()));

        let into_iter: MessageListIteratorOfListOfMsg = list.into_iter();
        let collected: Vec<MessageMsg> = into_iter.collect();

        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0].key(), "KEY_A");
        assert_eq!(collected[1].key(), "KEY_B");
        assert_eq!(collected[2].key(), "KEY_C");
    }
}
