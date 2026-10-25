use core::str::Utf8Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The str of length `n` is too long to fit in the given size.
    TooLong(usize),
    /// The str of length `n` is too short to be valid.
    TooShort(usize),
    Utf8Error(Utf8Error),
}

impl core::error::Error for Error {}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_error() {
        #[allow(invalid_from_utf8)]
        let utf8_error: Error = str::from_utf8(&[0xff]).unwrap_err().into();
        // TODO new macro once we update to 1.96 for something else
        assert!(matches!(utf8_error, Error::Utf8Error(_)));
    }
}
