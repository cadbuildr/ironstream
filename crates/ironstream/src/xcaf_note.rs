// FILE: rust/ironstream/crates/ironstream/src/xcaf_note.rs

// occt: XCAFNotes_Note type — enum Text, Balloon, Comment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XcafNoteType {
    /// Plain text note
    Text,
    /// Balloon annotation pinned to a shape
    Balloon,
    /// General comment attached to a shape
    Comment,
}

// occt: XCAFNotes_Note — a single annotation entry with identity, content and provenance
#[derive(Clone, Debug)]
pub struct XcafNote {
    id: usize,
    note_type: XcafNoteType,
    text: String,
    author: String,
    timestamp: String,
    shape_label: String,
}

impl XcafNote {
    /// Create a new note. `timestamp` and `shape_label` start empty.
    pub fn new(id: usize, note_type: XcafNoteType, text: &str, author: &str) -> Self {
        Self {
            id,
            note_type,
            text: text.to_owned(),
            author: author.to_owned(),
            timestamp: String::new(),
            shape_label: String::new(),
        }
    }

    /// Return the unique identifier of this note.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the note type.
    pub fn note_type(&self) -> XcafNoteType {
        self.note_type
    }

    /// Return the text content.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Return the author name.
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Return the timestamp string (may be empty if not set).
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    /// Set the timestamp string.
    pub fn set_timestamp(&mut self, ts: &str) {
        self.timestamp = ts.to_owned();
    }

    /// Return the shape label this note is attached to (may be empty if not set).
    pub fn shape_label(&self) -> &str {
        &self.shape_label
    }

    /// Attach this note to the shape identified by `label`.
    pub fn set_shape_label(&mut self, label: &str) {
        self.shape_label = label.to_owned();
    }
}

// occt: XCAFDoc_NotesTool — manages a collection of notes and their associations to shapes
pub struct XcafNotesTool {
    notes: Vec<XcafNote>,
    next_id: usize,
}

impl XcafNotesTool {
    /// Create an empty notes tool.
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a new note and return its assigned id.
    pub fn add_note(&mut self, note_type: XcafNoteType, text: &str, author: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.notes.push(XcafNote::new(id, note_type, text, author));
        id
    }

    /// Return the number of notes currently stored.
    pub fn nb_notes(&self) -> usize {
        self.notes.len()
    }

    /// Return a reference to the note at position `i` in insertion order.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    pub fn note(&self, i: usize) -> &XcafNote {
        &self.notes[i]
    }

    /// Return the ids of all notes whose author equals `author`.
    pub fn find_by_author(&self, author: &str) -> Vec<usize> {
        self.notes
            .iter()
            .filter(|n| n.author() == author)
            .map(|n| n.id())
            .collect()
    }

    /// Return the ids of all notes attached to `shape_label`.
    pub fn notes_for_shape(&self, shape_label: &str) -> Vec<usize> {
        self.notes
            .iter()
            .filter(|n| n.shape_label() == shape_label)
            .map(|n| n.id())
            .collect()
    }

    /// Remove the note with the given `id`. Returns `true` if found and removed.
    pub fn remove_note(&mut self, id: usize) -> bool {
        if let Some(pos) = self.notes.iter().position(|n| n.id() == id) {
            self.notes.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for XcafNotesTool {
    fn default() -> Self {
        Self::new()
    }
}
