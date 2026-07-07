// FILE: xdedraw_notes.rs
// occt: XDEDRAW_Notes

//! DRAW commands for annotation notes in XDE.
//! Original: Draw/TKXDEDRAW/XDEDRAW/XDEDRAW_Notes.hxx
//!
//! Provides commands to create, manage, and display annotation notes.

use std::collections::HashMap;

/// Note command handler for XDE documents.
#[derive(Clone, Debug)]
pub struct XDEDRAWNotes {
    notes: HashMap<String, String>, // Note ID -> Note text
    note_commands: Vec<String>,
    note_count: usize,
}

impl XDEDRAWNotes {
    /// Creates a new note command handler.
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
            note_commands: Vec::new(),
            note_count: 0,
        }
    }

    /// Registers a note command.
    pub fn register_note_command(&mut self, cmd_name: String) {
        self.note_commands.push(cmd_name);
    }

    /// Creates a new note with the given text. Returns a note ID.
    pub fn create_note(&mut self, text: String) -> String {
        self.note_count += 1;
        let note_id = format!("note_{}", self.note_count);
        self.notes.insert(note_id.clone(), text);
        note_id
    }

    /// Retrieves the text of a note by ID.
    pub fn get_note(&self, note_id: &str) -> Option<&str> {
        self.notes.get(note_id).map(|s| s.as_str())
    }

    /// Updates the text of an existing note. Returns true if successful.
    pub fn update_note(&mut self, note_id: &str, text: String) -> bool {
        if self.notes.contains_key(note_id) {
            self.notes.insert(note_id.to_string(), text);
            true
        } else {
            false
        }
    }

    /// Removes a note by ID. Returns true if it existed.
    pub fn remove_note(&mut self, note_id: &str) -> bool {
        self.notes.remove(note_id).is_some()
    }

    /// Returns the list of registered note commands.
    pub fn note_commands(&self) -> &[String] {
        &self.note_commands
    }

    /// Returns the number of notes.
    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    /// Clears all notes and commands.
    pub fn clear(&mut self) {
        self.notes.clear();
        self.note_commands.clear();
        self.note_count = 0;
    }

    /// Initializes standard note commands.
    pub fn init_standard_note_commands(&mut self) {
        self.note_commands.push("xde_create_note".to_string());
        self.note_commands.push("xde_get_note".to_string());
        self.note_commands.push("xde_update_note".to_string());
        self.note_commands.push("xde_remove_note".to_string());
    }
}

impl Default for XDEDRAWNotes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note_handler() {
        let handler = XDEDRAWNotes::new();
        assert_eq!(handler.note_count(), 0);
    }

    #[test]
    fn test_create_note() {
        let mut handler = XDEDRAWNotes::new();
        let id = handler.create_note("Test note".to_string());
        assert_eq!(id, "note_1");
        assert_eq!(handler.get_note("note_1"), Some("Test note"));
        assert_eq!(handler.note_count(), 1);
    }

    #[test]
    fn test_update_note() {
        let mut handler = XDEDRAWNotes::new();
        let id = handler.create_note("Original".to_string());
        assert!(handler.update_note(&id, "Updated".to_string()));
        assert_eq!(handler.get_note(&id), Some("Updated"));
        assert!(!handler.update_note("nonexistent", "Text".to_string()));
    }

    #[test]
    fn test_remove_note() {
        let mut handler = XDEDRAWNotes::new();
        let id = handler.create_note("Note".to_string());
        assert!(handler.remove_note(&id));
        assert_eq!(handler.note_count(), 0);
        assert!(!handler.remove_note(&id)); // Already removed
    }

    #[test]
    fn test_register_commands() {
        let mut handler = XDEDRAWNotes::new();
        handler.register_note_command("cmd1".to_string());
        handler.register_note_command("cmd2".to_string());
        assert_eq!(handler.note_commands().len(), 2);
    }

    #[test]
    fn test_init_standard_note_commands() {
        let mut handler = XDEDRAWNotes::new();
        handler.init_standard_note_commands();
        assert_eq!(handler.note_commands().len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut handler = XDEDRAWNotes::new();
        handler.create_note("Note 1".to_string());
        handler.create_note("Note 2".to_string());
        handler.register_note_command("cmd".to_string());
        assert_eq!(handler.note_count(), 2);

        handler.clear();
        assert_eq!(handler.note_count(), 0);
        assert_eq!(handler.note_commands().len(), 0);
    }
}
