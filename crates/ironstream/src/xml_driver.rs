// FILE: rust/ironstream/crates/ironstream/src/xml_driver.rs

// occt: XmlMDF_ADriver — OCAF XML/Bin persistence driver layer.
// Provides zero-dependency serialization of OCAF attributes to an XML-like
// text format, mirroring the XmlMDF_ADriver / BinMDF_ADriver contract.

// ---------------------------------------------------------------------------
// XmlAttrType
// ---------------------------------------------------------------------------

/// occt: XML attribute serialization type — mirrors XmlMDF_ADriver type tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlAttrType {
    /// occt: TDataStd_Name / TDataStd_Comment — UTF-8 string attribute.
    String,
    /// occt: TDataStd_Integer — 32-bit signed integer attribute.
    Integer,
    /// occt: TDataStd_Real — 64-bit IEEE-754 real attribute.
    Real,
    /// occt: TDataStd_UAttribute — boolean flag attribute.
    Boolean,
    /// occt: TDataStd_IntegerArray / TDataStd_RealArray — comma-separated array.
    Array,
}

impl XmlAttrType {
    /// Returns the canonical lowercase string tag used in serialized output.
    fn as_str(self) -> &'static str {
        match self {
            XmlAttrType::String => "string",
            XmlAttrType::Integer => "integer",
            XmlAttrType::Real => "real",
            XmlAttrType::Boolean => "boolean",
            XmlAttrType::Array => "array",
        }
    }

    /// Parses the canonical tag produced by `as_str`.  Returns `None` for
    /// unrecognised tags.
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "string" => Some(XmlAttrType::String),
            "integer" => Some(XmlAttrType::Integer),
            "real" => Some(XmlAttrType::Real),
            "boolean" => Some(XmlAttrType::Boolean),
            "array" => Some(XmlAttrType::Array),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// XmlDriverEntry
// ---------------------------------------------------------------------------

/// occt: one serialized OCAF attribute — mirrors a single XmlMDF_ADriver
/// entry stored in an XML persistence document.
pub struct XmlDriverEntry {
    label: String,
    attr_type: XmlAttrType,
    value: String,
}

impl XmlDriverEntry {
    /// occt: constructs an entry with the given label path, attribute type,
    /// and value serialized as a string (matching XmlMDF_ADriver::Paste /
    /// BinMDF_ADriver::Paste conventions).
    pub fn new(label: &str, attr_type: XmlAttrType, value: &str) -> Self {
        Self {
            label: label.to_owned(),
            attr_type,
            value: value.to_owned(),
        }
    }

    /// occt: returns the OCAF label entry path (e.g. "0:1:2").
    pub fn label(&self) -> &str {
        &self.label
    }

    /// occt: returns the attribute serialization type tag.
    pub fn attr_type(&self) -> XmlAttrType {
        self.attr_type
    }

    /// occt: returns the serialized attribute value string.
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ---------------------------------------------------------------------------
// XmlDocument
// ---------------------------------------------------------------------------

/// occt: XmlMDF_ADriver — XML document holding a format version and a
/// sequence of serialized OCAF attribute entries.
pub struct XmlDocument {
    format_version: u32,
    entries: Vec<XmlDriverEntry>,
}

impl XmlDocument {
    /// occt: creates an empty document with the given format version, matching
    /// the `<document formatVersion="...">` attribute in OCCT XML files.
    pub fn new(format_version: u32) -> Self {
        Self {
            format_version,
            entries: Vec::new(),
        }
    }

    /// occt: returns the document format version.
    pub fn format_version(&self) -> u32 {
        self.format_version
    }

    /// occt: appends a driver entry to the document (XmlMDF_ADriver::Add).
    pub fn add_entry(&mut self, e: XmlDriverEntry) {
        self.entries.push(e);
    }

    /// occt: returns the total number of entries in the document.
    pub fn nb_entries(&self) -> usize {
        self.entries.len()
    }

    /// occt: returns a reference to the entry at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_entries()`.
    pub fn entry(&self, i: usize) -> &XmlDriverEntry {
        &self.entries[i]
    }

    /// occt: looks up the first entry whose label matches `label`, mirroring
    /// XmlMDF_ADriver label-keyed lookup.  Returns `None` if not found.
    pub fn entry_by_label(&self, label: &str) -> Option<&XmlDriverEntry> {
        self.entries.iter().find(|e| e.label == label)
    }

    /// occt: serializes the document to a simple XML-like string, matching the
    /// envelope produced by XmlLDrivers_DocumentStorageDriver.
    pub fn serialize(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "<document formatVersion=\"{}\">\n",
            self.format_version
        ));
        for e in &self.entries {
            out.push_str("  ");
            out.push_str(&serialize_entry(e));
            out.push('\n');
        }
        out.push_str("</document>");
        out
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// occt: formats one XmlDriverEntry as a self-closing XML element, matching
/// the per-attribute element layout used by XmlMDF_ADriver::Paste output.
pub fn serialize_entry(e: &XmlDriverEntry) -> String {
    // Escape the five XML special characters inside attribute values so the
    // output is well-formed.
    fn escape(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '&' => out.push_str("&amp;"),
                '"' => out.push_str("&quot;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '\'' => out.push_str("&apos;"),
                c => out.push(c),
            }
        }
        out
    }
    format!(
        "<attr label=\"{}\" type=\"{}\" value=\"{}\"/>",
        escape(e.label()),
        e.attr_type().as_str(),
        escape(e.value()),
    )
}

/// occt: parses a single `<attr label="..." type="..." value="..."/>` line
/// produced by `serialize_entry`.  Returns `None` for malformed input.
pub fn deserialize_entry(s: &str) -> Option<XmlDriverEntry> {
    let s = s.trim();

    // Must start with `<attr ` and end with `/>`.
    if !s.starts_with("<attr ") || !s.ends_with("/>") {
        return None;
    }

    // Extract the three attribute values using a minimal hand-rolled parser.
    let label = extract_attr(s, "label")?;
    let type_str = extract_attr(s, "type")?;
    let value = extract_attr(s, "value")?;

    let attr_type = XmlAttrType::from_str(&type_str)?;
    Some(XmlDriverEntry::new(&label, attr_type, &value))
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extracts the value of a named XML attribute from a simple single-element
/// string of the form `... name="value" ...`.  Handles the five XML escapes
/// produced by `serialize_entry`.
fn extract_attr(s: &str, name: &str) -> Option<String> {
    // Pattern to locate: name="
    let needle = format!("{}=\"", name);
    let start = s.find(needle.as_str())?;
    let after_quote = start + needle.len();
    let rest = &s[after_quote..];

    // Scan for the closing `"`, respecting no nesting (attribute values here
    // contain only XML-escaped characters, never a literal `"`).
    let end = rest.find('"')?;
    Some(unescape(&rest[..end]))
}

/// Unescapes the five XML character references used by `serialize_entry`.
fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '&' {
            // Collect until ';'.
            let mut entity = String::new();
            for c in chars.by_ref() {
                if c == ';' {
                    break;
                }
                entity.push(c);
            }
            match entity.as_str() {
                "amp" => out.push('&'),
                "quot" => out.push('"'),
                "lt" => out.push('<'),
                "gt" => out.push('>'),
                "apos" => out.push('\''),
                other => {
                    // Unknown entity: emit as-is (best-effort).
                    out.push('&');
                    out.push_str(other);
                    out.push(';');
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}
