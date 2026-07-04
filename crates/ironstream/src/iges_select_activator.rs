// FILE: iges_select_activator.rs
// occt: IGESSelect_Activator

pub struct IGESSelectActivator;

impl IGESSelectActivator {
    pub fn new() -> Self {
        IGESSelectActivator
    }

    pub fn do_command(&self, _number: i32) -> i32 {
        0
    }

    pub fn help(&self, _number: i32) -> &'static str {
        ""
    }
}

impl Default for IGESSelectActivator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _activator = IGESSelectActivator::new();
    }
}
