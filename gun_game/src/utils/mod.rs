use std::{ffi::CString, path::Path, ptr};
use once_cell::sync::OnceCell;
use winapi::{ shared::minwindef::{HMODULE, MAX_PATH}, um::{errhandlingapi::GetLastError, libloaderapi::{GetModuleFileNameA, GetModuleHandleExA, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT}}};

pub struct Utils;

#[allow(unused)]
impl Utils {

    // Fetch dll path once it's called and store it in as static value 
    // in case of additional calls
    pub unsafe fn dll_path() -> &'static String {

        // Static value using OnceCell
        static DLL_PATH: OnceCell<String> = OnceCell::new();

        // Get the value or initate it if first time called
        DLL_PATH.get_or_init(|| {

            // This solution is based on https://stackoverflow.com/a/6924332, but in rust
            let mut path: [u8; MAX_PATH] = [0; MAX_PATH];
            let mut hm: HMODULE = ptr::null_mut();
            
            if GetModuleHandleExA(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                Utils::dll_path as *const i8 as *mut _,
                &mut hm as *mut _,
            ) == 0 {
                let ret = GetLastError();
                panic!("Failed to find dll path, GetModuleHandleExA error: {}", ret);
            }
    
            if GetModuleFileNameA(hm, path.as_mut_ptr() as *mut i8, MAX_PATH as u32) == 0 {
                let ret = GetLastError();
                panic!("Failed to find dll path, GetModuleFileNameA error: {}", ret);
            }
    
            let path = CString::from_vec_unchecked(path.to_vec());
            let path = path.to_string_lossy();
    
            if path.is_empty() {
                panic!("Failed to find dll path, Path is empty!");
            }
    
            Path::new(&path.to_string()).parent().unwrap().to_str().unwrap().to_string()
        })
    }
}

