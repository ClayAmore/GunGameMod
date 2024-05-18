use std::thread;

use hooks::hooks::Hooks;
use steam::steam::Steam;
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

// Item names, weapons names, etc...
mod db;

// Extensions for external crates
mod signature;

// Pointer
mod pointer;

// Hooks
mod hooks;

// Steam
mod steam;

// Inventory Management
mod inventory;

// PlayerIns
mod player;

// GameDataMan
mod game_data_man;

// Utils
mod utils;

// Main thread
unsafe fn main_thread() {
    // #[cfg(debug_assertions)]
    unsafe {AllocConsole();}

    // init hooks
    Hooks::init_hooks();

    // register steam callbcaks
    Steam::register_callbacks();
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