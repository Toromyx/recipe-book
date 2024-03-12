//! This module handles storage of binary files like images.

use std::path::PathBuf;

use tokio::fs;
use uuid::Uuid;

use crate::{file_storage::error::FileStorageError, path::app_data_dir};

pub mod error;

/// Creates a new file by copying from the source path.
///
/// It returns the new path.
///
/// # Errors
///
/// - [`FileStorageError::Io`] when there is an I/O error while writing the file
pub async fn create(source_path: &str, mime: &str) -> Result<PathBuf, FileStorageError> {
    let target_path = generate_file_path(mime);
    let source_path = PathBuf::from(&source_path);
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::copy(&source_path, &target_path).await?;
    Ok(target_path)
}

/// Deletes the stored file associated with a managed path.
///
/// Also delete all empty parent directories up to and including [`root_dir`].
///
/// This will not delete anything which is not inside [`root_dir`].
///
/// # Errors
///
/// - [`FileStorageError::Io`] when there is an I/O error while deleting the file
pub async fn delete(path: &str) -> Result<(), FileStorageError> {
    let path = PathBuf::from(path);
    if !path.starts_with(root_dir()) {
        log::warn!(
            "Tried to delete path {:?} which is not managed by file storage.",
            path
        );
        return Ok(());
    }
    fs::remove_file(&path).await?;
    let mut dir_option = path.parent();
    while let Some(dir) = dir_option {
        if !dir.starts_with(root_dir()) || fs::remove_dir(dir).await.is_err() {
            break;
        }
        dir_option = dir.parent();
    }
    Ok(())
}

/// Get the file storage root directory.
fn root_dir() -> PathBuf {
    let mut dir = app_data_dir();
    dir.push("files");
    dir
}

/// Generate a filename based on a mime type.
fn generate_file_name(mime: &str) -> String {
    let name = Uuid::new_v4();
    match mime2ext::mime2ext(mime) {
        Some(extension) => format!("{}.{extension}", name),
        None => name.to_string(),
    }
}

/// Generate a file path base on a mime type.
fn generate_file_path(mime: &str) -> PathBuf {
    let mut file_path = root_dir();
    file_path.push(generate_file_name(mime));
    file_path
}

#[cfg(test)]
mod tests {
    use mime_guess::mime::APPLICATION_OCTET_STREAM;

    use super::*;
    use crate::tests::{create_temp_file, TEST_NAME};
    #[tokio::test]
    async fn test_create() {
        TEST_NAME.set(Some("file_storage__test_create".to_string()));
        crate::tests::run();

        let expected_content = "content";
        let temp_path = create_temp_file("file_storage__test_create.bin", expected_content);
        assert!(!temp_path.starts_with(root_dir()));
        let path = create(
            &temp_path.to_string_lossy(),
            APPLICATION_OCTET_STREAM.essence_str(),
        )
        .await
        .unwrap();
        assert!(path.starts_with(root_dir()));
        let content = std::fs::read(&path).unwrap();
        assert_eq!(&content, expected_content.as_bytes());

        TEST_NAME.set(None);
    }

    #[tokio::test]
    async fn test_delete() {
        TEST_NAME.set(Some("file_storage__test_delete".to_string()));
        crate::tests::run();

        let mut folder_path = root_dir();
        folder_path.push("test_delete");
        let mut file_path = folder_path.clone();
        file_path.push("test_delete.bin");
        fs::create_dir_all(&folder_path).await.unwrap();
        fs::write(&file_path, "").await.unwrap();
        delete(&file_path.to_string_lossy()).await.unwrap();
        assert!(fs::metadata(&file_path).await.is_err());
        assert!(fs::metadata(&folder_path).await.is_err());

        TEST_NAME.set(None);
    }

    #[tokio::test]
    async fn test_delete_invalid() {
        crate::tests::run();

        let temp_path = create_temp_file("file_storage__test_delete_invalid.bin", "");
        delete(&temp_path.to_string_lossy()).await.unwrap();
        assert!(fs::metadata(&temp_path).await.is_ok());
    }
}
