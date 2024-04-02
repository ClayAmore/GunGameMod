use std::{ffi::c_void, path::Path};
use regex::Regex;
use widestring::{WideCStr, WideString};
use winapi::{ shared::{basetsd::UINT64, minwindef::DWORD}, um::{fileapi::CreateFileW, minwinbase::LPSECURITY_ATTRIBUTES, winnt::{HANDLE, LPCWSTR, WCHAR}}};

/* 
This is mostly ModEngine2 file mod_loader extension in rust.
https://github.com/soulsmods/ModEngine2/tree/main/src/modengine/ext/mod_loader
ModEngine2 is the recommended tool for file modding.

I needed file loading as it's own dll in order to inject it into EldenRing after Seamless 2.0 
launches EldenRing. That way I don't distrupt anything Seamless does but will still be able to 
load mod files like custom maps, action script, regulation etc...
*/ 
use crate::config::read::mod_files;

// A type for string used in VirtualToArchive path function
#[repr(C)]
pub struct DLString {
    unk: *const c_void,
    string: *mut WCHAR,
    unk2: *const c_void,
    length: UINT64,
    capacity: UINT64,
}

// Virtaul To Archive Hook for overrding virtual paths in order to make 
// Elden Ring attempt to fetch game files from game folder. Which will be intercepted and 
// changed to mod file paths in the FileWriteHook 
pub unsafe extern "system" fn virtual_to_archive_hook(
    path: *const DLString, 
    p2: UINT64, 
    p3: UINT64, 
    p4: UINT64, 
    p5: UINT64, 
    p6: UINT64
) -> *const c_void {

    let ptr = VIRUTAL_TO_ARCHIVE_OG_FN as *const ();
    let code: unsafe extern "system" fn(*const DLString, UINT64, UINT64, UINT64, UINT64, UINT64) -> *const c_void = std::mem::transmute(ptr);
    let ret = (code)(path, p2, p3, p4, p5, p6);
    let path_str = WideCStr::from_ptr_str((&*path).string).to_string_lossy();
    
    if let Some(name_os_str) = Path::new(&path_str).file_name() {
        if let Some(name_str) = name_os_str.to_str() {
            if mod_files().get(name_str).is_some() {
                let re = Regex::new(r"(data.:\/).+").unwrap();
                if let Some(caputres) = re.captures(&path_str) {
                    // println!("virtual_to_archive_hook Matched: {}", path_str);
                    let prefix_len = caputres[1].len();

                    *(&*path).string.offset(0) = '.' as u16;

                    for i in 1..std::cmp::min(prefix_len as isize, (&*path).length as isize) {
                        *(&*path).string.offset(i) = '/' as u16;
                    }
                }
            };
        }
    }

    ret
}
pub static mut VIRUTAL_TO_ARCHIVE_OG_FN: unsafe extern "system" fn(*const DLString, UINT64, UINT64, UINT64, UINT64, UINT64) -> *const c_void = virtual_to_archive_hook;

// Windows API FileWriteHook for overriding game file paths to game mod file paths.
pub unsafe fn file_write_hook(
    lp_file_name: LPCWSTR , 
    dw_desired_access: DWORD , 
    dw_share_mode: DWORD ,
    lp_security_attributes: LPSECURITY_ATTRIBUTES , 
    dw_creation_disposition: DWORD , 
    dw_flags_and_attributes: DWORD ,
    h_template_file: HANDLE
) -> HANDLE {
    
    let path_wide_str = WideCStr::from_ptr_str(lp_file_name);
    let path_str = path_wide_str.to_string_lossy();

    let mut output_lp_file_name = lp_file_name;
    if !path_str.is_empty() {
        let name_os_str = Path::new(&path_str).file_name();
        if name_os_str.is_none() { panic!("Failed to get file_name OsString from lp_file_name");}
        
        let name_str = name_os_str.unwrap().to_str();
        if name_str.is_none() { panic!("Failed to convert file_name OsString to Str!");}

        if let Some(mod_file_path) = mod_files().get(name_str.unwrap()) {
            // println!("file_write_hook Matched: {}" , mod_file_path);
            let mut mod_file_path_wstr = WideString::from_str(mod_file_path);
            mod_file_path_wstr.push_char('\0');
            output_lp_file_name = mod_file_path_wstr.as_ptr();
            // println!("{}", WideCStr::from_ptr_str(output_lp_file_name).to_string_lossy());
        }
    }


    CreateFileW(
        output_lp_file_name,
        dw_desired_access,
        dw_share_mode,
        lp_security_attributes,
        dw_creation_disposition,
        dw_flags_and_attributes,
        h_template_file,
    )
}
