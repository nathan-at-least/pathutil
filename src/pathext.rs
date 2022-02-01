use error_annotation::AnnotateResult;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind::Other, Result};
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
        self.o2r(os.to_str(), "invalid utf8")
    }

    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>;

    fn pe_file_stem(&self) -> Result<&OsStr>;

    fn pe_file_stem_str(&self) -> Result<&str> {
        let os = self.pe_file_stem()?;
        self.o2r(os.to_str(), "invalid utf8")
    }

    fn pe_extension(&self) -> Result<&OsStr>;

    fn pe_extension_str(&self) -> Result<&str> {
        let os = self.pe_extension()?;
        self.o2r(os.to_str(), "invalid utf8")
    }

    #[doc(hidden)]
    fn o2r<T>(&self, opt: Option<T>, issue: &str) -> Result<T> {
        opt.ok_or_else(|| Error::new(Other, issue.to_string()))
            .annotate_err_into("path", || self.as_ref().display())
    }
}

impl PathExt for Path {
    fn pe_to_str(&self) -> Result<&str> {
        self.o2r(self.to_str(), "invalid utf8")
    }

    fn pe_parent(&self) -> Result<&Path> {
        self.o2r(self.parent(), "no parent path")
    }

    fn pe_file_name(&self) -> Result<&OsStr> {
        self.o2r(self.file_name(), "no filename")
    }

    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>,
    {
        let bref = base.as_ref();
        self.strip_prefix(bref)
            .map_err(|_| Error::new(Other, "prefix mismatch"))
            .annotate_err_into("prefix", || bref.display())
            .annotate_err_into("path", || self.display())
    }

    fn pe_file_stem(&self) -> Result<&OsStr> {
        self.o2r(self.file_stem(), "no filename")
    }

    fn pe_extension(&self) -> Result<&OsStr> {
        self.o2r(self.extension(), "no filename or no extension")
    }
}
