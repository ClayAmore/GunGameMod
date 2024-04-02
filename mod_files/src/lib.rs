use hooks::hooks::Hooks;
use winapi::um::consoleapi::AllocConsole;
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

use std::thread;

// Extensions for external crates
mod ext;

// Config
mod config; 

// Hooks
mod hooks;

// Utils
mod utils;

// Main thread
unsafe fn main_thread() {
    #[cfg(debug_assertions)]
    AllocConsole();

    // init hooks
    Hooks::init_hooks(); 
}

// Dll entry
#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: LPVOID) -> BOOL {
if dw_reason == DLL_PROCESS_ATTACH {
    unsafe {
        DisableThreadLibraryCalls(h_module);
    }
    thread::spawn(|| {
        unsafe { main_thread() };
    });
}
TRUE
}