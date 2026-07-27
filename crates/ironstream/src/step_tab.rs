// FILE: step_tab.rs
// occt-note: step.tab

/// Bison parser tokens and symbols for STEP file parsing.
/// This is a Rust representation of the auto-generated Bison parser header.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Yyempty,
    Yyeof,
    YyError,
    Yyundef,
    Step,
    Header,
    Endsec,
    Data,
    Endstep,
    Scope,
    Endscope,
    Entity,
    Type,
    Integer,
    Float,
    Ident,
    Text,
    Nondef,
    Enum,
    Hexa,
    Quid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_kinds() {
        assert_ne!(TokenKind::Step, TokenKind::Data);
        assert_eq!(TokenKind::Yyempty, TokenKind::Yyempty);
    }
}
