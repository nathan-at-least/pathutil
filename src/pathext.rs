use crate::{BadPath, Reason, Result};
use error_annotation::AnnotateResult;
use std::ffi::OsStr;
use std::path::Path;

/// A trait to extend [std::path::Path] with methods (all prefixed with `pe_`) that return `Results` rather than `Option` in
/// a variety of cases.
///
/// User code can manage [BadPath] errors directly or can convert to [std::io::Error] via [From].
pub trait PathExt: AsRef<Path> {
    fn pe_to_str(&self) -> Result<&str>;
    fn pe_parent(&self) -> Result<&Path>;
    fn pe_file_name(&self) -> Result<&OsStr>;

    fn pe_file_name_str(&self) -> Result<&str> {
        let os = self.pe_file_name()?;
        os.to_str()
            .ok_or(Reason::InvalidUtf8)
            .annotate_err("path", || self.as_ref())
            .map_err(BadPath)
    }

    fn pe_file_stem(&self) -> Result<&OsStr>;

    fn pe_file_stem_str(&self) -> Result<&str> {
        let os = self.pe_file_stem()?;
        os.to_str()
            .ok_or(Reason::InvalidUtf8)
            .annotate_err("path", || self.as_ref())
            .map_err(BadPath)
    }

    fn pe_extension(&self) -> Result<&OsStr>;

    fn pe_extension_str(&self) -> Result<&str> {
        let os = self.pe_extension()?;
        os.to_str()
            .ok_or(Reason::InvalidUtf8)
            .annotate_err("path", || self.as_ref())
            .map_err(BadPath)
    }
}

impl PathExt for Path {
    fn pe_to_str(&self) -> Result<&str> {
        use crate::Reason::InvalidUtf8;
        self.to_str()
            .ok_or(InvalidUtf8)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }

    fn pe_parent(&self) -> Result<&Path> {
        use crate::Reason::NoParent;
        self.parent()
            .ok_or(NoParent)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }

    fn pe_file_name(&self) -> Result<&OsStr> {
        use crate::Reason::NoFilename;
        self.file_name()
            .ok_or(NoFilename)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }

    fn pe_file_stem(&self) -> Result<&OsStr> {
        use crate::Reason::NoFilename;
        self.file_stem()
            .ok_or(NoFilename)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }

    fn pe_extension(&self) -> Result<&OsStr> {
        use crate::Reason::NoFilenameOrNoExtension;
        self.extension()
            .ok_or(NoFilenameOrNoExtension)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }
}
