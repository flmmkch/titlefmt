use super::Error;

/// abbr
pub mod abbr;

/// ascii
#[cfg(feature = "unicode-normalization")]
pub mod ascii;

/// caps
pub mod caps;

/// caps
pub mod caps2;

/// directory
pub mod directory;

/// directory_path
pub mod directory_path;

/// ext
pub mod ext;

/// filename
pub mod filename;

/// hex
pub mod hex;

/// insert
pub mod insert;

/// left
pub mod left;

/// right
pub mod right;

/// substr
pub mod substr;
