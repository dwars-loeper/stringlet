//! `Display` and `Debug` for stringlet

use crate::*;

use core::fmt::{Debug, Display, Formatter, Result};

impl_for! {
    Display:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.pad(self.as_str())
    }
}

impl_for! {
    Debug:

    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        if fmt.alternate() {
            write!(
                fmt,
                "{} {{ '{}' ",
                core::any::type_name::<Self>(),
                Kind::ABBR as char
            )?;
            let len = self.len();
            if len < SIZE {
                write!(fmt, "len(): {len}, ")?;
            }
            write!(fmt, "[u8]: {:?}, ", self.str)?;
            if len < SIZE {
                write!(fmt, "str: [{:?}", self.as_str())?;
                for i in len..SIZE-1 {
                    write!(fmt, ", {}", self.str[i])?;
                }
                if Kind::POLY || Kind::BI {
                    write!(fmt, ", 0b11_{:06b}]", self.last() ^ TAG)?;
                } else {
                    write!(fmt, ", {}]", self.last())?;
                }
            } else {
                write!(fmt, "str: {:?}", self.as_str())?;
            }
        } else {
            fmt.write_str(Kind::NAME)?;
            if SIZE != 16 {
                write!(fmt, "<{SIZE}>")?;
            }
            write!(
                fmt,
                " {{ str: {:?}",
                self.as_str()
            )?;
        }
        fmt.write_str(" }")
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            TooLong(n) => write!(fmt, "len {n} too long"),
            TooShort(n) => write!(fmt, "len {n} too short"),
            Utf8Error(e) => write!(fmt, "{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn display() {
        let a = UniStringlet::<1>::from_str("a").unwrap();
        assert_eq!(a.to_string(), "a");
        assert_eq!(format!("|{a:-<3}|"), "|a--|");
        assert_eq!(format!("|{a:+>3}|"), "|++a|");
    }

    #[test]
    fn error_display() {
        assert_eq!(
            UniStringlet::<1>::from_str("").unwrap_err().to_string(),
            "len 0 too short"
        );
        assert_eq!(
            UniStringlet::<0>::from_str("a").unwrap_err().to_string(),
            "len 1 too long"
        );
        assert!(
            VarStringlet::from_utf8_bytes([200])
                .unwrap_err()
                .to_string()
                .contains("utf")
        );
    }
}
