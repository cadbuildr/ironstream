// FILE: qa_draw.rs
// occt: QADraw

//! QA test framework for Draw integration.

/// Draw command handler
#[derive(Debug, Clone)]
pub struct DrawCommand {
    name: String,
    description: String,
}

impl DrawCommand {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

/// QA Draw manager
#[derive(Debug)]
pub struct QADrawManager {
    commands: Vec<DrawCommand>,
}

impl QADrawManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn register_command(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    pub fn num_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn get_command(&self, name: &str) -> Option<&DrawCommand> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn list_commands(&self) -> Vec<&str> {
        self.commands.iter().map(|c| c.name.as_str()).collect()
    }
}

impl Default for QADrawManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_command() {
        let mut manager = QADrawManager::new();
        let cmd = DrawCommand::new("test".to_string(), "Test command".to_string());
        manager.register_command(cmd);

        assert_eq!(manager.num_commands(), 1);
    }

    #[test]
    fn test_get_command() {
        let mut manager = QADrawManager::new();
        let cmd = DrawCommand::new("test".to_string(), "Test".to_string());
        manager.register_command(cmd);

        assert!(manager.get_command("test").is_some());
        assert!(manager.get_command("nonexistent").is_none());
    }

    #[test]
    fn test_list_commands() {
        let mut manager = QADrawManager::new();
        manager.register_command(DrawCommand::new("cmd1".to_string(), "Cmd1".to_string()));
        manager.register_command(DrawCommand::new("cmd2".to_string(), "Cmd2".to_string()));

        let list = manager.list_commands();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"cmd1"));
    }
}
