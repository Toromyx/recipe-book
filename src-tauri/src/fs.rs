//! This module implements functions related to the filesystem.

use std::{io, io::ErrorKind, path::PathBuf};

use tokio::fs::OpenOptions;

/// Create a file if it doesn't already exist.
///
/// # Errors
///
/// - when [`OpenOptions::open`] errors
pub async fn touch(path_buf: &PathBuf) -> Result<(), io::Error> {
    match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path_buf)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(err),
        },
    }?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        fs::touch,
        tests::{create_temp_file, temp_file_path},
    };

    #[tokio::test]
    async fn test_touch_creates_new() {
        let file_path = temp_file_path("test_touch_creates_new");
        std::fs::remove_file(&file_path).ok();
        assert!(!file_path.exists());
        touch(&file_path).await.unwrap();
        assert!(file_path.exists());
    }

    #[tokio::test]
    async fn test_touch_not_overwrites_existing() {
        let expected_content = "my content\n\r";
        let file_path = create_temp_file("test_touch_not_overwrites_existing", expected_content);
        assert!(file_path.exists());
        touch(&file_path).await.unwrap();
        assert!(
            std::fs::read(file_path)
                .is_ok_and(|content| content.as_slice() == expected_content.as_bytes())
        )
    }
}
