// FILE: ais_o.rs
// occt: AIS

pub const AIS_VERSION: u32 = 1;

pub mod constants {
    pub const DEFAULT_WIDTH: u32 = 800;
    pub const DEFAULT_HEIGHT: u32 = 600;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(AIS_VERSION > 0);
    }
}
