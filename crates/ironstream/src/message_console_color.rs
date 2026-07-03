// FILE: message_console_color.rs
// occt: Message_ConsoleColor

/// Console color enumeration for colored output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageConsoleColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    Default = 8,
}

impl MessageConsoleColor {
    pub fn to_ansi_code(&self) -> &'static str {
        match self {
            MessageConsoleColor::Black => "\x1b[30m",
            MessageConsoleColor::Red => "\x1b[31m",
            MessageConsoleColor::Green => "\x1b[32m",
            MessageConsoleColor::Yellow => "\x1b[33m",
            MessageConsoleColor::Blue => "\x1b[34m",
            MessageConsoleColor::Magenta => "\x1b[35m",
            MessageConsoleColor::Cyan => "\x1b[36m",
            MessageConsoleColor::White => "\x1b[37m",
            MessageConsoleColor::Default => "\x1b[0m",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_codes() {
        assert_eq!(MessageConsoleColor::Red.to_ansi_code(), "\x1b[31m");
        assert_eq!(MessageConsoleColor::Green.to_ansi_code(), "\x1b[32m");
        assert_eq!(MessageConsoleColor::Default.to_ansi_code(), "\x1b[0m");
    }
}
