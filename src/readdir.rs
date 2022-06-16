use crate::PathDirEntry;
use error_annotation::AnnotateResult;
use std::fs::ReadDir;
use std::io::Result;
use std::path::Path;

/// A [ReadDir] with the originating [Path] for improved error messages.
///
/// This enables [std::io::Error] results to be annotated with the offending path.
#[derive(Debug)]
pub struct PathReadDir<'a> {
    path: &'a Path,
    rd: ReadDir,
}

impl<'a> PathReadDir<'a> {
    pub fn new(path: &'a Path, rd: ReadDir) -> Self {
        PathReadDir { path, rd }
    }

    /// Access associated [Path].
    pub fn path(&'a self) -> &'a Path {
        self.path
    }

    /// Access associated [ReadDir].
    pub fn readdir(&self) -> &ReadDir {
        &self.rd
    }

    /// Unwrap the underlying [ReadDir].
    pub fn unwrap(self) -> ReadDir {
        self.rd
    }
}

impl<'a> Iterator for PathReadDir<'a> {
    type Item = Result<PathDirEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rd.next().map(|de| {
            de.annotate_err_into("path", || self.path.display())
                .map(|de| PathDirEntry::new(self.path, de))
        })
    }
}
