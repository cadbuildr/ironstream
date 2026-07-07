// FILE: iges_to_b_rep_tool_container.rs
// occt: IGESToBRep_ToolContainer

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepToolContainer;

impl IgesToBRepToolContainer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _container = IgesToBRepToolContainer::new();
    }
}
