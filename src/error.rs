use std::io::{Error, ErrorKind::Other};

pub(crate) fn other_error(msg: String) -> Error {
    Error::new(Other, msg)
}

macro_rules! other_error_fmt {
    ( $tmpl:expr ) => {
        crate::other_error($tmpl.to_string())
    };

    ( $tmpl:expr, $( $a:expr ),* $(,)? ) => {
        crate::other_error(format!($tmpl, $( $a ),* ))
    }
}
