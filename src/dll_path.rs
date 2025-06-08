use super::error::DllLoaderError;
use std::{fs, os::windows::ffi::OsStrExt, path::PathBuf};
use windows::core::PCWSTR;

#[derive(Debug)]
pub struct DllPath {
    path_buf: PathBuf,
}
impl DllPath {
    pub fn new(path: &str) -> Result<Self, DllLoaderError> {
        let path = PathBuf::from(path);
        if !path.exists() {
            return Err(DllLoaderError::PathNotFound(path));
        }
        if !path.is_file() {
            return Err(DllLoaderError::NotAFile(path));
        }

        let dll_path = Self { path_buf: path };
        Ok(dll_path)
    }

    pub fn to_pcwstr(&self) -> (PCWSTR, Vec<u16>) {
        let wide_dll_path: Vec<u16> = self
            .path_buf
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        (PCWSTR(wide_dll_path.as_ptr()), wide_dll_path)
    }

    pub fn full_path(&self) -> Result<PathBuf, DllLoaderError> {
        fs::canonicalize(&self.path_buf)
            .map_err(|_| DllLoaderError::CanonicalizeFailed(self.path_buf.clone()))
    }
}
