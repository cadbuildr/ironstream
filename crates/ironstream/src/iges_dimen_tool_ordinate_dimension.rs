// FILE: iges_dimen_tool_ordinate_dimension.rs
// occt: IGESDimen_dimentoolordinatedimension

pub struct IGESDimen_dimentoolordinatedimension;

impl IGESDimen_dimentoolordinatedimension {
    pub fn new() -> Self {
        IGESDimen_dimentoolordinatedimension
    }
}

impl Default for IGESDimen_dimentoolordinatedimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolordinatedimension::new();
    }
}
