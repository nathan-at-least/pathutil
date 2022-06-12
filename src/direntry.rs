use crate::PathMetadata;
use error_annotation::AnnotateResult;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType};
use std::io::Result;
use std::path::{Path, PathBuf};

/// A [DirEntry] with the originating [Path] for improved error messages.
///
/// This enables [std::io::Error] results to be annotated with the offending path.
#[derive(Debug)]
pub struct PathDirEntry<'a> {
    dirpath: &'a Path,
    de: DirEntry,
}

impl<'a> PathDirEntry<'a> {
    pub fn new(dirpath: &'a Path, de: DirEntry) -> Self {
        PathDirEntry { dirpath, de }
    }

    /// Access containing directories associated [Path].
    pub fn dir_path(&'a self) -> &'a Path {
        self.dirpath
    }

    /// Access associated [DirEntry].
    pub fn direntry(&self) -> &DirEntry {
        &self.de
    }

    /// Unwrap the underlying [DirEntry].
    pub fn unwrap(self) -> DirEntry {
        self.de
    }

    /// Return the [PathBuf] corresponding to this entry.
    pub fn path(&self) -> PathBuf {
        self.de.path()
    }

    /// Return the [PathMetadata] for this entry, annotating errors with the containing directory.
    ///
    /// The returned [PathMetadata] when successfully loaded is associated with the path within the
    /// directory.
    pub fn metadata(&self) -> Result<PathMetadata> {
        let metadata = self
            .de
            .metadata()
            .annotate_err_into("parent-dir", || self.dirpath.display())?;

        Ok(PathMetadata::new(self.path(), metadata))
    }

    /// Return the [FileType] for this entry, annotating any errors with the entry's path.
    pub fn file_type(&self) -> Result<FileType> {
        self.de
            .file_type()
            .annotate_err_into("path", || self.path().display().to_string())
    }

    /// Returns the bare file name of this directory entry without any other leading path
    /// component.
    pub fn file_name(&self) -> OsString {
        self.de.file_name()
    }
}
