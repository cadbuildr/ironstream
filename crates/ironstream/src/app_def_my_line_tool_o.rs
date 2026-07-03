// FILE: app_def_my_line_tool_o.rs
// occt: AppDef_MyLineTool
pub struct LineTool { pub length: f64 }
impl LineTool { pub fn new() -> Self { LineTool { length: 0.0 } } }
#[cfg(test)]
mod tests { use super::*; #[test]
fn test_create() { let _ = LineTool::new(); } }
