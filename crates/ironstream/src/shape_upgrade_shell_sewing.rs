// FILE: shape_upgrade_shell_sewing.rs
// occt: ShapeUpgrade_ShellSewing

pub struct ShapeUpgradeShellSewing;

impl ShapeUpgradeShellSewing {
    pub fn new() -> Self {
        ShapeUpgradeShellSewing
    }
}

impl Default for ShapeUpgradeShellSewing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeShellSewing;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeShellSewing::new();
    }
}
