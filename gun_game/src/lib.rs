use std::thread;

use hooks::hooks::Hooks;
use inventory::inventory::Inventory;
use winapi::um::consoleapi::AllocConsole;
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

use crate::player::player::Player;

// Extensions for external crates
mod ext;

// Pointer
mod pointer;

// Hooks
mod hooks;

// Inventory Management
mod inventory;

// PlayerIns
mod player;

// Utils
mod utils;

// Main thread
unsafe fn main_thread() {
    #[cfg(debug_assertions)]
    unsafe {AllocConsole();}
    
    // init hooks
    unsafe { Hooks::init_hooks(); }

    loop {
        // Wait for playerins
        if !Player::is_player_loaded() { continue; }

        // Weapon matchmaking level
        let weapon_matchmaking_level_addr = Inventory::get_weapon_matchmaking_level();
        println!("Weapon matchmaking level is: {}", *weapon_matchmaking_level_addr.unwrap());
    }
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