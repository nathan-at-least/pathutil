use error_annotation::AnnotateResult;
use std::io::{Error, ErrorKind::Other, Result};

/// A pub-in-priv trait for helper methods.
pub trait PathExtPriv {
    fn pep_display(&self) -> std::path::Display;

    fn o2r<T>(&self, opt: Option<T>, errordesc: &str) -> Result<T> {
        opt.ok_or_else(|| Error::new(Other, errordesc.to_string()))
            .annotate_err_into("path", || self.pep_display())
    }
}
