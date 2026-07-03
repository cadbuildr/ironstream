// FILE: graphic3d_type_of_connection.rs
// occt: Graphic3d_TypeOfConnection

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfConnection {
    Ancestor = 0,
    Descendant = 1,
}

impl TypeOfConnection {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfConnection::Ancestor), 1 => Some(TypeOfConnection::Descendant), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() {
        assert_eq!(TypeOfConnection::Ancestor as u32, 0);
    }
}
