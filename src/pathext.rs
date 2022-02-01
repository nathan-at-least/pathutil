use error_annotation::AnnotateResult;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir};
use std::io::{Error, ErrorKind::Other, Result};
use std::path::{Path, PathBuf};

/// A trait to extend [std::path::Path] with methods (all prefixed with `pe_`) which include the
/// path in the [std::io::Error] description. Additionally [std::path::Path] methods which return
/// `Option` now return [std::io::Result] with [std::io::ErrorKind::Other] and a useful description
/// of the issue.
pub trait PathExt: AsRef<Path> {
    /// Returns the path as a utf8 `&str`, or the error explains "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = r#"
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
"#
    )]
    fn pe_to_str(&self) -> Result<&str>;

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
    fn pe_parent(&self) -> Result<&Path>;

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
    fn pe_file_name(&self) -> Result<&OsStr>;

    /// Returns the file name as a utf8 [&str], or the error explains "no file name" or else
    /// "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = r#"
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
"#
    )]
    fn pe_file_name_str(&self) -> Result<&str> {
        let os = self.pe_file_name()?;
        self.o2r(os.to_str(), "invalid utf8")
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
        P: AsRef<Path>;

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
    fn pe_file_stem(&self) -> Result<&OsStr>;

    /// Returns the file stem as a utf8 [&str], or the error explains "no file name" or else
    /// "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = r#"
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
"#
    )]
    fn pe_file_stem_str(&self) -> Result<&str> {
        let os = self.pe_file_stem()?;
        self.o2r(os.to_str(), "invalid utf8")
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
    fn pe_extension(&self) -> Result<&OsStr>;

    /// Returns the file extension as a utf8 [&str], or the error explains "no file name or no
    /// extension" or else "invalid utf8".
    ///
    #[cfg_attr(
        target_os = "linux",
        doc = r#"
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
"#
    )]
    fn pe_extension_str(&self) -> Result<&str> {
        let os = self.pe_extension()?;
        self.o2r(os.to_str(), "invalid utf8")
    }

    /// Return the path's [std::fs::Metadata] or include the path in the error description.
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
    fn pe_metadata(&self) -> Result<Metadata>;

    /// Return the symlink's [std::fs::Metadata] or include the path in the error description.
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
    fn pe_symlink_metadata(&self) -> Result<Metadata>;

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
    fn pe_canonicalize(&self) -> Result<PathBuf>;

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
    fn pe_read_link(&self) -> Result<PathBuf>;

    /// Read the directory at path or else include the path in the error description.
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
    fn pe_read_dir(&self) -> Result<ReadDir>;

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
        self.o2r(self.file_name(), "no file name")
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
        self.o2r(self.file_stem(), "no file name")
    }

    fn pe_extension(&self) -> Result<&OsStr> {
        self.o2r(self.extension(), "no file name or no extension")
    }

    fn pe_metadata(&self) -> Result<Metadata> {
        self.metadata().annotate_err_into("path", || self.display())
    }

    fn pe_symlink_metadata(&self) -> Result<Metadata> {
        self.symlink_metadata()
            .annotate_err_into("path", || self.display())
    }

    fn pe_canonicalize(&self) -> Result<PathBuf> {
        self.canonicalize()
            .annotate_err_into("path", || self.display())
    }

    fn pe_read_link(&self) -> Result<PathBuf> {
        self.read_link()
            .annotate_err_into("path", || self.display())
    }

    fn pe_read_dir(&self) -> Result<ReadDir> {
        self.read_dir().annotate_err_into("path", || self.display())
    }
}
