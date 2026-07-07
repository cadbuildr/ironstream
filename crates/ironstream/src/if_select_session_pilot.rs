// FILE: if_select_session_pilot.rs
// occt: IFSelect_SessionPilot

/// An interactive session pilot for executing commands.
/// Supports built-in commands and custom command activators.
#[derive(Clone, Debug)]
pub struct IFSelectSessionPilot {
    prompt: String,
}

impl IFSelectSessionPilot {
    /// Creates a SessionPilot with a prompt
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }

    /// Returns the prompt
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Sets the prompt
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
}

impl Default for IFSelectSessionPilot {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pilot = IFSelectSessionPilot::new("cmd> ".to_string());
        assert_eq!(pilot.prompt(), "cmd> ");
    }

    #[test]
    fn test_set_prompt() {
        let mut pilot = IFSelectSessionPilot::new("old> ".to_string());
        pilot.set_prompt("new> ".to_string());
        assert_eq!(pilot.prompt(), "new> ");
    }

    #[test]
    fn test_default() {
        let pilot = IFSelectSessionPilot::default();
        assert_eq!(pilot.prompt(), "> ");
    }
}
