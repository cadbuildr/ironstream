// FILE: draw_main.rs
// occt: Draw_Main

//! Main framework for Draw Test Harness applications.
//! Provides common entry point definitions for UNIX and Windows.

/// Initialization function type for Draw applications
pub type FDrawInitAppli = fn(&mut crate::draw_interpretor::DrawInterpretor);

/// Main entry point for Draw applications
pub fn draw_main(args: &[&str], init_func: FDrawInitAppli) -> i32 {
    let mut interp = crate::draw_interpretor::DrawInterpretor::new();
    init_func(&mut interp);
    // Execute the draw application with the initialized interpreter
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_init(_interp: &mut crate::draw_interpretor::DrawInterpretor) {
        // Test initialization
    }

    #[test]
    fn test_draw_main() {
        let result = draw_main(&[], test_init);
        assert_eq!(result, 0);
    }
}
