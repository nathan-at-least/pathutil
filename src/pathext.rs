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
        self.o2r(os.to_str(), Reason::InvalidUtf8)
    }

    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>;

    fn pe_file_stem(&self) -> Result<&OsStr>;

    fn pe_file_stem_str(&self) -> Result<&str> {
        let os = self.pe_file_stem()?;
        self.o2r(os.to_str(), Reason::InvalidUtf8)
    }

    fn pe_extension(&self) -> Result<&OsStr>;

    fn pe_extension_str(&self) -> Result<&str> {
        let os = self.pe_extension()?;
        self.o2r(os.to_str(), Reason::InvalidUtf8)
    }

    #[doc(hidden)]
    fn o2r<T>(&self, opt: Option<T>, reason: Reason) -> Result<T> {
        opt.ok_or(reason)
            .annotate_err("path", || self.as_ref())
            .map_err(BadPath)
    }
}

impl PathExt for Path {
    fn pe_to_str(&self) -> Result<&str> {
        self.o2r(self.to_str(), Reason::InvalidUtf8)
    }

    fn pe_parent(&self) -> Result<&Path> {
        self.o2r(self.parent(), Reason::NoParent)
    }

    fn pe_file_name(&self) -> Result<&OsStr> {
        self.o2r(self.file_name(), Reason::NoFilename)
    }

    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>,
    {
        self.strip_prefix(base)
            .map_err(|_| Reason::PrefixMismatch)
            .annotate_err("path", || self)
            .map_err(BadPath)
    }

    fn pe_file_stem(&self) -> Result<&OsStr> {
        self.o2r(self.file_stem(), Reason::NoFilename)
    }

    fn pe_extension(&self) -> Result<&OsStr> {
        self.o2r(self.extension(), Reason::NoFilenameOrNoExtension)
    }
}
