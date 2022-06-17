use crate::{other_error, PathDirEntry, PathMetadata, PathReadDir};
use error_annotation::AnnotateResult;
use indoc::indoc;
use std::ffi::OsStr;
use std::io::Result;
use std::path::{Path, PathBuf};

/// A trait to extend [std::path::Path] with error and [std::fs] operation improvements.
///
/// - All [std::path::Path] methods which return either `Option<T>` or `std::io::Result<T>`
/// are extended by [PathExt] with a `pe_…` prefix for disambiguation.
/// - All [PathExt] method errors are [std::io::Error] with included diagnostic information. This
/// always includes the path itself, and sometimes additional information, as the
/// [PathExt::pe_strip_prefix] example demonstrates.
/// - All [PathExt] methods which return `&OsStr` also have an associated `pe_…_str` method which
/// returns `&str` and performs utf8 conversion, or describing the utf8 conversion failure on
/// error. An example is [PathExt::pe_file_name_str].
pub trait PathExt: AsRef<Path> {
    /// Returns the path as a utf8 `&str`, or the error explains "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = indoc! {r#"
            # Example

            ```
            use pathutil::PathExt;
            use std::path::Path;
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let p = Path::new(OsStr::from_bytes(&[255u8]));
            let res = p.pe_to_str();
            assert!(res.is_err());

            let errstr = res.err().unwrap().to_string();
            assert_eq!(&errstr, "

            invalid utf8
            -with path: �

            ".trim());
            ```
        "#}
    )]
    fn pe_to_str(&self) -> Result<&str> {
        let path = self.as_ref();
        o2r(path, path.to_str(), "invalid utf8")
    }

    /// Returns the parent path, or the error explains "no parent".
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let pb = std::path::PathBuf::from("/");
    /// let res = pb.pe_parent();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// no parent path
    /// -with path: /
    ///
    /// ".trim());
    /// ```
    fn pe_parent(&self) -> Result<&Path> {
        let path = self.as_ref();
        o2r(path, path.parent(), "no parent path")
    }

    /// Returns the file name [std::ffi::OsStr], or the error explains "no file name".
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let pb = std::path::PathBuf::from("/tmp/..");
    /// let res = pb.pe_file_name();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// no file name
    /// -with path: /tmp/..
    ///
    /// ".trim());
    /// ```
    fn pe_file_name(&self) -> Result<&OsStr> {
        let path = self.as_ref();
        o2r(path, path.file_name(), "no file name")
    }

    /// Returns the file name as a utf8 [&str], or the error explains "no file name" or else
    /// "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = indoc! {r#"
            # Example

            ```
            use pathutil::PathExt;
            use std::path::Path;
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let p = Path::new(OsStr::from_bytes(&[255u8]));
            let res = p.pe_file_name_str();
            assert!(res.is_err());

            let errstr = res.err().unwrap().to_string();
            assert_eq!(&errstr, "

            invalid utf8
            -with path: �

            ".trim());
            ```
        "#}
    )]
    fn pe_file_name_str(&self) -> Result<&str> {
        let path = self.as_ref();
        let os = self.pe_file_name()?;
        o2r(path, os.to_str(), "invalid utf8")
    }

    /// Strip a given prefix from a path, or if the path does not begin with the prefix, describe
    /// both the prefix and the path.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let path = std::path::Path::new("/tmp/foo.txt");
    /// let prefix = std::path::Path::new("/temp/");
    /// let res = path.pe_strip_prefix(prefix);
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// prefix mismatch
    /// -with prefix: /temp/
    /// -with path: /tmp/foo.txt
    ///
    /// ".trim());
    /// ```
    fn pe_strip_prefix<P>(&self, base: P) -> Result<&Path>
    where
        P: AsRef<Path>,
    {
        let path = self.as_ref();
        let bref = base.as_ref();
        path.strip_prefix(bref)
            .map_err(|_| other_error_fmt!("prefix mismatch"))
            .annotate_err_into("prefix", || bref.display())
            .annotate_err_into("path", || path.display())
    }

    /// Returns the file stem [std::ffi::OsStr], or the error explains "no file name".
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/tmp/..");
    /// let res = p.pe_file_stem();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// no file name
    /// -with path: /tmp/..
    ///
    /// ".trim());
    /// ```
    fn pe_file_stem(&self) -> Result<&OsStr> {
        let path = self.as_ref();
        o2r(path, path.file_stem(), "no file name")
    }

    /// Returns the file stem as a utf8 [&str], or the error explains "no file name" or else
    /// "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = indoc! {r#"
            # Example

            ```
            use pathutil::PathExt;
            use std::path::Path;
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let p = Path::new(OsStr::from_bytes(&[255u8]));
            let res = p.pe_file_stem_str();
            assert!(res.is_err());

            let errstr = res.err().unwrap().to_string();
            assert_eq!(&errstr, "

            invalid utf8
            -with path: �

            ".trim());
            ```
        "#}
    )]
    fn pe_file_stem_str(&self) -> Result<&str> {
        let path = self.as_ref();
        let os = self.pe_file_stem()?;
        o2r(path, os.to_str(), "invalid utf8")
    }

    /// Return the extension [std::ffi::OsStr] or else describe there is no extension.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/tmp/no_extension");
    /// let res = p.pe_extension();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// no file name or no extension
    /// -with path: /tmp/no_extension
    ///
    /// ".trim());
    /// ```
    fn pe_extension(&self) -> Result<&OsStr> {
        let path = self.as_ref();
        o2r(path, path.extension(), "no file name or no extension")
    }

    /// Returns the file extension as a utf8 [&str], or the error explains "no file name or no
    /// extension" or else "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = indoc! {r#"
            # Example

            ```
            use pathutil::PathExt;
            use std::path::Path;
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let p = Path::new(OsStr::from_bytes(&[b'f', b'.', 255u8]));
            let res = p.pe_extension_str();
            assert!(res.is_err());

            let errstr = res.err().unwrap().to_string();
            assert_eq!(&errstr, "

            invalid utf8
            -with path: f.�

            ".trim());
            ```
        "#}
    )]
    fn pe_extension_str(&self) -> Result<&str> {
        let path = self.as_ref();
        let os = self.pe_extension()?;
        o2r(path, os.to_str(), "invalid utf8")
    }

    /// Return the path's [PathMetadata] or include the path in the error description.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/this/path/does/not/exist");
    /// let res = p.pe_metadata();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// No such file or directory (os error 2)
    /// -with path: /this/path/does/not/exist
    ///
    /// ".trim());
    /// ```
    fn pe_metadata(&self) -> Result<PathMetadata> {
        let path = self.as_ref();
        path.metadata()
            .annotate_err_into("path", || path.display())
            .map(|md| PathMetadata::new(path, md))
    }

    /// Return the symlink's [PathMetadata] or include the path in the error description.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/this/path/does/not/exist");
    /// let res = p.pe_symlink_metadata();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// No such file or directory (os error 2)
    /// -with path: /this/path/does/not/exist
    ///
    /// ".trim());
    /// ```
    fn pe_symlink_metadata(&self) -> Result<PathMetadata> {
        let path = self.as_ref();
        path.symlink_metadata()
            .annotate_err_into("path", || path.display())
            .map(|md| PathMetadata::new(path, md))
    }

    /// Return the canonicalized path or else include the path in the error description.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/this/path/does/not/exist");
    /// let res = p.pe_symlink_metadata();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// No such file or directory (os error 2)
    /// -with path: /this/path/does/not/exist
    ///
    /// ".trim());
    /// ```
    fn pe_canonicalize(&self) -> Result<PathBuf> {
        let path = self.as_ref();
        path.canonicalize()
            .annotate_err_into("path", || path.display())
    }

    /// Return the symlink's referent path or else include the path in the error description.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/this/path/does/not/exist");
    /// let res = p.pe_read_link();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// No such file or directory (os error 2)
    /// -with path: /this/path/does/not/exist
    ///
    /// ".trim());
    /// ```
    fn pe_read_link(&self) -> Result<PathBuf> {
        let path = self.as_ref();
        path.read_link()
            .annotate_err_into("path", || path.display())
    }

    /// Start reading the directory at path or else include the path in the error description.
    ///
    /// # Example
    ///
    /// ```
    /// use pathutil::PathExt;
    ///
    /// let p = std::path::Path::new("/this/path/does/not/exist");
    /// let res = p.pe_read_dir();
    /// assert!(res.is_err());
    ///
    /// let errstr = res.err().unwrap().to_string();
    /// assert_eq!(&errstr, "
    ///
    /// No such file or directory (os error 2)
    /// -with path: /this/path/does/not/exist
    ///
    /// ".trim());
    /// ```
    fn pe_read_dir(&self) -> Result<PathReadDir> {
        let path = self.as_ref();
        path.read_dir()
            .map(|rd| PathReadDir::new(path, rd))
            .annotate_err_into("path", || path.display())
    }

    /// Read the directory, collecting the entries, or return an error.
    fn pe_read_dir_entries(&self) -> Result<Vec<PathDirEntry>> {
        self.pe_read_dir()?.collect()
    }

    // Todo: other functions in `std::fs` in documentation order.
    /// Read to a string.
    fn pe_read_to_string(&self) -> Result<String> {
        let path = self.as_ref();
        std::fs::read_to_string(path).annotate_err_into("path", || path.display())
    }
}

fn o2r<T>(path: &Path, opt: Option<T>, errordesc: &str) -> Result<T> {
    opt.ok_or_else(|| other_error(errordesc.to_string()))
        .annotate_err_into("path", || path.display())
}

impl<P> PathExt for P where P: AsRef<Path> {}
