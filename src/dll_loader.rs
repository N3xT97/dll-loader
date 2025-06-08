use super::dll_path::DllPath;
use super::error::DllLoaderError;
use windows::Win32::System::LibraryLoader::LoadLibraryW;

pub struct DllLoader;
impl DllLoader {
    pub fn run_with_loop(dll_path: DllPath) -> Result<(), DllLoaderError> {
        let (dll_path_to_pcwstr, _) = dll_path.to_pcwstr();
        let h_mod = unsafe { LoadLibraryW(dll_path_to_pcwstr) }?;
        if h_mod.is_invalid() {
            return Err(DllLoaderError::LibraryLoadFailed(dll_path.full_path()?));
        }
        let full_path = dll_path.full_path()?;
        println!("[*] \"{}\" Loaded with loop...", full_path.display());

        loop {}
    }
}
