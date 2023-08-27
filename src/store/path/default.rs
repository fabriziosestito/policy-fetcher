use anyhow::Result;
use std::path::{Path, PathBuf};

/// Encode a path to a format that doesn't contain any invalid characters
/// for the target platform.
/// This is the default implementation for non-Windows platforms,
/// which just returns the path as-is.
pub fn encode_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().to_path_buf()
}

/// Retrieve a path that was transformed with `transform_path`.
pub fn decode_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    Ok(path.as_ref().to_path_buf())
}
