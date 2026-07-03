// FILE: file.rs
// occt: Class
#[derive(Clone, Debug)]
pub struct MyStruct;
impl MyStruct {
    pub fn new() -> Self { Self }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { let _ = MyStruct::new(); }
}
