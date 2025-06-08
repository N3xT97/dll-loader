use std::path::PathBuf;
use thiserror::Error as ThisError;
use windows::core::Error as WinError;

#[derive(Debug, ThisError)]
pub enum DllLoaderError {
    #[error("PathNotFound: {0}")]
    PathNotFound(PathBuf),
    #[error("NotAFile: {0}")]
    NotAFile(PathBuf),
    #[error("WindowsApi: {0}")]
    WindowsApi(#[from] WinError),
    #[error("CanonicalizeFailed: {0}")]
    CanonicalizeFailed(PathBuf),
    #[error("LibraryLoadFailed: {0}")]
    LibraryLoadFailed(PathBuf),
}
