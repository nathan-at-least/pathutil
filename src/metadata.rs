use crate::FileTypeEnum;
use error_annotation::AnnotateResult;
use std::borrow::Cow;
use std::fs::{FileType, Metadata, Permissions};
use std::io::Result;
use std::path::Path;
use std::time::SystemTime;

/// Extend [Metadata] with the originating [Path] for improved error messages.
///
/// This enables [std::io::Error] results to be annotated with the offending path.
#[derive(Debug)]
pub struct PathMetadata<'a> {
    path: Cow<'a, Path>,
    md: Metadata,
}

impl<'a> PathMetadata<'a> {
    /// Create a new `PathMetadata`.
    pub fn new<P>(path: P, md: Metadata) -> Self
    where
        Cow<'a, Path>: From<P>,
    {
        let path = Cow::from(path);
        PathMetadata { path, md }
    }

    /// Access associated [Path].
    pub fn path(&'a self) -> &'a Path {
        use std::borrow::Borrow;

        self.path.borrow()
    }

    /// Access associated [Metadata].
    pub fn metadata(&self) -> &Metadata {
        &self.md
    }

    /// Unwrap the underlying [Metadata].
    pub fn unwrap(self) -> Metadata {
        self.md
    }

    /// Identical to [Metadata::file_type].
    pub fn file_type(&self) -> FileType {
        self.md.file_type()
    }

    /// Identical to [Metadata::is_dir].
    pub fn is_dir(&self) -> bool {
        self.md.is_dir()
    }

    /// Identical to [Metadata::is_file].
    pub fn is_file(&self) -> bool {
        self.md.is_file()
    }

    /// Identical to [Metadata::is_symlink].
    pub fn is_symlink(&self) -> bool {
        self.md.is_symlink()
    }

    /// Identical to [Metadata::len].
    pub fn len(&self) -> u64 {
        self.md.len()
    }

    /// Returns `true` if the file has length 0.
    pub fn is_empty(&self) -> bool {
        self.md.len() == 0
    }

    /// Identical to [Metadata::permissions].
    pub fn permissions(&self) -> Permissions {
        self.md.permissions()
    }

    /// Annotate errors from [Metadata::modified] with the offending path.
    pub fn modified(&self) -> Result<SystemTime> {
        self.md
            .modified()
            .annotate_err_into("path", || self.path.display())
    }

    /// Annotate errors from [Metadata::accessed] with the offending path.
    pub fn accessed(&self) -> Result<SystemTime> {
        self.md
            .accessed()
            .annotate_err_into("path", || self.path.display())
    }

    /// Annotate errors from [Metadata::created] with the offending path.
    pub fn created(&self) -> Result<SystemTime> {
        self.md
            .created()
            .annotate_err_into("path", || self.path.display())
    }

    /// Return an error if the filetype does not match the expectation.
    pub fn require_file_type<T>(&self, expected: T) -> Result<()>
    where
        FileTypeEnum: From<T>,
    {
        let expfte: FileTypeEnum = expected.into();
        let found: FileTypeEnum = self.file_type().into();
        if found == expfte {
            Ok(())
        } else {
            Err(other_error_fmt!("found {:?}, expected {:?}", found, expfte))
        }
    }
}
