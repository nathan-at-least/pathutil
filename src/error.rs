use error_annotation::{Annotatable, ErrorAnnotation};
use std::io::{Error, ErrorKind::Other};
use std::path::Path;

pub type Result<'a, T> = std::result::Result<T, BadPath<'a>>;

#[derive(Debug)]
pub struct BadPath<'a>(pub ErrorAnnotation<Reason, &'a Path>);

impl<'a> From<BadPath<'a>> for Error {
    fn from(bp: BadPath<'a>) -> Error {
        Error::merge_annotation(
            bp.0.map_source(Error::from)
                .map_info(|p| ("path", p.display())),
        )
    }
}

#[derive(Debug)]
pub enum Reason {
    InvalidUtf8,
    NoParent,
    NoFilename,
    NoFilenameOrNoExtension,
}

impl From<Reason> for Error {
    fn from(reason: Reason) -> Error {
        use Reason::*;

        Error::new(
            Other,
            String::from(match reason {
                InvalidUtf8 => "invalid utf8",
                NoParent => "no parent path",
                NoFilename => "no filename",
                NoFilenameOrNoExtension => "no filename or else no extension",
            }),
        )
    }
}
