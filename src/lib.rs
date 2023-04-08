#![doc = include_str!("../README.md")]

#[macro_use]
mod error;

mod direntry;
mod filetype;
mod metadata;
mod pathext;
mod readdir;

pub use self::direntry::PathDirEntry;
pub use self::filetype::FileTypeEnum;
pub use self::metadata::PathMetadata;
pub use self::pathext::PathExt;
pub use self::readdir::PathReadDir;

use self::error::other_error;
