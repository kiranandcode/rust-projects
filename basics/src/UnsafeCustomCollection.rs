use std::ascii::AsciiExt;

/// Efficient string always containing valid ascii
#[derive(Debug, Eq, PartialEq)]
pub struct Ascii (Vec<u8>);

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
    }
}

// Error returned when trying to produce an ASCII string.
#[derive(Debug, Eq, PartialEq)]
pub struct NotAsciiError(pub Vec<u8>);




// converting ascii efficiently to ascii
impl From<Ascii> for String {
    fn from(ascii: Ascii) -> String {
        unsafe { String::from_utf8_unchecked(ascii.0)}
    }
}