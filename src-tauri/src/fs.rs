//! This module implements functions related to the filesystem.

use std::{io, path::PathBuf};

use tokio::fs::{File, OpenOptions};

/// Create and/or open a file.
///
/// # Errors
///
/// - when [`OpenOptions::open`] errors
pub async fn touch(path_buf: &PathBuf) -> Result<File, io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path_buf)
        .await?;
    Ok(file)
}
