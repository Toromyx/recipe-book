use std::{io, path::PathBuf};

use tokio::fs::{File, OpenOptions};

pub async fn touch(path_buf: &PathBuf) -> Result<File, io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path_buf)
        .await?;
    Ok(file)
}
