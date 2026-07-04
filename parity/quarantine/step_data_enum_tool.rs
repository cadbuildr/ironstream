// FILE: step_data_enum_tool.rs
// occt: StepData_EnumTool

//! This class gives a way of conversion between the value of an enumeration
//! and its representation in STEP format.
pub struct StepDataEnumTool {
    texts: Vec<String>,
    init_count: usize,
    optional: bool,
}

impl StepDataEnumTool {
    //! Creates an EnumTool with definitions given by e0 .. e39
    pub fn new(definitions: &[&str]) -> Self {
        let mut tool = StepDataEnumTool {
            texts: Vec::new(),
            init_count: 0,
            optional: true,
        };

        for def in definitions {
            tool.add_definition(def);
        }

        tool.init_count = tool.texts.len();
        tool
    }

    //! Processes a definition, splits it according to blanks if any
    pub fn add_definition(&mut self, term: &str) {
        if term.is_empty() {
            return;
        }

        let mut text = String::new();
        let mut in_word = false;

        for ch in term.chars() {
            if ch.is_whitespace() {
                if !text.is_empty() {
                    if !text.ends_with('.') && text != "$" {
                        text.push('.');
                    }
                    self.texts.push(text.clone());
                    text.clear();
                    in_word = false;
                }
            } else {
                if !in_word && ch != '.' && ch != '$' {
                    text.push('.');
                }
                text.push(ch);
                in_word = true;
            }
        }

        if !text.is_empty() {
            if !text.ends_with('.') && text != "$" {
                text.push('.');
            }
            self.texts.push(text);
        }
    }

    //! Returns True if at least one definition has been entered after creation time
    pub fn is_set(&self) -> bool {
        self.texts.len() > self.init_count
    }

    //! Returns the maximum integer for a suitable value
    pub fn max_value(&self) -> i32 {
        (self.texts.len() as i32) - 1
    }

    //! Sets or unsets the EnumTool to accept undefined value
    pub fn set_optional(&mut self, mode: bool) {
        self.optional = mode;
    }

    //! Returns the value attached to "null/undefined value"
    pub fn null_value(&self) -> i32 {
        if self.optional {
            self.value("$")
        } else {
            -1
        }
    }

    //! Returns the text which corresponds to a given numeric value
    pub fn text(&self, num: usize) -> String {
        if num >= self.texts.len() {
            return String::new();
        }
        self.texts[num].clone()
    }

    //! Returns the numeric value found for a text
    pub fn value(&self, txt: &str) -> i32 {
        for (i, text) in self.texts.iter().enumerate() {
            if text == txt {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_tool_new() {
        let tool = StepDataEnumTool::new(&["UNKNOWN", "DEFINED"]);
        assert_eq!(tool.max_value(), 1);
    }

    #[test]
    fn test_add_definition() {
        let mut tool = StepDataEnumTool::new(&[]);
        tool.add_definition("VALUE");
        assert!(tool.is_set());
    }

    #[test]
    fn test_value_lookup() {
        let tool = StepDataEnumTool::new(&["RED", "GREEN", "BLUE"]);
        assert_eq!(tool.value(".RED."), 0);
        assert_eq!(tool.value(".GREEN."), 1);
        assert_eq!(tool.value(".BLUE."), 2);
    }

    #[test]
    fn test_null_value() {
        let mut tool = StepDataEnumTool::new(&["$", "REAL"]);
        tool.set_optional(true);
        let null_val = tool.null_value();
        assert_eq!(null_val, 0);
    }

    #[test]
    fn test_text_lookup() {
        let tool = StepDataEnumTool::new(&["VALUE1", "VALUE2"]);
        assert_eq!(tool.text(0), ".VALUE1.");
        assert_eq!(tool.text(1), ".VALUE2.");
    }

    #[test]
    fn test_optional_mode() {
        let mut tool = StepDataEnumTool::new(&["$"]);
        tool.set_optional(false);
        assert_eq!(tool.null_value(), -1);
    }
}
