//! Provides a [PathExt] trait and impl for [std::path::Path] where error descriptions include the
//! offending path and other diagnostic information.
mod metadata;
mod pathext;

pub use self::metadata::PathMetadata;
pub use self::pathext::PathExt;
