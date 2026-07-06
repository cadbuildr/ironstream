// FILE: xsdraw_functions.rs
// occt: XSDRAW_Functions
//
// Faithful port of OCCT XSDRAW_Functions (Draw/TKXSDRAW/XSDRAW/XSDRAW_Functions.hxx),
// a namespace-like holder for XSDRAW utility functions (e.g. shape loading, conversion,
// query helpers). Minimal implementation: payload is a unit struct; methods would
// register Draw commands for generic shape tasks (readshape, writeshape, etc.).

/// Local helper: single registered Draw command entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XsdrawFunctionCommand {
    pub name: String,
    pub help: String,
}

/// Namespace-like holder for XSDRAW utility functions and their Draw command registry.
#[derive(Debug, Default)]
pub struct XsdrawFunctions {
    commands: Vec<XsdrawFunctionCommand>,
}

impl XsdrawFunctions {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a Draw command for a utility function.
    pub fn add_command(&mut self, name: &str, help: &str) {
        self.commands.push(XsdrawFunctionCommand {
            name: name.to_string(),
            help: help.to_string(),
        });
    }

    /// Retrieve all registered commands.
    pub fn commands(&self) -> &[XsdrawFunctionCommand] {
        &self.commands
    }

    /// Look up a command by name.
    pub fn find_command(&self, name: &str) -> Option<&XsdrawFunctionCommand> {
        self.commands.iter().find(|cmd| cmd.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xsdraw_functions_new() {
        let funcs = XsdrawFunctions::new();
        assert_eq!(funcs.commands.len(), 0);
    }

    #[test]
    fn test_add_command() {
        let mut funcs = XsdrawFunctions::new();
        funcs.add_command("readshape", "Read a shape from file");
        assert_eq!(funcs.commands.len(), 1);
        assert_eq!(funcs.commands[0].name, "readshape");
    }

    #[test]
    fn test_find_command() {
        let mut funcs = XsdrawFunctions::new();
        funcs.add_command("writeshape", "Write a shape to file");
        assert!(funcs.find_command("writeshape").is_some());
        assert!(funcs.find_command("notfound").is_none());
    }

    #[test]
    fn test_multiple_commands() {
        let mut funcs = XsdrawFunctions::new();
        funcs.add_command("cmd1", "help1");
        funcs.add_command("cmd2", "help2");
        assert_eq!(funcs.commands().len(), 2);
    }
}
