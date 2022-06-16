use crate::privtrait::PathExtPriv;
use crate::{PathExt, PathMetadata, PathReadDir};
use error_annotation::AnnotateResult;
use std::ffi::OsStr;
use std::io::Result;
use std::path::{Path, PathBuf};

impl PathExtPriv for Path {
    fn pep_display(&self) -> std::path::Display {
        self.display()
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
        self.o2r(self.file_name(), "no file name")
    }

    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>,
    {
        let bref = base.as_ref();
        self.strip_prefix(bref)
            .map_err(|_| other_error_fmt!("prefix mismatch"))
            .annotate_err_into("prefix", || bref.display())
            .annotate_err_into("path", || self.display())
    }

    fn pe_file_stem(&self) -> Result<&OsStr> {
        self.o2r(self.file_stem(), "no file name")
    }

    fn pe_extension(&self) -> Result<&OsStr> {
        self.o2r(self.extension(), "no file name or no extension")
    }

    fn pe_metadata(&self) -> Result<PathMetadata> {
        self.metadata()
            .annotate_err_into("path", || self.display())
            .map(|md| PathMetadata::new(self, md))
    }

    fn pe_symlink_metadata(&self) -> Result<PathMetadata> {
        self.symlink_metadata()
            .annotate_err_into("path", || self.display())
            .map(|md| PathMetadata::new(self, md))
    }

    fn pe_canonicalize(&self) -> Result<PathBuf> {
        self.canonicalize()
            .annotate_err_into("path", || self.display())
    }

    fn pe_read_link(&self) -> Result<PathBuf> {
        self.read_link()
            .annotate_err_into("path", || self.display())
    }

    fn pe_read_dir(&self) -> Result<PathReadDir> {
        self.read_dir()
            .map(|rd| PathReadDir::new(self, rd))
            .annotate_err_into("path", || self.display())
    }
}
