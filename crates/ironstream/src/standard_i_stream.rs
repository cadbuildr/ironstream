// FILE: standard_i_stream.rs
// occt: Standard_IStream

//! Standard_IStream is a type alias for std::io::Read in Rust.
//! It represents an input stream interface compatible with C++ std::istream.
//! This module provides Rust-idiomatic wrappers around stream reading operations.

use std::io::{self, Read};

/// Represents an input stream (Rust equivalent of C++ std::istream).
/// This is a type alias for any type implementing std::io::Read.
pub type StandardIStream = Box<dyn Read>;

/// Helper function to create a StandardIStream from a reader.
pub fn from_reader<R: Read + 'static>(reader: R) -> StandardIStream {
    Box::new(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_istream_from_cursor() {
        let data = b"Hello, World!";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = from_reader(cursor);

        let mut buffer = [0u8; 5];
        let n = stream.read(&mut buffer).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buffer, b"Hello");
    }

    #[test]
    fn test_istream_read_full() {
        let data = b"test";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = from_reader(cursor);

        let mut buf = Vec::new();
        let n = stream.read_to_end(&mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf, b"test");
    }

    #[test]
    fn test_istream_eof() {
        let data = b"";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = from_reader(cursor);

        let mut buffer = [0u8; 1];
        let n = stream.read(&mut buffer).unwrap();
        assert_eq!(n, 0);
    }
}
