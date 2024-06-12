use std::io::{Error, ErrorKind, Result};

/// Attempts to convert bytes in a buffer to string
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    }
}
