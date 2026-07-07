// FILE: draw_main.rs
// occt: Draw_Main

//! Main framework for Draw Test Harness applications.
//! Mirrors OCCT `Draw_Main` (Draw/TKDraw): defines the common entry point
//! used by DRAW-based executables. The entry point builds the command
//! interpreter, hands it to the application-supplied initialization
//! callback (`FDraw_InitAppli`), then processes commands and returns 0.
//!
//! The Tcl-backed `Draw_Interpretor` is external plumbing; it is modeled
//! here as a local helper type with real command registration/dispatch.

use std::collections::HashMap;

/// Local model of Draw_Interpretor: a command table with dispatch.
pub struct DrawInterpretor {
    commands: HashMap<String, DrawCommand>,
}

/// A registered Draw command: receives its arguments (argv, including the
/// command name) and returns output or an error message.
pub type DrawCommand = fn(&[&str]) -> Result<String, String>;

impl DrawInterpretor {
    /// Creates an interpreter with an empty command table.
    pub fn new() -> Self {
        DrawInterpretor {
            commands: HashMap::new(),
        }
    }

    /// Registers a command under the given name (Draw_Interpretor::Add).
    pub fn add(&mut self, name: &str, command: DrawCommand) {
        self.commands.insert(name.to_string(), command);
    }

    /// Returns true if a command with this name is registered.
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Number of registered commands.
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Evaluates a single command line (Draw_Interprete): splits it on
    /// whitespace and dispatches to the registered command.
    pub fn eval(&self, line: &str) -> Result<String, String> {
        let argv: Vec<&str> = line.split_whitespace().collect();
        match argv.first() {
            None => Ok(String::new()),
            Some(name) => match self.commands.get(*name) {
                Some(cmd) => cmd(&argv),
                None => Err(format!("invalid command name \"{}\"", name)),
            },
        }
    }
}

impl Default for DrawInterpretor {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialization function type for Draw applications (FDraw_InitAppli).
pub type FDrawInitAppli = fn(&mut DrawInterpretor);

/// Main entry point for Draw applications (Draw_Main).
///
/// Creates the interpreter, runs the application initialization callback,
/// then interprets each argument as a command line (batch mode). Returns 0
/// when all commands succeed, 1 if any command fails — mirroring the
/// process exit status of the OCCT entry point.
pub fn draw_main(args: &[&str], init_func: FDrawInitAppli) -> i32 {
    let mut interp = DrawInterpretor::new();
    init_func(&mut interp);

    let mut status = 0;
    for command in args {
        if command.trim().is_empty() {
            continue;
        }
        if let Err(msg) = interp.eval(command) {
            eprintln!("Draw_Main: {}", msg);
            status = 1;
        }
    }
    status
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd_echo(argv: &[&str]) -> Result<String, String> {
        Ok(argv[1..].join(" "))
    }

    fn cmd_fail(_argv: &[&str]) -> Result<String, String> {
        Err("intentional failure".to_string())
    }

    fn test_init(interp: &mut DrawInterpretor) {
        interp.add("echo", cmd_echo);
        interp.add("fail", cmd_fail);
    }

    fn empty_init(_interp: &mut DrawInterpretor) {}

    #[test]
    fn test_draw_main_no_args() {
        let result = draw_main(&[], empty_init);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_draw_main_runs_init_and_commands() {
        let result = draw_main(&["echo hello world"], test_init);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_draw_main_reports_failure() {
        let result = draw_main(&["fail"], test_init);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_draw_main_unknown_command_fails() {
        let result = draw_main(&["nosuchcommand"], test_init);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_interpretor_registration_and_dispatch() {
        let mut interp = DrawInterpretor::new();
        assert_eq!(interp.command_count(), 0);
        test_init(&mut interp);
        assert_eq!(interp.command_count(), 2);
        assert!(interp.has_command("echo"));
        assert!(!interp.has_command("missing"));

        assert_eq!(interp.eval("echo a b").unwrap(), "a b");
        assert!(interp.eval("missing").is_err());
        assert_eq!(interp.eval("   ").unwrap(), "");
    }
}
