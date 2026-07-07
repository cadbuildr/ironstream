// FILE: topo_ds_to_step_make_shell_based_surface_model.rs
// occt: TopoDSToStep_MakeShellBasedSurfaceModel

pub struct MakeShellBasedSurfaceModel {
    shell_based_surface_model: Option<ShellBasedSurfaceModel>,
    tessellated_item: Option<TessellatedItem>,
}

pub struct ShellBasedSurfaceModel;
pub struct TessellatedItem;

impl MakeShellBasedSurfaceModel {
    pub fn new() -> Self {
        MakeShellBasedSurfaceModel {
            shell_based_surface_model: None,
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&ShellBasedSurfaceModel> {
        self.shell_based_surface_model.as_ref()
    }

    pub fn tessellated_value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeShellBasedSurfaceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeShellBasedSurfaceModel::new();
        assert!(maker.value().is_none());
    }
}
